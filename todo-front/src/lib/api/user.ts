import type { NewUserPayload, User } from'../../types/user'

export const addUserItem = async (payload: NewUserPayload) => {
    const res = await fetch('http://localhost:3000/users', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    })
    if (!res.ok) {
        throw new Error('add user requst failed')
    }
    const json: User = await res.json()
    return json
}

export const getUserItems = async () => {
    const res = await fetch('http://localhost:3000/users')
    if (!res.ok) {
        throw new Error('get uses request failed')
    }

    const json: User[] = await res.json()
    return json
}

export const getUserItem = async (username: string) => {
    const res = await fetch(`http://localhost:3000/users/${username}`)
    if (!res.ok) {
        throw new Error('get user request failed')
    }

    const json: User = await res.json()
    return json
}