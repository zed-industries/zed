CREATE TABLE "notification_kinds" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_notification_kinds_on_name" ON "notification_kinds" ("name");

CREATE TABLE notifications (
    "id" SERIAL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "recipient_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "actor_id" INTEGER REFERENCES users (id) ON DELETE CASCADE,
    "kind" INTEGER NOT NULL REFERENCES notification_kinds (id),
    "content" TEXT,
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE,
    "response" BOOLEAN
);

CREATE INDEX "index_notifications_on_recipient_id" ON "notifications" ("recipient_id");
