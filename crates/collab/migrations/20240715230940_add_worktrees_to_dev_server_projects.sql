ALTER TABLE dev_server_projects ADD COLUMN paths JSONB NULL;
UPDATE dev_server_projects SET paths = to_json(ARRAY[path]);
ALTER TABLE dev_server_projects ALTER COLUMN paths SET NOT NULL;
ALTER TABLE dev_server_projects ALTER COLUMN path DROP NOT NULL;
