CREATE TABLE IF NOT EXISTS "embeddings" (
    "model" TEXT,
    "digest" BYTEA,
    "dimensions" FLOAT4[1536],
    "retrieved_at" TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY ("model", "digest")
);

CREATE INDEX IF NOT EXISTS "idx_retrieved_at_on_embeddings" ON "embeddings" ("retrieved_at");
