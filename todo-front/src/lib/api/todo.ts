import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../../types/todo'

export const addTodoItem = async (user_id: number, payload: NewTodoPayload) => {
    const res = await fetch(`http://localhost:3000/todos?user_id=${user_id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    })
    if (!res.ok) {
        throw new Error('add todo request failed')
    }
    const json: Todo = await res.json()
    return json
}

export const getTodoItems = async (user_id: number) => {
    const res = await fetch(`http://localhost:3000/todos?user_id=${user_id}`)
    if (!res.ok) {
        throw new Error('get todos request failed')
    }

    const json: Todo[] = await res.json()
    return json
}

export const updateTodoItem = async (user_id: number, payload: UpdateTodoPayload) => {
    const { id, ...updateTodo } = payload
    const res = await fetch(`http://localhost:3000/todos/${id}?user_id=${user_id}`, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(updateTodo),
    })
    if (!res.ok) {
        throw new Error('update todo request failed')
    }
    const json : Todo = await res.json()
    return json
}

export const deleteTodoItem = async (id: number, user_id: number) => {
    const res = await fetch(`http://localhost:3000/todos/${id}?user_id=${user_id}`, {
        method: 'DELETE',
    })
    if (!res.ok) {
        throw new Error('delete todo request failed')
    }
}