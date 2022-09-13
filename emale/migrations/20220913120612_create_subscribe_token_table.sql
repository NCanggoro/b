-- Add migration script here
CREATE TABLE subscriber_tokens(
  subscriber_token TEXT NOT NULL,
  subscriber_id uuid NOT NULL
    REFERENCES subscriber(id),
  PRIMARY KEY (subscriber_token)
)