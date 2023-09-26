CREATE TABLE "observed_channel_note_edits" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    PRIMARY KEY (user_id, channel_id)
);

CREATE UNIQUE INDEX "index_observed_notes_user_and_channel_id" ON "observed_channel_note_edits" ("user_id", "channel_id");
