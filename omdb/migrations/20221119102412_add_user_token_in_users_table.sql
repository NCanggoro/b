-- Add migration script here
ALTER TABLE users ADD auth_token TEXT NULL UNIQUE;