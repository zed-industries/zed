CREATE SEQUENCE metrics_id_seq;

CREATE TABLE IF NOT EXISTS "signups" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "email_address" VARCHAR NOT NULL,
    "email_confirmation_code" VARCHAR(64) NOT NULL,
    "email_confirmation_sent" BOOLEAN NOT NULL,
    "metrics_id" INTEGER NOT NULL DEFAULT nextval('metrics_id_seq'),
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" INTEGER REFERENCES users (id),
    "inviting_user_id" INTEGER REFERENCES users (id),

    "platform_mac" BOOLEAN NOT NULL,
    "platform_linux" BOOLEAN NOT NULL,
    "platform_windows" BOOLEAN NOT NULL,
    "platform_unknown" BOOLEAN NOT NULL,

    "editor_features" VARCHAR[],
    "programming_languages" VARCHAR[]
);

CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");
CREATE UNIQUE INDEX "index_signups_on_email_address" ON "signups" ("email_address");
CREATE INDEX "index_signups_on_email_confirmation_sent" ON "signups" ("email_confirmation_sent");

ALTER TABLE "users"
    ADD "metrics_id" INTEGER DEFAULT nextval('metrics_id_seq');

UPDATE users
SET metrics_id = nextval('metrics_id_seq');
