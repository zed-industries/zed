#!/usr/bin/env node
/**
 * Lesson seeder — SELF-CONTAINED variant for generated target repositories
 * (specs/011 container-only rule: generated configs must not reference files
 * outside the target repo). Zero dependencies: speaks MCP Streamable HTTP
 * with plain node:fetch instead of @modelcontextprotocol/sdk.
 *
 * Usage: node .guidance/scripts/seed-lessons.mjs <lessons.json>
 * Env:   EMMS_HTTP_URL (default http://localhost:3002/mcp)
 *        EMMS_LESSON_SCOPE (default thinking-mcp-lessons)
 *
 * Semantics mirror the package-internal insight seeder script (this file is
 * its dependency-free twin for generated repositories):
 * - empty lessons file = no-op success
 * - exit 1 on transport/tool errors or failed lessons
 * - per-lesson idempotency is enforced server-side (slug key)
 */
import { readFileSync } from "node:fs";

const inputPath = process.argv[2];
if (!inputPath) {
  console.error("Usage: node seed-lessons.mjs <lessons.json>");
  process.exit(1);
}

const LESSONS = JSON.parse(readFileSync(inputPath, "utf8"));
// Empty input = nothing to seed = success (no-op). The MCP tool rejects
// empty arrays (minItems 1), so guard here to keep gate semantics simple.
if (Array.isArray(LESSONS) && LESSONS.length === 0) {
  console.log("DONE: 0/0 seeded, 0 duplicates, 0 failed (empty lessons file)");
  process.exit(0);
}
const SCOPE = process.env.EMMS_LESSON_SCOPE ?? "thinking-mcp-lessons";
const HTTP_URL = process.env.EMMS_HTTP_URL ?? "http://localhost:3002/mcp";

const JSON_HEADERS = {
  "content-type": "application/json",
  accept: "application/json, text/event-stream",
};

/** Parse a streamable-HTTP body: plain JSON or an SSE data:-line stream. */
function parseBody(text, contentType) {
  if (!text) return null;
  if ((contentType ?? "").includes("text/event-stream")) {
    for (const line of text.split("\n")) {
      if (!line.startsWith("data:")) continue;
      const payload = line.slice(5).trim();
      if (!payload || payload === "[DONE]") continue;
      try {
        return JSON.parse(payload);
      } catch {
        /* keep scanning lines */
      }
    }
    return null;
  }
  return JSON.parse(text);
}

async function post(body, sessionId) {
  const headers = { ...JSON_HEADERS };
  if (sessionId) headers["mcp-session-id"] = sessionId;
  const res = await fetch(HTTP_URL, {
    method: "POST",
    headers,
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    throw new Error(
      `HTTP ${res.status} for ${body.method}: ${await res.text()}`,
    );
  }
  const message = parseBody(await res.text(), res.headers.get("content-type"));
  return { message, sessionId: res.headers.get("mcp-session-id") };
}

let sessionId;
try {
  // 1. initialize → server assigns the session id (response header).
  const init = await post({
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      protocolVersion: "2025-03-26",
      capabilities: {},
      clientInfo: { name: "lesson-seeder", version: "1.0" },
    },
  });
  sessionId = init.sessionId ?? undefined;
  // 2. initialized notification (no id, no response expected).
  await post(
    { jsonrpc: "2.0", method: "notifications/initialized" },
    sessionId,
  );
  // 3. tools/call.
  const call = await post(
    {
      jsonrpc: "2.0",
      id: 2,
      method: "tools/call",
      params: {
        name: "experience_seed_lessons",
        arguments: {
          lessons: LESSONS,
          scope_id: SCOPE,
          client_context: { scope_id: SCOPE, agent_id: "lesson-seeder" },
        },
      },
    },
    sessionId,
  );
  const msg = call.message;
  if (!msg || msg.error) {
    console.error("ERROR:", JSON.stringify(msg?.error ?? msg, null, 2));
    process.exit(1);
  }
  const out = JSON.parse(msg.result.content[0].text);
  if (out.error) {
    console.error("ERROR:", JSON.stringify(out.error, null, 2));
    process.exit(1);
  }
  const { seeded, duplicates, failed, lessons } = out.result;
  for (const r of lessons) {
    console.error(
      `${r.status.toUpperCase().padEnd(9)} ${r.slug}${r.message ? " — " + r.message : ""}`,
    );
  }
  console.error(
    `DONE: ${seeded}/${LESSONS.length} seeded, ${duplicates} duplicates, ${failed} failed`,
  );
  if (failed > 0) process.exit(1);
} catch (e) {
  console.error(
    `ERROR: cannot seed via ${HTTP_URL} (${e.message}).\n` +
      "Start the insight server (docker compose up -d in servers/server-insight) " +
      "or adjust EMMS_HTTP_URL.",
  );
  process.exit(1);
}
