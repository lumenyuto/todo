export type User = {
  sub: string | null
  name: string
  email: string | null
}

export type NewUserPayload = {
  sub: string
  name: string
  email: string
}

export type UpdateUserPayload = {
  name: string
}