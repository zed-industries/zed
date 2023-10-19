CREATE TABLE "notification_kinds" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_notification_kinds_on_name" ON "notification_kinds" ("name");

CREATE TABLE notifications (
    "id" SERIAL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "recipient_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "kind" INTEGER NOT NULL REFERENCES notification_kinds (id),
    "entity_id" INTEGER,
    "content" TEXT,
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE,
    "response" BOOLEAN
);

CREATE INDEX
    "index_notifications_on_recipient_id_is_read_kind_entity_id"
    ON "notifications"
    ("recipient_id", "is_read", "kind", "entity_id");
