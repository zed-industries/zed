CREATE TABLE "channel_message_mentions" (
    "message_id" INTEGER NOT NULL REFERENCES channel_messages (id) ON DELETE CASCADE,
    "start_offset" INTEGER NOT NULL,
    "end_offset" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY(message_id, start_offset)
);
