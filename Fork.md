# Fork Divergences

## Agent Profiles settings page (`settings_ui`)

**Purpose.** Expose the profile management of `ManageProfilesModal` as a
Settings child page under AI → Agent Configuration → Agent Profiles, reading
and writing the same `agent.profiles` / `agent.default_profile` user settings
schema. The modal, `agent::ManageProfiles`, and all upstream CRUD semantics are
unchanged.

**Touched symbols.**

- `crates/settings_ui/src/pages/agent_profiles_page.rs` — list/detail/form
  render functions, `AgentProfilePageState`, `AgentProfileForm`,
  `sync_agent_profile_page_state`, write helpers (`write_profile`,
  `set_builtin_tool_enabled`, `set_context_server_tool_enabled`,
  `set_enable_all_context_servers`, `set_profile_default_model`,
  `clear_profile_default_model`, `delete_agent_profile`), telemetry guards
  (`mark_*_configured`).
- `crates/settings_ui/src/components/language_model_picker.rs` —
  `render_language_model_picker` and `LanguageModelPickerDelegate` (sources
  mirror `agent_ui`'s `LanguageModelSelector::all_models`/`update_matches`).
- `crates/settings_ui/src/page_data.rs` — `Agent Profiles` `SubPageLink` after
  `Tool Permissions` in `agent_configuration_section`.
- `crates/settings_ui/src/settings_ui.rs` — `agent_profile_page` /
  `agent_profile_form` fields, `is_agent_profiles_page` header detection,
  `Add Profile` button, one `sync_agent_profile_page_state` call in the
  `SettingsStore` observer.
- `crates/settings_ui/src/{pages.rs,components.rs,Cargo.toml}` — registration
  and the `convert_case` workspace dependency (same derivation as
  `AgentProfile::create`).

**Upstream behaviors preserved.**

- All reads use `AgentSettings::get_global(cx).profiles` / `AgentProfile::
  available_profiles`; all writes go through `settings::update_settings_file`
  and produce exactly the `AgentProfileContent` shape that
  `AgentProfileSettings::save_to_settings` and `ToolPickerDelegate::confirm`
  write (`name`, `tools`, `enable_all_context_servers`,
  `context_servers.<server>.tools.<tool>`, `default_model`).
- Create/fork use `AgentProfile::create` unchanged (kebab id, copied
  tools/MCP/default model). Delete resets `agent.default_profile` to `write`
  when the deleted profile was the default; built-ins are not deleted.
- Telemetry event names and fields match the modal:
  `Agent Profile Created` (`profile_id`, `is_fork`, `base_profile_id`),
  `Agent Profile Deleted` (`profile_id`), `Agent Profile Tools/MCPs/
  Default Model Configured` (`profile_id`, `is_builtin`).

**Intentional divergences (D1–D6).**

- D1: the list orders profiles as `AgentProfile::available_profiles`
  (built-ins Write/Ask/Minimal, then custom in settings order), not the
  modal's alphabetical sort.
- D2: MCP tool toggles write the new switch state computed via
  `is_context_server_tool_enabled`; the modal ToolPicker first-click defect
  (writes `!false` under `enable_all_context_servers = true`) is not
  reproduced.
- D3: selecting or clearing a default model on an unmodified built-in
  materializes the full profile first (the modal's write silently no-ops
  there — F5).
- D4: `Clear` removes `default_model` and `Enable All MCP Servers` toggles
  `enable_all_context_servers`; neither exists in the modal (F8).
- D5: each `… Configured` event fires at most once per detail-page opening,
  on the first write of that section; the modal fires on screen open.
- D6: the add/fork form rejects empty names, symbol-only names (kebab id has
  no alphanumeric) and duplicate ids; the modal accepts them.
- The built-in tools list is not filtered by
  `agent::tool_supports_provider` (the modal's `active_model = None` path):
  when a native thread has a model selected, the modal's tool picker hides
  provider-unsupported tools and this page shows them.
- The built-in one-line descriptions duplicate `ProfileSelector::
  documentation` to keep `settings_ui` independent of `agent_ui`.

**Verification.**

- `cargo test -p settings_ui` — 86 tests incl. 10 new in
  `pages::agent_profiles_page::tests` (create/fork/delete JSON shape,
  materialization, MCP explicit `false`, model select+clear, name validation,
  external-delete pop, awaiting-creation stability).
- `./script/clippy` — clean.
- Prettier check on `docs/src/ai/agent-profiles.md` — clean.

**Upstream revision last verified:** `83323324a4`.

## External-agent model catalog (`project` + `agent_ui`)

**Purpose.** Per-project, in-memory catalog of the models exposed by each
installed external agent. `project` owns the dependency-free contract so
consumers such as `settings_ui` never depend on `agent_ui`; `agent_ui`
registers a source that populates the catalog from open `AgentPanel`
sessions or short-lived probe sessions.

**Touched symbols.**

- `crates/project/src/external_agent_models.rs` — `ExternalAgentModel`,
  `ExternalAgentModelsState`, `ExternalAgentModelCatalog`
  (`state`/`set_state`/`set_task`/`clear`),
  `set_external_agent_model_source`, `external_agent_model_catalog`,
  `refresh_external_agent_models`; a private `GlobalExternalAgentModelCatalogs`
  keyed by `EntityId` that drops entries whose project weak ref no longer
  upgrades.
- `crates/agent_ui/src/external_agent_models.rs` — `init` (one call from
  `agent_ui::init`), the source `refresh`, `find_agent_panel`
  (`workspace::AppState` workspace scan), `probe_agent`/`run_probe`/
  `run_probe_session`/`read_models`/`close_probe_session` helpers, and the
  `CatalogTestConnection`/`CatalogTestConfigOptions`/`CatalogTestModelSelector`/
  `CatalogTestSessionList` test types.

**Upstream behaviors preserved.**

- Connections are only ever obtained through the panel's
  `AgentConnectionStore::request_connection`; no external-agent process is
  spawned independently, and `Agent::server`/`CustomAgentServer` keep owning
  command construction.
- `AgentServersUpdated` still owns agent lifecycle: the catalog subscribes to
  it and clears all states and pending probe tasks rather than duplicating
  the update logic.
- `AuthRequired::from_error` remains the single classifier for auth errors;
  the probe never calls `authenticate`.

**Intentional divergences.**

- Probing an agent with no open session creates a throwaway ACP session
  (no `ConversationView`, sidebar entry, or `ThreadMetadataStore` row) to
  read its model list. The whole probe — the pending connection wait plus
  `new_session` — is bounded by one 30s executor-timer timeout; a
  late-resolving session is closed but its result is discarded, and a
  timed-out `Failed` entry is retried on the next refresh against the same
  pending `AgentConnectionStore` entry.
- Probe sessions are torn down via `close_session` (when
  `supports_close_session`) and `delete_session` (when the agent's session
  list supports deletion). **Approved limitation:** when an agent supports
  neither, the probe session may remain visible in that agent's
  history/import listings until the agent removes it.
- **Approved limitation:** cataloged external models are not yet applied
  to external sessions. Prompt 2b will persist the selection through the
  existing `LanguageModelSelection` `default_model` profile field with
  `provider = <AgentId>`; nothing reads that field for external agents
  today.
- States `Ready`/`AuthRequired`/`Loading` are not reprobed on subsequent
  refreshes; `Failed` is retried.

**Upstream code to re-check on upstream updates.**

- `AgentConnectionStore::request_connection` /
  `AgentConnectionStore::handle_agent_servers_updated` — the catalog relies
  on pending-entry reuse and on server updates clearing stale entries.
- `ConversationView::initial_state` — `is_via_collab` gating,
  `default_path_list` work dirs, and `AuthRequired::from_error`
  classification must keep matching how sessions are normally opened.
- The config-option vs `model_selector` precedence in `ConversationView`
  (around `:1636-1712`).
- `config_options.rs` — how the `Model` category `Select` option is
  located (`first_config_option_id_matching`).
- `AgentModelList` / `AgentModelInfo.disabled` semantics.
- `AgentConnection::close_session` / `AgentSessionList::delete_session`
  capability checks.
- `thread_import.rs` — where leftover probe sessions appear in
  history/import listings.
- Emitters of `AgentServersUpdated` — catalog invalidation depends on it
  firing whenever the installed-agent set changes.

**Verification.**

- `cargo test -p project external_agent_models` — 4 tests (no source,
  single source call + notifications, `AgentServersUpdated` clear,
  released-project purge).
- `cargo test -p agent_ui external_agent_models` — 16 tests (config-option
  extraction, selector fallback, auth, connect/session timeout + late
  cleanup, `AgentServersUpdated` mid-probe discard, loading-skip,
  open-session reuse, retry rules, close/delete, no side effects, collab +
  missing-panel messages).
- `./script/clippy` — clean.

**Upstream revision last verified:** `83323324a4`.
