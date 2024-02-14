-- Add migration script here

ALTER TABLE buffers ADD COLUMN is_notes BOOL NOT NULL DEFAULT TRUE;
ALTER TABLE buffers ADD COLUMN name TEXT;
UPDATE buffers SET name = 'notes';
ALTER TABLE buffers ALTER COLUMN name SET NOT NULL;
