CREATE TABLE IF NOT EXISTS "users" (
    "id" INTEGER PRIMARY KEY,
    "github_login" VARCHAR,
    "admin" BOOLEAN,
    "email_address" VARCHAR(255) DEFAULT NULL,
    "invite_code" VARCHAR(64),
    "invite_count" INTEGER NOT NULL DEFAULT 0,
    "inviter_id" INTEGER REFERENCES users (id),
    "connected_once" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP NOT NULL DEFAULT now,
    "metrics_id" VARCHAR(255),
    "github_user_id" INTEGER
);
CREATE UNIQUE INDEX "index_users_github_login" ON "users" ("github_login");
CREATE UNIQUE INDEX "index_invite_code_users" ON "users" ("invite_code");
CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");
CREATE INDEX "index_users_on_github_user_id" ON "users" ("github_user_id");

CREATE TABLE IF NOT EXISTS "access_tokens" (
    "id" INTEGER PRIMARY KEY,
    "user_id" INTEGER REFERENCES users (id),
    "hash" VARCHAR(128)
);
CREATE INDEX "index_access_tokens_user_id" ON "access_tokens" ("user_id");

CREATE TABLE IF NOT EXISTS "contacts" (
    "id" INTEGER PRIMARY KEY,
    "user_id_a" INTEGER REFERENCES users (id) NOT NULL,
    "user_id_b" INTEGER REFERENCES users (id) NOT NULL,
    "a_to_b" BOOLEAN NOT NULL,
    "should_notify" BOOLEAN NOT NULL,
    "accepted" BOOLEAN NOT NULL
);
CREATE UNIQUE INDEX "index_contacts_user_ids" ON "contacts" ("user_id_a", "user_id_b");
CREATE INDEX "index_contacts_user_id_b" ON "contacts" ("user_id_b");

CREATE TABLE IF NOT EXISTS "projects" (
    "id" INTEGER PRIMARY KEY,
    "host_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "unregistered" BOOLEAN NOT NULL DEFAULT false
);
