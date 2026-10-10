# Mobile Gateway

The Mobile Gateway is an opt-in, private-network companion API inside the running `Zed-Desktop` process. It is designed for a paired Android device to select an already-open workspace or reopen a known local, SSH, or Dev Container workspace from Zed's desktop history, continue a native Agent thread, review its retained transcript and diffs, inspect bounded tool output on demand, answer tool approvals and Agent questions, keep or reject the Agent's edits, and receive direct live task-completion events.

It is **not** a general file browser, terminal service, public HTTP API, or a replacement for Zed collaboration.

## Setup

1. Put the desktop and Android device on the same Tailscale tailnet.
2. Open projects or remote workspaces in Zed-Desktop at least once and configure an authenticated native Agent model. Zed stores those known workspaces in its desktop history.
3. In **Settings → Mobile Gateway**:
   - set **Bind Address** to this computer's Tailscale IP and a port, for example `100.99.221.90:7331`;
   - enable **Mobile Gateway**. Paired phones can use open workspaces and desktop-known local, SSH, and Dev Container workspaces; phones can answer approvals and questions and keep or reject Agent edits.

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
- Mobile tool approvals offer every option the desktop offers for the pending call (allow once, allow/deny always, pattern-scoped always rules, action choices). The phone echoes back only a desktop-issued `option_id` and, for always rules, indices into the desktop-issued pattern list; the gateway re-checks both against the call that is pending *now*, so stale or invented decisions are rejected. The phone never supplies tool input, rule text, or option kinds. When the desktop withholds allow options because a sandbox request contains suspicious Unicode, the gateway withholds them too until the warning is acknowledged on desktop.
- Agent questions (form elicitations) are validated by the same rules as the desktop form before they reach the Agent. URL-mode questions can only be declined or cancelled from a phone.
- Keep/Reject only targets buffers the thread's action log still tracks as unreviewed, matched by the exact verified path the gateway reported. A phone cannot name any other file. Review is refused while the Agent is generating.
- The Android app sets `FLAG_SECURE` and disables backups. **Stay connected for Agent notifications** is enabled by default after QR pairing and uses one authenticated, direct SSE connection per paired phone. It is a foreground Android service, so the operating system displays a low-priority ongoing connection notification. Live task alerts identify the desktop-derived project and thread title for eligible native Agent turns started on either desktop or phone.
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
| GET | `/v2/threads/{threadId}/transcript?before=&limit=` | One page (default 200, max 500 entries) of structured transcript entries ending before entry index `before`: user/assistant messages, thought chunks, compact tool metadata, plans, elicitations (with `elicitation_id` and `status`), and compaction state. `omitted_entries` counts older entries still available. |
| GET | `/v2/threads/{threadId}/tool-calls/{toolCallId}/output` | Read the selected retained tool call's bounded, display-only detailed output on demand. It never executes or replays a tool. |
| POST | `/v2/threads/{threadId}/messages` | Send a prompt. |
| POST | `/v2/threads/{threadId}/cancel` | Cancel the active generation. |
| GET | `/v2/threads/{threadId}/changes` | Recorded, root-verified textual tool diffs, each flagged `unreviewed` while it awaits Keep/Reject, plus `review.unreviewed_files` and `review.can_undo_reject`. Diffs over the budget are listed with counts and `truncated: true`. |
| GET | `/v2/threads/{threadId}/changes/file?path=&offset=&limit=` | One page of lines (default 400, max 2000) of a single changed file's unified diff, by a path `changes` reported. |
| POST | `/v2/threads/{threadId}/changes/review` | Body: `{"action":"keep"}` or `{"action":"reject"}`, optionally with `"paths":[…]`; omitting `paths` reviews every unreviewed file. |
| POST | `/v2/threads/{threadId}/changes/undo-reject` | Restore the edits removed by the most recent reject. |
| GET | `/v2/threads/{threadId}/approvals` | Current waiting tool approvals, each with `kind`, desktop `options` (`option_id`, `label`, `kind`, `enabled`), optional `patterns`, and `allow_blocked_reason`. |
| POST | `/v2/threads/{threadId}/approvals/{toolCallId}/resolve` | Body: `{"option_id":"…"}`, optionally with `"patterns":[0,…]` for an always option. Older clients may still send `{"decision":"allow"}` or `{"decision":"deny"}`. |
| GET | `/v2/threads/{threadId}/elicitations` | Pending Agent questions with `message`, `mode`, and the form `schema` (or `url`). |
| POST | `/v2/threads/{threadId}/elicitations/{elicitationId}/respond` | Body: `{"action":"accept","content":{…}}`, `{"action":"decline"}`, or `{"action":"cancel"}`. Invalid answers return HTTP 422 with per-field messages. |
| GET | `/v2/threads/{threadId}/configuration` | Current thread-local model/thinking configuration. |
| PUT | `/v2/threads/{threadId}/configuration` | Set a thread-local model, thinking enabled state, or supported effort. |
| GET | `/v2/events` | Authenticated Server-Sent Events stream. Emits ephemeral `thread_completed` or `thread_failed` records with opaque `project_id`/`thread_id`, bounded desktop-derived `project_name`/`thread_title` display labels, and a non-requestable notification `turn_id`. |

Thread model changes use the selected thread only; they do not change the desktop-wide Agent default model. Configuration changes are rejected while the thread is generating or awaiting an approval.

## Current limits

Connections are kept alive for up to 100 requests and closed after 15 idle seconds or any 401/403. Each request has a 30-second read/write deadline, 8 KiB headers, a 256 KiB body, and a 4 MiB JSON response (before compression). Responses of 1 KiB or more are gzip-compressed when the client sends `Accept-Encoding: gzip`. Long threads and large diffs are paged instead of rejected; only the query parameters `before`, `limit`, `offset`, and `path` are accepted.

Tool-call entries in thread snapshots deliberately include only compact metadata and `details_available`; terminal, diff, resource, and raw output are not transferred on each poll. A phone can request `GET /v2/threads/{threadId}/tool-calls/{toolCallId}/output` only for the already-authorized selected thread. That read-only response has a fixed mobile text cap, is marked when truncated, remains `Cache-Control: no-store`, and does not execute tools or accept a requested output size, path, command, or tool input.

`GET /v2/events` is the exception: it is a direct, authenticated SSE stream with a 20-second keep-alive and a bounded in-memory capacity of 16 events for each active paired device. It has no Firebase, event database, replay cursor, acknowledgement, or delivery retry. Completion/failure events are attached to every eligible native Agent conversation in open local and supported remote workspaces while at least one paired mobile event stream is connected, regardless of whether the turn started on desktop or phone.

## Remote workspaces

Remote catalog items are existing Zed workspace records only. SSH connection options are refreshed from the desktop's SSH settings before opening, and all host-key, password, MFA, connection-error, and retry interactions remain in Zed-Desktop. The Android app polls the catalog after a remote open request and prompts the user to complete those desktop interactions. A stopped/rebuilt Dev Container can fail to reconnect; the gateway does not create or reconfigure containers from the phone. WSL records may appear only on platforms where Zed supports WSL.
