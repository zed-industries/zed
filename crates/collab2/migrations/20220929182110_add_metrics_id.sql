ALTER TABLE "users"
    ADD "metrics_id" uuid NOT NULL DEFAULT gen_random_uuid();
