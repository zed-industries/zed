---
name: zed-cherry-pick
description: Cherry-pick one or more merged PRs and/or commits into Zed's `preview` or `stable` release branch. Use this whenever the user mentions cherry-picking to preview/stable, a failed cherry-pick run, or wants to manually port fix(es) into a release branch.
---

# Zed Cherry-Pick

Zed ships from two long-lived release branches that live on `origin`:

- `preview` channel → branch like `v1.4.x`
- `stable` channel → branch like `v1.3.x`

The version numbers change with each release. **Never hardcode them — always discover the current mapping** (see [Finding the target branch](#finding-the-target-branch)).

A merged PR on `main` gets ported to a release branch by `script/cherry-pick`, normally driven by the `cherry_pick` GitHub Actions workflow. When that workflow fails (almost always a merge conflict), use this skill to finish the job locally and open the cherry-pick PR by hand.

## When to use

Use this when the user asks to cherry-pick one or more commits and/or Pull Requests (by number or URL) to `preview` or `stable`.
Optionally, the user may specify whether to resolve merge conflicts; if unspecified, attempt the cherry-pick, and then if there are merge conflicts in practice, stop and inform the user that there are merge conflicts and offer to resolve them. (Users may prefer to resolve the merge conflicts themselves before continuing.)

## The script you're emulating

The canonical procedure lives in `script/cherry-pick` and the `cherry_pick` GitHub Actions workflow. Read the script first if anything looks off — your local steps must produce the same branch name, PR title, and PR body it would.

Signature: `script/cherry-pick <branch-name> <commit-sha> <channel>`

- `<branch-name>` is the release branch (e.g. `v1.4.x`), **not** the channel name.
- `<channel>` is `preview` or `stable`, used only for display text in the PR title/body.

It creates a local branch named `cherry-pick-<branch-name>-<short-sha>` (the short SHA is the first 8 chars of the commit), force-pushes it to `origin`, and opens a PR.

## Finding the target branch

The channel→branch mapping changes every release. Find the current one by inspecting the most recent `cherry_pick` workflow runs:

```
gh run list --workflow=cherry_pick.yml --limit 30 --json displayTitle,databaseId
# pick a recent run for the channel you want, then:
gh run view <id> --log 2>&1 | grep -E "BRANCH:|CHANNEL:"
```

A successful run prints both `BRANCH:` and `CHANNEL:` env vars; that's your mapping.

## Procedure

### 1. Gather context

You need three things: the **merge commit SHA**, the **target branch**, and the **channel name**.

If the user requested multiple PRs and/or commits, gather the metadata for all of them first and cherry-pick them in the order they landed on `main`, oldest to newest. For PRs, order by `mergedAt`; for raw commits, use their order on `main` when available, otherwise commit date. This tends to reduce avoidable conflicts because later changes may depend on earlier ones, but it does not guarantee a conflict-free cherry-pick when the release branch has diverged.

```
gh pr view <PR_NUMBER> --json title,number,mergeCommit,mergedAt,url
```

If the user said the workflow failed, fetch its log to see exactly which command failed and which file conflicted:

```
gh run list --workflow=cherry_pick.yml --limit 10 --json databaseId,displayTitle,status,conclusion
gh run view <failed_run_id> --log-failed
```

The failed-run log also confirms the `BRANCH` and `COMMIT` the workflow used — handy if there's any ambiguity.

### 2. Reproduce the script's setup locally

The repository may be a worktree (check `.git` — if it's a file, you're in a worktree pointing at a shared gitdir). That's fine; just operate normally.

```
git --no-pager fetch origin <branch-name> <commit-sha>
git checkout --force origin/<branch-name> -B cherry-pick-<branch-name>-<short-sha>
git cherry-pick <commit-sha>
```

The branch name **must** match `cherry-pick-<branch-name>-<short-sha>` exactly (script convention; reviewers and tooling expect it).

### 3. Check for missing prerequisite cherry-picks

If the cherry-pick conflicts, do not immediately resolve the conflicts manually.

First determine whether the conflict is likely caused by other PRs or commits that are already on `main` but missing from the release branch. If so, point out those candidate prerequisite PRs/commits to the user, including PR links, and offer to either resolve the conflicts manually or let the user run the GitHub cherry-pick workflow for those commits first.

If the user wants to run the workflow for the missing prerequisites, stop here. This often keeps cherry-picks clean and eligible for automatic approval.

Only resolve conflicts manually if:
- no likely missing prerequisites are found, or
- the user chooses manual conflict resolution instead of cherry-picking the prerequisites first.

### 4. Resolve the conflicts manually

Do this only after checking for missing prerequisite cherry-picks.

- Inspect every conflicted file with `grep -n '<<<<<<<\\|>>>>>>>\\|=======' <path>` to find the markers.
- Conflicts are usually `diff3` style with three sections: HEAD (release branch), `||||||| parent of <sha>` (merge base on `main`), and the incoming change.
- Read the **original commit** (`git --no-pager show <commit-sha> -- <path>`) to understand the author's intent, then pick the resolution that produces the equivalent end state on the release branch.
- Don't grab unrelated changes from `main` that happen to surround the conflict — keep the cherry-pick minimal.

### 5. Validate

Always build and (if reasonable) test the affected crate(s) before continuing the cherry-pick.

```
cargo check -p <affected_crate>
cargo test  -p <affected_crate>
```

If validation fails, fix the resolution — do **not** continue with a broken build. If you can't reach a clean state, abort with `git cherry-pick --abort` and report back to the user.

### 6. Finish the cherry-pick

`git cherry-pick --continue` opens an editor by default. Prevent that:

```
git add <resolved_files>
GIT_EDITOR=true git cherry-pick --continue
```

This preserves the original commit message verbatim, which is what the script does.

### 7. Push and open the PR

```
git push origin -f cherry-pick-<branch-name>-<short-sha>
```

Then create the PR with the **exact** title and body format `script/cherry-pick` uses, so it's indistinguishable from an automated one.

**Title:**

```
<original commit subject> (cherry-pick to <channel>)
```

The original commit subject already ends in ` (#<original_pr_number>)`; keep it.

**Body** (when the original commit title ends in `(#<N>)`, which is the normal case):

```
Cherry-pick of #<original_pr_number> to <channel>

----
<original commit body, verbatim>
```

Create it with `gh pr create`, writing the body to a temp file to keep formatting intact:

```
git --no-pager log -1 --pretty=format:"%b" > /tmp/cp-body-tail.md
printf 'Cherry-pick of #%s to %s\n\n----\n' <PR_NUMBER> <channel> | cat - /tmp/cp-body-tail.md > /tmp/cp-body.md
gh pr create --base <branch-name> --head cherry-pick-<branch-name>-<short-sha> \\
  --title "<commit subject> (cherry-pick to <channel>)" \\
  --body-file /tmp/cp-body.md
```

Do **not** add a `Release Notes:` section — the original commit body already has one (or already says `N/A`), and you don't want it duplicated.

## Final report to the user

Tell the user:
- The new PR URL.
- A one-line summary of the conflict and how you resolved it.
- What validation you ran (commands + result).
- That their local branch is now `cherry-pick-<branch-name>-<short-sha>`, in case they want you to switch back.

## Gotchas

- **`--no-pager` and `GIT_EDITOR=true`**: required for non-interactive git in this environment. Forgetting `GIT_EDITOR=true` on `cherry-pick --continue` hangs the terminal.
- **Worktree index lock**: if a previous git command was interrupted, you may see `index.lock` errors. The lock lives at `<gitdir>/index.lock` where `<gitdir>` is what `cat .git` points to (for a worktree). Remove it only if you're sure no git process is running.
- **Don't expand the cherry-pick's scope**: when resolving conflicts, never pull in unrelated changes from `main` just because they sit next to the conflict region. The PR should be the smallest diff that reproduces the original commit's intent on the release branch.
- **Channel branches are not called `preview`/`stable`**: don't try to `git fetch origin preview`. Look up the actual `vX.Y.x` branch name first.

## When Finished

After everything is finished, the last thing to do is to provide a link to the opened pull request(s) for the cherry-pick(s).
