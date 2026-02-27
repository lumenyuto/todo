-- Add migration script here
CREATE TABLE labels
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    user_id   INTEGER NOT NULL REFERENCES users (id),
    UNIQUE (user_id, name)
);

CREATE TABLE todo_labels
(
    id       SERIAL PRIMARY KEY,
    todo_id  INTEGER NOT NULL REFERENCES todos (id) DEFERRABLE INITIALLY DEFERRED,
    label_id INTEGER NOT NULL REFERENCES labels (id) DEFERRABLE INITIALLY DEFERRED,
    UNIQUE (todo_id, label_id)
);