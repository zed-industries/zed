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
