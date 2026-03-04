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
import type { Team, NewTeamPayload } from '../types/team'

type Props = {
  labels: Label[]
  filterLabelId: number | null
  onSelectLabel: (label: Label | null) => void
  onSubmitNewLabel: (newLabel: NewLabelPayload) => void
  onDeleteLabel: (id: number) => void
  teams: Team[]
  teamId: number | null
  onSelectTeam: (teamId: number | null) => void
  onSubmitNewTeam: (payload: NewTeamPayload) => void
}

export const SideNav: FC<Props> = ({
  labels,
  filterLabelId,
  onSelectLabel,
  onSubmitNewLabel,
  onDeleteLabel,
  teams,
  teamId,
  onSelectTeam,
  onSubmitNewTeam,
}) => {
  const [editName, setEditName] = useState('')
  const [openLabelModal, setOpenLabelModal] = useState(false)
  const [openTeamModal, setOpenTeamModal] = useState(false)
  const [newTeamName, setNewTeamName] = useState('')
  const [newTeamUserIds, setNewTeamUserIds] = useState('')

  const onSubmitLabel = () => {
    if (!editName) return
    onSubmitNewLabel({ name: editName })
    setEditName('')
  }

  const onSubmitTeam = () => {
    if (!newTeamName) return
    const userIds = newTeamUserIds
      .split(',')
      .map((s) => s.trim())
      .filter((s) => s !== '')
      .map(Number)
      .filter((n) => !isNaN(n))
    onSubmitNewTeam({ name: newTeamName, user_ids: userIds })
    setNewTeamName('')
    setNewTeamUserIds('')
    setOpenTeamModal(false)
  }

  return (
    <>
      <List>
        <ListItem disablePadding>
          <ListItemButton
            onClick={() => onSelectTeam(null)}
            selected={teamId === null}
          >
            <Stack direction="row" alignItems="center" spacing={1}>
              <PersonIcon fontSize="small" />
              <span>My Todos</span>
            </Stack>
          </ListItemButton>
        </ListItem>

        <ListSubheader>Teams</ListSubheader>
        {teams.map((team) => (
          <ListItem key={team.id} disablePadding>
            <ListItemButton
              onClick={() =>
                onSelectTeam(team.id === teamId ? null : team.id)
              }
              selected={team.id === teamId}
            >
              <Stack direction="row" alignItems="center" spacing={1}>
                <GroupsIcon fontSize="small" />
                <span>{team.name}</span>
              </Stack>
            </ListItemButton>
          </ListItem>
        ))}
        <ListItem disablePadding>
          <ListItemButton onClick={() => setOpenTeamModal(true)}>
            <Stack direction="row" alignItems="center" spacing={1}>
              <AddIcon fontSize="small" />
              <span>create team</span>
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

      <Modal open={openTeamModal} onClose={() => setOpenTeamModal(false)}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={3}>
            <Typography variant="subtitle1">create team</Typography>
            <TextField
              label="team name"
              variant="filled"
              fullWidth
              value={newTeamName}
              onChange={(e) => setNewTeamName(e.target.value)}
            />
            <TextField
              label="member user IDs (comma separated)"
              variant="filled"
              fullWidth
              value={newTeamUserIds}
              onChange={(e) => setNewTeamUserIds(e.target.value)}
              placeholder="1, 2, 3"
            />
            <Box textAlign="right">
              <Button onClick={onSubmitTeam} variant="contained">
                create
              </Button>
            </Box>
          </Stack>
        </Box>
      </Modal>
    </>
  )
}
