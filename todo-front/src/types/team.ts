import type { User } from './user'

export type Team = {
  id: number
  name: string
  users: User[]
}

export type NewTeamPayload = {
  name: string
  user_ids: number[]
}
