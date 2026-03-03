import type { FC, ReactNode } from 'react'
import { useAuth0 } from '@auth0/auth0-react'

export const PrivateRoute: FC<{children: ReactNode}> = ({children}) => {
  const { isAuthenticated, isLoading, loginWithRedirect } = useAuth0()

  if (isAuthenticated) {
    loginWithRedirect()
    return null
  }
  return <>{children}</>
}