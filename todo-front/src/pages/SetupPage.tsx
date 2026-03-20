import { useState, type FC } from 'react'
import { useAuth0 } from '@auth0/auth0-react'
import { Box, Button, Paper, Stack, Typography } from '@mui/material'
import AddIcon from '@mui/icons-material/Add'

import { createWorkspace } from '../lib/api/workspace'

type Props = {
  userName: string
  onComplete: () => void
}

export const SetupPage: FC<Props> = ({ userName, onComplete }) => {
  const { getAccessTokenSilently } = useAuth0()
  const [loading, setLoading] = useState(false)

  const handleCreate = async () => {
    setLoading(true)
    try {
      const token = await getAccessTokenSilently()
      await createWorkspace(token, {
        name: `${userName}'s workspace`,
        is_personal: true,
      })
      onComplete()
    } finally {
      setLoading(false)
    }
  }

  return (
    <Box
      sx={{
        flex: 1,
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        p: { xs: 2, sm: 5 },
      }}
    >
      <Paper
        elevation={3}
        sx={{
          p: { xs: 4, sm: 6 },
          borderRadius: 4,
          textAlign: 'center',
          maxWidth: 480,
          width: '100%',
        }}
      >
        <Stack spacing={3} alignItems="center">
          <Typography variant="h5" sx={{ fontWeight: 'bold', color: '#2e7d32' }}>
            Workspaceを作成しましょう
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Todoを管理するために、まず個人Workspaceを作成してください。
          </Typography>
          <Button
            variant="contained"
            color="success"
            size="large"
            startIcon={<AddIcon />}
            onClick={handleCreate}
            disabled={loading}
            sx={{ borderRadius: 3, px: 4, py: 1.5 }}
          >
            {loading ? '作成中...' : '個人Workspaceを作成'}
          </Button>
        </Stack>
      </Paper>
    </Box>
  )
}
