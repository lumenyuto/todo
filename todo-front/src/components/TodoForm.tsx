import { type FC, useState } from 'react'
import type { Label } from '../types/label'
import type { NewTodoPayload } from '../types/todo'
import {
  Box,
  Button,
  TextField,
  FormControlLabel,
  Checkbox,
  Stack,
  Paper,
  Modal,
  Grid,
  Chip,
} from '@mui/material'
import { toggleLabels } from '../lib/toggleLabels'
import { modalInnerStyle } from '../styles/modal'

type Props = {
  onSubmit: (newTodo: NewTodoPayload) => void
  labels: Label[]
  teamId: number | null
}

export const TodoForm: FC<Props> = ({ onSubmit, labels, teamId }) => {
  const [editText, setEditText] = useState('')
  const [editLabels, setEditLabels] = useState<Label[]>([])
  const [openLabelModal, setOpenLabelModal] = useState(false)

  const addTodoHandler = async () => {
    if (!editText) return

    onSubmit({
      team_id: teamId,
      text: editText,
      label_ids: editLabels.map((label) => label.id),
    })
    setEditText('')
    setEditLabels([])
  }

  return (
    <Paper elevation={2}>
      <Box sx={{ p: { xs: 2, sm: 3 } }}>
        <Grid container rowSpacing={2} columnSpacing={2}>
          <Grid size={{ xs: 12 }}>
            <TextField
              label="new todo text"
              variant="filled"
              value={editText}
              onChange={(e) => setEditText(e.target.value)}
              fullWidth
              color="success"
            />
          </Grid>

          {editLabels.length > 0 && (
            <Grid size={{ xs: 12 }}>
              <Stack direction="row" flexWrap="wrap" useFlexGap spacing={1}>
                {editLabels.map((label) => (
                  <Chip
                    key={label.id}
                    label={label.name}
                    color="success"
                    variant="outlined"
                  />
                ))}
              </Stack>
            </Grid>
          )}

          <Grid size={{ xs: 12, sm: 6 }}>
            <Button
              onClick={() => setOpenLabelModal(true)}
              fullWidth
              variant="outlined"
              color="success"
            >
              select label
            </Button>
          </Grid>
          
          <Grid size={{ xs: 12, sm: 6 }}>
            <Button
              onClick={addTodoHandler}
              fullWidth
              variant="contained"
              color="success"
            >
              add todo
            </Button>
          </Grid>

          <Modal open={openLabelModal} onClose={() => setOpenLabelModal(false)}>
            <Box sx={modalInnerStyle}>
              <Stack>
                {labels.map((label) => (
                  <FormControlLabel
                    key={label.id}
                    control={
                      <Checkbox
                        checked={editLabels.some((l) => l.id === label.id)}
                        color="success"
                      />
                    }
                    label={label.name}
                    onChange={() =>
                      setEditLabels((prev) => toggleLabels(prev, label))
                    }
                  />
                ))}
              </Stack>
            </Box>
          </Modal>
        </Grid>
      </Box>
    </Paper>
  )
}