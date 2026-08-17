-- https://github.com/prisma/database-schema-examples/tree/master/postgres/basic-twitter#basic-twitter
CREATE TABLE tweet (
    id BIGINT NOT NULL PRIMARY KEY,
    text TEXT NOT NULL,
    is_sent BOOLEAN NOT NULL DEFAULT TRUE,
    owner_id BIGINT
);
INSERT INTO tweet(id, text, owner_id)
VALUES (1, '#sqlx is pretty cool!', 1);
--
CREATE TABLE tweet_reply (
    id BIGINT NOT NULL PRIMARY KEY,
    tweet_id BIGINT NOT NULL,
    text TEXT NOT NULL,
    owner_id BIGINT,
    CONSTRAINT tweet_id_fk FOREIGN KEY (tweet_id) REFERENCES tweet(id)
);
INSERT INTO tweet_reply(id, tweet_id, text, owner_id)
VALUES (1, 1, 'Yeah! #sqlx is indeed pretty cool!', 1);
--
CREATE TABLE accounts (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    is_active BOOLEAN
);
INSERT INTO accounts(id, name, is_active)
VALUES (1, 'Herp Derpinson', 1);
CREATE VIEW accounts_view as
SELECT *
FROM accounts;
--
CREATE TABLE products (
    product_no INTEGER,
    name TEXT,
    price NUMERIC,
    CONSTRAINT price_greater_than_zero CHECK (price > 0)
);
