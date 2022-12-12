-- Add migration script here
-- Add migration script here
INSERT INTO users (user_id, username, password_hash) 
VALUES (
  'a4e6e7f1-b3bc-41a6-b524-c7c9ac92ea7b',
  'deeznuts',
  '$argon2id$v=19$m=15000,t=2,p=1$K69+agHFciO5ZGkAC1y4zQ$iSqg190x2O6btEWUPdvxZF9oTgKOHQCmSVkPDZW07Jo'
)

