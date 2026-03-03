import type { Label, NewLabelPayload } from '../../types/label'

const API_URL = import.meta.env.VITE_API_URL

export const addLabelItem = async (token: string, payload: NewLabelPayload) => {
  const res = await fetch(`${API_URL}/labels`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok)
    throw new Error('add label request failed')
  const json: Label = await res.json()
  return json
}

export const getLabelItems = async (token: string) => {
  const res = await fetch(`${API_URL}/labels`, {
    headers: {
      Authorization: `Bearer ${token}`
    },
  })
  if (!res.ok)
    throw new Error('get label request failed')
  const json: Label[] = await res.json()
  return json
}

export const deleteLabelItem = async (token: string, id: number) => {
  const res = await fetch(`${API_URL}/labels/${id}`, {
    method: 'DELETE',
    headers: {
      Authorization: `Bearer ${token}`
    },
  })
  if (!res.ok)
    throw new Error('delete label request failed')
}