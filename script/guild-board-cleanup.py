#!/usr/bin/env python3

import os

import requests

GITHUB_API = "https://api.github.com"
OWNER = "zed-industries"
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
            projectV2(number: $number) {
              id
              fields(first: 50) {
                nodes {
                  ... on ProjectV2SingleSelectField { id name options { id name } }
                }
              }
            }
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
                          state
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


def set_single_select_field(project, item_id, field_name, option_name):
    field = next((field for field in project["fields"]["nodes"] if field.get("name") == field_name), None)
    if not field:
        print(f"Field '{field_name}' missing, skipping")
        return
    option = next((option for option in field.get("options", []) if option["name"] == option_name), None)
    if not option:
        print(f"Option '{option_name}' missing in '{field_name}', skipping")
        return
    graphql("""
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
            value: { singleSelectOptionId: $optionId }
          }) { projectV2Item { id } }
        }
    """, {
        "projectId": project["id"],
        "itemId": item_id,
        "fieldId": field["id"],
        "optionId": option["id"],
    })
    print(f"Set '{field_name}' = '{option_name}' on {item_id}")


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


def main():
    project = fetch_project(int(os.environ.get("PROJECT_NUMBER", "74")))
    shipped = 0

    for item in list_project_items(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue" or content.get("state") != "CLOSED":
            continue
        current_status = next(
            (field_value.get("name") for field_value in item.get("fieldValues", {}).get("nodes", [])
             if field_value.get("field", {}).get("name") == "Status"),
            None,
        )
        if current_status == SHIPPED_STATUS:
            continue
        assignees = [assignee["login"] for assignee in content.get("assignees", {}).get("nodes", [])]
        if not any(is_guild_member(assignee) for assignee in assignees):
            continue

        set_single_select_field(project, item["id"], "Status", SHIPPED_STATUS)
        shipped += 1

    print(f"Cleanup: {shipped} marked shipped")


if __name__ == "__main__":
    HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    main()
