import type { FC, ReactNode } from 'react'
import { Navigate, Route, Routes } from 'react-router-dom'
import HomePage from '../pages/HomePage'
import { LandingPage } from '../pages/LandingPage'
import { SigninPage } from '../pages/SigninPage'
import { SignupPage } from '../pages/SignupPage'
import { useAuth } from './AuthContext'

const PrivateRouter: FC<{children: ReactNode}> = ({ children }) => {
    const { authUser } = useAuth()
    if (!authUser) {
        return <Navigate to="/signin" replace />
    }
    return <>{children}</>
}

const GuestRouter: FC<{children: ReactNode}> = ({ children }) => {
    const { authUser } = useAuth()
    if (authUser) {
        return <Navigate to="/" replace />
    }
    return <>{children}</>
}

const Router = () => {
    const { authUser } = useAuth()
    
    return (
        <Routes>
            <Route 
                path="/"
                element={
                    authUser ? <HomePage /> : <LandingPage />
                }
            />

            <Route
                path="/signin"
                element={
                    <GuestRouter>
                        <SigninPage />
                    </GuestRouter>
                }
            />
            
            <Route
                path="/signup"
                element={
                    <GuestRouter>
                        <SignupPage />
                    </GuestRouter>
                }
            />
            <Route path="*" element={<>PAGE NOT FOUND 404</>} />
        </Routes>
    )
}

export default Router