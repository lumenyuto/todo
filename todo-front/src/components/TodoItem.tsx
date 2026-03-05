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
  teamId: number | null
  onUpdate: (todo: UpdateTodoPayload) => void
  onDelete: (id: number) => void
}

export const TodoItem: FC<Props> = ({ todo, labels, teamId, onUpdate, onDelete }) => {
  const [editing, setEditing] = useState(false)
  const [editText, setEditText] = useState('')
  const [editLabels, setEditLabels] = useState<Label[]>([])

  useEffect(() => {
    setEditText(todo.text)
    setEditLabels(todo.labels)
  }, [todo, editing])

  const handleCompletedCheckbox: ChangeEventHandler = () => {
    onUpdate({
      team_id: teamId,
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
      team_id: teamId,
      id: todo.id,
      text: editText,
      label_ids: editLabels.map((label) => label.id),
    })
    setEditing(false)
  }

  return (
    <Card 
      sx={{
        p: 2.5,
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
      <Grid container spacing={2} alignItems="center">
        <Grid size = {{xs: 1}}>
          <Checkbox
            onChange={handleCompletedCheckbox}
            checked={todo.completed}
            color="success" // チェックボックスを緑色に
            sx={{ '& .MuiSvgIcon-root': { fontSize: 28 } }} // 少し大きめにして押しやすく
          />
        </Grid>
        <Grid size = {{xs: 8}}>
            <Stack spacing={1}>
              <Typography
                variant="body1"
                fontSize={16}
                sx={{
                  color: todo.completed ? 'text.disabled' : '#2e3d32', // 完了時はグレー、通常時は深い緑みを帯びた黒
                  textDecoration: todo.completed ? 'line-through' : 'none',
                  fontWeight: 500,
                }}
              >
                {todo.text}
              </Typography>
              <Stack direction="row" spacing={1}>
                {todo.labels?.map((label) => (
                  <Chip
                    key={label.id}
                    label={label.name}
                    size="small"
                    sx={{
                      bgcolor: '#e8f5e9', // 非常に薄い緑色の背景
                      color: '#2e7d32', // 文字は濃い緑
                      fontWeight: 500,
                      border: 'none',
                    }}
                  />
                ))}
              </Stack>
            </Stack>
        </Grid>
        <Grid size={{ xs: 3 }} sx={{ display: 'flex', justifyContent: 'flex-end' }}>
          <Stack direction="row" spacing={1}>
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
                '&:hover': { bgcolor: '#e8f5e9' },
              }}
            >
              編集
            </Button>
            <Button
              onClick={handleDelete}
              size="small"
              sx={{
                color: '#9e9e9e', // 普段は目立たないグレー
                borderRadius: 2,
                '&:hover': { color: '#d32f2f', bgcolor: '#ffebee' }, // ホバー時だけ赤くして誤操作防止
              }}
            >
              削除
            </Button>
          </Stack>
        </Grid>
      </Grid>
      <Modal open={editing} onClose={handleCancel}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Typography variant="h6" sx={{ color: '#2e7d32', fontWeight: 'bold' }}>
              タスクの編集
            </Typography>
            
            <TextField
              fullWidth
              variant="outlined"
              label="Todo text"
              defaultValue={todo.text}
              onChange={(e) => setEditText(e.target.value)}
              color="success" // フォーカス時の枠線を緑に
              sx={{
                '& .MuiOutlinedInput-root': { borderRadius: 2 }
              }}
            />
            
            <Stack spacing={1}>
              <Typography variant="subtitle2" sx={{ color: 'text.secondary' }}>
                ラベル
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', pl: 1 }}>
                {labels.map((label) => (
                  <FormControlLabel
                    key={label.id}
                    label={label.name}
                    control={
                      <Checkbox
                        color="success"
                        // ★ 修正1: defaultChecked ではなく checked を使う
                        // ★ 修正2: todo.labels ではなく、現在の state である editLabels の中身を見る
                        checked={editLabels.some(
                          (editLabel) => editLabel.id === label.id
                        )}
                        // ★ 修正3: onChange を Checkbox の中に移動
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