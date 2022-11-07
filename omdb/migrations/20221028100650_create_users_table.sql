-- Add migration script here
CREATE TABLE users (
	user_id SERIAL PRIMARY KEY,
	email TEXT NOT NULL UNIQUE,
	username TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL
)