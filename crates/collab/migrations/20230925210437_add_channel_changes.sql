CREATE TABLE IF NOT EXISTS "observed_buffer_edits" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    PRIMARY KEY (user_id, buffer_id)
);

CREATE UNIQUE INDEX "index_observed_buffer_user_and_buffer_id" ON "observed_buffer_edits" ("user_id", "buffer_id");

CREATE TABLE IF NOT EXISTS "observed_channel_messages" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "channel_message_id" INTEGER NOT NULL,
    PRIMARY KEY (user_id, channel_id)
);

CREATE UNIQUE INDEX "index_observed_channel_messages_user_and_channel_id" ON "observed_channel_messages" ("user_id", "channel_id");
