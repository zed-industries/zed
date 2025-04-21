CREATE TABLE "users" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "github_login" VARCHAR,
    "admin" BOOLEAN,
    "email_address" VARCHAR(255) DEFAULT NULL,
    "name" TEXT,
    "invite_code" VARCHAR(64),
    "invite_count" INTEGER NOT NULL DEFAULT 0,
    "inviter_id" INTEGER REFERENCES users (id),
    "connected_once" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "metrics_id" TEXT,
    "github_user_id" INTEGER NOT NULL,
    "accepted_tos_at" TIMESTAMP WITHOUT TIME ZONE,
    "github_user_created_at" TIMESTAMP WITHOUT TIME ZONE,
    "custom_llm_monthly_allowance_in_cents" INTEGER
);

CREATE UNIQUE INDEX "index_users_github_login" ON "users" ("github_login");

CREATE UNIQUE INDEX "index_invite_code_users" ON "users" ("invite_code");

CREATE INDEX "index_users_on_email_address" ON "users" ("email_address");

CREATE UNIQUE INDEX "index_users_on_github_user_id" ON "users" ("github_user_id");

CREATE TABLE "access_tokens" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER REFERENCES users (id),
    "impersonated_user_id" INTEGER REFERENCES users (id),
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
    "live_kit_room" VARCHAR NOT NULL,
    "environment" VARCHAR,
    "channel_id" INTEGER REFERENCES channels (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX "index_rooms_on_channel_id" ON "rooms" ("channel_id");

CREATE TABLE "projects" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id" INTEGER REFERENCES rooms (id) ON DELETE CASCADE,
    "host_user_id" INTEGER REFERENCES users (id),
    "host_connection_id" INTEGER,
    "host_connection_server_id" INTEGER REFERENCES servers (id) ON DELETE CASCADE,
    "unregistered" BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX "index_projects_on_host_connection_server_id" ON "projects" ("host_connection_server_id");

CREATE INDEX "index_projects_on_host_connection_id_and_host_connection_server_id" ON "projects" ("host_connection_id", "host_connection_server_id");

CREATE TABLE "worktrees" (
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "id" INTEGER NOT NULL,
    "root_name" VARCHAR NOT NULL,
    "abs_path" VARCHAR NOT NULL,
    "visible" BOOL NOT NULL,
    "scan_id" INTEGER NOT NULL,
    "is_complete" BOOL NOT NULL DEFAULT FALSE,
    "completed_scan_id" INTEGER NOT NULL,
    PRIMARY KEY (project_id, id)
);

CREATE INDEX "index_worktrees_on_project_id" ON "worktrees" ("project_id");

CREATE TABLE "worktree_entries" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "scan_id" INTEGER NOT NULL,
    "id" INTEGER NOT NULL,
    "is_dir" BOOL NOT NULL,
    "path" VARCHAR NOT NULL,
    "canonical_path" TEXT,
    "inode" INTEGER NOT NULL,
    "mtime_seconds" INTEGER NOT NULL,
    "mtime_nanos" INTEGER NOT NULL,
    "is_external" BOOL NOT NULL,
    "is_ignored" BOOL NOT NULL,
    "is_deleted" BOOL NOT NULL,
    "git_status" INTEGER,
    "is_fifo" BOOL NOT NULL,
    PRIMARY KEY (project_id, worktree_id, id),
    FOREIGN KEY (project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);

CREATE INDEX "index_worktree_entries_on_project_id" ON "worktree_entries" ("project_id");

CREATE INDEX "index_worktree_entries_on_project_id_and_worktree_id" ON "worktree_entries" ("project_id", "worktree_id");

CREATE TABLE "project_repositories" (
    "project_id" INTEGER NOT NULL,
    "abs_path" VARCHAR,
    "id" INTEGER NOT NULL,
    "entry_ids" VARCHAR,
    "legacy_worktree_id" INTEGER,
    "branch" VARCHAR,
    "scan_id" INTEGER NOT NULL,
    "is_deleted" BOOL NOT NULL,
    "current_merge_conflicts" VARCHAR,
    "branch_summary" VARCHAR,
    "head_commit_details" VARCHAR,
    PRIMARY KEY (project_id, id)
);

CREATE INDEX "index_project_repositories_on_project_id" ON "project_repositories" ("project_id");

CREATE TABLE "project_repository_statuses" (
    "project_id" INTEGER NOT NULL,
    "repository_id" INTEGER NOT NULL,
    "repo_path" VARCHAR NOT NULL,
    "status" INT8 NOT NULL,
    "status_kind" INT4 NOT NULL,
    "first_status" INT4 NULL,
    "second_status" INT4 NULL,
    "scan_id" INT8 NOT NULL,
    "is_deleted" BOOL NOT NULL,
    PRIMARY KEY (project_id, repository_id, repo_path)
);

CREATE INDEX "index_project_repos_statuses_on_project_id" ON "project_repository_statuses" ("project_id");

CREATE INDEX "index_project_repos_statuses_on_project_id_and_repo_id" ON "project_repository_statuses" ("project_id", "repository_id");

CREATE TABLE "worktree_settings_files" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "path" VARCHAR NOT NULL,
    "content" TEXT,
    "kind" VARCHAR,
    PRIMARY KEY (project_id, worktree_id, path),
    FOREIGN KEY (project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);

CREATE INDEX "index_worktree_settings_files_on_project_id" ON "worktree_settings_files" ("project_id");

CREATE INDEX "index_worktree_settings_files_on_project_id_and_worktree_id" ON "worktree_settings_files" ("project_id", "worktree_id");

CREATE TABLE "worktree_diagnostic_summaries" (
    "project_id" INTEGER NOT NULL,
    "worktree_id" INTEGER NOT NULL,
    "path" VARCHAR NOT NULL,
    "language_server_id" INTEGER NOT NULL,
    "error_count" INTEGER NOT NULL,
    "warning_count" INTEGER NOT NULL,
    PRIMARY KEY (project_id, worktree_id, path),
    FOREIGN KEY (project_id, worktree_id) REFERENCES worktrees (project_id, id) ON DELETE CASCADE
);

CREATE INDEX "index_worktree_diagnostic_summaries_on_project_id" ON "worktree_diagnostic_summaries" ("project_id");

CREATE INDEX "index_worktree_diagnostic_summaries_on_project_id_and_worktree_id" ON "worktree_diagnostic_summaries" ("project_id", "worktree_id");

CREATE TABLE "language_servers" (
    "id" INTEGER NOT NULL,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "name" VARCHAR NOT NULL,
    PRIMARY KEY (project_id, id)
);

CREATE INDEX "index_language_servers_on_project_id" ON "language_servers" ("project_id");

CREATE TABLE "project_collaborators" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "user_id" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "is_host" BOOLEAN NOT NULL
);

CREATE INDEX "index_project_collaborators_on_project_id" ON "project_collaborators" ("project_id");

CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_and_replica_id" ON "project_collaborators" ("project_id", "replica_id");

CREATE INDEX "index_project_collaborators_on_connection_server_id" ON "project_collaborators" ("connection_server_id");

CREATE INDEX "index_project_collaborators_on_connection_id" ON "project_collaborators" ("connection_id");

CREATE UNIQUE INDEX "index_project_collaborators_on_project_id_connection_id_and_server_id" ON "project_collaborators" (
    "project_id",
    "connection_id",
    "connection_server_id"
);

CREATE TABLE "room_participants" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id),
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "answering_connection_id" INTEGER,
    "answering_connection_server_id" INTEGER REFERENCES servers (id) ON DELETE CASCADE,
    "answering_connection_lost" BOOLEAN NOT NULL,
    "location_kind" INTEGER,
    "location_project_id" INTEGER,
    "initial_project_id" INTEGER,
    "calling_user_id" INTEGER NOT NULL REFERENCES users (id),
    "calling_connection_id" INTEGER NOT NULL,
    "calling_connection_server_id" INTEGER REFERENCES servers (id) ON DELETE SET NULL,
    "participant_index" INTEGER,
    "role" TEXT,
    "in_call" BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX "index_room_participants_on_user_id" ON "room_participants" ("user_id");

CREATE INDEX "index_room_participants_on_room_id" ON "room_participants" ("room_id");

CREATE INDEX "index_room_participants_on_answering_connection_server_id" ON "room_participants" ("answering_connection_server_id");

CREATE INDEX "index_room_participants_on_calling_connection_server_id" ON "room_participants" ("calling_connection_server_id");

CREATE INDEX "index_room_participants_on_answering_connection_id" ON "room_participants" ("answering_connection_id");

CREATE UNIQUE INDEX "index_room_participants_on_answering_connection_id_and_answering_connection_server_id" ON "room_participants" (
    "answering_connection_id",
    "answering_connection_server_id"
);

CREATE TABLE "servers" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "environment" VARCHAR NOT NULL
);

CREATE TABLE "followers" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "room_id" INTEGER NOT NULL REFERENCES rooms (id) ON DELETE CASCADE,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "leader_connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "leader_connection_id" INTEGER NOT NULL,
    "follower_connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "follower_connection_id" INTEGER NOT NULL
);

CREATE UNIQUE INDEX "index_followers_on_project_id_and_leader_connection_server_id_and_leader_connection_id_and_follower_connection_server_id_and_follower_connection_id" ON "followers" (
    "project_id",
    "leader_connection_server_id",
    "leader_connection_id",
    "follower_connection_server_id",
    "follower_connection_id"
);

CREATE INDEX "index_followers_on_room_id" ON "followers" ("room_id");

CREATE TABLE "channels" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "visibility" VARCHAR NOT NULL,
    "parent_path" TEXT NOT NULL,
    "requires_zed_cla" BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX "index_channels_on_parent_path" ON "channels" ("parent_path");

CREATE TABLE IF NOT EXISTS "channel_chat_participants" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER NOT NULL REFERENCES users (id),
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE
);

CREATE INDEX "index_channel_chat_participants_on_channel_id" ON "channel_chat_participants" ("channel_id");

CREATE TABLE IF NOT EXISTS "channel_messages" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "sender_id" INTEGER NOT NULL REFERENCES users (id),
    "body" TEXT NOT NULL,
    "sent_at" TIMESTAMP,
    "edited_at" TIMESTAMP,
    "nonce" BLOB NOT NULL,
    "reply_to_message_id" INTEGER DEFAULT NULL
);

CREATE INDEX "index_channel_messages_on_channel_id" ON "channel_messages" ("channel_id");

CREATE UNIQUE INDEX "index_channel_messages_on_sender_id_nonce" ON "channel_messages" ("sender_id", "nonce");

CREATE TABLE "channel_message_mentions" (
    "message_id" INTEGER NOT NULL REFERENCES channel_messages (id) ON DELETE CASCADE,
    "start_offset" INTEGER NOT NULL,
    "end_offset" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY (message_id, start_offset)
);

CREATE TABLE "channel_members" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "role" VARCHAR NOT NULL,
    "accepted" BOOLEAN NOT NULL DEFAULT false,
    "updated_at" TIMESTAMP NOT NULL DEFAULT now
);

CREATE UNIQUE INDEX "index_channel_members_on_channel_id_and_user_id" ON "channel_members" ("channel_id", "user_id");

CREATE TABLE "buffers" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL DEFAULT 0,
    "latest_operation_epoch" INTEGER,
    "latest_operation_replica_id" INTEGER,
    "latest_operation_lamport_timestamp" INTEGER
);

CREATE INDEX "index_buffers_on_channel_id" ON "buffers" ("channel_id");

CREATE TABLE "buffer_operations" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    "value" BLOB NOT NULL,
    PRIMARY KEY (buffer_id, epoch, lamport_timestamp, replica_id)
);

CREATE TABLE "buffer_snapshots" (
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "text" TEXT NOT NULL,
    "operation_serialization_version" INTEGER NOT NULL,
    PRIMARY KEY (buffer_id, epoch)
);

CREATE TABLE "channel_buffer_collaborators" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "connection_id" INTEGER NOT NULL,
    "connection_server_id" INTEGER NOT NULL REFERENCES servers (id) ON DELETE CASCADE,
    "connection_lost" BOOLEAN NOT NULL DEFAULT false,
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "replica_id" INTEGER NOT NULL
);

CREATE INDEX "index_channel_buffer_collaborators_on_channel_id" ON "channel_buffer_collaborators" ("channel_id");

CREATE UNIQUE INDEX "index_channel_buffer_collaborators_on_channel_id_and_replica_id" ON "channel_buffer_collaborators" ("channel_id", "replica_id");

CREATE INDEX "index_channel_buffer_collaborators_on_connection_server_id" ON "channel_buffer_collaborators" ("connection_server_id");

CREATE INDEX "index_channel_buffer_collaborators_on_connection_id" ON "channel_buffer_collaborators" ("connection_id");

CREATE UNIQUE INDEX "index_channel_buffer_collaborators_on_channel_id_connection_id_and_server_id" ON "channel_buffer_collaborators" (
    "channel_id",
    "connection_id",
    "connection_server_id"
);

CREATE TABLE "feature_flags" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "flag" TEXT NOT NULL UNIQUE,
    "enabled_for_all" BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX "index_feature_flags" ON "feature_flags" ("id");

CREATE TABLE "user_features" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "feature_id" INTEGER NOT NULL REFERENCES feature_flags (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, feature_id)
);

CREATE UNIQUE INDEX "index_user_features_user_id_and_feature_id" ON "user_features" ("user_id", "feature_id");

CREATE INDEX "index_user_features_on_user_id" ON "user_features" ("user_id");

CREATE INDEX "index_user_features_on_feature_id" ON "user_features" ("feature_id");

CREATE TABLE "observed_buffer_edits" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "buffer_id" INTEGER NOT NULL REFERENCES buffers (id) ON DELETE CASCADE,
    "epoch" INTEGER NOT NULL,
    "lamport_timestamp" INTEGER NOT NULL,
    "replica_id" INTEGER NOT NULL,
    PRIMARY KEY (user_id, buffer_id)
);

CREATE UNIQUE INDEX "index_observed_buffers_user_and_buffer_id" ON "observed_buffer_edits" ("user_id", "buffer_id");

CREATE TABLE IF NOT EXISTS "observed_channel_messages" (
    "user_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "channel_id" INTEGER NOT NULL REFERENCES channels (id) ON DELETE CASCADE,
    "channel_message_id" INTEGER NOT NULL,
    PRIMARY KEY (user_id, channel_id)
);

CREATE UNIQUE INDEX "index_observed_channel_messages_user_and_channel_id" ON "observed_channel_messages" ("user_id", "channel_id");

CREATE TABLE "notification_kinds" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" VARCHAR NOT NULL
);

CREATE UNIQUE INDEX "index_notification_kinds_on_name" ON "notification_kinds" ("name");

CREATE TABLE "notifications" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "created_at" TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    "recipient_id" INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "kind" INTEGER NOT NULL REFERENCES notification_kinds (id),
    "entity_id" INTEGER,
    "content" TEXT,
    "is_read" BOOLEAN NOT NULL DEFAULT FALSE,
    "response" BOOLEAN
);

CREATE INDEX "index_notifications_on_recipient_id_is_read_kind_entity_id" ON "notifications" ("recipient_id", "is_read", "kind", "entity_id");

CREATE TABLE contributors (
    user_id INTEGER REFERENCES users (id),
    signed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
);

CREATE TABLE extensions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    external_id TEXT NOT NULL,
    name TEXT NOT NULL,
    latest_version TEXT NOT NULL,
    total_download_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE extension_versions (
    extension_id INTEGER REFERENCES extensions (id),
    version TEXT NOT NULL,
    published_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    authors TEXT NOT NULL,
    repository TEXT NOT NULL,
    description TEXT NOT NULL,
    schema_version INTEGER NOT NULL DEFAULT 0,
    wasm_api_version TEXT,
    download_count INTEGER NOT NULL DEFAULT 0,
    provides_themes BOOLEAN NOT NULL DEFAULT FALSE,
    provides_icon_themes BOOLEAN NOT NULL DEFAULT FALSE,
    provides_languages BOOLEAN NOT NULL DEFAULT FALSE,
    provides_grammars BOOLEAN NOT NULL DEFAULT FALSE,
    provides_language_servers BOOLEAN NOT NULL DEFAULT FALSE,
    provides_context_servers BOOLEAN NOT NULL DEFAULT FALSE,
    provides_slash_commands BOOLEAN NOT NULL DEFAULT FALSE,
    provides_indexed_docs_providers BOOLEAN NOT NULL DEFAULT FALSE,
    provides_snippets BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (extension_id, version)
);

CREATE UNIQUE INDEX "index_extensions_external_id" ON "extensions" ("external_id");

CREATE INDEX "index_extensions_total_download_count" ON "extensions" ("total_download_count");

CREATE TABLE rate_buckets (
    user_id INT NOT NULL,
    rate_limit_name VARCHAR(255) NOT NULL,
    token_count INT NOT NULL,
    last_refill TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    PRIMARY KEY (user_id, rate_limit_name),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX idx_user_id_rate_limit ON rate_buckets (user_id, rate_limit_name);

CREATE TABLE IF NOT EXISTS billing_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users (id),
    max_monthly_llm_usage_spending_in_cents INTEGER NOT NULL
);

CREATE UNIQUE INDEX "uix_billing_preferences_on_user_id" ON billing_preferences (user_id);

CREATE TABLE IF NOT EXISTS billing_customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users (id),
    has_overdue_invoices BOOLEAN NOT NULL DEFAULT FALSE,
    stripe_customer_id TEXT NOT NULL
);

CREATE UNIQUE INDEX "uix_billing_customers_on_user_id" ON billing_customers (user_id);

CREATE UNIQUE INDEX "uix_billing_customers_on_stripe_customer_id" ON billing_customers (stripe_customer_id);

CREATE TABLE IF NOT EXISTS billing_subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    billing_customer_id INTEGER NOT NULL REFERENCES billing_customers (id),
    stripe_subscription_id TEXT NOT NULL,
    stripe_subscription_status TEXT NOT NULL,
    stripe_cancel_at TIMESTAMP,
    stripe_cancellation_reason TEXT,
    kind TEXT,
    stripe_current_period_start BIGINT,
    stripe_current_period_end BIGINT
);

CREATE INDEX "ix_billing_subscriptions_on_billing_customer_id" ON billing_subscriptions (billing_customer_id);

CREATE UNIQUE INDEX "uix_billing_subscriptions_on_stripe_subscription_id" ON billing_subscriptions (stripe_subscription_id);

CREATE TABLE IF NOT EXISTS processed_stripe_events (
    stripe_event_id TEXT PRIMARY KEY,
    stripe_event_type TEXT NOT NULL,
    stripe_event_created_timestamp INTEGER NOT NULL,
    processed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX "ix_processed_stripe_events_on_stripe_event_created_timestamp" ON processed_stripe_events (stripe_event_created_timestamp);

CREATE TABLE IF NOT EXISTS "breakpoints" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "project_id" INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    "position" INTEGER NOT NULL,
    "log_message" TEXT NULL,
    "worktree_id" BIGINT NOT NULL,
    "path" TEXT NOT NULL,
    "kind" VARCHAR NOT NULL
);

CREATE INDEX "index_breakpoints_on_project_id" ON "breakpoints" ("project_id");
