-- Your SQL goes here

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
)