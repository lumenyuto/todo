CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    sub TEXT UNIQUE,
    name TEXT NOT NULL,
    email TEXT
);