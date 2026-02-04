# Branch Strategy for Zed Cachy Experiment

This document explains the branching strategy for this fork of Zed and how to maintain it separately from upstream updates.

## Default Branch: `home`

The `home` branch is the default branch for this repository. This branch contains our custom experiment and modifications to Zed, and **should remain separate from Zed's upstream updates**.

## Why Keep `home` Separate?

This fork implements a model-first IDE experiment that diverges significantly from the upstream Zed project. Automatically syncing with upstream updates could:

- Break our custom model API implementation
- Introduce unwanted changes to the experiment
- Create merge conflicts that are difficult to resolve
- Dilute the focus of our specific experiment

## Branch Structure

```
home (default branch)
  ↓
  Our custom experiment branch
  - Model API features
  - Custom modifications
  - Experiment-specific code
  
upstream/main (not tracked by default)
  ↓
  Original Zed project
  - Should NOT be automatically merged into home
```

## Setting `home` as Default Branch

To make `home` the default branch in GitHub:

1. Go to your repository on GitHub: `https://github.com/Jounikka1918/zed_cachy_experiment`
2. Click on **Settings** (requires admin access)
3. In the left sidebar, click on **Branches**
4. Under "Default branch", click the pencil/edit icon
5. Select `home` from the dropdown
6. Click **Update** and confirm the change

## Keeping `home` Separate from Upstream

### Option 1: No Upstream Remote (Recommended)

The simplest approach is to not add Zed's upstream repository as a remote. This prevents accidental syncing:

```bash
# Check current remotes
git remote -v

# If upstream exists, remove it
git remote remove upstream
```

### Option 2: Add Upstream Remote (For Selective Updates Only)

If you occasionally need to cherry-pick specific features from upstream Zed:

```bash
# Add upstream remote (only if needed)
git remote add upstream https://github.com/zed-industries/zed.git

# Fetch upstream changes (doesn't merge them)
git fetch upstream

# View what's new in upstream
git log home..upstream/main --oneline

# Cherry-pick specific commits if needed
git cherry-pick <commit-hash>
```

**⚠️ Important**: Never run `git merge upstream/main` or `git pull upstream main` on the `home` branch, as this will bring in all upstream changes.

### Option 3: Automation to Prevent Accidental Syncs

The repository includes a pre-commit hook and GitHub Action to warn against accidental upstream merges (see `.github/workflows/prevent-upstream-sync.yml`).

## Workflow for Development

1. **Always work on the `home` branch or feature branches off of `home`**:
   ```bash
   git checkout home
   git checkout -b feature/my-new-feature
   ```

2. **Create pull requests targeting `home`**:
   - Base branch: `home`
   - Compare branch: `feature/my-new-feature`

3. **Never merge or rebase with upstream/main**:
   - Avoid: `git merge upstream/main`
   - Avoid: `git rebase upstream/main`
   - Avoid: `git pull upstream main`

## If You Accidentally Merged Upstream

If you accidentally merged upstream changes:

```bash
# Find the merge commit
git log --oneline --graph

# Reset to before the merge (replace COMMIT_HASH with the commit before merge)
git reset --hard <COMMIT_HASH>

# Force push (be careful!)
git push --force-with-lease origin home
```

⚠️ **Warning**: Force pushing rewrites history. Only do this if you're certain no one else has based work on the merged commits.

## Selectively Adopting Upstream Features

If you want to adopt a specific feature from upstream:

1. **Identify the commits** related to that feature in upstream
2. **Cherry-pick** those specific commits:
   ```bash
   git fetch upstream
   git cherry-pick <commit-hash>
   ```
3. **Resolve conflicts** if any
4. **Test thoroughly** to ensure compatibility with our experiment

## Questions?

For questions about the branch strategy, contact the repository maintainer or open an issue.

---

**Last Updated**: February 2026
