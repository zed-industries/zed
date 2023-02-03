CREATE TABLE IF NOT EXISTS "followers" (
    "id" SERIAL PRIMARY KEY,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id) ON DELETE CASCADE,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "leader_connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "leader_connection_id" INTEGER NOT NULL,
    "follower_connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "follower_connection_id" INTEGER NOT NULL
);

CREATE UNIQUE INDEX 
    "index_followers_on_project_id_and_leader_connection_server_id_and_leader_connection_id_and_follower_connection_server_id_and_follower_connection_id"
ON "followers" ("project_id", "leader_connection_server_id", "leader_connection_id", "follower_connection_server_id", "follower_connection_id");

CREATE INDEX "index_followers_on_room_id" ON "followers" ("room_id");
