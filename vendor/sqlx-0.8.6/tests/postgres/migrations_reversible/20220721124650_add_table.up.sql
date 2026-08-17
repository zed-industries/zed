CREATE TABLE migrations_reversible_test (
    some_id BIGINT NOT NULL PRIMARY KEY,
    some_payload BIGINT NOT NUll
);

INSERT INTO migrations_reversible_test (some_id, some_payload)
VALUES (1, 100);
