CREATE INDEX trigram_index_channels_on_parent_path ON channels USING GIN(parent_path gin_trgm_ops);
