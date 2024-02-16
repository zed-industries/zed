CREATE TABLE IF NOT EXISTS extensions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    external_id TEXT NOT NULL,
    latest_version TEXT NOT NULL,
    total_download_count BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS extension_versions (
    extension_id INTEGER REFERENCES extensions(id),
    version TEXT NOT NULL,
    published_at TIMESTAMP NOT NULL DEFAULT now(),
    authors TEXT NOT NULL,
    repository TEXT NOT NULL,
    description TEXT NOT NULL,
    download_count BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY(extension_id, version)
);

CREATE UNIQUE INDEX "index_extensions_external_id" ON "extensions" ("external_id");
CREATE INDEX "trigram_index_extensions_name" ON "extensions" USING GIN(name gin_trgm_ops);
CREATE INDEX "index_extensions_total_download_count" ON "extensions" ("total_download_count");
