DROP TABLE signups;

ALTER TABLE users
    DROP COLUMN metrics_id;

DROP SEQUENCE metrics_id_seq;
