-- Add channel_order column to channels table with default value
ALTER TABLE channels ADD COLUMN channel_order INTEGER NOT NULL DEFAULT 1;

-- Update channel_order for existing channels using ROW_NUMBER for deterministic ordering
UPDATE channels
SET channel_order = (
    SELECT ROW_NUMBER() OVER (
        PARTITION BY parent_path
        ORDER BY name, id
    )
    FROM channels c2
    WHERE c2.id = channels.id
);

-- Create index for efficient ordering queries
CREATE INDEX "index_channels_on_parent_path_and_order" ON "channels" ("parent_path", "channel_order");
