import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../../types/todo'

const API_URL = import.meta.env.VITE_API_URL

export const addTodoItem = async (token: string, payload: NewTodoPayload) => {
  const { team_id, ...addTodo } = payload
  const res = await fetch(`${API_URL}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(addTodo),
  })
  if (!res.ok) {
    throw new Error('add todo request failed')
  }
  const json: Todo = await res.json()
  return json
}

export const addTeamTodoItem = async (token: string, payload: NewTodoPayload) => {
  const { team_id, ...addTodo } = payload
  const res = await fetch(`${API_URL}/teams/${team_id}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(addTodo),
  })
  if (!res.ok) {
    throw new Error('create team todo request failed')
  }
  const json: Todo = await res.json()
  return json
}

export const getTodoItems = async (token: string) => {
  const res = await fetch(`${API_URL}/todos`, {
    headers: {
      Authorization: `Bearer ${token}`
    },
  })
  if (!res.ok) {
    throw new Error('get todos request failed')
  }

  const json: Todo[] = await res.json()
  return json
}

export const getTeamTodoItems = async (token: string, teamId: number) => {
  const res = await fetch(`${API_URL}/teams/${teamId}/todos`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('get team todos request failed')
  }
  const json: Todo[] = await res.json()
  return json
}

export const updateTodoItem = async (token: string, payload: UpdateTodoPayload) => {
  const { id, ...updateTodo } = payload
  const res = await fetch(`${API_URL}/todos/${id}`, {
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
  const json : Todo = await res.json()
  return json
}

export const updateTeamTodoItem = async (token: string, payload: UpdateTodoPayload) => {
  const { team_id, id, ...updateTodo } = payload
  const res = await fetch(`${API_URL}/teams/${team_id}/todos/${id}`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(updateTodo),
  })
  if (!res.ok) {
    throw new Error('get team todos request failed')
  }
  const json: Todo[] = await res.json()
  return json
}

export const deleteTodoItem = async (token: string, id: number) => {
  const res = await fetch(`${API_URL}/todos/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${token}`
    },
  })
  if (!res.ok) {
    throw new Error('delete todo request failed')
  }
}