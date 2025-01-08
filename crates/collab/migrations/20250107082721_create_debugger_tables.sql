CREATE TABLE IF NOT EXISTS "debug_clients" (
    id BIGINT NOT NULL,
    project_id INTEGER NOT NULL,
    session_id BIGINT NOT NULL,
    capabilities INTEGER NOT NULL,
    PRIMARY KEY (id, project_id, session_id),
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);

CREATE INDEX "index_debug_client_on_project_id" ON "debug_clients" ("project_id");

CREATE TABLE IF NOT EXISTS "debug_panel_items" (
    id BIGINT NOT NULL,
    project_id INTEGER NOT NULL,
    thread_id BIGINT NOT NULL,
    session_id BIGINT NOT NULL,
    active_thread_item INTEGER NOT NULL,
    seassion_name TEXT NOT NULL,
    console BYTEA NOT NULL,
    module_list BYTEA NOT NULL,
    thread_state BYTEA NOT NULL,
    variable_list BYTEA NOT NULL,
    stack_frame_list BYTEA NOT NULL,
    loaded_source_list BYTEA NOT NULL,
    PRIMARY KEY (id, project_id, session_id, thread_id),
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (id, project_id, session_id) REFERENCES debug_clients (id, project_id, session_id) ON DELETE CASCADE
);

CREATE INDEX "index_debug_panel_items_on_project_id" ON "debug_panel_items" ("project_id");
CREATE INDEX "index_debug_panel_items_on_session_id" ON "debug_panel_items" ("session_id");
CREATE INDEX "index_debug_panel_items_on_thread_id" ON "debug_panel_items" ("thread_id");
CREATE INDEX "index_debug_panel_items_on_debug_client" ON "debug_panel_items" ("id", "project_id", "session_id");
