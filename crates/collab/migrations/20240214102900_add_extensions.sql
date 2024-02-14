CREATE TABLE IF NOT EXISTS extensions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    external_id TEXT NOT NULL,
    latest_version INTEGER,
    total_download_count BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS extension_versions (
    id SERIAL PRIMARY KEY,
    extension_id INTEGER REFERENCES extensions(id),
    published_at TIMESTAMP NOT NULL DEFAULT now(),
    version TEXT NOT NULL,
    authors TEXT NOT NULL,
    repository TEXT NOT NULL,
    description TEXT NOT NULL,
    download_count BIGINT NOT NULL DEFAULT 0
);

ALTER TABLE extensions ADD CONSTRAINT extensions_latest_version_fkey FOREIGN KEY (latest_version) REFERENCES extension_versions (id);

CREATE UNIQUE INDEX "index_extensions_external_id" ON "extensions" ("external_id");
CREATE INDEX "trigram_index_extensions_name" ON "extensions" USING GIN(name gin_trgm_ops);
CREATE INDEX "index_extensions_total_download_count" ON "extensions" ("total_download_count");
CREATE UNIQUE INDEX "index_extension_versions_extension_id_version" ON "extension_versions" ("extension_id", "version");
