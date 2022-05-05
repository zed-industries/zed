CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX trigram_index_users_on_github_login ON users USING GIN(github_login gin_trgm_ops);
