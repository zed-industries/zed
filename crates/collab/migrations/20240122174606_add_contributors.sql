CREATE TABLE contributors (
    user_id INTEGER REFERENCES users(id),
    signed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id)
);
