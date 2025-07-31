-- Add migration script here

CREATE UNIQUE INDEX uix_channels_parent_path_name ON channels(parent_path, name) WHERE (parent_path IS NOT NULL AND parent_path != '');
