import { useState, type FC } from 'react'
import { Link as RouterLink, useNavigate } from 'react-router-dom'
import { Box, Button, Link, TextField, Typography } from '@mui/material'
import { useAuth } from '../router/AuthContext'
import { addUserItem, getUserItems } from '../lib/api/user'

export const SignupPage: FC = () => {  
    const { login } = useAuth()
    const navigate = useNavigate()
    const [name, setName] = useState('')
    const [error, setError] = useState('')

    const handleSignup = async () => {
        const trimmed = name.trim()
        if (!trimmed) {
            setError('ユーザー名を入力してください')
            return
        }

        const users = await getUserItems()
        const exist_user = users.find((u) => u.name === trimmed)
        
        if (exist_user) {
            setError('そのユーザー名はすでに使用されています。')
            return
        }

        const new_user = await addUserItem({ name: trimmed})

        login(new_user)
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
                新規登録
            </Typography>
            <TextField
                label="ユーザー名"
                value={name}
                onChange={(e) => {
                setName(e.target.value)
                setError('')
                }}
                onKeyDown={(e) => e.key === 'Enter' && handleSignup()}
                error={!!error}
                helperText={error}
                sx={{ width: 300 }}
            />
            <Button variant="contained" onClick={handleSignup} sx={{ width: 300 }}>
                登録する
            </Button>
            <Typography
                variant="body2"
                color="text.secondary"
                sx={{ mt: 2}}
            >
                もしアカウントを持っている場合は
                <Link
                component={RouterLink}
                to="/signin"
                sx={{
                    color: 'primary.main',
                    textDecoration: 'none',
                    fontWeight: 'bold',
                    '&:hover': { textDecoration: 'underline' }
                }}
                >
                サインイン
                </Link>
                から。
            </Typography>
        </Box>
    )
}