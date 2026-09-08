#!/usr/bin/env python3
"""
Maintain a GitHub project as a search filter for second-line issue triage.

GitHub issue search can't express "created at least one business day ago", so
this script keeps the project's membership equal to the open issues created in
that window, adding issues that have become eligible and removing the ones that
have aged out.

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
# The eligibility window's boundaries are calendar days in this timezone. GitHub
# search interprets bare dates as UTC, so the window is pinned to a fixed zone to
# stay aligned with a consistent business day regardless of where the job runs.
# ZoneInfo (not a fixed offset) so daylight saving is handled automatically.
BUSINESS_DAY_TZ = ZoneInfo("America/Los_Angeles")


def day_start(day):
    # First instant of a calendar day in the business-day timezone.
    return datetime(day.year, day.month, day.day, 0, 0, 0, tzinfo=BUSINESS_DAY_TZ)


def day_end(day):
    # Last instant of a calendar day in the business-day timezone.
    return datetime(day.year, day.month, day.day, 23, 59, 59, tzinfo=BUSINESS_DAY_TZ)


def eligibility_window(today):
    # Returns the inclusive (start, end) day-edge instants bounding the window.
    weekday = today.isoweekday()  # Monday = 1
    # Nothing advances over the weekend; evaluate as of the preceding Friday.
    if weekday >= 6:
        today -= timedelta(days=weekday - 5)
        weekday = 5
    this_monday = today - timedelta(days=weekday - 1)
    # Floored at the prior week's Thursday; the Monday reset drops older issues.
    window_start = this_monday - timedelta(days=4)
    # Eligible once a full business day has elapsed since creation: two business
    # days back from today, so Mon and Tue reach across the weekend.
    window_end = today - timedelta(days=4 if weekday <= 2 else 2)
    return day_start(window_start), day_end(window_end)


def eligible_issues(window_start, window_end):
    query = (
        f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open "
        f"created:{window_start.isoformat()}..{window_end.isoformat()}"
    )
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
    today = datetime.now(BUSINESS_DAY_TZ).date()
    window_start, window_end = eligibility_window(today)
    print(f"window: created:{window_start.isoformat()}..{window_end.isoformat()}")

    eligible = eligible_issues(window_start, window_end)
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
