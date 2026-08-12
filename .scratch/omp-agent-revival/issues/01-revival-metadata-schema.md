# 01 — Revival metadata schema and persistence

**What to build:** The terminal metadata store can persist, list, migrate, and clear the revival fields that make an OMP terminal a resumable sleeping session: profile id, opaque resume path, session-boundary state (`live` / `sleeping` / `cleared`), and claim key. Adding the fields must not break existing terminal rows or the existing 10-column schema.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] Migration adds the revival columns to `sidebar_terminal_threads` without data loss.
- [ ] Saving a terminal with revival fields round-trips them through the store.
- [ ] Clearing a terminal removes its revival record.
- [ ] Existing terminals without revival fields load unchanged.
- [ ] Store tests cover migration, persistence, and delete (S1 seam).