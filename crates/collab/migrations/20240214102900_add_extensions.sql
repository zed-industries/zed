CREATE TABLE IF NOT EXISTS extensions (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    external_id TEXT NOT NULL,
    latest_version INTEGER REFERENCES extension_versions(id),
    total_download_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS extension_versions (
    id SERIAL PRIMARY KEY,
    extension_id INTEGER REFERENCES extensions(id),
    published_at TIMESTAMP NOT NULL DEFAULT now(),
    description TEXT NOT NULL,
    authors TEXT NOT NULL,
    version TEXT NOT NULL,
    download_count INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX "index_extensions_external_id" ON "extensions" ("external_id");
CREATE INDEX "index_extensions_total_download_count" ON "extensions" ("total_download_count");
CREATE INDEX "index_extensions_name" ON "extensions" ("name");
CREATE UNIQUE INDEX "index_extension_versions_extension_id_version" ON "extension_versions" ("extension_id", "version");

CREATE INDEX trigram_index_extensions_on_name ON extensions USING GIN(name gin_trgm_ops);
