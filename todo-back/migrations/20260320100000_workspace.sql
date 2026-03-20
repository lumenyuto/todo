-- teams → workspaces
ALTER TABLE teams RENAME TO workspaces;
ALTER TABLE workspaces ADD COLUMN is_personal BOOLEAN NOT NULL DEFAULT false;

-- team_users → workspace_users
ALTER TABLE team_users RENAME TO workspace_users;
ALTER TABLE workspace_users RENAME COLUMN team_id TO workspace_id;

-- todos: team_id → workspace_id
ALTER TABLE todos RENAME COLUMN team_id TO workspace_id;

-- 既存ユーザーに個人workspace作成 & 紐付け
INSERT INTO workspaces (name, is_personal)
SELECT u.name || '''s workspace', true
FROM users u
WHERE NOT EXISTS (
    SELECT 1 FROM workspaces w
    JOIN workspace_users wu ON w.id = wu.workspace_id
    WHERE wu.user_id = u.id AND w.is_personal = true
);

INSERT INTO workspace_users (workspace_id, user_id)
SELECT w.id, u.id
FROM workspaces w
JOIN users u ON w.name = u.name || '''s workspace'
WHERE w.is_personal = true
AND NOT EXISTS (
    SELECT 1 FROM workspace_users wu
    WHERE wu.workspace_id = w.id AND wu.user_id = u.id
);

-- 個人todo(workspace_id IS NULL)を個人workspaceに紐付け
UPDATE todos SET workspace_id = (
    SELECT w.id FROM workspaces w
    JOIN workspace_users wu ON w.id = wu.workspace_id
    WHERE wu.user_id = todos.user_id AND w.is_personal = true
    LIMIT 1
) WHERE workspace_id IS NULL;

-- workspace_id を NOT NULL に
ALTER TABLE todos ALTER COLUMN workspace_id SET NOT NULL;
