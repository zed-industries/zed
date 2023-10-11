-- Add migration script here

ALTER TABLE channel_members ADD COLUMN role TEXT;
UPDATE channel_members SET role = CASE WHEN admin THEN 'admin' ELSE 'member' END;
