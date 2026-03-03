import { useEffect, useState, type FC } from 'react'
import { useAuth0 } from '@auth0/auth0-react'
import { Avatar, Box, Stack, Typography, Button, TextField, IconButton } from '@mui/material'
import EditIcon from '@mui/icons-material/Edit'
import CheckIcon from '@mui/icons-material/Check'
import CloseIcon from '@mui/icons-material/Close'

import { TodoList } from '../components/TodoList'
import { TodoForm } from '../components/TodoForm'
import { SideNav } from '../components/SideNav'

import {
  addLabelItem,
  deleteLabelItem,
  getLabelItems,
} from '../lib/api/label'
import {
  addTodoItem,
  getTodoItems,
  deleteTodoItem,
  updateTodoItem,
} from '../lib/api/todo'
import {
  addUserItem,
  updateUserName,
} from '../lib/api/user'
import {
  getTeamItems,
  createTeamItem,
  getTeamTodoItems,
  addTeamTodoItem,
} from '../lib/api/team'

import type { Label, NewLabelPayload } from '../types/label'
import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../types/todo'
import type { Team, NewTeamPayload } from '../types/team'

export const HomePage: FC = () => {
  const { user, getAccessTokenSilently, logout } = useAuth0()
  const [labels, setLabels] = useState<Label[]>([])
  const [todos, setTodos] = useState<Todo[]>([])
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null)
  const [userName, setUserName] = useState('')
  const [isEditingUserName, setIsEditingUserName] = useState(false)
  const [tempName, setTempName] = useState('')
  const [teams, setTeams] = useState<Team[]>([])
  const [selectedTeamId, setSelectedTeamId] = useState<number | null>(null)

  useEffect(() => {
    ;(async () => {
      const token = await getAccessTokenSilently()
      const User = await addUserItem(
        token,
        {
          sub: user!.sub!,
          name: user!.name ?? '',
          email: user!.email ?? '',
        },
      )
      setUserName(User.name)
      const [labels, todos, teams] = await Promise.all([
        getLabelItems(token),
        getTodoItems(token),
        getTeamItems(token).catch(() => [] as Team[]),
      ])
      setLabels(labels)
      setTodos(todos)
      setTeams(teams)
    })()
  }, [getAccessTokenSilently, user])

  const getToken = () => getAccessTokenSilently()

  const fetchTodos = async (token: string, teamId: number | null) => {
    if (teamId !== null) {
      return await getTeamTodoItems(token, teamId)
    }
    return await getTodoItems(token)
  }

  //todo
  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) return
    const token = await getToken()
    if (selectedTeamId !== null) {
      await addTeamTodoItem(token, selectedTeamId, payload)
    } else {
      await addTodoItem(token, payload)
    }
    const todos = await fetchTodos(token, selectedTeamId)
    setTodos(todos)
  }

  const onUpdate = async (updateTodo: UpdateTodoPayload) => {
    const token = await getToken()
    await updateTodoItem(token, updateTodo)
    const todos = await fetchTodos(token, selectedTeamId)
    setTodos(todos)
  }

  const onDelete = async (id: number) => {
    const token = await getToken()
    await deleteTodoItem(token, id)
    const todos = await fetchTodos(token, selectedTeamId)
    setTodos(todos)
  }

  
  // team
  const onSelectTeam = async (teamId: number | null) => {
    setSelectedTeamId(teamId)
    setFilterLabelId(null)
    const token = await getToken()
    const todos = await fetchTodos(token, teamId)
    setTodos(todos)
  }

  const onSubmitNewTeam = async (payload: NewTeamPayload) => {
    const token = await getToken()
    await createTeamItem(token, payload)
    const teams = await getTeamItems(token).catch(() => [] as Team[])
    setTeams(teams)
  }

  // label
  const onSelectLabel = (label: Label | null) => {
    setFilterLabelId(label?.id ?? null)
  }

  const onSubmitNewLabel = async (newLabel: NewLabelPayload) => {
    const token = await getToken()
    await addLabelItem(token, newLabel)
    const labels = await getLabelItems(token)
    setLabels(labels)
  }

  const onDeleteLabel = async (id: number) => {
    const token = await getToken()
    await deleteLabelItem(token, id)
    const labels = await getLabelItems(token)
    setLabels(labels)
  }

  // user
  const onUpdateUser = async () => {
    const token = await getToken()
    const updateUser = await updateUserName(token, { name: tempName.trim() })
    setUserName(updateUser.name)
    setIsEditingUserName(false)
  }

  const dispTodo = filterLabelId
    ? todos.filter((todo) =>
      todo.labels.some((label) => label.id === filterLabelId)
    )
    : todos

  const selectedTeam = teams.find((t) => t.id === selectedTeamId)

  return (
    <>
      <Box
        sx={{
          backgroundColor: 'white',
          borderBottom: '1px solid gray',
          display: 'flex',
          alignItems: 'center',
          position: 'fixed',
          top: 0,
          p: 2,
          width: '100%',
          height: 80,
          zIndex: 3,
        }}
      >
        <Typography variant="h1">Todo App</Typography>
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="flex-end"
          gap={2}
          sx={{ flex: 1 }}
        >
          <Avatar
            sx={{
              width: 32,
              height: 32,
              fontSize: 14,
              bgcolor: 'primary.main',
            }}
          >
            {userName ? userName.charAt(0).toUpperCase() : '?'}
          </Avatar>
          {isEditingUserName ? (
            <>
              <TextField
                size="small"
                value={tempName}
                onChange={(e) => setTempName(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter') onUpdateUser() }}
                autoFocus
                sx={{ width: 150 }}
              />
              <IconButton size="small" onClick={onUpdateUser}><CheckIcon fontSize="small" /></IconButton>
              <IconButton size="small" onClick={() => setIsEditingUserName(false)}><CloseIcon fontSize="small" /></IconButton>
            </>
          ) : (
            <>
              <Typography variant="body2" color="text.secondary" sx={{ display: { xs: 'none', md: 'block' } }}>
                {userName} さん
              </Typography>
              <IconButton size="small" onClick={() => { setTempName(userName); setIsEditingUserName(true) }}>
                <EditIcon fontSize="small" />
              </IconButton>
            </>
          )}
          <Button
            variant="outlined"
            size="small"
            onClick={() => logout({ logoutParams: { returnTo: window.location.origin } })}
            sx={{ borderRadius: '20px', textTransform: 'none' }}
          >
            Logout
          </Button>
        </Stack>
      </Box>
    <Box
      sx={{
          backgroundColor: 'white',
          borderRight: '1px solid gray',
          position: 'fixed',
          top: 80,
          height: 'calc(100% - 80px)',
          width: 200,
          zIndex: 2,
          left: 0,
          overflowY: 'auto',
      }}
    >
      <SideNav
        labels={labels}
        onSelectLabel={onSelectLabel}
        filterLabelId={filterLabelId}
        onSubmitNewLabel={onSubmitNewLabel}
        onDeleteLabel={onDeleteLabel}
        teams={teams}
        selectedTeamId={selectedTeamId}
        onSelectTeam={onSelectTeam}
        onSubmitNewTeam={onSubmitNewTeam}
      />
    </Box>
    <Box
      sx={{
          display: 'flex',
          justifyContent: 'center',
          p: 5,
          mt: 10,
          ml: '200px'
      }}
    >
        <Box maxWidth={700} width="100%">
          <Stack spacing={5}>
            {selectedTeam && (
              <Typography variant="h2" color="primary">
                {selectedTeam.name}
              </Typography>
            )}
            <TodoForm onSubmit={onSubmit} labels={labels} />
            <TodoList
              todos={dispTodo}
              labels={labels}
              onUpdate={onUpdate}
              onDelete={onDelete}
            />
          </Stack>
        </Box>
      </Box>
    </>
  )
}
