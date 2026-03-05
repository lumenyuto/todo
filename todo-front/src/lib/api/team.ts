import type { Team, NewTeamPayload } from '../../types/team'

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
