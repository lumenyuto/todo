import type { FC } from 'react'
import { Box, CircularProgress, Typography, Stack } from '@mui/material'

export const LoadingPage: FC = () => {
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        width: '100vw',
        background: 'linear-gradient(135deg, #e8f5e9 0%, #ffffff 50%, #f1f8e9 100%)',
      }}
    >
      <Stack spacing={3} alignItems="center">
        <CircularProgress
          size={48}
          thickness={4}
          sx={{ color: '#2e7d32' }}
        />
        <Typography
          variant="body1"
          sx={{ color: '#4caf50', fontWeight: 500, letterSpacing: 1 }}
        >
          Loading...
        </Typography>
      </Stack>
    </Box>
  )
}
