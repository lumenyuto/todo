use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
}

#[derive(Debug, Serialize)]
struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Option<Vec<GroqChoice>>,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: GroqMessageResponse,
}

#[derive(Debug, Deserialize)]
struct GroqMessageResponse {
    content: String,
}

pub async fn recommend_todos(
    api_key: &str,
    existing_todos: &[String],
) -> anyhow::Result<Vec<String>> {
    let todo_list = if existing_todos.is_empty() {
        "（まだタスクがありません）".to_string()
    } else {
        existing_todos
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {}", i + 1, t))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let prompt = format!(
        "以下はユーザーの既存のTodoリストです:\n\n\
         {}\n\n\
         上記のタスクを踏まえて、ユーザーに役立ちそうな新しいタスクを3つ提案してください。\n\
         各タスクは簡潔に1行で記述してください。\n\
         回答は以下のJSON配列形式のみで返してください。説明文は不要です:\n\
         [\"タスク1\", \"タスク2\", \"タスク3\"]",
        todo_list
    );

    let request_body = GroqRequest {
        model: "llama-3.3-70b-versatile".to_string(),
        messages: vec![
            GroqMessage {
                role: "system".to_string(),
                content: "あなたはタスク管理のアシスタントです。JSON配列のみで回答してください。".to_string(),
            },
            GroqMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::error!("Groq API error: {} - {}", status, body);
        anyhow::bail!("Groq API error: {} - {}", status, body);
    }

    let groq_response: GroqResponse = response.json().await?;

    let text = groq_response
        .choices
        .and_then(|c| c.into_iter().next())
        .map(|c| c.message.content)
        .unwrap_or_default();

    // Extract JSON array from response (may be wrapped in ```json ... ```)
    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let recommendations: Vec<String> = serde_json::from_str(json_str)
        .unwrap_or_else(|_| {
            // Fallback: split by newlines if JSON parsing fails
            text.lines()
                .map(|l| l.trim().trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ').to_string())
                .filter(|l| !l.is_empty())
                .take(3)
                .collect()
        });

    Ok(recommendations)
}
