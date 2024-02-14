-- Add migration script here

ALTER TABLE channel_buffer_collaborators ADD COLUMN buffer_id INT REFERENCES buffers(id);

UPDATE channel_buffer_collaborators SET buffer_id = buffers.id
FROM buffers
WHERE buffers.channel_id = channel_buffer_collaborators.channel_id;

ALTER TABLE channel_buffer_collaborators ALTER COLUMN channel_id DROP NOT NULL;
ALTER TABLE channel_buffer_collaborators ALTER COLUMN buffer_id SET NOT NULL;
