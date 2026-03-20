import { type FC, useState } from 'react'
import {
  Box,
  Button,
  Checkbox,
  CircularProgress,
  FormControlLabel,
  Modal,
  Stack,
  Typography,
} from '@mui/material'
import AutoAwesomeIcon from '@mui/icons-material/AutoAwesome'
import { modalInnerStyle } from '../styles/modal'
import type { RecommendedTodo } from '../types/todo'

type Props = {
  onRecommend: () => Promise<RecommendedTodo[]>
  onAddTodos: (texts: string[]) => void
}

export const RecommendButton: FC<Props> = ({ onRecommend, onAddTodos }) => {
  const [loading, setLoading] = useState(false)
  const [open, setOpen] = useState(false)
  const [recommendations, setRecommendations] = useState<RecommendedTodo[]>([])
  const [selected, setSelected] = useState<Set<number>>(new Set())
  const [error, setError] = useState<string | null>(null)

  const handleClick = async () => {
    setLoading(true)
    setError(null)
    try {
      const recs = await onRecommend()
      setRecommendations(recs)
      setSelected(new Set())
      setOpen(true)
    } catch {
      setError('レコメンドの取得に失敗しました')
    } finally {
      setLoading(false)
    }
  }

  const toggleSelect = (index: number) => {
    setSelected((prev) => {
      const next = new Set(prev)
      if (next.has(index)) {
        next.delete(index)
      } else {
        next.add(index)
      }
      return next
    })
  }

  const handleAdd = () => {
    const texts = recommendations
      .filter((_, i) => selected.has(i))
      .map((r) => r.text)
    if (texts.length > 0) {
      onAddTodos(texts)
    }
    setOpen(false)
  }

  return (
    <>
      <Button
        onClick={handleClick}
        variant="outlined"
        color="secondary"
        disabled={loading}
        startIcon={loading ? <CircularProgress size={18} /> : <AutoAwesomeIcon />}
        fullWidth
      >
        {loading ? 'AI分析中...' : 'AIレコメンド'}
      </Button>

      {error && (
        <Typography color="error" variant="body2" sx={{ mt: 1 }}>
          {error}
        </Typography>
      )}

      <Modal open={open} onClose={() => setOpen(false)}>
        <Box sx={modalInnerStyle}>
          <Typography variant="h6" sx={{ mb: 2 }}>
            AIおすすめタスク
          </Typography>
          {recommendations.length === 0 ? (
            <Typography color="text.secondary">
              レコメンドがありません
            </Typography>
          ) : (
            <Stack spacing={1}>
              {recommendations.map((rec, i) => (
                <FormControlLabel
                  key={i}
                  control={
                    <Checkbox
                      checked={selected.has(i)}
                      onChange={() => toggleSelect(i)}
                      color="secondary"
                    />
                  }
                  label={rec.text}
                />
              ))}
            </Stack>
          )}
          <Stack direction="row" spacing={2} sx={{ mt: 3 }} justifyContent="flex-end">
            <Button onClick={() => setOpen(false)} color="inherit">
              キャンセル
            </Button>
            <Button
              onClick={handleAdd}
              variant="contained"
              color="secondary"
              disabled={selected.size === 0}
            >
              選択したタスクを追加
            </Button>
          </Stack>
        </Box>
      </Modal>
    </>
  )
}
