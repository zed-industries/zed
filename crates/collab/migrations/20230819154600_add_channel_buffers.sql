CREATE TABLE "buffers" (
    "id" SERIAL PRIMARY KEY,
    "epoch" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE "buffer_operations" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "local_timestamp" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    "version" BYTEA NOT NULL,
    "is_undo" BOOLEAN NOT NULL,
    "value" BYTEA NOT NULL,
    PRIMARY KEY(buffer_id, epoch, lamport_timestamp, replica_id)
);

CREATE TABLE "buffer_snapshots" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "text" TEXT NOT NULL,
    PRIMARY KEY(buffer_id, epoch)
);

ALTER TABLE "channels" ADD COLUMN "main_buffer_id" INTEGER REFERENCES buffers (id);
