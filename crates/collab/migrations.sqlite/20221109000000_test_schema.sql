CREATE TABLE "users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "github_login" VARCHAR,
    "admin" BOOLEAN,
    "email_address" VARCHAR(255) DEFAULT NULL,
    "invite_code" VARCHAR(64),
    "invite_count" INTEGER NOT NULL DEFAULT 0,
    "inviter_id" INTEGER REFERENCES users (id),
    "connected_once" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP NOT NULL DEFAULT now,
    "metrics_id" TEXT,
    "github_user_id" INTEGER
);
CREATE UNIQUE INDEX "index_users_github_login" ON "users" ("github_login");
CREATE UNIQUE INDEX "index_invite_code_users" ON "users" ("invite_code");
CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");
CREATE INDEX "index_users_on_github_user_id" ON "users" ("github_user_id");

CREATE TABLE "access_tokens" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER REFERENCES users (id),
    "hash" VARCHAR(128)
);
CREATE INDEX "index_access_tokens_user_id" ON "access_tokens" ("user_id");

CREATE TABLE "contacts" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "user_id_a" INTEGER REFERENCES users (id) NOT NULL,
    "user_id_b" INTEGER REFERENCES users (id) NOT NULL,
    "a_to_b" BOOLEAN NOT NULL,
    "should_notify" BOOLEAN NOT NULL,
    "accepted" BOOLEAN NOT NULL
);
CREATE UNIQUE INDEX "index_contacts_user_ids" ON "contacts" ("user_id_a", "user_id_b");
CREATE INDEX "index_contacts_user_id_b" ON "contacts" ("user_id_b");

CREATE TABLE "rooms" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "live_kit_room" VARCHAR NOT NULL
);

CREATE TABLE "projects" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id" INTEGER REFERENCES rooms (id) NOT NULL,
    "host_user_id" INTEGER REFERENCES users (id) NOT NULL,
    "host_connection_id" INTEGER NOT NULL,
    "host_connection_epoch" TEXT NOT NULL
);
CREATE INDEX "index_projects_on_host_connection_epoch" ON "projects" ("host_connection_epoch");

CREATE TABLE "worktrees" (
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "id" INTEGER NOT NULL,
    "root_name" VARCHAR NOT NULL,
    "abs_path" VARCHAR NOT NULL,
    "visible" BOOL NOT NULL,
    "scan_id" INTEGER NOT NULL,
    "is_complete" BOOL NOT NULL,
    PRIMARY KEY(project_id, id)
);
CREATE INDEX "index_worktrees_on_project_id" ON "worktrees" ("project_id");

CREATE TABLE "worktree_entries" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "id" INTEGER NOT NULL,
    "is_dir" BOOL NOT NULL,
    "path" VARCHAR NOT NULL,
    "inode" INTEGER NOT NULL,
    "mtime_seconds" INTEGER NOT NULL,
    "mtime_nanos" INTEGER NOT NULL,
    "is_symlink" BOOL NOT NULL,
    "is_ignored" BOOL NOT NULL,
    PRIMARY KEY(project_id, worktree_id, id),
    FOREIGN KEY(project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);
CREATE INDEX "index_worktree_entries_on_project_id" ON "worktree_entries" ("project_id");
CREATE INDEX "index_worktree_entries_on_project_id_and_worktree_id" ON "worktree_entries" ("project_id", "worktree_id");

CREATE TABLE "worktree_diagnostic_summaries" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "path" VARCHAR NOT NULL,
    "language_server_id" INTEGER NOT NULL,
    "error_count" INTEGER NOT NULL,
    "warning_count" INTEGER NOT NULL,
    PRIMARY KEY(project_id, worktree_id, path),
    FOREIGN KEY(project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);
CREATE INDEX "index_worktree_diagnostic_summaries_on_project_id" ON "worktree_diagnostic_summaries" ("project_id");
CREATE INDEX "index_worktree_diagnostic_summaries_on_project_id_and_worktree_id" ON "worktree_diagnostic_summaries" ("project_id", "worktree_id");

CREATE TABLE "language_servers" (
    "id" INTEGER NOT NULL,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY(project_id, id)
);
CREATE INDEX "index_language_servers_on_project_id" ON "language_servers" ("project_id");

CREATE TABLE "project_collaborators" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_epoch" TEXT NOT NULL,
    "user_id" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "is_host" BOOLEAN NOT NULL
);
CREATE INDEX "index_project_collaborators_on_project_id" ON "project_collaborators" ("project_id");
CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_and_replica_id" ON "project_collaborators" ("project_id", "replica_id");
CREATE INDEX "index_project_collaborators_on_connection_epoch" ON "project_collaborators" ("connection_epoch");
CREATE INDEX "index_project_collaborators_on_connection_id" ON "project_collaborators" ("connection_id");
CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_connection_id_and_epoch" ON "project_collaborators" ("project_id", "connection_id", "connection_epoch");

CREATE TABLE "room_participants" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id),
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "answering_connection_id" INTEGER,
    "answering_connection_epoch" TEXT,
    "answering_connection_lost" BOOLEAN NOT NULL,
    "location_kind" INTEGER,
    "location_project_id" INTEGER,
    "initial_project_id" INTEGER,
    "calling_user_id" INTEGER NOT NULL REFERENCES users (id),
    "calling_connection_id" INTEGER NOT NULL,
    "calling_connection_epoch" TEXT NOT NULL
);
CREATE UNIQUE INDEX "index_room_participants_on_user_id" ON "room_participants" ("user_id");
CREATE INDEX "index_room_participants_on_answering_connection_epoch" ON "room_participants" ("answering_connection_epoch");
CREATE INDEX "index_room_participants_on_calling_connection_epoch" ON "room_participants" ("calling_connection_epoch");
CREATE INDEX "index_room_participants_on_answering_connection_id" ON "room_participants" ("answering_connection_id");
CREATE UNIQUE INDEX "index_room_participants_on_answering_connection_id_and_answering_connection_epoch" ON "room_participants" ("answering_connection_id", "answering_connection_epoch");
