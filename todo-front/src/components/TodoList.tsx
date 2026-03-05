import { type FC } from 'react'
import type { Label } from '../types/label'
import type { Todo , UpdateTodoPayload } from '../types/todo'
import { Stack, Typography } from '@mui/material'
import { TodoItem }from './TodoItem'

type Props = {
  todos: Todo[]
  labels: Label[]
  teamId: number | null
  onUpdate: (todo: UpdateTodoPayload) => void
  onDelete: (id: number) => void
}

export const TodoList: FC<Props> = ({ todos, labels, teamId, onUpdate, onDelete }) => {
  return (
    <Stack spacing={2}>
      <Typography variant="h2">todo list </Typography>
      <Stack spacing={2}>
        {todos.map((todo) => (
          <TodoItem
            key={todo.id}
            todo={todo}
            labels={labels}
            teamId={teamId}
            onUpdate={onUpdate}
            onDelete={onDelete}  
          />
        ))}
      </Stack>
    </Stack>
  )
}