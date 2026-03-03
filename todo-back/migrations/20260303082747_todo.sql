CREATE TABLE todos
(
    id        SERIAL  PRIMARY KEY,
    text      TEXT    NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false,
    user_id   INTEGER NOT NULL REFERENCES users (id),
    team_id   INTEGER REFERENCES teams (id)
);