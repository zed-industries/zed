#!/usr/bin/env node
/**
 * Spec status drift gate — GENERIC variant for generated target repositories
 * (specs/012 FR-983/984). This is the deployment-independent core of the
 * guidance package's docs-drift check: spec status hygiene only.
 *
 * Rule (FR-951.4 semantics): for every specs/<id>/spec.md that has a
 * tasks.md sibling — if the status line is still "Draft" while tasks.md has
 * no open checkboxes, that is drift. Escape hatch: a spec.md containing the
 * comment marker `docs-drift: status ok` is skipped (documented override).
 *
 * Usage: node .guidance/scripts/check-spec-drift.mjs [repoRoot=cwd]
 * Read-only; findings on stderr; exit 1 on drift.
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(process.argv[2] ?? process.cwd());
const findings = [];

const read = (p) => {
  try {
    return readFileSync(p, "utf-8");
  } catch {
    return null;
  }
};

const specsDir = join(repoRoot, "specs");
let checked = 0;
if (existsSync(specsDir)) {
  for (const id of readdirSync(specsDir)) {
    if (id.endsWith(".bak") || id.includes("~")) continue; // backups out
    const specFile = join(specsDir, id, "spec.md");
    const tasksFile = join(specsDir, id, "tasks.md");
    if (!existsSync(specFile) || !existsSync(tasksFile)) continue;
    const spec = read(specFile);
    if (spec === null) continue;
    checked++;
    const statusMatch = spec.match(/^\*\*Status:\*\* (.+)$/m);
    // FR-951.4: override comment lives in spec.md (documented escape hatch).
    if (
      !statusMatch ||
      !statusMatch[1].startsWith("Draft") ||
      spec.includes("docs-drift: status ok")
    )
      continue;
    const tasks = read(tasksFile) ?? "";
    const open = (tasks.match(/^\s*- \[ \] /gm) ?? []).length;
    if (open === 0) {
      findings.push(
        `spec drift: spec ${id} — expected: status updated (no open checkboxes, status still Draft) vs found: Draft`,
      );
    }
  }
}

if (findings.length > 0) {
  for (const f of findings) console.error(f);
  process.exit(1);
}
console.log(`spec drift check: OK (${checked} specs checked)`);
