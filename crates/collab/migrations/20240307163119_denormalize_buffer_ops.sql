-- Add migration script here

ALTER TABLE buffers ADD COLUMN latest_operation_epoch INTEGER;
ALTER TABLE buffers ADD COLUMN latest_operation_lamport_timestamp INTEGER;
ALTER TABLE buffers ADD COLUMN latest_operation_replica_id INTEGER;

WITH ops AS (
    SELECT DISTINCT ON (buffer_id) buffer_id, epoch, lamport_timestamp, replica_id
    FROM buffer_operations
    ORDER BY buffer_id, epoch DESC, lamport_timestamp DESC, replica_id DESC
)
UPDATE buffers
SET latest_operation_epoch = ops.epoch,
    latest_operation_lamport_timestamp = ops.lamport_timestamp,
    latest_operation_replica_id = ops.replica_id
FROM ops
WHERE buffers.id = ops.buffer_id;
