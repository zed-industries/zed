ALTER TABLE channels ADD COLUMN parent_path TEXT;

UPDATE channels
SET parent_path = substr(
    channel_paths.id_path,
    2,
    length(channel_paths.id_path) - length('/' || channel_paths.channel_id::text || '/')
)
FROM channel_paths
WHERE channel_paths.channel_id = channels.id;

CREATE INDEX "index_channels_on_parent_path" ON "channels" ("parent_path");
