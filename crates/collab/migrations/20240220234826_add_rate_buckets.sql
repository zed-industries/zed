CREATE TABLE IF NOT EXISTS rate_buckets (
    user_id INT NOT NULL,
    rate_limit_name VARCHAR(255) NOT NULL,
    token_count INT NOT NULL,
    last_refill TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    PRIMARY KEY (user_id, rate_limit_name),
    CONSTRAINT fk_user
        FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_user_id_rate_limit ON rate_buckets (user_id, rate_limit_name);
