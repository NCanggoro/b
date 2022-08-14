-- Add migration script here
create table subscriber(
  id UUID NOT NULL,
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at TIMESTAMPTZ NOT NULL,

  PRIMARY KEY(id)
)