CREATE TABLE teams
(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE team_users
(
    team_id INTEGER NOT NULL REFERENCES teams (id),
    user_id INTEGER NOT NULL REFERENCES users (id),
    PRIMARY KEY (team_id, user_id)
);