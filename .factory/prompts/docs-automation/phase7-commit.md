# Phase 7: Commit and Open PR

You are creating a git branch, committing documentation changes, and opening a PR.

## Objective
Package documentation updates into a reviewable pull request.

## Input
You will receive:
- Summary from Phase 6
- List of modified files

## Instructions

1. **Create Branch**
   ```sh
   git checkout -b docs/auto-update-{date}
   ```
   Use format: `docs/auto-update-YYYY-MM-DD` or `docs/auto-update-{short-sha}`

2. **Stage and Commit**
   - Stage only documentation files in `docs/src/`
   - Do not stage any other files
   
   Commit message format:
   ```
   docs: auto-update documentation for [brief description]
   
   [Summary from Phase 6, condensed]
   
   Triggered by: [commit SHA or PR reference]
   
   Co-authored-by: factory-droid[bot] <138933559+factory-droid[bot]@users.noreply.github.com>
   ```

3. **Push Branch**
   ```sh
   git push -u origin docs/auto-update-{date}
   ```

4. **Create Pull Request**
   Use the Phase 6 summary as the PR body.
   
   PR Title: `docs: [Brief description of documentation updates]`
   
   Labels (if available): `documentation`, `automated`
   
   Base branch: `main`

## Constraints
- Do NOT auto-merge
- Do NOT request specific reviewers (let CODEOWNERS handle it)
- Do NOT modify files outside `docs/src/`
- If no changes to commit, exit gracefully with message "No documentation changes to commit"

## Output
```markdown
## PR Created

- **Branch**: docs/auto-update-{date}
- **PR URL**: https://github.com/zed-industries/zed/pull/XXXX
- **Status**: Ready for review

### Commit
- SHA: {commit-sha}
- Files: {count} documentation files modified
```
