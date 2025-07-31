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

ALTER TABLE "users"
    ADD "github_user_id" INTEGER;

CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");
CREATE INDEX "index_users_on_github_user_id" ON "users" ("github_user_id");
