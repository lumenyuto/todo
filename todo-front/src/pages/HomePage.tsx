import { useEffect, useState, type FC } from 'react'
import { useAuth0 } from '@auth0/auth0-react'
import { Avatar, Box, Stack, Typography, Button, TextField, IconButton, Drawer, Paper } from '@mui/material'
import EditIcon from '@mui/icons-material/Edit'
import CheckIcon from '@mui/icons-material/Check'
import CloseIcon from '@mui/icons-material/Close'
import MenuIcon from '@mui/icons-material/Menu'
import AddIcon from '@mui/icons-material/Add'

import { TodoList } from '../components/TodoList'
import { TodoForm } from '../components/TodoForm'
import { RecommendButton } from '../components/RecommendButton'
import { SideNav } from '../components/SideNav'

import {
  addLabelItem,
  deleteLabelItem,
  getLabelItems,
} from '../lib/api/label'
import {
  addTodoItem,
  getTodoItems,
  updateTodoItem,
  deleteTodoItem,
  getRecommendations,
} from '../lib/api/todo'
import {
  addUserItem,
  updateUserName,
} from '../lib/api/user'
import {
  getWorkspaces,
  createWorkspace,
} from '../lib/api/workspace'

import type { Label, NewLabelPayload } from '../types/label'
import type { NewTodoPayload, Todo, UpdateTodoPayload } from '../types/todo'
import type { Workspace, NewWorkspacePayload } from '../types/workspace'

export const HomePage: FC = () => {
  const { user, getAccessTokenSilently, logout } = useAuth0()
  const [labels, setLabels] = useState<Label[]>([])
  const [todos, setTodos] = useState<Todo[]>([])
  const [filterLabelId, setFilterLabelId] = useState<number | null>(null)
  const [userName, setUserName] = useState('')
  const [isEditingUserName, setIsEditingUserName] = useState(false)
  const [tempName, setTempName] = useState('')
  const [workspaces, setWorkspaces] = useState<Workspace[]>([])
  const [workspaceId, setWorkspaceId] = useState<number | null>(null)
  const [showSetup, setShowSetup] = useState(false)
  const [setupLoading, setSetupLoading] = useState(false)

  const [mobileOpen, setMobileOpen] = useState(false)

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen)
  }

  const handleLabelSelect = (label: Label | null) => {
    onSelectLabel(label)
    setMobileOpen(false)
  }

  const handleWorkspaceSelect = (id: number) => {
    onSelectWorkspace(id)
    setMobileOpen(false)
  }

  useEffect(() => {
    if (!user?.sub) return
    ;(async () => {
      const token = await getAccessTokenSilently()
      const User = await addUserItem(
        token,
        {
          sub: user.sub ?? '',
          name: user.name ?? '',
          email: user.email ?? '',
        },
      )
      setUserName(User.name)
      const [labels, fetchedWorkspaces] = await Promise.all([
        getLabelItems(token),
        getWorkspaces(token).catch(() => [] as Workspace[]),
      ])
      setLabels(labels)
      setWorkspaces(fetchedWorkspaces)

      if (fetchedWorkspaces.length === 0) {
        setShowSetup(true)
      } else {
        const personalWs = fetchedWorkspaces.find((w) => w.is_personal)
        const defaultWs = personalWs ?? fetchedWorkspaces[0]
        setWorkspaceId(defaultWs.id)
        const todos = await getTodoItems(token, defaultWs.id)
        setTodos(todos)
      }
    })()
  }, [getAccessTokenSilently, user?.sub])

  const getToken = () => getAccessTokenSilently()

  // setup
  const onCreatePersonalWorkspace = async () => {
    setSetupLoading(true)
    try {
      const token = await getToken()
      const ws = await createWorkspace(token, {
        name: `${userName}'s workspace`,
        is_personal: true,
      })
      const fetchedWorkspaces = await getWorkspaces(token).catch(() => [ws])
      setWorkspaces(fetchedWorkspaces)
      setWorkspaceId(ws.id)
      const todos = await getTodoItems(token, ws.id)
      setTodos(todos)
      setShowSetup(false)
    } finally {
      setSetupLoading(false)
    }
  }

  //todo
  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text || workspaceId === null) return
    const token = await getToken()
    await addTodoItem(token, workspaceId, payload)
    const todos = await getTodoItems(token, workspaceId)
    setTodos(todos)
  }

  const onUpdate = async (updateTodo: UpdateTodoPayload) => {
    if (workspaceId === null) return
    const token = await getToken()
    await updateTodoItem(token, workspaceId, updateTodo)
    const todos = await getTodoItems(token, workspaceId)
    setTodos(todos)
  }

  const onRecommend = async () => {
    if (workspaceId === null) throw new Error('no workspace selected')
    const token = await getToken()
    return await getRecommendations(token, workspaceId)
  }

  const onAddRecommendedTodos = async (texts: string[]) => {
    if (workspaceId === null) return
    const token = await getToken()
    for (const text of texts) {
      const payload = { text, label_ids: [] }
      await addTodoItem(token, workspaceId, payload)
    }
    const todos = await getTodoItems(token, workspaceId)
    setTodos(todos)
  }

  const onDelete = async (id: number) => {
    if (workspaceId === null) return
    const token = await getToken()
    await deleteTodoItem(token, workspaceId, id)
    const todos = await getTodoItems(token, workspaceId)
    setTodos(todos)
  }

  // workspace
  const onSelectWorkspace = async (wsId: number) => {
    setWorkspaceId(wsId)
    setFilterLabelId(null)
    const token = await getToken()
    const todos = await getTodoItems(token, wsId)
    setTodos(todos)
  }

  const onSubmitNewWorkspace = async (payload: NewWorkspacePayload) => {
    const token = await getToken()
    await createWorkspace(token, payload)
    const workspaces = await getWorkspaces(token).catch(() => [] as Workspace[])
    setWorkspaces(workspaces)
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

  const currentWorkspace = workspaces.find((w) => w.id === workspaceId)

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
      <Box
        sx={{
          backgroundColor: 'white',
          borderBottom: '1px solid gray',
          display: 'flex',
          alignItems: 'center',
          position: 'fixed',
          top: 0,
          p: { xs: 1, sm: 2 },
          width: '100%',
          height: 80,
          zIndex: 3,
        }}
      >
        <IconButton
          aria-label="open drawer"
          edge="start"
          onClick={handleDrawerToggle}
          sx={{ mr: 1, display: { sm: 'none' } }}
        >
          <MenuIcon />
        </IconButton>
        <Typography variant="h1">Todo App</Typography>
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="flex-end"
          gap={{ xs: 1, sm: 2 }}
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
          display: 'flex',
          mt: '80px',
          height: 'calc(100vh - 80px)',
        }}
      >
        {showSetup ? (
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
                  onClick={onCreatePersonalWorkspace}
                  disabled={setupLoading}
                  sx={{ borderRadius: 3, px: 4, py: 1.5 }}
                >
                  {setupLoading ? '作成中...' : '個人Workspaceを作成'}
                </Button>
              </Stack>
            </Paper>
          </Box>
        ) : (
          <>
            <Drawer
              variant="temporary"
              open={mobileOpen}
              onClose={handleDrawerToggle}
              ModalProps={{
                keepMounted: true,
              }}
              sx={{
                display: { xs: 'block', sm: 'none' },
                '& .MuiDrawer-paper': {
                  boxSizing: 'border-box',
                  width: 240,
                  mt: '80px',
                  height: 'calc(100vh - 80px)',
                },
              }}
            >
              <SideNav
                labels={labels}
                onSelectLabel={handleLabelSelect}
                filterLabelId={filterLabelId}
                onSubmitNewLabel={onSubmitNewLabel}
                onDeleteLabel={onDeleteLabel}
                workspaces={workspaces}
                workspaceId={workspaceId}
                onSelectWorkspace={handleWorkspaceSelect}
                onSubmitNewWorkspace={onSubmitNewWorkspace}
              />
            </Drawer>
            <Box
              sx={{
                backgroundColor: 'white',
                borderRight: '1px solid gray',
                width: '200px',
                flexShrink: 0,
                overflowY: 'auto',
                display: { xs: 'none', sm: 'block' },
              }}
            >
              <SideNav
                labels={labels}
                onSelectLabel={onSelectLabel}
                filterLabelId={filterLabelId}
                onSubmitNewLabel={onSubmitNewLabel}
                onDeleteLabel={onDeleteLabel}
                workspaces={workspaces}
                workspaceId={workspaceId}
                onSelectWorkspace={onSelectWorkspace}
                onSubmitNewWorkspace={onSubmitNewWorkspace}
              />
            </Box>
            <Box
              sx={{
                flex: 1,
                display: 'flex',
                justifyContent: 'center',
                p: { xs: 2, sm: 5 },
                overflowY: 'auto',
              }}
            >
              <Box maxWidth={700} width="100%">
                <Stack spacing={5}>
                  {currentWorkspace && !currentWorkspace.is_personal && (
                    <Typography variant="h2" color="primary">
                      {currentWorkspace.name}
                    </Typography>
                  )}
                  <TodoForm onSubmit={onSubmit} labels={labels} />
                  <RecommendButton onRecommend={onRecommend} onAddTodos={onAddRecommendedTodos} />
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
        )}
      </Box>
    </Box>
  )
}
