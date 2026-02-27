import { type FC, useState } from 'react'
import type { Label } from '../types/label'
import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../types/todo'
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
import { modalInnerStyle } from '../styles/modal'
import { toggleLabels } from '../lib/toggleLabels'

type Props = {
    onSubmit: (newTodo: NewTodoPayload) => void
    labels: Label[]
}

const TodoForm: FC<Props> = ({ onSubmit, labels }) => {
    const [editText, setEditText] = useState('')
    const [editLabels, setEditLabels] = useState<Label[]>([])
    const [openLabelModal, setOpenLabelModal] = useState(false)

    const addTodoHandler = async () => {
        if (!editText) return

        onSubmit({
            text: editText,
            label_ids: editLabels.map((label) => label.id),
        })
        setEditText('')
        setEditLabels([]) // 追加後にラベルの選択状態をリセットする場合
    }

    return (
        <Paper elevation={2}>
            <Box sx={{ p: 2 }}>
                <Grid container rowSpacing={2} columnSpacing={5}>
                    <Grid size={{ xs: 12 }}>
                        <TextField
                            label="new todo text"
                            variant="filled"
                            value={editText}
                            onChange={(e) => setEditText(e.target.value)}
                            fullWidth
                        />
                    </Grid>
                    <Grid size={{ xs: 12 }}>
                        <Stack direction="row" spacing={1}>
                            {editLabels.map((label) => (
                                <Chip key={label.id} label={label.name} />
                            ))}
                        </Stack>
                    </Grid>
                    <Grid size={{ xs: 3, md: 7 }}>
                        <Button
                            onClick={() => setOpenLabelModal(true)}
                            fullWidth
                            color="secondary"
                        >
                            select label
                        </Button>
                    </Grid>
                    <Grid size={{ xs: 6 }} />
                    <Grid size={{ xs: 3 }}>
                        <Button onClick={addTodoHandler} fullWidth variant="contained">
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

export default TodoForm