CREATE TABLE dev_servers (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  channel_id INT NOT NULL REFERENCES channels(id),
  name TEXT NOT NULL,
  hashed_token TEXT NOT NULL
);
CREATE INDEX idx_dev_servers_on_channel_id ON dev_servers (channel_id);
