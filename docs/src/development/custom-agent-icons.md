# Custom Agent Icons

This document describes how custom ACP agent icons should reuse Zed's existing
external agent icon infrastructure. It covers icons configured in
`settings.json` and a later extension for icons declared by ACP agents.

## Goals {#custom-agent-icon-goals}

- Let a custom agent use an SVG icon selected in the External Agents settings
  UI or configured directly in `settings.json`.
- Reuse `ExternalAgentEntry::icon`, `AgentServerStore::agent_icon`, and
  `Icon::from_external_svg`, which already render ACP Registry icons.
- Keep icon failures independent from agent startup and ACP connection errors.
- Define a path from local configuration to a future ACP-standard icon field.

This work does not add another icon registry, raster image support, or an icon
field to the `AgentServer` behavior trait.

## Existing Architecture {#custom-agent-icon-existing-architecture}

ACP Registry icons currently follow this path:

```text
Registry entry icon URL
    -> AgentRegistryStore download and cache
    -> RegistryAgentMetadata.icon_path
    -> ExternalAgentEntry.icon
    -> AgentServerStore::agent_icon()
    -> Icon::from_external_svg()
```

The last two steps are already used by the agent selector, active conversation,
thread import, archived threads, the External Agents settings page, and the
sidebar. A custom agent should enter this path at `ExternalAgentEntry.icon`. UI
call sites do not need a second custom agent icon implementation.

The main ownership boundaries are:

- `settings_content` owns the serialized setting and generated JSON schema.
- `project::AgentServerStore` resolves configured agent metadata and exposes it
  to the UI.
- `agent_servers` owns process startup and ACP communication, not configured
  icon storage.
- `ui::Icon` and GPUI own SVG loading and rendering.

## Phase One: Configured Local Icons {#custom-agent-icon-local-configuration}

Add an optional `icon` property to custom agent settings:

```json [settings]
{
  "agent_servers": {
    "my-agent": {
      "type": "custom",
      "command": "/absolute/path/to/my-agent",
      "args": ["--acp"],
      "env": {},
      "icon": "/absolute/path/to/my-agent.svg"
    }
  }
}
```

`icon` is optional. Omitting it preserves the current fallback icon. It changes
only presentation; `command`, `args`, `env`, modes, config options, process
startup, and ACP behavior remain unchanged.

### Path rules {#custom-agent-icon-path-rules}

The path must:

- be an absolute filesystem path or a tilde-prefixed home path (`~/...`) according to the current platform;
- identify an SVG file;
- be readable by the Zed UI process.

Tilde (`~`) expansion is supported using `shellexpand::tilde`, consistent with
the `command` executable path in `CustomAgentServerSettings::Custom`. Relative
paths (such as `./` or `../`) are not supported to avoid ambiguous resolution
against the settings directory, project root, current working directory, or agent
executable.

The Settings UI exposes an optional **Icon** field with an **Upload** button to
browse and select a local monochrome `.svg` icon. Uploading validates the SVG and
copies a persistent copy to Zed's `external_agents/icons/` directory, saving the
persisted path in `settings.json` so the icon remains intact even if the original
file is deleted or moved. Saving the form also automatically copies any manually
entered path to `external_agents/icons/`.

These names refer to the same value at different layers:

- `icon` is the public `settings.json` property.
- **Icon** is the Settings UI field label.
- `icon_path` may be used as an internal Rust variable name, but is not a
  serialized setting.

An unreadable file or SVG parse failure must not prevent the agent from being
listed or started. Zed logs the failure and renders the existing fallback icon.

### SVG input contract {#custom-agent-icon-svg-contract}

Custom agent icons use `Icon::from_external_svg`. GPUI parses the SVG with
`usvg`, rasterizes it to an alpha mask, and paints that mask with the semantic
icon color selected by the surrounding UI. Consequently:

- Zed discards the SVG's RGB colors during icon painting. Multiple source
  colors do not remain distinct.
- Authors do not need to run a color-removal step before importing an icon.
- Icons should nevertheless be authored as monochrome SVGs with transparent
  backgrounds. Use `fill="currentColor"` or `stroke="currentColor"` so the
  file also behaves correctly in other conforming SVG consumers.
- A filled background rectangle becomes a filled square after alpha-mask
  rendering and should be removed unless it is intentionally part of the icon.
- Alpha and opacity differences are retained. They should not be required for
  the icon to remain understandable.

Use a self-contained, static SVG with a square view box. The ACP Registry
convention is the preferred input format: 16 by 16, monochrome, and based on
`currentColor`. For example:

```svg
<svg
  xmlns="http://www.w3.org/2000/svg"
  width="16"
  height="16"
  viewBox="0 0 16 16"
  fill="none"
>
  <path
    d="M3 8h10M8 3v10"
    stroke="currentColor"
    stroke-width="1.5"
    stroke-linecap="round"
  />
</svg>
```

Avoid scripts, animation, linked images, external stylesheets, and remote
resources. Convert text to paths when the exact shape matters, because font
availability differs by platform. A `viewBox` is important for predictable
scaling; GPUI preserves the source aspect ratio and centers the result in the
icon bounds.

Zed should not rewrite or sanitize the SVG in phase one. It should validate the
absolute path and `.svg` extension, then rely on the existing GPUI parser and
renderer. This keeps configured icons and Registry icons on the same rendering
path.

### Data flow {#custom-agent-icon-data-flow}

The serialized settings type adds `icon: Option<PathBuf>` to the `Custom`
variant. The project-layer settings conversion retains the path. When
`AgentServerStore` rebuilds its external agent entries, it supplies the path to
`ExternalAgentEntry::new`. Existing consumers then obtain it through
`AgentServerStore::agent_icon()`.

Configured icons have the following precedence:

1. An explicit `icon` in custom agent settings.
2. An icon supplied by the ACP Registry for a Registry agent.
3. In phase two, an icon declared by the connected ACP agent.
4. Zed's existing fallback icon.

The first phase does not add `icon` to Registry agent settings. Registry agents
continue to use Registry metadata.

### Remote development {#custom-agent-icon-remote-development}

An external SVG path is opened by the UI process. A path on an SSH host cannot
be sent to a local UI and rendered as though it were a local path. The current
`ExternalAgentsUpdated` message also carries only agent names.

Phase one therefore treats `icon` as a path on the machine running the Zed UI.
It must not send an absolute filesystem path across the remote-project protocol.
If custom agent settings are sourced only from the remote host, the UI falls
back to the default icon. Full remote support requires reading validated SVG
bytes on the remote host, transferring bounded content, and caching it on the
UI host. That should be implemented with the same client-side icon cache used
for agent-declared icons rather than exposing remote paths.

## Phase Two: Agent-Declared Icons {#custom-agent-icon-acp-declaration}

ACP 2.1 implementation information contains `name`, `title`, and `version`, but
does not contain a standard icon field. The preferred long-term result is a
standard ACP field whose SVG requirements match the ACP Registry.

Before such a field is standardized, Zed can experiment with a namespaced
value in the initialize response `_meta` object. The value should be a URL, not
a filesystem path, so it has the same meaning when the agent runs locally, over
SSH, or in a development container. For example:

```json
{
  "_meta": {
    "dev.zed.agent-icon": {
      "url": "https://example.com/my-agent.svg"
    }
  }
}
```

The exact experimental key and shape must be reviewed against the current ACP
extensibility guidance before implementation. Isolating parsing behind a small
resolver makes migration to a future standard field mechanical.

The client should accept only HTTPS SVG resources, enforce request and body-size
limits, cache the validated bytes under Zed's external agent data directory,
update the matching `ExternalAgentEntry.icon`, and emit `AgentServersUpdated`.
Download or validation failures must fall back without failing ACP
initialization.

## Implementation Scope {#custom-agent-icon-implementation-scope}

Phase one changes these files:

- `crates/settings_content/src/agent.rs`: add the serialized optional `icon`
  path and schema documentation.
- `crates/project/src/agent_server_store.rs`: retain, validate, and expose the
  custom icon through `ExternalAgentEntry`.
- `crates/settings_ui/src/pages/external_agents_page.rs`: add the Icon Path
  field, preserve it during edits, and validate absolute paths.
- `docs/src/ai/external-agents.md`: document the user-facing setting after the
  feature is implemented.
- Test construction sites in `crates/agent_servers/src/acp.rs` and
  `crates/agent_ui/src/agent_panel.rs`: initialize the new optional field.

The existing icon consumers in `agent_ui`, `sidebar`, and `ui` should not need
feature-specific changes.

Phase two is expected to touch:

- `crates/agent_servers/src/acp.rs` to read icon metadata from initialization;
- `crates/project/src/agent_server_store.rs` to update shared agent metadata;
- `crates/project/src/agent_registry_store.rs`, or a nearby shared module, to
  extract reusable SVG fetching and caching behavior;
- internal remote protocol types only if icon bytes must cross a Zed remote
  connection.

## Verification {#custom-agent-icon-verification}

Phase one should include tests for:

- settings parsing with and without `icon`;
- rejection of relative paths and non-SVG paths by the Settings UI;
- preservation of `icon` when an existing agent is edited;
- propagation from settings into `AgentServerStore::agent_icon()`;
- fallback behavior for missing and malformed SVG files;
- unchanged Registry icon behavior;
- light and dark theme rendering of a `currentColor` icon.

Phase two should additionally test metadata precedence, unsupported metadata,
URL and size validation, download failures, cache reuse, reconnect updates, and
remote projects.

## Design Revisions & Discrepancy Record {#custom-agent-icon-revisions}

This section explicitly documents the architectural review refinements and
differences compared to the original revision of this specification:

### 1. Tilde (`~`) Path Expansion Support
- **Original Plan**: Forbade all `~` expansion, requiring strict absolute paths
  like `/Users/...`.
- **Revision**: Expanded via `shellexpand::tilde`, aligning with how the `command`
  executable path is handled in `CustomAgentServerSettings::Custom`
  (`crates/project/src/agent_server_store.rs`). Relative paths (`./`, `../`)
  remain strictly disallowed.
- **Rationale**: User home directories vary across machines. Disallowing `~`
  breaks portability when syncing `settings.json` across multiple workstations
  (e.g., macOS and Linux laptops). Since `command` already supports `shellexpand::tilde`,
  supporting it for `icon` eliminates arbitrary user inconsistency while maintaining
  deterministic resolution.

### 2. Phase Two Offline & Local CLI Agent Icon Support
- **Original Plan**: Allowed only remote HTTPS URLs (`https://...`).
- **Revision**: In addition to HTTPS URLs, support data URIs (`data:image/svg+xml;base64,...`)
  or inline SVG payloads (bounded to 64KB) in `_meta`.
- **Rationale**: Many custom ACP agents run purely as local command-line tools
  without external hosting or internet connectivity. Requiring an HTTPS endpoint
  prevents air-gapped or local-only agents from declaring their icons. Inline
  data is validated and written directly to the external agent icon cache directory.

### 3. ACP Metadata Location Hierarchy
- **Original Plan**: Only specified root-level `_meta`.
- **Revision**: Check `response.agent_info.and_then(|info| info.meta)` first,
  falling back to root `response.meta`.
- **Rationale**: In the ACP wire format, `agent_info` represents the agent's
  implementation identity (`Implementation` struct). Scoping agent presentation
  metadata to `agent_info` is semantically cleaner and standard across ACP
  implementations, while checking root `_meta` ensures backward compatibility.

### 4. Shared Icon Download and Cache Infrastructure
- **Original Plan**: Suggested touching `AgentRegistryStore` or a nearby module.
- **Revision**: Factor out icon fetching, size/type validation, and disk caching
  into a shared helper module in `crates/project`, shared between `AgentRegistryStore`
  and ACP agent runtime self-declaration.
- **Rationale**: Avoids code duplication and guarantees that Registry icons and
  ACP-declared icons follow identical cache directory hierarchies, timeouts, and
  validation semantics.

