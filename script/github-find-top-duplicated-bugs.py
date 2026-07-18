#!/usr/bin/env python3
"""
Find open issues that have the most duplicates filed against them and update
a GitHub issue with the results.

Queries open issues and looks for MarkedAsDuplicateEvent in their timelines.
Only includes issues that have been re-reported at least twice (2+ duplicates
closed against them). Groups results by area: label. The output is formatted
as markdown with issue URLs (GitHub renders the titles automatically).

This script is run regularly by the update_duplicate_magnets.yml workflow.

Requires: requests (pip install requests)
GitHub token permissions: issues:write

Usage:
    # Print to stdout only for testing:
    python github-find-top-duplicated-bugs.py --github-token ghp_xxx

    # Update a GitHub issue:
    python github-find-top-duplicated-bugs.py --github-token ghp_xxx --issue-number 46355
"""

import argparse
import os
import sys
from collections import Counter, defaultdict

import requests

OWNER = "zed-industries"
REPO = "zed"

GRAPHQL_URL = "https://api.github.com/graphql"
REST_API_URL = "https://api.github.com"

headers = None

ISSUES_WITH_DUPLICATES_QUERY = """
query($owner: String!, $repo: String!, $cursor: String) {
  repository(owner: $owner, name: $repo) {
    issues(
      first: 100
      after: $cursor
      states: [OPEN]
      orderBy: {field: UPDATED_AT, direction: DESC}
    ) {
      pageInfo {
        hasNextPage
        endCursor
      }
      nodes {
        number
        url
        labels(first: 20) {
          nodes {
            name
          }
        }
        timelineItems(first: 100, itemTypes: [MARKED_AS_DUPLICATE_EVENT]) {
          nodes {
            ... on MarkedAsDuplicateEvent {
              duplicate {
                ... on Issue {
                  number
                  state
                }
              }
            }
          }
        }
      }
    }
  }
}
"""


def extract_duplicate_info(issue):
    """Extract duplicate count and info from an issue. Returns None if < 2 duplicates."""
    seen_duplicates = set()
    for event in issue["timelineItems"]["nodes"]:
        try:
            if event["duplicate"]["state"] == "CLOSED":
                seen_duplicates.add(event["duplicate"]["number"])
        except (KeyError, TypeError):
            continue

    if len(seen_duplicates) < 2:
        return None

    labels = [l["name"] for l in issue["labels"]["nodes"]]
    areas = [l.replace("area:", "") for l in labels if l.startswith("area:")]

    return {
        "number": issue["number"],
        "url": issue["url"],
        "areas": areas if areas else ["(unlabeled)"],
        "duplicate_count": len(seen_duplicates),
    }


def fetch_canonical_issues_with_duplicates(max_pages=100):
    """Fetch open issues and count how many duplicates point to each."""
    print(f"Finding open issues with the most duplicates in {OWNER}/{REPO}")

    cursor = None
    duplicate_magnets = []
    total_issues_scanned = 0

    for page in range(max_pages):
        response = requests.post(
            GRAPHQL_URL,
            headers=headers,
            json={
                "query": ISSUES_WITH_DUPLICATES_QUERY,
                "variables": {"owner": OWNER, "repo": REPO, "cursor": cursor},
            },
        )
        response.raise_for_status()
        data = response.json()

        if "errors" in data:
            print(f"GraphQL errors: {data['errors']}")
            break

        issues = data["data"]["repository"]["issues"]
        total_issues_scanned += len(issues["nodes"])

        for issue in issues["nodes"]:
            if info := extract_duplicate_info(issue):
                duplicate_magnets.append(info)

        page_info = issues["pageInfo"]
        if not page_info["hasNextPage"]:
            print(f"Done: scanned {total_issues_scanned} open issues")
            break
        cursor = page_info["endCursor"]

        print(
            f"Page {page + 1}: scanned {total_issues_scanned} open issues, "
            f"{len(duplicate_magnets)} have duplicates"
        )

    return duplicate_magnets


def build_markdown_body(duplicate_magnets):
    """Group results by area and build markdown body for the GitHub issue.

    NOTE: the output format is parsed by fetch_duplicate_magnets() in
    github-check-new-issue-for-duplicates.py — update that if you change this.
    """
    by_area = defaultdict(list)
    area_totals = Counter()
    for info in duplicate_magnets:
        for area in info["areas"]:
            by_area[area].append(info)
            area_totals[area] += info["duplicate_count"]

    lines = [
        "These are the issues that are frequently re-reported. "
        "The list is generated regularly by running a script."
    ]

    for area, _ in area_totals.most_common():
        issues = sorted(by_area[area], key=lambda x: x["duplicate_count"], reverse=True)

        lines.append("")
        lines.append(f"## {area}")
        lines.append("")

        for info in issues:
            lines.append(
                f"-   [{info['duplicate_count']:2d} dupes] {info['url']}"
            )

    return "\n".join(lines)


def update_github_issue(issue_number, body):
    """Update the body of a GitHub issue."""
    url = f"{REST_API_URL}/repos/{OWNER}/{REPO}/issues/{issue_number}"
    response = requests.patch(url, headers=headers, json={"body": body})
    response.raise_for_status()
    print(f"Updated issue #{issue_number}")


def parse_args():
    parser = argparse.ArgumentParser(
        description="Find open issues with the most duplicates filed against them."
    )
    parser.add_argument(
        "--github-token",
        default=os.environ.get("GITHUB_TOKEN"),
        help="GitHub token (or set GITHUB_TOKEN env var)",
    )
    parser.add_argument(
        "--issue-number",
        type=int,
        help="GitHub issue number to update (if not provided, prints to stdout)",
    )
    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()

    if not args.github_token:
        print("Error: --github-token is required (or set GITHUB_TOKEN env var)")
        sys.exit(1)

    headers = {
        "Authorization": f"Bearer {args.github_token}",
        "Content-Type": "application/json",
    }

    if duplicate_magnets := fetch_canonical_issues_with_duplicates():
        body = build_markdown_body(duplicate_magnets)
        if args.issue_number:
            update_github_issue(args.issue_number, body)
        else:
            print(body)
