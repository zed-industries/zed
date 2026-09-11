#!/usr/bin/env python3
"""
Clean up the community pull request queue:
    - retry CLA check and close PRs if that didn't help
    - close stale draft PRs

Requires: requests
"""

import argparse
import os
import time
from datetime import datetime, timedelta, timezone
from functools import cache

import requests

GITHUB_API_URL = "https://api.github.com"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
CLA_SIGNED_LABEL = "cla-signed"

DRAFT_WARNING_AFTER = timedelta(days=21)
DRAFT_CLOSE_AFTER_WARNING = timedelta(days=7)
CLA_RETRY_AFTER = timedelta(days=7)
CLA_CLOSE_AFTER_RETRY = timedelta(days=1)

DRAFT_WARNING_MARKER = "<!-- zed-community-automation:stale-draft-warning -->"
DRAFT_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=stale_draft -->"
CLA_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=CLA-not-signed -->"
COMMUNITY_BOT_LOGIN = "zed-community-bot[bot]"

DRAFT_WARNING_COMMENT = f"""{DRAFT_WARNING_MARKER}

This pull request has remained in draft without new commits for three weeks.
If it remains a draft without new commits for another week, it will be closed
automatically."""

DRAFT_CLOSE_COMMENT = f"""{DRAFT_CLOSE_MARKER}

Closing this pull request because it remained in draft without new commits for
another week after the ping above.

We close long-running drafts to keep the pull request queue focused on actionable work
and to avoid accumulating conflicts with `main`. If you want to continue this work,
please open a new pull request when it is ready for review."""

CLA_RETRY_COMMENT = "@cla-bot check"

CLA_CLOSE_COMMENT = f"""{CLA_CLOSE_MARKER}

Closing this pull request because Zed's contribution requirements have not been
met: the Contributor License Agreement is unsigned after seven days and a
final automated re-check.

We need a signed CLA before we can review or merge a contribution. If you would like
to continue, please sign the CLA at https://zed.dev/cla and open a new pull request.
If the CLA bot does not recognize your completed signature, mention that in the
new pull request so a maintainer can help."""

RETRYABLE_STATUS_CODES = {429, 500, 502, 503, 504}
MAX_RETRIES = 3
RETRY_DELAY_SECONDS = 5

GITHUB_HEADERS = {}


def parse_datetime(value):
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def github_graphql(query, variables):
    for attempt in range(MAX_RETRIES + 1):
        response = requests.post(
            f"{GITHUB_API_URL}/graphql",
            headers=GITHUB_HEADERS,
            json={"query": query, "variables": variables},
            timeout=30,
        )
        if response.status_code in RETRYABLE_STATUS_CODES and attempt < MAX_RETRIES:
            time.sleep(RETRY_DELAY_SECONDS)
            continue
        response.raise_for_status()
        result = response.json()
        if "errors" in result:
            raise RuntimeError(f"GraphQL error: {result['errors']}")
        return result["data"]
    raise RuntimeError("github_graphql: retry loop exited without return")


def github_rest_request(method, path, body=None):
    for attempt in range(MAX_RETRIES + 1):
        response = requests.request(
            method, f"{GITHUB_API_URL}/{path}", headers=GITHUB_HEADERS, json=body, timeout=30)
        if response.status_code in RETRYABLE_STATUS_CODES and attempt < MAX_RETRIES:
            time.sleep(RETRY_DELAY_SECONDS)
            continue
        response.raise_for_status()
        if response.status_code == 204 or not response.content:
            return None
        return response.json()
    raise RuntimeError("github_rest_request: retry loop exited without return")


def github_rest_get_paginated(path):
    results = []
    page = 1
    while True:
        separator = "&" if "?" in path else "?"
        batch = github_rest_request("GET", f"{path}{separator}per_page=100&page={page}")
        if not batch:
            return results
        results.extend(batch)
        if len(batch) < 100:
            return results
        page += 1


def fetch_open_pull_requests():
    pull_requests = []
    cursor = None
    while True:
        data = github_graphql(
            """
            query($owner: String!, $repo: String!, $cursor: String) {
              repository(owner: $owner, name: $repo) {
                pullRequests(states: OPEN, first: 100, after: $cursor) {
                  pageInfo { hasNextPage endCursor }
                  nodes {
                    number
                    state
                    isDraft
                    createdAt
                    author { __typename login }
                    labels(first: 100) { nodes { name } }
                    commits(last: 1) {
                      nodes {
                        commit { pushedDate committedDate }
                      }
                    }
                    timelineItems(
                      last: 20,
                      itemTypes: [CONVERT_TO_DRAFT_EVENT, READY_FOR_REVIEW_EVENT]
                    ) {
                      nodes {
                        __typename
                        ... on ConvertToDraftEvent { createdAt }
                        ... on ReadyForReviewEvent { createdAt }
                      }
                    }
                  }
                }
              }
            }
            """,
            {"owner": REPO_OWNER, "repo": REPO_NAME, "cursor": cursor},
        )
        page = data["repository"]["pullRequests"]
        pull_requests.extend(page["nodes"])
        if not page["pageInfo"]["hasNextPage"]:
            return pull_requests
        cursor = page["pageInfo"]["endCursor"]


@cache
def fetch_comments(number):
    return github_rest_get_paginated(
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{number}/comments"
    )


def post_comment(number, body, dry_run):
    if dry_run:
        print(f"  Would comment on PR #{number}:\n{body}\n")
    else:
        github_rest_request(
            "POST",
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{number}/comments",
        {"body": body},
    )


def close_pull_request(number, dry_run):
    if dry_run:
        print(f"  Would close PR #{number}")
    else:
        github_rest_request(
            "PATCH",
            f"repos/{REPO_OWNER}/{REPO_NAME}/pulls/{number}",
            {"state": "closed"},
        )


def label_names(pull_request):
    return {
        label["name"].lower()
        for label in (pull_request.get("labels") or {}).get("nodes", [])
    }


def latest_commit_at(pull_request):
    commits = (pull_request.get("commits") or {}).get("nodes", [])
    if not commits:
        return None
    commit = commits[-1]["commit"]
    timestamp = commit.get("pushedDate") or commit.get("committedDate")
    return parse_datetime(timestamp) if timestamp else None


def current_draft_started_at(pull_request):
    if pull_request.get("state") != "OPEN" or not pull_request.get("isDraft"):
        return None

    transitions = (pull_request.get("timelineItems") or {}).get("nodes", [])
    if not transitions:
        return parse_datetime(pull_request["createdAt"])

    latest_transition = max(
        transitions, key=lambda transition: parse_datetime(transition["createdAt"])
    )
    if latest_transition["__typename"] != "ConvertToDraftEvent":
        return None
    return parse_datetime(latest_transition["createdAt"])


def latest_draft_activity_at(pull_request):
    draft_started_at = current_draft_started_at(pull_request)
    if draft_started_at is None:
        return None
    commit_at = latest_commit_at(pull_request)
    if commit_at is None:
        return draft_started_at
    return max(draft_started_at, commit_at)


def latest_marker_comment_at(comments, marker, not_before=None):
    latest_timestamp = None
    for comment in comments:
        if marker not in (comment.get("body") or ""):
            continue
        timestamp = parse_datetime(comment["created_at"])
        if not_before is not None and timestamp < not_before:
            continue
        if latest_timestamp is None or timestamp > latest_timestamp:
            latest_timestamp = timestamp
    return latest_timestamp


def find_stale_drafts(pull_requests, now):
    stale_drafts = []
    for pull_request in pull_requests:
        activity_at = latest_draft_activity_at(pull_request)
        if activity_at is not None and now - activity_at >= DRAFT_WARNING_AFTER:
            stale_drafts.append((pull_request, activity_at))
    return stale_drafts


def draft_action(activity_at, comments, now):
    # A previous run may have posted the explanation but failed to close the PR.
    if latest_marker_comment_at(comments, DRAFT_CLOSE_MARKER, activity_at):
        return None, True

    warning_at = latest_marker_comment_at(
        comments, DRAFT_WARNING_MARKER, activity_at
    )
    if warning_at is not None:
        if now - warning_at >= DRAFT_CLOSE_AFTER_WARNING:
            return DRAFT_CLOSE_COMMENT, True
        return None
    return DRAFT_WARNING_COMMENT, False


def is_bare_cla_check(comment_body):
    return " ".join((comment_body or "").lower().split()) == "@cla-bot check"


def author_has_comment_other_than_cla_check(comments, author):
    return any(
        (comment.get("user") or {}).get("login", "").lower() == author.lower()
        and not is_bare_cla_check(comment.get("body"))
        for comment in comments
    )


def latest_cla_retry_at(comments):
    return max(
        (
            parse_datetime(comment["created_at"])
            for comment in comments
            if (comment.get("user") or {}).get("login", "").lower()
            == COMMUNITY_BOT_LOGIN
            and is_bare_cla_check(comment.get("body"))
        ),
        default=None,
    )


def cla_action(author, comments, now):
    if author_has_comment_other_than_cla_check(comments, author):
        return None

    retry_at = latest_cla_retry_at(comments)
    if retry_at is None:
        return CLA_RETRY_COMMENT, False
    if now - retry_at < CLA_CLOSE_AFTER_RETRY:
        return None
    # A previous run may have posted the explanation but failed to close the PR.
    if latest_marker_comment_at(comments, CLA_CLOSE_MARKER) is not None:
        return None, True
    return CLA_CLOSE_COMMENT, True


def needs_cla_follow_up(pull_request, now):
    author = pull_request.get("author") or {}
    return (
        now - parse_datetime(pull_request["createdAt"]) >= CLA_RETRY_AFTER
        and author.get("__typename") == "User"
        and bool(author.get("login"))
        and CLA_SIGNED_LABEL not in label_names(pull_request)
    )


def retry_or_close_unsigned_pull_requests(pull_requests, now, dry_run):
    closed = 0
    for pull_request in pull_requests:
        number = pull_request["number"]
        comments = fetch_comments(number)
        action = cla_action(pull_request["author"]["login"], comments, now)
        if action is None:
            continue
        comment, should_close = action
        if should_close:
            print(f"PR #{number}: closing because the CLA is unsigned")
            if comment is not None:
                post_comment(number, comment, dry_run)
            close_pull_request(number, dry_run)
            closed += 1
        else:
            print(f"PR #{number}: asking the CLA bot to check again")
            post_comment(number, comment, dry_run)
    return closed


def warn_or_close_stale_draft_pull_requests(stale_drafts, now, dry_run):
    warned = closed = 0
    for pull_request, activity_at in stale_drafts:
        number = pull_request["number"]
        comments = fetch_comments(number)
        action = draft_action(activity_at, comments, now)
        if action is None:
            continue
        comment, should_close = action
        if should_close:
            print(f"PR #{number}: closing a stale draft")
            if comment is not None:
                post_comment(number, comment, dry_run)
            close_pull_request(number, dry_run)
            closed += 1
        else:
            print(f"PR #{number}: warning about a stale draft")
            post_comment(number, comment, dry_run)
            warned += 1
    return warned, closed


def run(now, dry_run):
    pull_requests = fetch_open_pull_requests()
    print(f"Checking {len(pull_requests)} open pull requests")
    if dry_run:
        print("Dry-run mode: no comments or closures will be made")

    unsigned_pull_requests_closed = retry_or_close_unsigned_pull_requests(
        [
            pull_request
            for pull_request in pull_requests
            if needs_cla_follow_up(pull_request, now)
        ],
        now,
        dry_run,
    )
    draft_warned, draft_closed = warn_or_close_stale_draft_pull_requests(
        find_stale_drafts(pull_requests, now), now, dry_run
    )
    print(
        "Cleanup complete: "
        f"{unsigned_pull_requests_closed} unsigned PR closures, "
        f"{draft_warned} draft warnings, "
        f"{draft_closed} draft closures"
    )


if __name__ == "__main__":
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Report actions without posting comments or closing pull requests",
    )
    arguments = argument_parser.parse_args()

    GITHUB_HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    run(datetime.now(timezone.utc), arguments.dry_run)
