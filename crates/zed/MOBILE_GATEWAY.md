# Mobile Gateway

The Mobile Gateway is an opt-in, private-network companion API inside the running `Zed-Desktop` process. It is designed for a paired Android device to select an already-open workspace or reopen a known local, SSH, or Dev Container workspace from Zed's desktop history, continue a native Agent thread, review its complete retained transcript and diffs, make one-time tool decisions, and receive direct live task-completion events.

It is **not** a general file browser, terminal service, public HTTP API, or a replacement for Zed collaboration.

## Setup

1. Put the desktop and Android device on the same Tailscale tailnet.
2. Open projects or remote workspaces in Zed-Desktop at least once and configure an authenticated native Agent model. Zed stores those known workspaces in its desktop history.
3. In **Settings → Mobile Gateway**:
   - set **Bind Address** to this computer's Tailscale IP and a port, for example `100.99.221.90:7331`;
   - enable **Mobile Gateway**. Paired phones can use open workspaces and desktop-known local, SSH, and Dev Container workspaces; phone-based one-time Allow/Deny decisions are always enabled.

On Linux, the fork installer writes a default XDG autostart entry for a resident gateway host. It starts no window and exits if this setting is disabled. With the gateway enabled, closing the final Zed window leaves the private host running; use **Quit** to stop it. A project selected from the phone opens a normal Zed window when native workspace or remote-authentication UI is needed.
4. Select **Connect mobile device**, then scan the QR code in Zed Mobile.

The phone stores its per-device credential with Android Keystore encryption. Zed-Desktop stores only its SHA-256 hash, device label, and pairing timestamp in its user settings. Paired devices are visible under **Settings → Mobile Gateway** and can be removed individually; removal immediately rejects that phone on its next request.

The old shared `auth_token` setting and `/v1` endpoints remain only as a migration fallback. QR pairing never exposes or uses that token.

```json
{
  "mobile_gateway": {
    "enabled": true,
    "bind_address": "100.99.221.90:7331",
    "paired_devices": [
      {
        "id": "opaque-device-id",
        "label": "Pixel 9",
        "credential_hash": "sha256-hex-digest-only",
        "created_at": "2026-09-26T08:00:00+00:00"
      }
    ]
  }
}
```

Do not add a raw device credential to `settings.json`; Zed never stores it there.

## Security boundary

- The gateway is disabled by default and binds only a literal loopback or Tailscale address: IPv4 `100.64.0.0/10` or IPv6 `fd7a:115c:a1e0::/48`. Wildcards, normal LAN/public addresses, DNS names, mapped IPv6 addresses, and port zero are rejected.
- The gateway is HTTP-only. Use Tailscale ACLs or a secure local tunnel; never publish it through a public proxy, port forward, or reverse tunnel.
- QR tickets are random, in-memory, single-use, and expire after two minutes. The unauthenticated claim endpoint consumes a ticket before checking its secret to prevent replay attempts.
- All v2 routes except `POST /v2/pair/claim` require `Authorization: Bearer <device credential>`. Duplicate authorization headers, browser origins, URL queries/fragments, chunked transfer encoding, invalid paths, and oversized bodies are rejected.
- Devices have no global filesystem or shell API. They can see open workspaces plus local and supported remote workspaces already recorded in Zed-Desktop's recent-workspace database. They cannot send an arbitrary local path, SSH host, remote path, container ID, or connection option.
- Agent prompts use the same desktop Agent/model/provider permissions, data sharing, and costs as the desktop UI. Mobile access is not an execution sandbox.
- Mobile tool approvals always permit the current Agent tool call's server-selected **one-time** Allow or Deny option. There is no duplicate desktop prompt. The client cannot submit permanent rules, tool input, terminal patterns, option IDs, or stale decisions.
- The Android app sets `FLAG_SECURE` and disables backups. **Stay connected for Agent notifications** is enabled by default after QR pairing and uses one authenticated, direct SSE connection per paired phone. It is a foreground Android service, so the operating system displays a low-priority ongoing connection notification.
- Event delivery is live-only: the gateway keeps no event history, retries nothing, and sends no event to a phone that is disconnected. A bounded stream that cannot keep up is closed and its pending event is dropped. The phone may reconnect only to receive future events.

## API v2

All IDs are opaque path segments. Successful responses are JSON HTTP 200. Errors use `{"error":"…"}` and are never cached.

| Method | Route | Purpose |
| --- | --- | --- |
| POST | `/v2/pair/claim` | Claim a QR ticket without a bearer. Body: `ticket_id`, `secret`, `device_label`; response: `device_id`, `credential`. |
| POST | `/v2/devices/current/revoke` | Revoke the authenticated device credential. |
| GET | `/v2/status` | Gateway version and desktop name. |
| GET | `/v2/projects` | Allowed currently open workspaces. |
| GET | `/v2/projects/catalog` | Open workspaces and recently used local, SSH, and Dev Container workspaces. Each item identifies whether it is open and its source kind. |
| POST | `/v2/projects/catalog/{projectId}/open` | Open an allowed catalog workspace on Desktop. The phone supplies only the opaque catalog ID, never a filesystem path or remote connection data. Remote opens return `opening: true` while Zed handles desktop-only connection prompts. |
| GET | `/v2/projects/{projectId}/threads` | Active/retained native Agent conversations in that workspace. |
| GET | `/v2/projects/{projectId}/models` | Authenticated desktop models and their thinking capabilities. |
| POST | `/v2/threads` | Create a native Agent thread. |
| GET | `/v2/threads/{threadId}` | Thread summary, legacy visible messages, and the structured retained transcript. |
| GET | `/v2/threads/{threadId}/transcript` | Structured transcript entries: user/assistant messages, thought chunks, tool calls, plans, elicitation marker, and compaction state. |
| POST | `/v2/threads/{threadId}/messages` | Send a prompt. |
| POST | `/v2/threads/{threadId}/cancel` | Cancel the active generation. |
| GET | `/v2/threads/{threadId}/changes` | Recorded, root-verified textual tool diffs. |
| GET | `/v2/threads/{threadId}/approvals` | Current waiting tool approvals when remote approvals are enabled. |
| POST | `/v2/threads/{threadId}/approvals/{toolCallId}/resolve` | Body: `{"decision":"allow"}` or `{"decision":"deny"}`. |
| GET | `/v2/threads/{threadId}/configuration` | Current thread-local model/thinking configuration. |
| PUT | `/v2/threads/{threadId}/configuration` | Set a thread-local model, thinking enabled state, or supported effort. |
| GET | `/v2/events` | Authenticated Server-Sent Events stream. Emits ephemeral `thread_completed` or `thread_failed` records with `project_id`, `thread_id`, and a non-requestable notification `turn_id`. |

Thread model changes use the selected thread only; they do not change the desktop-wide Agent default model. Configuration changes are rejected while the thread is generating or awaiting an approval.

## Current limits

Ordinary gateway requests use one bounded request per connection with five-second read/write deadlines, 8 KiB headers, 64 KiB request bodies, and 2 MiB JSON responses. Long retained threads or diffs may be rejected with HTTP 413; open them on desktop in that case.

`GET /v2/events` is the exception: it is a direct, authenticated SSE stream with a 20-second keep-alive and a bounded in-memory capacity of 16 events for each active paired device. It has no Firebase, event database, replay cursor, acknowledgement, or delivery retry. Completion/failure events are attached to native Agent conversations in open eligible local and supported remote workspaces while at least one paired mobile event stream is connected.

## Remote workspaces

Remote catalog items are existing Zed workspace records only. SSH connection options are refreshed from the desktop's SSH settings before opening, and all host-key, password, MFA, connection-error, and retry interactions remain in Zed-Desktop. The Android app polls the catalog after a remote open request and prompts the user to complete those desktop interactions. A stopped/rebuilt Dev Container can fail to reconnect; the gateway does not create or reconfigure containers from the phone. WSL records may appear only on platforms where Zed supports WSL.
