#!/usr/bin/env python3

import os
from datetime import datetime, timedelta, timezone

import requests

GITHUB_API = "https://api.github.com"
OWNER = "zed-industries"
REPO = "zed"
SHIPPED_STATUS = "Shipped by Guild"


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
                          ... on ProjectV2ItemFieldSingleSelectValue {
                            field { ... on ProjectV2SingleSelectField { name } }
                            name
                          }
                        }
                      }
                      content {
                        __typename
                        ... on Issue {
                          number
                          title
                          closedAt
                          assignees(first: 10) { nodes { login } }
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


def repository_id():
    return graphql("""
        query($owner: String!, $name: String!) {
          repository(owner: $owner, name: $name) { id }
        }
    """, {"owner": OWNER, "name": REPO})["repository"]["id"]


def post_discussion(title, body):
    category_id = os.environ.get("DISCUSSION_CATEGORY_ID")
    if not category_id:
        print("DISCUSSION_CATEGORY_ID not set, printing instead:\n")
        print(f"# {title}\n\n{body}")
        return

    graphql("""
        mutation($repositoryId: ID!, $categoryId: ID!, $title: String!, $body: String!) {
          createDiscussion(input: {
            repositoryId: $repositoryId, categoryId: $categoryId,
            title: $title, body: $body
          }) { discussion { url } }
        }
    """, {
        "repositoryId": repository_id(),
        "categoryId": category_id,
        "title": title,
        "body": body,
    })
    print(f"Posted discussion: {title}")


def main():
    project = fetch_project(int(os.environ.get("PROJECT_NUMBER", "74")))
    now = datetime.now(timezone.utc)
    week_ago = now - timedelta(days=7)
    shipped = []

    for item in list_project_items(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue":
            continue
        status = next(
            (field_value.get("name") for field_value in item.get("fieldValues", {}).get("nodes", [])
             if field_value.get("field", {}).get("name") == "Status"),
            None,
        )
        if status != SHIPPED_STATUS:
            continue
        closed_at = content.get("closedAt")
        if not closed_at:
            continue
        closed_datetime = datetime.fromisoformat(closed_at.replace("Z", "+00:00"))
        if closed_datetime < week_ago:
            continue

        shipped.append({
            "number": content["number"],
            "title": content["title"],
            "assignees": [assignee["login"] for assignee in content.get("assignees", {}).get("nodes", [])],
        })

    if not shipped:
        print("Nothing shipped this week, skipping discussion post.")
        return

    title = f"Shipped by the Guild - Week of {now.strftime('%b %d, %Y')}"
    lines = ["Here's what the Zed Guild shipped this week:\n"]
    for item in shipped:
        credit = ", ".join(f"@{assignee}" for assignee in item["assignees"]) if item["assignees"] else "unassigned"
        lines.append(f"- #{item['number']} {item['title']} ({credit})")
    lines.append(f"\n{len(shipped)} issue{'s' if len(shipped) != 1 else ''} shipped. Nice work.")

    post_discussion(title, "\n".join(lines))


if __name__ == "__main__":
    HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    main()
