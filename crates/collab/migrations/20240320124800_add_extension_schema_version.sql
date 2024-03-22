-- Add migration script here
ALTER TABLE extension_versions ADD COLUMN schema_version INTEGER NOT NULL DEFAULT 0;
