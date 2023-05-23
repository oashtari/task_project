-- Your SQL goes here

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_id 
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,

)