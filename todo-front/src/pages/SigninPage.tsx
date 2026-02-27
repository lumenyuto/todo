import { useState, type FC } from 'react'
import { Link as RouterLink, useNavigate } from 'react-router-dom'
import { Box, Button, Link, TextField, Typography } from '@mui/material'
import { useAuth } from '../router/AuthContext'
import { getUserItems } from '../lib/api/user'

export const SigninPage: FC = () => {
    const { login } = useAuth()
    const navigate = useNavigate()
    const [name, setName] = useState('')
    const [error, setError] = useState('')

    const handleSignin = async () => {
        const trimmed = name.trim()
        if (!trimmed) {
            setError('ユーザー名を入力してください')
            return
        }

        const users = await getUserItems()
        const exist_user = users.find((u) => u.name === trimmed)
        if (!exist_user) {
            setError('ユーザー名が間違っています')
            return
        }

        login(exist_user)
        navigate('/')
    }

    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                height: '100vh',
                width: '100vw',
                gap: 2,
            }}
            >
            <Typography variant="h1">My App</Typography>
            <Typography variant="h2" color="text.secondary">
                サインイン
            </Typography>
            <TextField
                label="ユーザー名"
                value={name}
                onChange={(e) => {
                setName(e.target.value)
                setError('')
                }}
                onKeyDown={(e) => e.key === 'Enter' && handleSignin()}
                error={!!error}
                helperText={error}
                sx={{ width: 300 }}
            />
            <Button variant="contained" onClick={handleSignin} sx={{ width: 300 }}>
                ログイン
            </Button>
            <Typography
                variant="body2"
                color="text.secondary"
                sx={{ mt: 2}}
            >
                アカウントを持っていない場合は
                <Link
                component={RouterLink}
                to="/signup"
                sx={{
                    color: 'primary.main',
                    textDecoration: 'none',
                    fontWeight: 'bold',
                    '&:hover': { textDecoration: 'underline' }
                }}
                >
                新規登録
                </Link>
                から。
            </Typography>
        </Box>
    )
}