import { Route, Routes } from 'react-router-dom'
import { useAuth0 } from '@auth0/auth0-react'
import { HomePage } from '../pages/HomePage'
import { LandingPage } from '../pages/LandingPage'

const Router = () => {
  const { isAuthenticated, isLoading } = useAuth0()

  if (isLoading) return <div>Loading</div>

  return (
    <Routes>
      <Route
        path="/"
        element={isAuthenticated ? <HomePage /> : <LandingPage />}
      />
      <Route path="*" element={<>PAGE NOT FOUND 404</>} />
    </Routes>
  )
}

export default Router