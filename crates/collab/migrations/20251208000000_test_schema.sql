CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;

CREATE TABLE public.access_tokens (
    id integer NOT NULL,
    user_id integer,
    hash character varying(128),
    impersonated_user_id integer
);

CREATE SEQUENCE public.access_tokens_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.access_tokens_id_seq OWNED BY public.access_tokens.id;

CREATE TABLE public.breakpoints (
    id integer NOT NULL,
    project_id integer NOT NULL,
    "position" integer NOT NULL,
    log_message text,
    worktree_id bigint NOT NULL,
    path text NOT NULL,
    kind character varying NOT NULL
);

CREATE SEQUENCE public.breakpoints_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.breakpoints_id_seq OWNED BY public.breakpoints.id;

CREATE TABLE public.buffer_operations (
    buffer_id integer NOT NULL,
    epoch integer NOT NULL,
    replica_id integer NOT NULL,
    lamport_timestamp integer NOT NULL,
    value bytea NOT NULL
);

CREATE TABLE public.buffer_snapshots (
    buffer_id integer NOT NULL,
    epoch integer NOT NULL,
    text text NOT NULL,
    operation_serialization_version integer NOT NULL
);

CREATE TABLE public.buffers (
    id integer NOT NULL,
    channel_id integer NOT NULL,
    epoch integer DEFAULT 0 NOT NULL,
    latest_operation_epoch integer,
    latest_operation_lamport_timestamp integer,
    latest_operation_replica_id integer
);

CREATE SEQUENCE public.buffers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.buffers_id_seq OWNED BY public.buffers.id;

CREATE TABLE public.channel_buffer_collaborators (
    id integer NOT NULL,
    channel_id integer NOT NULL,
    connection_id integer NOT NULL,
    connection_server_id integer NOT NULL,
    connection_lost boolean DEFAULT false NOT NULL,
    user_id integer NOT NULL,
    replica_id integer NOT NULL
);

CREATE SEQUENCE public.channel_buffer_collaborators_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.channel_buffer_collaborators_id_seq OWNED BY public.channel_buffer_collaborators.id;

CREATE TABLE public.channel_chat_participants (
    id integer NOT NULL,
    user_id integer NOT NULL,
    channel_id integer NOT NULL,
    connection_id integer NOT NULL,
    connection_server_id integer NOT NULL
);

CREATE SEQUENCE public.channel_chat_participants_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.channel_chat_participants_id_seq OWNED BY public.channel_chat_participants.id;

CREATE TABLE public.channel_members (
    id integer NOT NULL,
    channel_id integer NOT NULL,
    user_id integer NOT NULL,
    accepted boolean DEFAULT false NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    role text NOT NULL
);

CREATE SEQUENCE public.channel_members_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.channel_members_id_seq OWNED BY public.channel_members.id;

CREATE TABLE public.channels (
    id integer NOT NULL,
    name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    visibility text DEFAULT 'members'::text NOT NULL,
    parent_path text NOT NULL,
    requires_zed_cla boolean DEFAULT false NOT NULL,
    channel_order integer DEFAULT 1 NOT NULL
);

CREATE SEQUENCE public.channels_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.channels_id_seq OWNED BY public.channels.id;

CREATE TABLE public.contacts (
    id integer NOT NULL,
    user_id_a integer NOT NULL,
    user_id_b integer NOT NULL,
    a_to_b boolean NOT NULL,
    should_notify boolean NOT NULL,
    accepted boolean NOT NULL
);

CREATE SEQUENCE public.contacts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.contacts_id_seq OWNED BY public.contacts.id;

CREATE TABLE public.contributors (
    user_id integer NOT NULL,
    signed_at timestamp without time zone DEFAULT now() NOT NULL
);

CREATE TABLE public.extension_versions (
    extension_id integer NOT NULL,
    version text NOT NULL,
    published_at timestamp without time zone DEFAULT now() NOT NULL,
    authors text NOT NULL,
    repository text NOT NULL,
    description text NOT NULL,
    download_count bigint DEFAULT 0 NOT NULL,
    schema_version integer DEFAULT 0 NOT NULL,
    wasm_api_version text,
    provides_themes boolean DEFAULT false NOT NULL,
    provides_icon_themes boolean DEFAULT false NOT NULL,
    provides_languages boolean DEFAULT false NOT NULL,
    provides_grammars boolean DEFAULT false NOT NULL,
    provides_language_servers boolean DEFAULT false NOT NULL,
    provides_context_servers boolean DEFAULT false NOT NULL,
    provides_slash_commands boolean DEFAULT false NOT NULL,
    provides_indexed_docs_providers boolean DEFAULT false NOT NULL,
    provides_snippets boolean DEFAULT false NOT NULL,
    provides_debug_adapters boolean DEFAULT false NOT NULL,
    provides_agent_servers boolean DEFAULT false NOT NULL
);

CREATE TABLE public.extensions (
    id integer NOT NULL,
    name text NOT NULL,
    external_id text NOT NULL,
    latest_version text NOT NULL,
    total_download_count bigint DEFAULT 0 NOT NULL
);

CREATE SEQUENCE public.extensions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.extensions_id_seq OWNED BY public.extensions.id;

CREATE TABLE public.feature_flags (
    id integer NOT NULL,
    flag character varying(255) NOT NULL,
    enabled_for_all boolean DEFAULT false NOT NULL
);

CREATE SEQUENCE public.feature_flags_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.feature_flags_id_seq OWNED BY public.feature_flags.id;

CREATE TABLE public.followers (
    id integer NOT NULL,
    room_id integer NOT NULL,
    project_id integer NOT NULL,
    leader_connection_server_id integer NOT NULL,
    leader_connection_id integer NOT NULL,
    follower_connection_server_id integer NOT NULL,
    follower_connection_id integer NOT NULL
);

CREATE SEQUENCE public.followers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.followers_id_seq OWNED BY public.followers.id;

CREATE TABLE public.language_servers (
    project_id integer NOT NULL,
    id bigint NOT NULL,
    name character varying NOT NULL,
    capabilities text NOT NULL,
    worktree_id bigint
);

CREATE TABLE public.notification_kinds (
    id integer NOT NULL,
    name character varying NOT NULL
);

CREATE SEQUENCE public.notification_kinds_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.notification_kinds_id_seq OWNED BY public.notification_kinds.id;

CREATE TABLE public.notifications (
    id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    recipient_id integer NOT NULL,
    kind integer NOT NULL,
    entity_id integer,
    content text,
    is_read boolean DEFAULT false NOT NULL,
    response boolean
);

CREATE SEQUENCE public.notifications_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.notifications_id_seq OWNED BY public.notifications.id;

CREATE TABLE public.observed_buffer_edits (
    user_id integer NOT NULL,
    buffer_id integer NOT NULL,
    epoch integer NOT NULL,
    lamport_timestamp integer NOT NULL,
    replica_id integer NOT NULL
);

CREATE TABLE public.project_collaborators (
    id integer NOT NULL,
    project_id integer NOT NULL,
    connection_id integer NOT NULL,
    user_id integer NOT NULL,
    replica_id integer NOT NULL,
    is_host boolean NOT NULL,
    connection_server_id integer NOT NULL,
    committer_name character varying,
    committer_email character varying
);

CREATE SEQUENCE public.project_collaborators_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.project_collaborators_id_seq OWNED BY public.project_collaborators.id;

CREATE TABLE public.project_repositories (
    project_id integer NOT NULL,
    abs_path character varying,
    id bigint NOT NULL,
    legacy_worktree_id bigint,
    entry_ids character varying,
    branch character varying,
    scan_id bigint NOT NULL,
    is_deleted boolean NOT NULL,
    current_merge_conflicts character varying,
    branch_summary character varying,
    head_commit_details character varying,
    merge_message character varying,
    remote_upstream_url character varying,
    remote_origin_url character varying
);

CREATE TABLE public.project_repository_statuses (
    project_id integer NOT NULL,
    repository_id bigint NOT NULL,
    repo_path character varying NOT NULL,
    status bigint NOT NULL,
    status_kind integer NOT NULL,
    first_status integer,
    second_status integer,
    scan_id bigint NOT NULL,
    is_deleted boolean NOT NULL
);

CREATE TABLE public.projects (
    id integer NOT NULL,
    host_user_id integer,
    unregistered boolean DEFAULT false NOT NULL,
    room_id integer,
    host_connection_id integer,
    host_connection_server_id integer,
    windows_paths boolean DEFAULT false
);

CREATE SEQUENCE public.projects_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.projects_id_seq OWNED BY public.projects.id;

CREATE TABLE public.room_participants (
    id integer NOT NULL,
    room_id integer NOT NULL,
    user_id integer NOT NULL,
    answering_connection_id integer,
    location_kind integer,
    location_project_id integer,
    initial_project_id integer,
    calling_user_id integer NOT NULL,
    calling_connection_id integer NOT NULL,
    answering_connection_lost boolean DEFAULT false NOT NULL,
    answering_connection_server_id integer,
    calling_connection_server_id integer,
    participant_index integer,
    role text
);

CREATE SEQUENCE public.room_participants_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.room_participants_id_seq OWNED BY public.room_participants.id;

CREATE TABLE public.rooms (
    id integer NOT NULL,
    live_kit_room character varying NOT NULL,
    channel_id integer
);

CREATE SEQUENCE public.rooms_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.rooms_id_seq OWNED BY public.rooms.id;

CREATE TABLE public.servers (
    id integer NOT NULL,
    environment character varying NOT NULL
);

CREATE SEQUENCE public.servers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.servers_id_seq OWNED BY public.servers.id;

CREATE TABLE public.shared_threads (
    id uuid NOT NULL,
    user_id integer NOT NULL,
    title text NOT NULL,
    data bytea NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);

CREATE TABLE public.user_features (
    user_id integer NOT NULL,
    feature_id integer NOT NULL
);

CREATE TABLE public.users (
    id integer NOT NULL,
    github_login character varying,
    admin boolean NOT NULL,
    email_address character varying(255) DEFAULT NULL::character varying,
    connected_once boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    github_user_id integer NOT NULL,
    metrics_id uuid DEFAULT gen_random_uuid() NOT NULL,
    accepted_tos_at timestamp without time zone,
    github_user_created_at timestamp without time zone,
    custom_llm_monthly_allowance_in_cents integer,
    name text
);

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;

CREATE TABLE public.worktree_diagnostic_summaries (
    project_id integer NOT NULL,
    worktree_id bigint NOT NULL,
    path character varying NOT NULL,
    language_server_id bigint NOT NULL,
    error_count integer NOT NULL,
    warning_count integer NOT NULL
);

CREATE TABLE public.worktree_entries (
    project_id integer NOT NULL,
    worktree_id bigint NOT NULL,
    id bigint NOT NULL,
    is_dir boolean NOT NULL,
    path character varying NOT NULL,
    inode bigint NOT NULL,
    mtime_seconds bigint NOT NULL,
    mtime_nanos integer NOT NULL,
    is_symlink boolean DEFAULT false NOT NULL,
    is_ignored boolean NOT NULL,
    scan_id bigint,
    is_deleted boolean,
    git_status bigint,
    is_external boolean DEFAULT false NOT NULL,
    is_fifo boolean DEFAULT false NOT NULL,
    canonical_path text,
    is_hidden boolean DEFAULT false NOT NULL
);

CREATE TABLE public.worktree_settings_files (
    project_id integer NOT NULL,
    worktree_id bigint NOT NULL,
    path character varying NOT NULL,
    content text NOT NULL,
    kind character varying
);

CREATE TABLE public.worktrees (
    project_id integer NOT NULL,
    id bigint NOT NULL,
    root_name character varying NOT NULL,
    abs_path character varying NOT NULL,
    visible boolean NOT NULL,
    scan_id bigint NOT NULL,
    is_complete boolean DEFAULT false NOT NULL,
    completed_scan_id bigint
);

ALTER TABLE ONLY public.access_tokens ALTER COLUMN id SET DEFAULT nextval('public.access_tokens_id_seq'::regclass);

ALTER TABLE ONLY public.breakpoints ALTER COLUMN id SET DEFAULT nextval('public.breakpoints_id_seq'::regclass);

ALTER TABLE ONLY public.buffers ALTER COLUMN id SET DEFAULT nextval('public.buffers_id_seq'::regclass);

ALTER TABLE ONLY public.channel_buffer_collaborators ALTER COLUMN id SET DEFAULT nextval('public.channel_buffer_collaborators_id_seq'::regclass);

ALTER TABLE ONLY public.channel_chat_participants ALTER COLUMN id SET DEFAULT nextval('public.channel_chat_participants_id_seq'::regclass);

ALTER TABLE ONLY public.channel_members ALTER COLUMN id SET DEFAULT nextval('public.channel_members_id_seq'::regclass);

ALTER TABLE ONLY public.channels ALTER COLUMN id SET DEFAULT nextval('public.channels_id_seq'::regclass);

ALTER TABLE ONLY public.contacts ALTER COLUMN id SET DEFAULT nextval('public.contacts_id_seq'::regclass);

ALTER TABLE ONLY public.extensions ALTER COLUMN id SET DEFAULT nextval('public.extensions_id_seq'::regclass);

ALTER TABLE ONLY public.feature_flags ALTER COLUMN id SET DEFAULT nextval('public.feature_flags_id_seq'::regclass);

ALTER TABLE ONLY public.followers ALTER COLUMN id SET DEFAULT nextval('public.followers_id_seq'::regclass);

ALTER TABLE ONLY public.notification_kinds ALTER COLUMN id SET DEFAULT nextval('public.notification_kinds_id_seq'::regclass);

ALTER TABLE ONLY public.notifications ALTER COLUMN id SET DEFAULT nextval('public.notifications_id_seq'::regclass);

ALTER TABLE ONLY public.project_collaborators ALTER COLUMN id SET DEFAULT nextval('public.project_collaborators_id_seq'::regclass);

ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public.projects_id_seq'::regclass);

ALTER TABLE ONLY public.room_participants ALTER COLUMN id SET DEFAULT nextval('public.room_participants_id_seq'::regclass);

ALTER TABLE ONLY public.rooms ALTER COLUMN id SET DEFAULT nextval('public.rooms_id_seq'::regclass);

ALTER TABLE ONLY public.servers ALTER COLUMN id SET DEFAULT nextval('public.servers_id_seq'::regclass);

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);

ALTER TABLE ONLY public.access_tokens
    ADD CONSTRAINT access_tokens_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.breakpoints
    ADD CONSTRAINT breakpoints_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.buffer_operations
    ADD CONSTRAINT buffer_operations_pkey PRIMARY KEY (buffer_id, epoch, lamport_timestamp, replica_id);

ALTER TABLE ONLY public.buffer_snapshots
    ADD CONSTRAINT buffer_snapshots_pkey PRIMARY KEY (buffer_id, epoch);

ALTER TABLE ONLY public.buffers
    ADD CONSTRAINT buffers_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.channel_buffer_collaborators
    ADD CONSTRAINT channel_buffer_collaborators_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.channel_chat_participants
    ADD CONSTRAINT channel_chat_participants_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.channel_members
    ADD CONSTRAINT channel_members_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.channels
    ADD CONSTRAINT channels_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.contributors
    ADD CONSTRAINT contributors_pkey PRIMARY KEY (user_id);

ALTER TABLE ONLY public.extension_versions
    ADD CONSTRAINT extension_versions_pkey PRIMARY KEY (extension_id, version);

ALTER TABLE ONLY public.extensions
    ADD CONSTRAINT extensions_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.feature_flags
    ADD CONSTRAINT feature_flags_flag_key UNIQUE (flag);

ALTER TABLE ONLY public.feature_flags
    ADD CONSTRAINT feature_flags_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.followers
    ADD CONSTRAINT followers_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.language_servers
    ADD CONSTRAINT language_servers_pkey PRIMARY KEY (project_id, id);

ALTER TABLE ONLY public.notification_kinds
    ADD CONSTRAINT notification_kinds_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.observed_buffer_edits
    ADD CONSTRAINT observed_buffer_edits_pkey PRIMARY KEY (user_id, buffer_id);

ALTER TABLE ONLY public.project_collaborators
    ADD CONSTRAINT project_collaborators_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.project_repositories
    ADD CONSTRAINT project_repositories_pkey PRIMARY KEY (project_id, id);

ALTER TABLE ONLY public.project_repository_statuses
    ADD CONSTRAINT project_repository_statuses_pkey PRIMARY KEY (project_id, repository_id, repo_path);

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.servers
    ADD CONSTRAINT servers_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.shared_threads
    ADD CONSTRAINT shared_threads_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.user_features
    ADD CONSTRAINT user_features_pkey PRIMARY KEY (user_id, feature_id);

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.worktree_diagnostic_summaries
    ADD CONSTRAINT worktree_diagnostic_summaries_pkey PRIMARY KEY (project_id, worktree_id, path);

ALTER TABLE ONLY public.worktree_entries
    ADD CONSTRAINT worktree_entries_pkey PRIMARY KEY (project_id, worktree_id, id);

ALTER TABLE ONLY public.worktree_settings_files
    ADD CONSTRAINT worktree_settings_files_pkey PRIMARY KEY (project_id, worktree_id, path);

ALTER TABLE ONLY public.worktrees
    ADD CONSTRAINT worktrees_pkey PRIMARY KEY (project_id, id);

CREATE INDEX idx_shared_threads_user_id ON public.shared_threads USING btree (user_id);

CREATE INDEX index_access_tokens_user_id ON public.access_tokens USING btree (user_id);

CREATE INDEX index_breakpoints_on_project_id ON public.breakpoints USING btree (project_id);

CREATE INDEX index_buffers_on_channel_id ON public.buffers USING btree (channel_id);

CREATE INDEX index_channel_buffer_collaborators_on_channel_id ON public.channel_buffer_collaborators USING btree (channel_id);

CREATE UNIQUE INDEX index_channel_buffer_collaborators_on_channel_id_and_replica_id ON public.channel_buffer_collaborators USING btree (channel_id, replica_id);

CREATE UNIQUE INDEX index_channel_buffer_collaborators_on_channel_id_connection_id_ ON public.channel_buffer_collaborators USING btree (channel_id, connection_id, connection_server_id);

CREATE INDEX index_channel_buffer_collaborators_on_connection_id ON public.channel_buffer_collaborators USING btree (connection_id);

CREATE INDEX index_channel_buffer_collaborators_on_connection_server_id ON public.channel_buffer_collaborators USING btree (connection_server_id);

CREATE INDEX index_channel_chat_participants_on_channel_id ON public.channel_chat_participants USING btree (channel_id);

CREATE UNIQUE INDEX index_channel_members_on_channel_id_and_user_id ON public.channel_members USING btree (channel_id, user_id);

CREATE INDEX index_channels_on_parent_path ON public.channels USING btree (parent_path text_pattern_ops);

CREATE INDEX index_channels_on_parent_path_and_order ON public.channels USING btree (parent_path, channel_order);

CREATE INDEX index_contacts_user_id_b ON public.contacts USING btree (user_id_b);

CREATE UNIQUE INDEX index_contacts_user_ids ON public.contacts USING btree (user_id_a, user_id_b);

CREATE UNIQUE INDEX index_extensions_external_id ON public.extensions USING btree (external_id);

CREATE INDEX index_extensions_total_download_count ON public.extensions USING btree (total_download_count);

CREATE UNIQUE INDEX index_feature_flags ON public.feature_flags USING btree (id);

CREATE UNIQUE INDEX index_followers_on_project_id_and_leader_connection_server_id_a ON public.followers USING btree (project_id, leader_connection_server_id, leader_connection_id, follower_connection_server_id, follower_connection_id);

CREATE INDEX index_followers_on_room_id ON public.followers USING btree (room_id);

CREATE INDEX index_language_servers_on_project_id ON public.language_servers USING btree (project_id);

CREATE UNIQUE INDEX index_notification_kinds_on_name ON public.notification_kinds USING btree (name);

CREATE INDEX index_notifications_on_recipient_id_is_read_kind_entity_id ON public.notifications USING btree (recipient_id, is_read, kind, entity_id);

CREATE UNIQUE INDEX index_observed_buffer_user_and_buffer_id ON public.observed_buffer_edits USING btree (user_id, buffer_id);

CREATE INDEX index_project_collaborators_on_connection_id ON public.project_collaborators USING btree (connection_id);

CREATE INDEX index_project_collaborators_on_connection_server_id ON public.project_collaborators USING btree (connection_server_id);

CREATE INDEX index_project_collaborators_on_project_id ON public.project_collaborators USING btree (project_id);

CREATE UNIQUE INDEX index_project_collaborators_on_project_id_and_replica_id ON public.project_collaborators USING btree (project_id, replica_id);

CREATE UNIQUE INDEX index_project_collaborators_on_project_id_connection_id_and_ser ON public.project_collaborators USING btree (project_id, connection_id, connection_server_id);

CREATE INDEX index_project_repos_statuses_on_project_id ON public.project_repository_statuses USING btree (project_id);

CREATE INDEX index_project_repos_statuses_on_project_id_and_repo_id ON public.project_repository_statuses USING btree (project_id, repository_id);

CREATE INDEX index_project_repositories_on_project_id ON public.project_repositories USING btree (project_id);

CREATE INDEX index_projects_on_host_connection_id_and_host_connection_server ON public.projects USING btree (host_connection_id, host_connection_server_id);

CREATE INDEX index_projects_on_host_connection_server_id ON public.projects USING btree (host_connection_server_id);

CREATE INDEX index_room_participants_on_answering_connection_id ON public.room_participants USING btree (answering_connection_id);

CREATE UNIQUE INDEX index_room_participants_on_answering_connection_id_and_answerin ON public.room_participants USING btree (answering_connection_id, answering_connection_server_id);

CREATE INDEX index_room_participants_on_answering_connection_server_id ON public.room_participants USING btree (answering_connection_server_id);

CREATE INDEX index_room_participants_on_calling_connection_server_id ON public.room_participants USING btree (calling_connection_server_id);

CREATE INDEX index_room_participants_on_room_id ON public.room_participants USING btree (room_id);

CREATE UNIQUE INDEX index_room_participants_on_user_id ON public.room_participants USING btree (user_id);

CREATE UNIQUE INDEX index_rooms_on_channel_id ON public.rooms USING btree (channel_id);

CREATE INDEX index_settings_files_on_project_id ON public.worktree_settings_files USING btree (project_id);

CREATE INDEX index_settings_files_on_project_id_and_wt_id ON public.worktree_settings_files USING btree (project_id, worktree_id);

CREATE INDEX index_user_features_on_feature_id ON public.user_features USING btree (feature_id);

CREATE INDEX index_user_features_on_user_id ON public.user_features USING btree (user_id);

CREATE UNIQUE INDEX index_user_features_user_id_and_feature_id ON public.user_features USING btree (user_id, feature_id);

CREATE UNIQUE INDEX index_users_github_login ON public.users USING btree (github_login);

CREATE INDEX index_users_on_email_address ON public.users USING btree (email_address);

CREATE INDEX index_worktree_diagnostic_summaries_on_project_id ON public.worktree_diagnostic_summaries USING btree (project_id);

CREATE INDEX index_worktree_diagnostic_summaries_on_project_id_and_worktree_ ON public.worktree_diagnostic_summaries USING btree (project_id, worktree_id);

CREATE INDEX index_worktree_entries_on_project_id ON public.worktree_entries USING btree (project_id);

CREATE INDEX index_worktree_entries_on_project_id_and_worktree_id ON public.worktree_entries USING btree (project_id, worktree_id);

CREATE INDEX index_worktrees_on_project_id ON public.worktrees USING btree (project_id);

CREATE INDEX trigram_index_extensions_name ON public.extensions USING gin (name public.gin_trgm_ops);

CREATE INDEX trigram_index_users_on_github_login ON public.users USING gin (github_login public.gin_trgm_ops);

CREATE UNIQUE INDEX uix_channels_parent_path_name ON public.channels USING btree (parent_path, name) WHERE ((parent_path IS NOT NULL) AND (parent_path <> ''::text));

CREATE UNIQUE INDEX uix_users_on_github_user_id ON public.users USING btree (github_user_id);

ALTER TABLE ONLY public.access_tokens
    ADD CONSTRAINT access_tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.breakpoints
    ADD CONSTRAINT breakpoints_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.buffer_operations
    ADD CONSTRAINT buffer_operations_buffer_id_fkey FOREIGN KEY (buffer_id) REFERENCES public.buffers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.buffer_snapshots
    ADD CONSTRAINT buffer_snapshots_buffer_id_fkey FOREIGN KEY (buffer_id) REFERENCES public.buffers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.buffers
    ADD CONSTRAINT buffers_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_buffer_collaborators
    ADD CONSTRAINT channel_buffer_collaborators_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_buffer_collaborators
    ADD CONSTRAINT channel_buffer_collaborators_connection_server_id_fkey FOREIGN KEY (connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_buffer_collaborators
    ADD CONSTRAINT channel_buffer_collaborators_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_chat_participants
    ADD CONSTRAINT channel_chat_participants_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_chat_participants
    ADD CONSTRAINT channel_chat_participants_connection_server_id_fkey FOREIGN KEY (connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_chat_participants
    ADD CONSTRAINT channel_chat_participants_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);

ALTER TABLE ONLY public.channel_members
    ADD CONSTRAINT channel_members_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.channel_members
    ADD CONSTRAINT channel_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_user_id_a_fkey FOREIGN KEY (user_id_a) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_user_id_b_fkey FOREIGN KEY (user_id_b) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.contributors
    ADD CONSTRAINT contributors_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);

ALTER TABLE ONLY public.extension_versions
    ADD CONSTRAINT extension_versions_extension_id_fkey FOREIGN KEY (extension_id) REFERENCES public.extensions(id);

ALTER TABLE ONLY public.project_repositories
    ADD CONSTRAINT fk_project_repositories_project_id FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.project_repository_statuses
    ADD CONSTRAINT fk_project_repository_statuses_project_id FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.followers
    ADD CONSTRAINT followers_follower_connection_server_id_fkey FOREIGN KEY (follower_connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.followers
    ADD CONSTRAINT followers_leader_connection_server_id_fkey FOREIGN KEY (leader_connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.followers
    ADD CONSTRAINT followers_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.followers
    ADD CONSTRAINT followers_room_id_fkey FOREIGN KEY (room_id) REFERENCES public.rooms(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.language_servers
    ADD CONSTRAINT language_servers_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_kind_fkey FOREIGN KEY (kind) REFERENCES public.notification_kinds(id);

ALTER TABLE ONLY public.notifications
    ADD CONSTRAINT notifications_recipient_id_fkey FOREIGN KEY (recipient_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.observed_buffer_edits
    ADD CONSTRAINT observed_buffer_edits_buffer_id_fkey FOREIGN KEY (buffer_id) REFERENCES public.buffers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.observed_buffer_edits
    ADD CONSTRAINT observed_buffer_edits_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.project_collaborators
    ADD CONSTRAINT project_collaborators_connection_server_id_fkey FOREIGN KEY (connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.project_collaborators
    ADD CONSTRAINT project_collaborators_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_host_connection_server_id_fkey FOREIGN KEY (host_connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_host_user_id_fkey FOREIGN KEY (host_user_id) REFERENCES public.users(id);

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_room_id_fkey FOREIGN KEY (room_id) REFERENCES public.rooms(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_answering_connection_server_id_fkey FOREIGN KEY (answering_connection_server_id) REFERENCES public.servers(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_calling_connection_server_id_fkey FOREIGN KEY (calling_connection_server_id) REFERENCES public.servers(id) ON DELETE SET NULL;

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_calling_user_id_fkey FOREIGN KEY (calling_user_id) REFERENCES public.users(id);

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_room_id_fkey FOREIGN KEY (room_id) REFERENCES public.rooms(id);

ALTER TABLE ONLY public.room_participants
    ADD CONSTRAINT room_participants_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT rooms_channel_id_fkey FOREIGN KEY (channel_id) REFERENCES public.channels(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.shared_threads
    ADD CONSTRAINT shared_threads_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.user_features
    ADD CONSTRAINT user_features_feature_id_fkey FOREIGN KEY (feature_id) REFERENCES public.feature_flags(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.user_features
    ADD CONSTRAINT user_features_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.worktree_diagnostic_summaries
    ADD CONSTRAINT worktree_diagnostic_summaries_project_id_worktree_id_fkey FOREIGN KEY (project_id, worktree_id) REFERENCES public.worktrees(project_id, id) ON DELETE CASCADE;

ALTER TABLE ONLY public.worktree_entries
    ADD CONSTRAINT worktree_entries_project_id_worktree_id_fkey FOREIGN KEY (project_id, worktree_id) REFERENCES public.worktrees(project_id, id) ON DELETE CASCADE;

ALTER TABLE ONLY public.worktree_settings_files
    ADD CONSTRAINT worktree_settings_files_project_id_worktree_id_fkey FOREIGN KEY (project_id, worktree_id) REFERENCES public.worktrees(project_id, id) ON DELETE CASCADE;

ALTER TABLE ONLY public.worktrees
    ADD CONSTRAINT worktrees_project_id_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON DELETE CASCADE;
