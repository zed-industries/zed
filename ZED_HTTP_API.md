# Zed Agent HTTP API

When Zed is running, it exposes a local HTTP API for managing agent sessions. The server listens on `127.0.0.1:8765` by default.

## Endpoints

### `GET /healthz`

Health check. Returns `ok` if the server is running.

```bash
curl http://127.0.0.1:8765/healthz
# ok
```

---

### `POST /agents`

Create a new agent session. Uses Zed's currently configured default model unless a `model` override is specified.

**Request body:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workdir` | string | no | Working directory for the agent. Defaults to the workspace root. |
| `model` | string | no | Model in `provider/model` format. Defaults to Zed's configured model. |
| `title` | string | no | Display title for the agent thread. |

```bash
curl -X POST http://127.0.0.1:8765/agents \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response (200):**

```json
{
  "session_id": "22edfebf-fd77-4328-a40c-142321208b03",
  "model": "OpenRouter2/@preset/v4-pro-noresoning",
  "workdir": "F:\\Lepip\\test"
}
```

---

### `POST /agents/:id/prompt`

Send a prompt to an existing agent session. Blocks until the agent finishes its turn.

**Request body:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prompt` | string | yes | The prompt text to send to the agent. |

```bash
curl -X POST http://127.0.0.1:8765/agents/22edfebf-fd77-4328-a40c-142321208b03/prompt \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Fix the bug in src/main.rs"}'
```

**Response (200):**

```json
{
  "session_id": "22edfebf-fd77-4328-a40c-142321208b03",
  "stop_reason": "EndTurn",
  "input_tokens": 13571,
  "output_tokens": 512
}
```

---

### `GET /agents/:id`

Get the status of an agent session.

```bash
curl http://127.0.0.1:8765/agents/22edfebf-fd77-4328-a40c-142321208b03
```

**Response (200):**

```json
{
  "session_id": "22edfebf-fd77-4328-a40c-142321208b03",
  "model": "OpenRouter2/@preset/v4-pro-noresoning",
  "workdir": "F:\\Lepip\\test",
  "entry_count": 4,
  "status": "Idle"
}
```

---

### `DELETE /agents/:id`

Close an agent session.

```bash
curl -X DELETE http://127.0.0.1:8765/agents/22edfebf-fd77-4328-a40c-142321208b03
```

**Response (200):**

```json
{"status": "closed"}
```

## Errors

All errors return a JSON body with an `error` field:

```json
{"error": "Session not found"}
```

Common HTTP status codes:
- `400` — Invalid request body
- `404` — Unknown endpoint
- `500` — Server error or session not found
