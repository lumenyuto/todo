import type { User } from './user'

export type Workspace = {
  id: number
  name: string
  is_personal: boolean
  users: User[]
}

export type NewWorkspacePayload = {
  name: string
  is_personal?: boolean
  user_emails?: string[]
}
