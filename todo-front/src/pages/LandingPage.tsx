import type { FC } from 'react'
import { Box, Typography, Button } from '@mui/material'
import { useAuth0 } from '@auth0/auth0-react'

export const LandingPage: FC = () => {
  const { loginWithRedirect } = useAuth0()

  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        width: '100vw',
        gap: 4,
      }}
    >
      <Typography variant="h1">
        Welcome to My App!
      </Typography>
      
      <Button 
        variant="contained" 
        size="large" 
        onClick={() => loginWithRedirect()}
      >
        サインイン
      </Button>
      <Button
        variant="contained"
        size="large"
        onClick={() => loginWithRedirect({
          authorizationParams: {
            screen_hint: 'signup',
          }
        })}
      >
        新規登録
      </Button>
    </Box>
  )
}