import type { Label } from './label'

export type Todo = {
  id: number
  text: string
  completed: boolean
  labels: Label[]
  user_id: number
  workspace_id: number
}

export type NewTodoPayload = {
  text: string
  label_ids: number[]
}

export type RecommendedTodo = {
  text: string
}

export type UpdateTodoPayload = {
  id: number
  text?: string
  completed?: boolean
  label_ids?: number[]
}
