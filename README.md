# Documentation Suggestions Queue

This branch contains batched documentation suggestions for the next Preview release.

Each file represents suggestions from a merged PR. At preview branch cut time,
run `script/docs-suggest-publish` to create a documentation PR from these suggestions.

## Structure

- `suggestions/PR-XXXXX.md` - Suggestions for PR #XXXXX
- `manifest.json` - Index of all pending suggestions

## Workflow

1. PRs merged to main trigger documentation analysis
2. Suggestions are committed here as individual files
3. At preview release, suggestions are collected into a docs PR
4. After docs PR is created, this branch is reset
