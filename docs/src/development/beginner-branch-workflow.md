---
title: Beginner’s Branch Workflow (Hello Branch)
description: My first contribution documenting how to keep main clean, use a sandbox branch, and start a feature branch.
---

# Beginner’s Branch Workflow (Hello Branch)

This document captures my journey as a new Rust developer and first-time contributor to Zed.

## Branch Model

- **main** → kept pristine, always synced with `upstream/main`.
- **sandbox/genesis-branch** → my rebased personal base branch, aligned with upstream.
- **hello-branch** → my first feature branch, where I made this doc.

## Daily Workflow

```bash
# Keep main pristine
just pristine-main

# Rebase sandbox on upstream/main
just rebase-sandbox

# Create new feature branch
git switch sandbox/genesis-branch
git switch -c hello-branch
git push -u origin hello-branch
