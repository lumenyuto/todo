import type { NewUserPayload, UpdateUserPayload, User } from '../../types/user'

const API_URL = import.meta.env.VITE_API_URL

export const addUserItem = async (token: string, payload: NewUserPayload) => {
  const res = await fetch(`${API_URL}/users`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('create user request failed')
  }
  const json: User = await res.json()
  return json
}

export const updateUserName = async (token: string, payload: UpdateUserPayload) => {
  const res = await fetch(`${API_URL}/users/me`, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('update user request failed')
  }
  const json: User = await res.json()
  return json
}