ALTER TABLE access_tokens ADD COLUMN impersonator_id integer;

CREATE INDEX "index_access_tokens_impersonator_id" ON "access_tokens" ("impersonator_id");
