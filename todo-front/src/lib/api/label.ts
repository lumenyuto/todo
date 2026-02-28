import type { Label, NewLabelPayload } from '../../types/label'

export const getLabelItems = async (user_id: number) => {
    const res = await fetch(`http://localhost:3000/labels?user_id=${user_id}`)
    if (!res.ok) {
        throw new Error('get label request failed')
    }
    const json: Label[] = await res.json()

    return json
}

export const addLabelItem = async (user_id: number, payload: NewLabelPayload) => {
    const res = await fetch(`http://localhost:3000/labels?user_id=${user_id}`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    })
    if (!res.ok) {
        throw new Error('add label request failed')
    }
    const json: Label = await res.json()
    return json
}

export const deleteLabelItem = async (id: number, user_id: number) => {
    const res = await fetch(`http://localhost:3000/labels/${id}?user_id=${user_id}`, {
        method: 'DELETE',
    })
    if (!res.ok) {
        throw new Error('delete label request failed')
    }
}