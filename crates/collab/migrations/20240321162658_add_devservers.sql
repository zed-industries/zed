CREATE TABLE devservers (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  channel_id INT NOT NULL REFERENCES channels(id),
  name TEXT NOT NULL,
  hashed_token TEXT NOT NULL,
)
CREATE INDEX idx_devservers_on_channel_id ON devservers (channel_id);
