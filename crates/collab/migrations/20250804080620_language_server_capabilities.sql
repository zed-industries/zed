ALTER TABLE language_servers
    ADD COLUMN capabilities TEXT NOT NULL DEFAULT '{}';

ALTER TABLE language_servers
    ALTER COLUMN capabilities DROP DEFAULT;
