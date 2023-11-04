CREATE TABLE IF NOT EXISTS "channel_messages" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "sender_id" INTEGER NOT NULL REFERENCES users (id),
    "body" TEXT NOT NULL,
    "sent_at" TIMESTAMP,
    "nonce" UUID NOT NULL
);
CREATE INDEX "index_channel_messages_on_channel_id" ON "channel_messages" ("channel_id");
CREATE UNIQUE INDEX "index_channel_messages_on_nonce" ON "channel_messages" ("nonce");

CREATE TABLE IF NOT EXISTS "channel_chat_participants" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE
);
CREATE INDEX "index_channel_chat_participants_on_channel_id" ON "channel_chat_participants" ("channel_id");
