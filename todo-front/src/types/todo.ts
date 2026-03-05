import type { Label } from './label' 

export type Todo = {
  id: number
  text: string
  completed: boolean
  labels: Label[]
  user_id: number
  team_id: number | null
}

export type NewTodoPayload = {
  team_id: number | null
  text: string
  label_ids: number[]
}

export type UpdateTodoPayload = {
  team_id: number | null
  id: number
  text?: string
  completed?: boolean
  label_ids?: number[]
}