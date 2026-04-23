#!/usr/bin/env python3
"""
Compare additions/deletions between Git commits and their corresponding
GitHub pull requests.

Usage:
    python3 script/compare-commit-pr-stats.py <start_sha> <end_sha>

Requires:
    - GITHUB_TOKEN environment variable (or gh CLI authenticated)
    - Runs from inside the zed repo
"""

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request


def get_github_token():
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        return token
    # Fall back to the gh CLI
    try:
        result = subprocess.run(
            ["gh", "auth", "token"], capture_output=True, text=True, check=True
        )
        return result.stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        print(
            "Error: Set GITHUB_TOKEN or authenticate with `gh auth login`.",
            file=sys.stderr,
        )
        sys.exit(1)


def github_get(path, token):
    url = f"https://api.github.com{path}"
    request = urllib.request.Request(
        url,
        headers={
            "Authorization": f"Bearer {token}",
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request) as response:
            return json.loads(response.read())
    except urllib.error.HTTPError as error:
        body = error.read().decode()
        print(
            f"GitHub API error {error.code} for {url}: {body}", file=sys.stderr
        )
        sys.exit(1)


def get_commits_in_range(start_sha, end_sha):
    """Return list of (sha, title) tuples for commits in [start, end]."""
    result = subprocess.run(
        ["git", "log", "--format=%H %s", f"{start_sha}~1..{end_sha}"],
        capture_output=True,
        text=True,
        check=True,
    )
    commits = []
    for line in result.stdout.strip().splitlines():
        if not line:
            continue
        sha, title = line.split(" ", 1)
        commits.append((sha, title))
    return commits


def extract_pr_number(title):
    match = re.search(r"\(#(\d+)\)\s*$", title)
    if match:
        return int(match.group(1))
    return None


OWNER = "zed-industries"
REPO = "zed"


def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <start_sha> <end_sha>", file=sys.stderr)
        sys.exit(1)

    start_sha = sys.argv[1]
    end_sha = sys.argv[2]
    token = get_github_token()

    commits = get_commits_in_range(start_sha, end_sha)
    print(f"Found {len(commits)} commit(s) in range.\n")

    for sha, title in commits:
        pr_number = extract_pr_number(title)
        short_sha = sha[:12]

        if pr_number is None:
            print(f"  {short_sha}  {title}")
            print(f"             ⚠  No PR number found in title, skipping.\n")
            continue

        commit_data = github_get(f"/repos/{OWNER}/{REPO}/commits/{sha}", token)
        commit_stats = commit_data.get("stats", {})
        commit_additions = commit_stats.get("additions", 0)
        commit_deletions = commit_stats.get("deletions", 0)
        commit_changes = commit_stats.get("total", 0)

        pr_data = github_get(f"/repos/{OWNER}/{REPO}/pulls/{pr_number}", token)
        pr_additions = pr_data.get("additions", 0)
        pr_deletions = pr_data.get("deletions", 0)
        pr_changes = pr_additions + pr_deletions

        identical = (
            commit_additions == pr_additions
            and commit_deletions == pr_deletions
        )

        status = "✅ IDENTICAL" if identical else "❌ DIFFERENT"

        print(f"  {short_sha}  #{pr_number}  {title}")
        print(
            f"    commit : +{commit_additions}  -{commit_deletions}  "
            f"(total {commit_changes})"
        )
        print(
            f"    PR     : +{pr_additions}  -{pr_deletions}  "
            f"(total {pr_changes})"
        )
        print(f"    {status}\n")


if __name__ == "__main__":
    main()
