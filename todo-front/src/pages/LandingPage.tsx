import type { FC } from 'react'
import { Box, Typography, Button, Stack, Paper } from '@mui/material'
import { useAuth0 } from '@auth0/auth0-react'
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline'
import GroupWorkIcon from '@mui/icons-material/GroupWork'
import AutoAwesomeIcon from '@mui/icons-material/AutoAwesome'

export const LandingPage: FC = () => {
  const { loginWithRedirect } = useAuth0()

  return (
    <Box
      sx={{
        minHeight: '100vh',
        width: '100vw',
        background: 'linear-gradient(135deg, #e8f5e9 0%, #ffffff 50%, #f1f8e9 100%)',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      {/* Hero Section */}
      <Box
        sx={{
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          px: { xs: 3, sm: 6 },
          py: { xs: 6, sm: 10 },
          textAlign: 'center',
        }}
      >
        <Typography
          variant="h1"
          sx={{
            fontWeight: 800,
            fontSize: { xs: 36, sm: 48 },
            color: '#1b5e20',
            mb: 2,
          }}
        >
          Todo App
        </Typography>
        <Typography
          variant="h6"
          sx={{
            color: '#546e7a',
            fontWeight: 400,
            maxWidth: 500,
            mb: 5,
            lineHeight: 1.7,
          }}
        >
          タスク管理をもっとシンプルに。
          <br />
          チームでもソロでも、あなたの生産性を高めます。
        </Typography>

        <Stack
          direction={{ xs: 'column', sm: 'row' }}
          spacing={2}
          sx={{ mb: 8 }}
        >
          <Button
            variant="contained"
            size="large"
            onClick={() => loginWithRedirect()}
            sx={{
              borderRadius: 3,
              px: 5,
              py: 1.5,
              fontSize: 16,
              fontWeight: 600,
              bgcolor: '#2e7d32',
              '&:hover': { bgcolor: '#1b5e20' },
              textTransform: 'none',
            }}
          >
            サインイン
          </Button>
          <Button
            variant="outlined"
            size="large"
            onClick={() =>
              loginWithRedirect({
                authorizationParams: {
                  screen_hint: 'signup',
                },
              })
            }
            sx={{
              borderRadius: 3,
              px: 5,
              py: 1.5,
              fontSize: 16,
              fontWeight: 600,
              borderColor: '#2e7d32',
              color: '#2e7d32',
              '&:hover': {
                borderColor: '#1b5e20',
                bgcolor: 'rgba(46, 125, 50, 0.04)',
              },
              textTransform: 'none',
            }}
          >
            新規登録
          </Button>
        </Stack>

        {/* Feature Cards */}
        <Stack
          direction={{ xs: 'column', sm: 'row' }}
          spacing={3}
          sx={{ maxWidth: 800, width: '100%' }}
        >
          <Paper
            elevation={0}
            sx={{
              flex: 1,
              p: 3,
              borderRadius: 3,
              textAlign: 'center',
              bgcolor: 'rgba(255, 255, 255, 0.8)',
              border: '1px solid #e0e0e0',
            }}
          >
            <CheckCircleOutlineIcon sx={{ fontSize: 40, color: '#4caf50', mb: 1 }} />
            <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 0.5 }}>
              シンプル管理
            </Typography>
            <Typography variant="body2" color="text.secondary">
              直感的なUIでタスクを素早く作成・完了
            </Typography>
          </Paper>
          <Paper
            elevation={0}
            sx={{
              flex: 1,
              p: 3,
              borderRadius: 3,
              textAlign: 'center',
              bgcolor: 'rgba(255, 255, 255, 0.8)',
              border: '1px solid #e0e0e0',
            }}
          >
            <GroupWorkIcon sx={{ fontSize: 40, color: '#4caf50', mb: 1 }} />
            <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 0.5 }}>
              チーム対応
            </Typography>
            <Typography variant="body2" color="text.secondary">
              Workspaceでチームのタスクを共有
            </Typography>
          </Paper>
          <Paper
            elevation={0}
            sx={{
              flex: 1,
              p: 3,
              borderRadius: 3,
              textAlign: 'center',
              bgcolor: 'rgba(255, 255, 255, 0.8)',
              border: '1px solid #e0e0e0',
            }}
          >
            <AutoAwesomeIcon sx={{ fontSize: 40, color: '#4caf50', mb: 1 }} />
            <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 0.5 }}>
              AI レコメンド
            </Typography>
            <Typography variant="body2" color="text.secondary">
              AIがあなたに最適なタスクを提案
            </Typography>
          </Paper>
        </Stack>
      </Box>

      {/* Footer */}
      <Box sx={{ textAlign: 'center', py: 3 }}>
        <Typography variant="caption" color="text.secondary">
          &copy; 2026 Todo App
        </Typography>
      </Box>
    </Box>
  )
}
