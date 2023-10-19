CREATE TABLE "channel_message_mentions" (
    "message_id" INTEGER NOT NULL REFERENCES channel_messages (id) ON DELETE CASCADE,
    "start_offset" INTEGER NOT NULL,
    "end_offset" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY(message_id, start_offset)
);

-- We use 'on conflict update' with this index, so it should be per-user.
CREATE UNIQUE INDEX "index_channel_messages_on_sender_id_nonce" ON "channel_messages" ("sender_id", "nonce");
DROP INDEX "index_channel_messages_on_nonce";
