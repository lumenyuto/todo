import { useState, type FC } from 'react'
import {
  Box,
  Button,
  Chip,
  Divider,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Modal,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import LabelIcon from '@mui/icons-material/Label'
import DeleteIcon from '@mui/icons-material/Delete'
import GroupsIcon from '@mui/icons-material/Groups'
import AddIcon from '@mui/icons-material/Add'
import PersonIcon from '@mui/icons-material/Person'
import WorkspacesIcon from '@mui/icons-material/Workspaces'
import StyleIcon from '@mui/icons-material/Style'

import { modalInnerStyle } from '../styles/modal'
import type { Label, NewLabelPayload } from '../types/label'
import type { Workspace, NewWorkspacePayload } from '../types/workspace'

type Props = {
  labels: Label[]
  filterLabelId: number | null
  onSelectLabel: (label: Label | null) => void
  onSubmitNewLabel: (newLabel: NewLabelPayload) => void
  onDeleteLabel: (id: number) => void
  workspaces: Workspace[]
  workspaceId: number | null
  onSelectWorkspace: (workspaceId: number) => void
  onSubmitNewWorkspace: (payload: NewWorkspacePayload) => void
}

export const SideNav: FC<Props> = ({
  labels,
  filterLabelId,
  onSelectLabel,
  onSubmitNewLabel,
  onDeleteLabel,
  workspaces,
  workspaceId,
  onSelectWorkspace,
  onSubmitNewWorkspace,
}) => {
  const [editName, setEditName] = useState('')
  const [openLabelModal, setOpenLabelModal] = useState(false)
  const [openWorkspaceModal, setOpenWorkspaceModal] = useState(false)
  const [newWorkspaceName, setNewWorkspaceName] = useState('')
  const [newWorkspaceEmails, setNewWorkspaceEmails] = useState('')

  const onSubmitLabel = () => {
    if (!editName) return
    onSubmitNewLabel({ name: editName })
    setEditName('')
  }

  const onSubmitWorkspace = () => {
    if (!newWorkspaceName) return
    const emails = newWorkspaceEmails
      .split(',')
      .map((s) => s.trim())
      .filter((s) => s !== '')
    onSubmitNewWorkspace({ name: newWorkspaceName, user_emails: emails })
    setNewWorkspaceName('')
    setNewWorkspaceEmails('')
    setOpenWorkspaceModal(false)
  }

  const personalWorkspaces = workspaces.filter((w) => w.is_personal || w.users.length <= 1)
  const teamWorkspaces = workspaces.filter((w) => !w.is_personal && w.users.length > 1)

  const sectionHeaderSx = {
    px: 2,
    pt: 2.5,
    pb: 0.5,
    display: 'flex',
    alignItems: 'center',
    gap: 1,
  }

  const subHeaderSx = {
    px: 2,
    pt: 1.5,
    pb: 0.5,
  }

  const selectedItemSx = {
    borderRadius: 2,
    mx: 1,
    '&.Mui-selected': {
      bgcolor: 'rgba(46, 125, 50, 0.08)',
      '&:hover': {
        bgcolor: 'rgba(46, 125, 50, 0.12)',
      },
    },
    '&:hover': {
      borderRadius: 2,
      bgcolor: 'rgba(0, 0, 0, 0.04)',
    },
  }

  return (
    <Box sx={{ py: 1 }}>
      {/* App Title */}
      <Box sx={{ px: 2, pt: 1.5, pb: 2 }}>
        <Typography variant="h1" sx={{ color: 'primary.main' }}>
          Todo App
        </Typography>
      </Box>
      <Divider sx={{ mx: 2, mb: 1 }} />

      {/* Workspace Section */}
      <Box sx={sectionHeaderSx}>
        <WorkspacesIcon sx={{ fontSize: 18, color: 'primary.main' }} />
        <Typography variant="subtitle2" sx={{ fontWeight: 700, color: 'text.primary', letterSpacing: 0.5 }}>
          Workspace
        </Typography>
      </Box>

      {/* Personal */}
      <Box sx={subHeaderSx}>
        <Typography variant="caption" sx={{ fontWeight: 600, color: 'text.disabled', textTransform: 'uppercase', letterSpacing: 0.8 }}>
          Personal
        </Typography>
      </Box>
      <List disablePadding dense>
        {personalWorkspaces.map((w) => (
          <ListItem key={w.id} disablePadding>
            <ListItemButton
              onClick={() => onSelectWorkspace(w.id)}
              selected={workspaceId === w.id}
              sx={selectedItemSx}
            >
              <ListItemIcon sx={{ minWidth: 32 }}>
                <PersonIcon fontSize="small" sx={{ color: workspaceId === w.id ? 'primary.main' : 'text.secondary' }} />
              </ListItemIcon>
              <ListItemText
                primary={w.is_personal ? 'My Todos' : w.name}
                primaryTypographyProps={{
                  variant: 'body2',
                  fontWeight: workspaceId === w.id ? 600 : 400,
                  color: workspaceId === w.id ? 'primary.main' : 'text.primary',
                }}
              />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      {/* Team */}
      <Box sx={subHeaderSx}>
        <Typography variant="caption" sx={{ fontWeight: 600, color: 'text.disabled', textTransform: 'uppercase', letterSpacing: 0.8 }}>
          Team
        </Typography>
      </Box>
      <List disablePadding dense>
        {teamWorkspaces.map((w) => (
          <ListItem key={w.id} disablePadding>
            <ListItemButton
              onClick={() => onSelectWorkspace(w.id)}
              selected={w.id === workspaceId}
              sx={selectedItemSx}
            >
              <ListItemIcon sx={{ minWidth: 32 }}>
                <GroupsIcon fontSize="small" sx={{ color: w.id === workspaceId ? 'primary.main' : 'text.secondary' }} />
              </ListItemIcon>
              <ListItemText
                primary={w.name}
                primaryTypographyProps={{
                  variant: 'body2',
                  fontWeight: w.id === workspaceId ? 600 : 400,
                  color: w.id === workspaceId ? 'primary.main' : 'text.primary',
                }}
              />
              <Chip
                label={`${w.users.length}`}
                size="small"
                sx={{ height: 20, fontSize: 11, bgcolor: 'rgba(0,0,0,0.06)' }}
              />
            </ListItemButton>
          </ListItem>
        ))}
        {teamWorkspaces.length === 0 && (
          <Typography variant="caption" sx={{ px: 3, py: 0.5, color: 'text.disabled', display: 'block' }}>
            まだありません
          </Typography>
        )}
        <ListItem disablePadding>
          <ListItemButton onClick={() => setOpenWorkspaceModal(true)} sx={{ ...selectedItemSx, color: 'primary.main' }}>
            <ListItemIcon sx={{ minWidth: 32 }}>
              <AddIcon fontSize="small" sx={{ color: 'primary.main' }} />
            </ListItemIcon>
            <ListItemText
              primary="新規作成"
              primaryTypographyProps={{ variant: 'body2', fontWeight: 500, color: 'primary.main' }}
            />
          </ListItemButton>
        </ListItem>
      </List>

      <Divider sx={{ my: 1.5, mx: 2 }} />

      {/* Labels Section */}
      <Box sx={sectionHeaderSx}>
        <StyleIcon sx={{ fontSize: 18, color: 'primary.main' }} />
        <Typography variant="subtitle2" sx={{ fontWeight: 700, color: 'text.primary', letterSpacing: 0.5 }}>
          Labels
        </Typography>
      </Box>
      <List disablePadding dense>
        <ListItem disablePadding>
          <ListItemButton
            onClick={() => onSelectLabel(null)}
            selected={filterLabelId === null}
            sx={selectedItemSx}
          >
            <ListItemIcon sx={{ minWidth: 32 }}>
              <LabelIcon fontSize="small" sx={{ color: filterLabelId === null ? 'primary.main' : 'text.secondary' }} />
            </ListItemIcon>
            <ListItemText
              primary="すべて"
              primaryTypographyProps={{
                variant: 'body2',
                fontWeight: filterLabelId === null ? 600 : 400,
                color: filterLabelId === null ? 'primary.main' : 'text.primary',
              }}
            />
          </ListItemButton>
        </ListItem>
        {labels.map((label) => (
          <ListItem key={label.id} disablePadding>
            <ListItemButton
              onClick={() =>
                onSelectLabel(label.id === filterLabelId ? null : label)
              }
              selected={label.id === filterLabelId}
              sx={selectedItemSx}
            >
              <ListItemIcon sx={{ minWidth: 32 }}>
                <LabelIcon fontSize="small" sx={{ color: label.id === filterLabelId ? 'primary.main' : 'text.secondary' }} />
              </ListItemIcon>
              <ListItemText
                primary={label.name}
                primaryTypographyProps={{
                  variant: 'body2',
                  fontWeight: label.id === filterLabelId ? 600 : 400,
                  color: label.id === filterLabelId ? 'primary.main' : 'text.primary',
                }}
              />
            </ListItemButton>
          </ListItem>
        ))}
        <ListItem disablePadding>
          <ListItemButton onClick={() => setOpenLabelModal(true)} sx={{ ...selectedItemSx, color: 'text.secondary' }}>
            <ListItemIcon sx={{ minWidth: 32 }}>
              <AddIcon fontSize="small" sx={{ color: 'text.secondary' }} />
            </ListItemIcon>
            <ListItemText
              primary="ラベル管理"
              primaryTypographyProps={{ variant: 'body2', fontWeight: 500, color: 'text.secondary' }}
            />
          </ListItemButton>
        </ListItem>
      </List>

      {/* Label Modal */}
      <Modal open={openLabelModal} onClose={() => setOpenLabelModal(false)}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Typography variant="subtitle1" sx={{ fontWeight: 700 }}>
              ラベル管理
            </Typography>
            <Stack direction="row" spacing={1} alignItems="flex-end">
              <TextField
                label="新しいラベル"
                variant="outlined"
                size="small"
                fullWidth
                value={editName}
                onChange={(e) => setEditName(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter') onSubmitLabel() }}
              />
              <Button
                onClick={onSubmitLabel}
                variant="contained"
                size="small"
                disabled={!editName}
                sx={{ whiteSpace: 'nowrap', minWidth: 64 }}
              >
                追加
              </Button>
            </Stack>
            {labels.length > 0 && <Divider />}
            <Stack spacing={0.5}>
              {labels.map((label) => (
                <Stack
                  key={label.id}
                  direction="row"
                  alignItems="center"
                  justifyContent="space-between"
                  sx={{
                    px: 1.5,
                    py: 0.75,
                    borderRadius: 2,
                    '&:hover': { bgcolor: 'rgba(0,0,0,0.03)' },
                  }}
                >
                  <Stack direction="row" alignItems="center" spacing={1}>
                    <LabelIcon fontSize="small" sx={{ color: 'primary.main' }} />
                    <Typography variant="body2">{label.name}</Typography>
                  </Stack>
                  <IconButton
                    size="small"
                    onClick={() => onDeleteLabel(label.id)}
                    sx={{ color: 'text.disabled', '&:hover': { color: 'error.main' } }}
                  >
                    <DeleteIcon fontSize="small" />
                  </IconButton>
                </Stack>
              ))}
            </Stack>
          </Stack>
        </Box>
      </Modal>

      {/* Workspace Modal */}
      <Modal open={openWorkspaceModal} onClose={() => setOpenWorkspaceModal(false)}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Typography variant="subtitle1" sx={{ fontWeight: 700 }}>
              Workspace を作成
            </Typography>
            <TextField
              label="Workspace名"
              variant="outlined"
              size="small"
              fullWidth
              value={newWorkspaceName}
              onChange={(e) => setNewWorkspaceName(e.target.value)}
            />
            <TextField
              label="メンバーのメール（カンマ区切り）"
              variant="outlined"
              size="small"
              fullWidth
              value={newWorkspaceEmails}
              onChange={(e) => setNewWorkspaceEmails(e.target.value)}
              placeholder="user1@example.com, user2@example.com"
            />
            <Stack direction="row" justifyContent="flex-end" spacing={1}>
              <Button onClick={() => setOpenWorkspaceModal(false)} size="small">
                キャンセル
              </Button>
              <Button
                onClick={onSubmitWorkspace}
                variant="contained"
                size="small"
                disabled={!newWorkspaceName}
              >
                作成
              </Button>
            </Stack>
          </Stack>
        </Box>
      </Modal>
    </Box>
  )
}
