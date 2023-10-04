CREATE TABLE "notification_kinds" (
    "id" INTEGER PRIMARY KEY NOT NULL,
    "name" VARCHAR NOT NULL,
);

CREATE UNIQUE INDEX "index_notification_kinds_on_name" ON "notification_kinds" ("name");

CREATE TABLE notifications (
    "id" SERIAL PRIMARY KEY,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "recipent_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "kind" INTEGER NOT NULL REFERENCES notification_kinds (id),
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE
    "entity_id_1" INTEGER,
    "entity_id_2" INTEGER
);

CREATE INDEX "index_notifications_on_recipient_id" ON "notifications" ("recipient_id");
