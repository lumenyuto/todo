import { useState, type FC } from 'react'
import {
  Box,
  Button,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListSubheader,
  Modal,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import LabelIcon from '@mui/icons-material/Label'
import EditIcon from '@mui/icons-material/Edit'
import DeleteIcon from '@mui/icons-material/Delete'
import GroupsIcon from '@mui/icons-material/Groups'
import AddIcon from '@mui/icons-material/Add'
import PersonIcon from '@mui/icons-material/Person'

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

  return (
    <>
      <List>
        <ListSubheader>Workspace</ListSubheader>

        <ListSubheader sx={{ lineHeight: '30px', fontSize: '0.75rem', color: 'text.disabled', pl: 4 }}>
          Personal
        </ListSubheader>
        {personalWorkspaces.map((w) => (
          <ListItem key={w.id} disablePadding>
            <ListItemButton
              onClick={() => onSelectWorkspace(w.id)}
              selected={workspaceId === w.id}
              sx={{ pl: 4 }}
            >
              <Stack direction="row" alignItems="center" spacing={1}>
                <PersonIcon fontSize="small" />
                <span>{w.is_personal ? 'My Todos' : w.name}</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        ))}

        <ListSubheader sx={{ lineHeight: '30px', fontSize: '0.75rem', color: 'text.disabled', pl: 4 }}>
          Team
        </ListSubheader>
        {teamWorkspaces.map((workspace) => (
          <ListItem key={workspace.id} disablePadding>
            <ListItemButton
              onClick={() => onSelectWorkspace(workspace.id)}
              selected={workspace.id === workspaceId}
              sx={{ pl: 4 }}
            >
              <Stack direction="row" alignItems="center" spacing={1}>
                <GroupsIcon fontSize="small" />
                <span>{workspace.name}</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        ))}
        <ListItem disablePadding>
          <ListItemButton onClick={() => setOpenWorkspaceModal(true)} sx={{ pl: 4 }}>
            <Stack direction="row" alignItems="center" spacing={1}>
              <AddIcon fontSize="small" />
              <span>create workspace</span>
            </Stack>
          </ListItemButton>
        </ListItem>

        <ListSubheader>Labels</ListSubheader>
        {labels.map((label) => (
          <ListItem key={label.id} disablePadding>
            <ListItemButton
              onClick={() =>
                onSelectLabel(label.id === filterLabelId ? null : label)
              }
              selected={label.id === filterLabelId}
            >
              <Stack direction="row" alignItems="center" spacing={1}>
                <LabelIcon fontSize="small" />
                <span>{label.name}</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        ))}
        <ListItem disablePadding>
          <ListItemButton onClick={() => setOpenLabelModal(true)}>
            <Stack direction="row" alignItems="center" spacing={1}>
              <EditIcon fontSize="small" />
              <span>edit label</span>
            </Stack>
          </ListItemButton>
        </ListItem>
      </List>
      <Modal open={openLabelModal} onClose={() => setOpenLabelModal(false)}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Stack spacing={1}>
              <Typography variant="subtitle1">new label</Typography>
              <TextField
                label="new label"
                variant="filled"
                fullWidth
                value={editName}
                onChange={(e) => setEditName(e.target.value)}
              />
              <Box textAlign="right">
                <Button onClick={onSubmitLabel}>submit</Button>
              </Box>
            </Stack>
            <Stack spacing={1}>
              {labels.map((label) => (
                <Stack
                  key={label.id}
                  direction="row"
                  alignItems="center"
                  spacing={1}
                >
                  <IconButton
                    size="small"
                    onClick={() => onDeleteLabel(label.id)}
                  >
                    <DeleteIcon fontSize="small" />
                  </IconButton>
                  <span>{label.name}</span>
                </Stack>
              ))}
            </Stack>
          </Stack>
        </Box>
      </Modal>

      <Modal open={openWorkspaceModal} onClose={() => setOpenWorkspaceModal(false)}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Typography variant="subtitle1">create workspace</Typography>
            <TextField
              label="workspace name"
              variant="filled"
              fullWidth
              value={newWorkspaceName}
              onChange={(e) => setNewWorkspaceName(e.target.value)}
            />
            <TextField
              label="member emails (comma separated)"
              variant="filled"
              fullWidth
              value={newWorkspaceEmails}
              onChange={(e) => setNewWorkspaceEmails(e.target.value)}
              placeholder="user1@example.com, user2@example.com"
            />
            <Box textAlign="right">
              <Button onClick={onSubmitWorkspace} variant="contained">
                create
              </Button>
            </Box>
          </Stack>
        </Box>
      </Modal>
    </>
  )
}
