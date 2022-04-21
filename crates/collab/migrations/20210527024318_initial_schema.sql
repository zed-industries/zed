CREATE TABLE IF NOT EXISTS "sessions" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "expires" TIMESTAMP WITH TIME ZONE NULL,
    "session" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "users" (
    "id" SERIAL PRIMARY KEY,
    "github_login" VARCHAR,
    "admin" BOOLEAN
);

CREATE UNIQUE INDEX "index_users_github_login" ON "users" ("github_login");

CREATE TABLE IF NOT EXISTS "signups" (
    "id" SERIAL PRIMARY KEY,
    "github_login" VARCHAR,
    "email_address" VARCHAR,
    "about" TEXT
);

INSERT INTO users (github_login, admin)
VALUES
    ('nathansobo', true),
    ('maxbrunsfeld', true),
    ('as-cii', true);
