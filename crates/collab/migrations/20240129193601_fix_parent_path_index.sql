-- Add migration script here

DROP INDEX index_channels_on_parent_path;
CREATE INDEX index_channels_on_parent_path ON channels (parent_path text_pattern_ops);
