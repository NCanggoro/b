-- Add migration script here
BEGIN;
  UPDATE subscriber
    SET status = 'ok'
    WHERE status is NULL;
  ALTER TABLE subscriber ALTER COLUMN status SET NOT NULL;
COMMIT;