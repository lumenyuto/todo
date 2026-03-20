import type { NewTodoPayload, RecommendedTodo, Todo, UpdateTodoPayload } from '../../types/todo'

const API_URL = import.meta.env.VITE_API_URL

export const addTodoItem = async (token: string, workspaceId: number, payload: NewTodoPayload) => {
  const res = await fetch(`${API_URL}/workspaces/${workspaceId}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('add todo request failed')
  }
  const json: Todo = await res.json()
  return json
}

export const getTodoItems = async (token: string, workspaceId: number) => {
  const res = await fetch(`${API_URL}/workspaces/${workspaceId}/todos`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('get todos request failed')
  }
  const json: Todo[] = await res.json()
  return json
}

export const updateTodoItem = async (token: string, workspaceId: number, payload: UpdateTodoPayload) => {
  const { id, ...updateTodo } = payload
  const res = await fetch(`${API_URL}/workspaces/${workspaceId}/todos/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(updateTodo),
  })
  if (!res.ok) {
    throw new Error('update todo request failed')
  }
  const json: Todo = await res.json()
  return json
}

export const deleteTodoItem = async (token: string, workspaceId: number, id: number) => {
  const res = await fetch(`${API_URL}/workspaces/${workspaceId}/todos/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('delete todo request failed')
  }
}

export const getRecommendations = async (token: string, workspaceId: number): Promise<RecommendedTodo[]> => {
  const res = await fetch(`${API_URL}/workspaces/${workspaceId}/todos/recommend`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('get recommendations request failed')
  }
  const json: RecommendedTodo[] = await res.json()
  return json
}
