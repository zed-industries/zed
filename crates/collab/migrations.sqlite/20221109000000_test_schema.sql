CREATE TABLE IF NOT EXISTS "sessions" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "expires" TIMESTAMP WITH TIME ZONE NULL,
    "session" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "github_login" VARCHAR,
    "admin" BOOLEAN,
    email_address VARCHAR(255) DEFAULT NULL,
    invite_code VARCHAR(64),
    invite_count INTEGER NOT NULL DEFAULT 0,
    inviter_id INTEGER REFERENCES users (id),
    connected_once BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now,
    "github_user_id" INTEGER
);
CREATE UNIQUE INDEX "index_users_github_login" ON "users" ("github_login");
CREATE UNIQUE INDEX "index_invite_code_users" ON "users" ("invite_code");
CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");
CREATE INDEX "index_users_on_github_user_id" ON "users" ("github_user_id");

CREATE TABLE IF NOT EXISTS "access_tokens" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER REFERENCES users (id),
    "hash" VARCHAR(128)
);
CREATE INDEX "index_access_tokens_user_id" ON "access_tokens" ("user_id");

CREATE TABLE IF NOT EXISTS "orgs" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "slug" VARCHAR NOT NULL
);
CREATE UNIQUE INDEX "index_orgs_slug" ON "orgs" ("slug");

CREATE TABLE IF NOT EXISTS "org_memberships" (
    "id" SERIAL PRIMARY KEY,
    "org_id" INTEGER REFERENCES orgs (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);
CREATE INDEX "index_org_memberships_user_id" ON "org_memberships" ("user_id");
CREATE UNIQUE INDEX "index_org_memberships_org_id_and_user_id" ON "org_memberships" ("org_id", "user_id");

CREATE TABLE IF NOT EXISTS "channels" (
    "id" SERIAL PRIMARY KEY,
    "owner_id" INTEGER NOT NULL,
    "owner_is_user" BOOLEAN NOT NULL,
    "name" VARCHAR NOT NULL
);
CREATE UNIQUE INDEX "index_channels_owner_and_name" ON "channels" ("owner_is_user", "owner_id", "name");

CREATE TABLE IF NOT EXISTS "channel_memberships" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "admin" BOOLEAN NOT NULL
);
CREATE INDEX "index_channel_memberships_user_id" ON "channel_memberships" ("user_id");
CREATE UNIQUE INDEX "index_channel_memberships_channel_id_and_user_id" ON "channel_memberships" ("channel_id", "user_id");

CREATE TABLE IF NOT EXISTS "channel_messages" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER REFERENCES channels (id) NOT NULL,
    "sender_id" INTEGER REFERENCES users (id) NOT NULL,
    "body" TEXT NOT NULL,
    "sent_at" TIMESTAMP
);
CREATE INDEX "index_channel_messages_channel_id" ON "channel_messages" ("channel_id");

CREATE TABLE IF NOT EXISTS "contacts" (
    "id" SERIAL PRIMARY KEY,
    "user_id_a" INTEGER REFERENCES users (id) NOT NULL,
    "user_id_b" INTEGER REFERENCES users (id) NOT NULL,
    "a_to_b" BOOLEAN NOT NULL,
    "should_notify" BOOLEAN NOT NULL,
    "accepted" BOOLEAN NOT NULL
);
CREATE UNIQUE INDEX "index_contacts_user_ids" ON "contacts" ("user_id_a", "user_id_b");
CREATE INDEX "index_contacts_user_id_b" ON "contacts" ("user_id_b");

CREATE TABLE IF NOT EXISTS "projects" (
    "id" SERIAL PRIMARY KEY,
    "host_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "unregistered" BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS "worktree_extensions" (
    "id" SERIAL PRIMARY KEY,
    "project_id" INTEGER REFERENCES projects (id) NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "extension" VARCHAR(255),
    "count" INTEGER NOT NULL
);
CREATE UNIQUE INDEX "index_worktree_extensions_on_project_id_and_worktree_id_and_extension" ON "worktree_extensions" ("project_id", "worktree_id", "extension");

CREATE TABLE IF NOT EXISTS "project_activity_periods" (
    "id" SERIAL PRIMARY KEY,
    "duration_millis" INTEGER NOT NULL,
    "ended_at" TIMESTAMP NOT NULL,
    "user_id" INTEGER REFERENCES users (id) NOT NULL,
    "project_id" INTEGER REFERENCES projects (id) NOT NULL
);
CREATE INDEX "index_project_activity_periods_on_ended_at" ON "project_activity_periods" ("ended_at");

CREATE TABLE IF NOT EXISTS "signups" (
    "id" SERIAL PRIMARY KEY,
    "email_address" VARCHAR NOT NULL,
    "email_confirmation_code" VARCHAR(64) NOT NULL,
    "email_confirmation_sent" BOOLEAN NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "device_id" VARCHAR,
    "user_id" INTEGER REFERENCES users (id) ON DELETE CASCADE,
    "inviting_user_id" INTEGER REFERENCES users (id) ON DELETE SET NULL,

    "platform_mac" BOOLEAN NOT NULL,
    "platform_linux" BOOLEAN NOT NULL,
    "platform_windows" BOOLEAN NOT NULL,
    "platform_unknown" BOOLEAN NOT NULL,

    "editor_features" VARCHAR[],
    "programming_languages" VARCHAR[]
);
CREATE UNIQUE INDEX "index_signups_on_email_address" ON "signups" ("email_address");
CREATE INDEX "index_signups_on_email_confirmation_sent" ON "signups" ("email_confirmation_sent");
