alter table models
    add column max_input_tokens_per_minute integer not null default 0,
    add column max_output_tokens_per_minute integer not null default 0;
