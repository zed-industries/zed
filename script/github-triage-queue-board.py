#!/usr/bin/env python3
"""
Maintain a GitHub project as a search filter for the current issue triage shift.

GitHub issue search can't express "created since last Friday 14:00 CET", so
this script keeps the project's membership equal to the open issues created
since then, adding new issues and removing the ones that have been closed or
have aged out at the Monday reset.

Requires:
    requests (pip install requests)

Usage (called by the workflow, not directly):
    python github-triage-queue-board.py
"""

import os
import urllib.parse
from datetime import datetime, timedelta
from zoneinfo import ZoneInfo

from github_helpers import add_github_project_item, github_graphql, github_rest_api

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
TRIAGE_TZ = ZoneInfo("Europe/Berlin")
HANDOFF_HOUR = 14


def window_start(now):
    # The Friday handoff before this week's Monday. The board resets on Monday
    # rather than at the handoff itself so the outgoing shift keeps its leftovers
    # through Friday afternoon.
    friday = now.date() - timedelta(days=now.isoweekday() + 2)
    return datetime(friday.year, friday.month, friday.day, HANDOFF_HOUR, tzinfo=TRIAGE_TZ)


def eligible_issues(start):
    query = f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open created:>={start.isoformat()}"
    issues = {}
    page = 1
    while True:
        data = github_rest_api(
            "GET", f"search/issues?q={urllib.parse.quote(query)}&per_page=100&page={page}"
        )
        for item in data["items"]:
            issues[item["node_id"]] = item["number"]
        if len(data["items"]) < 100 or page >= 10:
            return issues
        page += 1


def fetch_project_id(project_number):
    data = github_graphql(
        """
        query($owner: String!, $number: Int!) {
          organization(login: $owner) { projectV2(number: $number) { id } }
        }
        """,
        {"owner": REPO_OWNER, "number": project_number},
    )
    project = data["organization"]["projectV2"]
    if not project:
        raise RuntimeError(f"project #{project_number} not found")
    return project["id"]


def project_items(project_id):
    # Yields (item_id, content_id, number) for each issue on the project.
    # content_id is the issue's global node id, which is the same value the REST
    # search API returns as node_id, so it can be compared directly against the
    # keys of eligible_issues().
    cursor = None
    while True:
        data = github_graphql(
            """
            query($project: ID!, $cursor: String) {
              node(id: $project) {
                ... on ProjectV2 {
                  items(first: 100, after: $cursor) {
                    pageInfo { hasNextPage endCursor }
                    nodes { id content { ... on Issue { id number } } }
                  }
                }
              }
            }
            """,
            {"project": project_id, "cursor": cursor},
        )
        items = data["node"]["items"]
        for item in items["nodes"]:
            content = item["content"] or {}
            if "id" in content:  # skip drafts / PRs (no Issue content)
                yield item["id"], content["id"], content["number"]
        if not items["pageInfo"]["hasNextPage"]:
            return
        cursor = items["pageInfo"]["endCursor"]


def remove_from_project(project_id, item_id):
    github_graphql(
        "mutation($p: ID!, $i: ID!) { deleteProjectV2Item(input: {projectId: $p, itemId: $i}) { deletedItemId } }",
        {"p": project_id, "i": item_id},
    )


def sync_project(project_id, dry_run=False):
    start = window_start(datetime.now(TRIAGE_TZ))
    print(f"window: created:>={start.isoformat()}")

    eligible = eligible_issues(start)
    current = list(project_items(project_id))
    on_board = {content_id for _, content_id, _ in current}

    for content_id, number in eligible.items():
        if content_id not in on_board:
            print(f"+ #{number}")
            if not dry_run:
                add_github_project_item(project_id, content_id)

    for item_id, content_id, number in current:
        if content_id not in eligible:
            print(f"- #{number}")
            if not dry_run:
                remove_from_project(project_id, item_id)


if __name__ == "__main__":
    project_number = int(os.environ["PROJECT_NUMBER"])
    project_id = fetch_project_id(project_number)
    sync_project(project_id, os.environ.get("DRY_RUN") == "true")
