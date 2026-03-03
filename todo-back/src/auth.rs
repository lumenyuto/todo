use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iss: String,
    pub aud: serde_json::Value,
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub sub: String,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

static JWKS_CACHE: OnceLock<RwLock<Vec<Jwk>>> = OnceLock::new();

fn get_jwks_cache() -> &'static RwLock<Vec<Jwk>> {
    JWKS_CACHE.get_or_init(|| RwLock::new(Vec::new()))
}

async fn fetch_jwks(domain: &str) -> Result<Vec<Jwk>, StatusCode> {
    let url = format!("https://{}/.well-known/jwks.json", domain);
    let response = reqwest::get(&url)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let jwks: JwksResponse = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(jwks.keys)
}

async fn get_decoding_key(domain: &str, kid: &str) -> Result<DecodingKey, StatusCode> {
    let cache = get_jwks_cache();

    {
        let keys = cache.read().await;
        if let Some(jwk) = keys.iter().find(|k| k.kid == kid) {
            return DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    let keys = fetch_jwks(domain).await?;
    let jwk = keys
        .iter()
        .find(|k| k.kid == kid)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    {
        let mut cached = cache.write().await;
        *cached = keys;
    }

    Ok(decoding_key)
}

impl<S> FromRequestParts<S> for AuthenticatedUser 
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        #[cfg(test)]
        {
            if let Some(test_sub) = parts
                .headers
                .get("X-Test-Sub")
                .and_then(|v| v.to_str().ok())
            {
                return Ok(AuthenticatedUser {
                    sub: test_sub.to_string(),
                });
            }
        }

        let domain = env::var("AUTH0_DOMAIN")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let audience = env::var("AUTH0_AUDIENCE")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let header = decode_header(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let kid = header.kid.ok_or(StatusCode::UNAUTHORIZED)?;

        let decoding_key = get_decoding_key(&domain, &kid).await?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&audience]);
        validation.set_issuer(&[format!("https://{}/", domain)]);

        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser {
            sub: token_data.claims.sub,
        })
    }
}