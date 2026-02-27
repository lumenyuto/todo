import { createContext, useContext, useState, type FC, type ReactNode } from 'react'
import type { User } from '../types/user'

type AuthUser = {
    id: number
    name: string
}

type AuthContextType = {
    authUser: AuthUser | null
    login: (user: User) => void
    logout: () => void
}

const AuthContext = createContext<AuthContextType | null>(null)

export const AuthProvider: FC<{ children: ReactNode }> = ({ children }) => {
    const [authUser, setAuthUser] = useState<AuthUser | null>(() => {
        const saved = localStorage.getItem('authUser')
        if (saved) {
            return JSON.parse(saved)
        }
        return null
    })

    const login = (user: User) => {
        setAuthUser(user)
        localStorage.setItem('authUser', JSON.stringify(user))
    }

    const logout = () => {
        setAuthUser(null)
        localStorage.removeItem('authUser')
    }

    return (
        <AuthContext.Provider value={{ authUser, login, logout }}>
            {children}
        </AuthContext.Provider>
    )
}

export const useAuth = () => {
    const ctx = useContext(AuthContext)
    if (!ctx) throw new Error('useAuth must be used within AuthProvider')
    return ctx
}