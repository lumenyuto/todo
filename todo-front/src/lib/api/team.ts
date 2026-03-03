import type { Team, NewTeamPayload } from '../../types/team'
import type { Todo, NewTodoPayload } from '../../types/todo'

const API_URL = import.meta.env.VITE_API_URL

export const getTeamItems = async (token: string) => {
  const res = await fetch(`${API_URL}/teams`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('get teams request failed')
  }
  const json: Team[] = await res.json()
  return json
}

export const createTeamItem = async (token: string, payload: NewTeamPayload) => {
  const res = await fetch(`${API_URL}/teams`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('create team request failed')
  }
  const json: Team = await res.json()
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

export const addTeamTodoItem = async (token: string, teamId: number, payload: NewTodoPayload) => {
  const res = await fetch(`${API_URL}/teams/${teamId}/todos`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('create team todo request failed')
  }
  const json: Todo = await res.json()
  return json
}
