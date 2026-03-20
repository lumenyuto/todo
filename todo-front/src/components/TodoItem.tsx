import { type ChangeEventHandler, type FC, useState, useEffect } from 'react'
import {
  Typography,
  Button,
  Card,
  Grid,
  Modal,
  Stack,
  Box,
  Chip,
  Checkbox,
  TextField,
  FormControlLabel,
} from '@mui/material'
import { modalInnerStyle } from '../styles/modal'
import type { Label } from '../types/label'
import type { Todo, UpdateTodoPayload } from '../types/todo'
import { toggleLabels } from '../lib/toggleLabels'

type Props = {
  todo: Todo
  labels: Label[]
  onUpdate: (todo: UpdateTodoPayload) => void
  onDelete: (id: number) => void
}

export const TodoItem: FC<Props> = ({ todo, labels, onUpdate, onDelete }) => {
  const [editing, setEditing] = useState(false)
  const [editText, setEditText] = useState('')
  const [editLabels, setEditLabels] = useState<Label[]>([])

  useEffect(() => {
    setEditText(todo.text)
    setEditLabels(todo.labels)
  }, [todo, editing])

  const handleCompletedCheckbox: ChangeEventHandler = () => {
    onUpdate({
      id: todo.id,
      completed: !todo.completed,
      label_ids: todo.labels.map((label) => label.id),
    })
  }

  const handleDelete = () => onDelete(todo.id)

  const handleCancel = () => {
    setEditing(false)
  }
  const handleSave = () => {
    onUpdate({
      id: todo.id,
      text: editText,
      label_ids: editLabels.map((label) => label.id),
    })
    setEditing(false)
  }

  return (
    <Card
      sx={{
        p: { xs: 1.5, sm: 2.5 },
        mb: 2,
        borderRadius: 3,
        boxShadow: '0 2px 12px rgba(46, 125, 50, 0.08)',
        border: '1px solid rgba(46, 125, 50, 0.1)',
        transition: 'all 0.2s ease',
        '&:hover': {
          boxShadow: '0 4px 16px rgba(46, 125, 50, 0.15)',
          transform: 'translateY(-2px)',
        },
      }}
    >
      <Grid container spacing={1} alignItems="flex-start">
        <Grid size={{ xs: 2, sm: 1 }}>
          <Checkbox
            onChange={handleCompletedCheckbox}
            checked={todo.completed}
            color="success"
            sx={{ p: 0.5, '& .MuiSvgIcon-root': { fontSize: { xs: 24, sm: 28 } } }}
          />
        </Grid>

        <Grid size={{ xs: 10, sm: 8 }}>
          <Stack spacing={1}>
            <Typography
              variant="body1"
              sx={{
                fontSize: { xs: 15, sm: 16 },
                color: todo.completed ? 'text.disabled' : '#2e3d32',
                textDecoration: todo.completed ? 'line-through' : 'none',
                fontWeight: 500,
                wordBreak: 'break-word',
              }}
            >
              {todo.text}
            </Typography>
            <Stack direction="row" flexWrap="wrap" useFlexGap spacing={0.5}>
              {todo.labels?.map((label) => (
                <Chip
                  key={label.id}
                  label={label.name}
                  size="small"
                  sx={{
                    bgcolor: '#e8f5e9',
                    color: '#2e7d32',
                    fontWeight: 500,
                    fontSize: '0.7rem',
                  }}
                />
              ))}
            </Stack>
          </Stack>
        </Grid>

        <Grid size={{ xs: 12, sm: 3 }} sx={{ display: 'flex', justifyContent: 'flex-end', mt: { xs: 1, sm: 0 } }}>
          <Stack direction="row" spacing={0.5}>
            <Button
              onClick={() => {
                setEditing(true)
                setEditText(todo.text)
                setEditLabels(todo.labels)
              }}
              color="success"
              variant="text"
              size="small"
              sx={{
                borderRadius: 2,
                fontSize: { xs: '0.75rem', sm: '0.8125rem' },
                '&:hover': { bgcolor: '#e8f5e9' },
              }}
            >
              編集
            </Button>
            <Button
              onClick={handleDelete}
              size="small"
              sx={{
                color: '#9e9e9e',
                borderRadius: 2,
                fontSize: { xs: '0.75rem', sm: '0.8125rem' },
                '&:hover': { color: '#d32f2f', bgcolor: '#ffebee' },
              }}
            >
              削除
            </Button>
          </Stack>
        </Grid>
      </Grid>

      <Modal open={editing} onClose={handleCancel}>
        <Box sx={{ ...modalInnerStyle, width: { xs: '90%', sm: 400 }, maxHeight: '90vh', overflowY: 'auto' }}>
          <Stack spacing={3}>
            <Typography variant="h6" sx={{ color: '#2e7d32', fontWeight: 'bold' }}>
              タスクの編集
            </Typography>

            <TextField
              fullWidth
              variant="outlined"
              label="Todo text"
              value={editText}
              onChange={(e) => setEditText(e.target.value)}
              color="success"
              sx={{
                '& .MuiOutlinedInput-root': { borderRadius: 2 }
              }}
            />

            <Stack spacing={1}>
              <Typography variant="subtitle2" sx={{ color: 'text.secondary' }}>
                ラベル
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', pl: 1, maxHeight: 200, overflowY: 'auto' }}>
                {labels.map((label) => (
                  <FormControlLabel
                    key={label.id}
                    label={label.name}
                    control={
                      <Checkbox
                        color="success"
                        checked={editLabels.some(
                          (editLabel) => editLabel.id === label.id
                        )}
                        onChange={() =>
                          setEditLabels((prev) => toggleLabels(prev, label))
                        }
                      />
                    }
                  />
                ))}
              </Box>
            </Stack>
            <Box sx={{ display: 'flex', justifyContent: 'flex-end', pt: 2 }}>
              <Button onClick={handleCancel} color="inherit" sx={{ mr: 1 }}>
                キャンセル
              </Button>
              <Button
                onClick={handleSave}
                variant="contained"
                color="success"
                sx={{ borderRadius: 2, boxShadow: 'none' }}
              >
                保存
              </Button>
            </Box>
          </Stack>
        </Box>
      </Modal>
    </Card>
  )
}
