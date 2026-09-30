#!/usr/bin/env node
/**
 * Final-Review Evidence Gate (spec 002, amendment 003 — FR-120/FR-121).
 *
 * Validates `.guidance/state/final-review.json` BEFORE complete_workflow is
 * accepted: the evidence must cover exactly the current git HEAD (any commit
 * after the review invalidates the gate, FR-122) and must not carry open
 * HIGH/CRITICAL findings (findings with severity high|critical and status
 * != "fixed").
 *
 * Pure Node, no git binary, no dependencies (readGitHead mirrors
 * check-index-freshness.mjs — loose refs, packed-refs, worktree .git files).
 *
 * Usage: node check-final-review.mjs [repoRoot] [evidenceRelPath]
 *   repoRoot        default "."
 *   evidenceRelPath default ".guidance/state/final-review.json"
 * Exit codes: 0 = evidence valid and fresh; 1 = gate fails (reason on stderr).
 */
import { readFileSync, existsSync, statSync } from "node:fs";
import { join, resolve } from "node:path";

const HEX40 = /^[0-9a-f]{40}$/;
const SEVERITIES = new Set(["high", "critical", "medium", "low", "info"]);
const STATUSES = new Set(["fixed", "tracked", "accepted"]);
const EVIDENCE_KEYS = new Set([
  "formatVersion", "sessionId", "reviewerRef", "reviewScope",
  "baseCommit", "headCommit", "commits", "reviewedAt",
  "openHighCritical", "findings",
]);
const FINDING_KEYS = new Set(["id", "severity", "status", "evidence"]);

function fail(msg) {
  process.stderr.write(`FINAL REVIEW GATE FAILED: ${msg}\n`);
  process.exit(1);
}

function readGitHead(repoRoot) {
  const gitPath = join(repoRoot, ".git");
  if (!existsSync(gitPath)) fail(`no .git at ${gitPath} — not a git repository?`);
  let gitDir = gitPath;
  // Linked worktrees: .git is a FILE with 'gitdir: <path>'. Same host-path
  // mappings as check-index-freshness.mjs (guidance container mounts the
  // main repo at /workspace).
  if (!statIsDir(gitPath)) {
    const pointer = readFileSync(gitPath, "utf8").trim();
    if (!pointer.startsWith("gitdir:")) fail(`cannot parse ${gitPath} (.git file without gitdir pointer)`);
    gitDir = resolve(repoRoot, pointer.slice("gitdir:".length).trim());
    // GDS-5: generic candidate resolution - raw relative, WSL drive mapping
    // (D:/... -> /mnt/d/...), and the guidance container mounts (/workspace,
    // /workspaces/<name>). First existing candidate wins.
    const rawPointer = pointer.slice("gitdir:".length).trim().replace(/\\/g, "/");
    const candidates = [gitDir];
    const driveMatch = rawPointer.match(/^([A-Za-z]):\/(.*)$/);
    if (driveMatch) {
      candidates.push("/mnt/" + driveMatch[1].toLowerCase() + "/" + driveMatch[2]);
      candidates.push("/workspace/" + driveMatch[2]);
      candidates.push("/workspaces/" + driveMatch[2]);
      const reposMount = rawPointer.match(/^[A-Za-z]:\/(?:repos|workspaces)\/([^\/]+)(\/.*)$/);
      if (reposMount) candidates.push("/workspaces/" + reposMount[1] + reposMount[2]);
      candidates.push("/workspaces/" + driveMatch[2]);
    }
    const hit = candidates.find((c) => existsSync(c));
    if (!hit) fail(`worktree git dir does not exist in this container (tried: ${candidates.join(", ")})`);
    gitDir = hit;
  }
  const headPath = join(gitDir, "HEAD");
  if (!existsSync(headPath)) fail(`no HEAD at ${headPath}`);
  const head = readFileSync(headPath, "utf8").trim();
  if (!head.startsWith("ref: ")) return head; // detached HEAD
  const ref = head.slice(5).trim();
  const candidates = [join(gitDir, ref)];
  const commonDir = join(gitDir, "commondir");
  if (existsSync(commonDir)) {
    candidates.push(resolve(gitDir, readFileSync(commonDir, "utf8").trim(), ref));
  }
  for (const candidate of candidates) {
    if (existsSync(candidate)) return readFileSync(candidate, "utf8").trim();
  }
  for (const packedPath of [join(gitDir, "packed-refs"), existsSync(commonDir) ? resolve(gitDir, readFileSync(commonDir, "utf8").trim(), "packed-refs") : null]) {
    if (!packedPath || !existsSync(packedPath)) continue;
    for (const line of readFileSync(packedPath, "utf8").split("\n")) {
      if (line.endsWith(` ${ref}`)) return line.split(" ")[0].trim();
    }
  }
  fail(`cannot resolve ${ref} (loose ref and packed-refs)`);
}

function statIsDir(p) {
  try {
    return statSync(p).isDirectory();
  } catch {
    return false;
  }
}

/* ---------- main ---------- */

const root = resolve(process.argv[2] ?? ".");
const evidencePath = resolve(root, process.argv[3] ?? ".guidance/state/final-review.json");
if (!existsSync(evidencePath)) {
  fail(`evidence file missing: ${evidencePath} — run the independent final review and write the evidence file first (amendment 003, FR-120)`);
}

let ev;
try {
  ev = JSON.parse(readFileSync(evidencePath, "utf8"));
} catch (err) {
  fail(`evidence file is not valid JSON: ${err.message}`);
}

// Strict schema (FR-121.1): unknown fields rejected.
const keys = new Set(Object.keys(ev ?? {}));
for (const k of EVIDENCE_KEYS) keys.delete(k);
if (keys.size > 0) fail(`unknown evidence fields: ${[...keys].join(", ")}`);
for (const k of EVIDENCE_KEYS) {
  if (ev[k] === undefined) fail(`missing evidence field: ${k}`);
}
if (ev.formatVersion !== 1) fail(`unsupported formatVersion ${JSON.stringify(ev.formatVersion)}`);
for (const k of ["sessionId", "reviewerRef", "reviewScope"]) {
  if (typeof ev[k] !== "string" || ev[k].length === 0) fail(`evidence.${k} must be a non-empty string`);
}
for (const k of ["baseCommit", "headCommit"]) {
  if (typeof ev[k] !== "string" || !HEX40.test(ev[k])) fail(`evidence.${k} must be a full 40-hex commit hash`);
}
if (!Array.isArray(ev.commits) || ev.commits.length === 0 || !ev.commits.every((c) => typeof c === "string" && HEX40.test(c))) {
  fail("evidence.commits must be a non-empty array of 40-hex commit hashes");
}
if (!ev.commits.includes(ev.headCommit) || !ev.commits.includes(ev.baseCommit)) {
  fail("evidence.commits must include both headCommit and baseCommit");
}
if (Number.isNaN(Date.parse(ev.reviewedAt))) fail("evidence.reviewedAt must be an ISO timestamp");
if (!Array.isArray(ev.findings) || ev.findings.length === 0) {
  fail("evidence.findings must be a non-empty array (at least the review outcome summary row)");
}
for (const f of ev.findings) {
  const fk = new Set(Object.keys(f ?? {}));
  for (const k of FINDING_KEYS) fk.delete(k);
  if (fk.size > 0) fail(`finding ${JSON.stringify(f?.id)} has unknown fields: ${[...fk].join(", ")}`);
  for (const k of FINDING_KEYS) {
    if (typeof f[k] !== "string" || f[k].length === 0) fail(`finding.${k} must be a non-empty string`);
  }
  if (!SEVERITIES.has(f.severity)) fail(`finding ${f.id}: unknown severity ${f.severity}`);
  if (!STATUSES.has(f.status)) fail(`finding ${f.id}: unknown status ${f.status}`);
}

// FR-121.4: open HIGH/CRITICAL findings are computed, not trusted.
const openHighCritical = ev.findings.filter(
  (f) => (f.severity === "high" || f.severity === "critical") && f.status !== "fixed",
).length;
if (openHighCritical > 0) {
  fail(`${openHighCritical} open HIGH/CRITICAL finding(s) — completing with open HIGH/CRITICAL is not allowed`);
}
if (ev.openHighCritical !== openHighCritical) {
  fail(`evidence.openHighCritical (${JSON.stringify(ev.openHighCritical)}) does not match computed count (${openHighCritical})`);
}

// FR-122: any commit after the review invalidates the gate.
const headCommit = readGitHead(root);
if (ev.headCommit !== headCommit) {
  fail(`stale review: evidence.headCommit ${ev.headCommit.slice(0, 12)} != HEAD ${headCommit.slice(0, 12)} — commits landed after the review; re-review required`);
}

console.log(
  `FINAL REVIEW EVIDENCE OK: HEAD ${headCommit.slice(0, 12)}, ${ev.findings.length} finding(s), ${openHighCritical} open HIGH/CRITICAL, reviewerRef ${ev.reviewerRef}.`,
);
