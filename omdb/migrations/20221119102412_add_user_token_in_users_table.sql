-- Add migration script here
ALTER TABLE users ADD auth_token UNIQUE TEXT NULL;