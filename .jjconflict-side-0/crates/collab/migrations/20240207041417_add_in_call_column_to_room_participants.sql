-- Add migration script here

ALTER TABLE room_participants ADD COLUMN in_call BOOL NOT NULL DEFAULT FALSE;
