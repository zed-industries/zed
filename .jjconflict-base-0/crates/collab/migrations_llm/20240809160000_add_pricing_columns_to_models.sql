ALTER TABLE models
    ADD COLUMN price_per_million_input_tokens integer NOT NULL DEFAULT 0,
    ADD COLUMN price_per_million_output_tokens integer NOT NULL DEFAULT 0;
