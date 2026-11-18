#!/usr/bin/env python3
"""
Post a welcome comment on a pull request labeled `first contribution`, unless
the community bot has already welcomed its author on any of their pull requests.

Requires:
    requests (pip install requests)

Usage:
    python github-welcome-first-contribution.py <pr_number> [--dry-run]

Environment variables:
    GITHUB_TOKEN - GitHub token (issues: write, pull requests: read)
"""

import argparse

from github_helpers import (
    github_rest_api,
    github_rest_get_paginated,
    post_github_comment,
)

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
FIRST_CONTRIBUTION_LABEL = "first contribution"
COMMUNITY_BOT_LOGIN = "zed-community-bot[bot]"
WELCOME_MARKER = "<!-- zed-community-automation:first-contribution-welcome -->"
SEARCH_PAGE_SIZE = 100
# The search API returns at most 1000 results which should never be relevant here but hey.
MAX_SEARCH_PAGES = 10

WELCOME_COMMENT = f"""{WELCOME_MARKER}

Thanks for your first contribution to Zed! For the best chances of merge, please read our [contributing guide](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md) if you haven't already. The key points for a first PR:

- **Show how you tested it.** Add evidence that fits the change: screenshots/video for UI changes, benchmarks for performance work, etc. Using LLMs or other help is fine, but you must understand and test the changes yourself. **Write review replies in your own words;** a human is reading them.
- **One PR at a time.** Things go smoother once your first PR lands; several open at once tend to go stale and collect the same feedback.
- **Discuss features first.** If this adds a feature that staff haven't agreed to in an issue, please start a [GitHub discussion](https://github.com/zed-industries/zed/discussions) instead of a PR.
- **Don't ping staff by username** unless they've told you that's okay."""


def search_pull_requests_by_author(author):
    pull_requests = []
    for page in range(1, MAX_SEARCH_PAGES + 1):
        result = github_rest_api(
            "GET",
            "search/issues",
            params={
                "q": f"repo:{REPO_OWNER}/{REPO_NAME} is:pr author:{author}",
                "sort": "created",
                "order": "asc",
                "per_page": SEARCH_PAGE_SIZE,
                "page": page,
            },
        )
        pull_requests.extend(result["items"])
        if len(result["items"]) < SEARCH_PAGE_SIZE:
            break
    return pull_requests


def find_welcomed_pull_request(pull_requests):
    for pull_request in pull_requests:
        comments = github_rest_get_paginated(
            f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{pull_request['number']}/comments"
        )
        if any(
            (comment["user"] or {}).get("login") == COMMUNITY_BOT_LOGIN
            and WELCOME_MARKER in (comment["body"] or "")
            for comment in comments
        ):
            return pull_request
    return None


def welcome_first_time_contributor(pr_number, dry_run):
    pull_request = github_rest_api(
        "GET", f"repos/{REPO_OWNER}/{REPO_NAME}/pulls/{pr_number}"
    )
    author = pull_request["user"]["login"]

    if not any(
        label["name"] == FIRST_CONTRIBUTION_LABEL for label in pull_request["labels"]
    ):
        print(f"PR #{pr_number} has no '{FIRST_CONTRIBUTION_LABEL}' label")
        return

    # Search results can lag behind a just-opened PR, so check it explicitly.
    welcomed_pull_request = find_welcomed_pull_request([pull_request])
    if welcomed_pull_request is None:
        other_pull_requests = [
            other_pull_request
            for other_pull_request in search_pull_requests_by_author(author)
            if other_pull_request["number"] != pr_number
        ]
        welcomed_pull_request = find_welcomed_pull_request(other_pull_requests)
    if welcomed_pull_request is not None:
        print(
            f"{author} was already welcomed on PR #{welcomed_pull_request['number']}"
        )
        return

    if dry_run:
        print(f"Would post on PR #{pr_number}:\n\n{WELCOME_COMMENT}")
    else:
        post_github_comment(REPO_OWNER, REPO_NAME, pr_number, WELCOME_COMMENT)


if __name__ == "__main__":
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument("pr_number", type=int)
    argument_parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print the comment instead of posting it",
    )
    arguments = argument_parser.parse_args()
    welcome_first_time_contributor(arguments.pr_number, arguments.dry_run)
