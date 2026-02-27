import { useEffect, useState, type FC } from 'react'
import { Box, Stack, Typography } from '@mui/material'
import TodoList from '../components/TodoList'
import TodoForm from '../components/TodoForm'
import SideNav from '../components/SideNav'
import { useAuth } from '../router/AuthContext'
import {
    addTodoItem,
    getTodoItems,
    deleteTodoItem,
    updateTodoItem
} from '../lib/api/todo'
import {
    addLabelItem,
    deleteLabelItem,
    getLabelItems,
} from '../lib/api/label'
import type { Label, NewLabelPayload } from '../types/label'
import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../types/todo'

const HomePage: FC = () => {
    const { authUser } = useAuth()
    const [todos, setTodos] = useState<Todo[]>([])
    const [labels, setLabels] = useState<Label[]>([])
    const [filterLabelId, setFilterLabelId] = useState<number | null>(null)

    const onSubmit = async (payload: NewTodoPayload) => {
        if (!payload.text) return
        await addTodoItem(payload)
        const todos = await getTodoItems()
        setTodos(todos)
    }

    const onUpdate = async (updateTodo: UpdateTodoPayload) => {
        await updateTodoItem(updateTodo)
        const todos = await getTodoItems()
        setTodos(todos)
    }

    const onDelete = async (id: number) => {
        await deleteTodoItem(id)
        const todos = await getTodoItems()
        setTodos(todos)
    }

    const onSelectLabel = (label: Label | null) => {
        setFilterLabelId(label?.id ?? null)
    }

    const onSubmitNewLabel = async (newLabel: NewLabelPayload) => {
        if (!labels.some((label) => label.name === newLabel.name)) {
            const res = await addLabelItem(newLabel)
            setLabels([...labels, res])
        }
    }

    const onDeleteLabel = async (id: number) => {
        await deleteLabelItem(id)
        setLabels((prev) => prev.filter((label) => label.id !== id))
    }

    const dispTodo = filterLabelId
        ? todos.filter((todo) =>
            todo.labels.some((label) => label.id === filterLabelId)
        )
        : todos

    useEffect(() => {
        ;(async () => {
            const todos = await getTodoItems()
            setTodos(todos)
            const labelResponse = await getLabelItems()
            setLabels(labelResponse)
        })()
    }, [])

    return (
        <>
            <Box
                sx={{
                    backgroundColor: 'white',
                    borderBottom: '1px solid gray',
                    display: 'flex',
                    alignItems: 'center',
                    position: 'fixed',
                    top: 0,
                    p: 2,
                    width: '100%',
                    height: 80,
                    zIndex: 3,
                }}
            >
                <Typography variant="h1">Todo App</Typography>
            </Box>
            <Box
                sx={{
                    backgroundColor: 'white',
                    borderRight: '1px solid gray',
                    position: 'fixed',
                    top: 80,
                    height: 'calc(100% - 80px)',
                    width: 200,
                    zIndex: 2,
                    left: 0,
                }}
            >
                <SideNav
                    labels={labels}
                    onSelectLabel={onSelectLabel}
                    filterLabelId={filterLabelId}
                    onSubmitNewLabel={onSubmitNewLabel}
                    onDeleteLabel={onDeleteLabel}
                />
            </Box>
            <Box
                sx={{
                    display: 'flex',
                    justifyContent: 'center',
                    p: 5,
                    mt: 10,
                    ml: '200px'
                }}
            >
                <Box maxWidth={700} width="100%">
                    <Stack spacing={5}>
                        <TodoForm onSubmit={onSubmit} labels={labels} />
                        <TodoList
                            todos={dispTodo}
                            labels={labels}
                            onUpdate={onUpdate}
                            onDelete={onDelete}
                        />
                    </Stack>
                </Box>
            </Box>
        </>
    )
}

export default HomePage