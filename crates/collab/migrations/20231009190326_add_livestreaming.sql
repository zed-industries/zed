-- Add migration script here

ALTER TABLE rooms ADD COLUMN public BOOLEAN NOT NULL DEFAULT false;
