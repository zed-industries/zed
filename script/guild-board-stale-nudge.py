#!/usr/bin/env python3

import os
from datetime import datetime, timezone

import requests

GITHUB_API = "https://api.github.com"
OWNER = "zed-industries"
REPO = "zed"
STALE_DAYS = 14


def graphql(query, variables):
    response = requests.post(
        f"{GITHUB_API}/graphql",
        headers=HEADERS,
        json={"query": query, "variables": variables},
    )
    response.raise_for_status()
    data = response.json()
    if "errors" in data:
        raise RuntimeError(f"GraphQL: {data['errors']}")
    return data["data"]


def rest_post(path, body):
    response = requests.post(f"{GITHUB_API}/{path}", headers=HEADERS, json=body)
    response.raise_for_status()


def fetch_project(number):
    data = graphql("""
        query($owner: String!, $number: Int!) {
          organization(login: $owner) {
            projectV2(number: $number) { id }
          }
        }
    """, {"owner": OWNER, "number": number})
    project = data["organization"]["projectV2"]
    if not project:
        raise RuntimeError(f"Could not fetch project #{number}")
    return project


def list_project_items(project_id):
    cursor = None
    while True:
        data = graphql("""
            query($projectId: ID!, $cursor: String) {
              node(id: $projectId) {
                ... on ProjectV2 {
                  items(first: 100, after: $cursor) {
                    pageInfo { hasNextPage endCursor }
                    nodes {
                      id
                      isArchived
                      fieldValues(first: 20) {
                        nodes {
                          ... on ProjectV2ItemFieldDateValue {
                            field { ... on ProjectV2Field { name } }
                            date
                          }
                        }
                      }
                      content {
                        __typename
                        ... on Issue {
                          number
                          state
                          assignees(first: 10) { nodes { login } }
                          timelineItems(first: 20, itemTypes: [CROSS_REFERENCED_EVENT]) {
                            nodes {
                              ... on CrossReferencedEvent {
                                source { __typename ... on PullRequest { state } }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
        """, {"projectId": project_id, "cursor": cursor})
        page = data["node"]["items"]
        yield from page["nodes"]
        if not page["pageInfo"]["hasNextPage"]:
            return
        cursor = page["pageInfo"]["endCursor"]


def add_comment(issue_number, body):
    rest_post(f"repos/{OWNER}/{REPO}/issues/{issue_number}/comments", {"body": body})
    print(f"Commented on #{issue_number}")


_guild_cache = {}


def is_guild_member(login):
    if login in _guild_cache:
        return _guild_cache[login]
    team_slug = os.environ.get("GUILD_TEAM_SLUG", "zed-guild")
    response = requests.get(
        f"{GITHUB_API}/orgs/{OWNER}/teams/{team_slug}/members/{login}",
        headers=HEADERS,
    )
    _guild_cache[login] = response.status_code == 204
    return _guild_cache[login]


def has_recent_nudge(issue_number):
    response = requests.get(
        f"{GITHUB_API}/repos/{OWNER}/{REPO}/issues/{issue_number}/comments",
        headers=HEADERS,
        params={"per_page": 10, "direction": "desc"},
    )
    response.raise_for_status()
    for comment in response.json():
        if "This check runs twice a month" in comment.get("body", ""):
            return True
    return False


def has_open_pull_request(content):
    return any(
        (timeline_item.get("source") or {}).get("__typename") == "PullRequest"
        and (timeline_item.get("source") or {}).get("state") == "OPEN"
        for timeline_item in content.get("timelineItems", {}).get("nodes", [])
    )


def main():
    project = fetch_project(int(os.environ.get("PROJECT_NUMBER", "74")))
    now = datetime.now(timezone.utc)
    nudged = 0

    for item in list_project_items(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue" or content.get("state") != "OPEN":
            continue
        assignees = [a["login"] for a in content.get("assignees", {}).get("nodes", []) if is_guild_member(a["login"])]
        if not assignees or has_open_pull_request(content):
            continue
        assigned_date = next(
            (field_value.get("date") for field_value in item.get("fieldValues", {}).get("nodes", [])
             if field_value.get("field", {}).get("name") == "Assigned Date"),
            None,
        )
        if not assigned_date:
            continue
        days = (now - datetime.fromisoformat(assigned_date).replace(tzinfo=timezone.utc)).days
        if days < STALE_DAYS:
            continue

        if has_recent_nudge(content["number"]):
            print(f"#{content['number']} already nudged recently, skipping")
            continue

        assignee_list = ", ".join(f"@{assignee}" for assignee in assignees)
        add_comment(
            content["number"],
            f"Hey {assignee_list}, this issue has been assigned for "
            f"{days} days with no linked PR yet. If you're still working on it, "
            f"no action needed. If you're no longer planning to pick this up, "
            f"feel free to unassign yourself so another Guild member can grab it. "
            f"(This check runs twice a month.)",
        )
        nudged += 1

    print(f"Stale nudge: {nudged} issues nudged")


if __name__ == "__main__":
    HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    main()
