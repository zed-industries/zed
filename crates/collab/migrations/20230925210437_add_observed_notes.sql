CREATE TABLE "observed_buffer_edits" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    PRIMARY KEY (user_id, buffer_id)
);

CREATE UNIQUE INDEX "index_observed_buffer_user_and_buffer_id" ON "observed_buffer_edits" ("user_id", "buffer_id");
