ALTER TABLE models
    ALTER COLUMN max_requests_per_minute TYPE bigint,
    ALTER COLUMN max_tokens_per_minute TYPE bigint,
    ALTER COLUMN max_tokens_per_day TYPE bigint;
