-- Add migration script here

ALTER TABLE channel_members ALTER role SET NOT NULL;
ALTER TABLE channel_members DROP COLUMN admin;
