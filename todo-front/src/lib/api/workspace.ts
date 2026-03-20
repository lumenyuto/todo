import type { Workspace, NewWorkspacePayload } from '../../types/workspace'

const API_URL = import.meta.env.VITE_API_URL

export const getWorkspaces = async (token: string) => {
  const res = await fetch(`${API_URL}/workspaces`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  if (!res.ok) {
    throw new Error('get workspaces request failed')
  }
  const json: Workspace[] = await res.json()
  return json
}

export const createWorkspace = async (token: string, payload: NewWorkspacePayload) => {
  const res = await fetch(`${API_URL}/workspaces`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(payload),
  })
  if (!res.ok) {
    throw new Error('create workspace request failed')
  }
  const json: Workspace = await res.json()
  return json
}
