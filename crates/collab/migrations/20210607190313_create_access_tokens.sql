CREATE TABLE IF NOT EXISTS "access_tokens" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INTEGER REFERENCES users (id),
    "hash" VARCHAR(128)
);

CREATE INDEX "index_access_tokens_user_id" ON "access_tokens" ("user_id");
