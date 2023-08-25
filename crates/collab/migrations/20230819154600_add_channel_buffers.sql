CREATE TABLE "buffers" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX "index_buffers_on_channel_id" ON "buffers" ("channel_id");

CREATE TABLE "buffer_operations" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    "value" BYTEA NOT NULL,
    PRIMARY KEY(buffer_id, epoch, lamport_timestamp, replica_id)
);

CREATE TABLE "buffer_snapshots" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "text" TEXT NOT NULL,
    "operation_serialization_version" INTEGER NOT NULL,
    PRIMARY KEY(buffer_id, epoch)
);

CREATE TABLE "channel_buffer_collaborators" (
    "id" SERIAL PRIMARY KEY,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "connection_lost" BOOLEAN NOT NULL DEFAULT FALSE,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "replica_id" INTEGER NOT NULL
);

CREATE INDEX "index_channel_buffer_collaborators_on_channel_id" ON "channel_buffer_collaborators" ("channel_id");
CREATE UNIQUE INDEX "index_channel_buffer_collaborators_on_channel_id_and_replica_id" ON "channel_buffer_collaborators" ("channel_id", "replica_id");
CREATE INDEX "index_channel_buffer_collaborators_on_connection_server_id" ON "channel_buffer_collaborators" ("connection_server_id");
CREATE INDEX "index_channel_buffer_collaborators_on_connection_id" ON "channel_buffer_collaborators" ("connection_id");
CREATE UNIQUE INDEX "index_channel_buffer_collaborators_on_channel_id_connection_id_and_server_id" ON "channel_buffer_collaborators" ("channel_id", "connection_id", "connection_server_id");
