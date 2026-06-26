#!/usr/bin/env python3

import json
import os
from datetime import datetime, timezone

import requests

GITHUB_API = "https://api.github.com"
OWNER = "zed-industries"
REPO = "zed"
IN_PROGRESS_STATUS = "In Progress"


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
                  ... on ProjectV2Field { id name dataType }
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


def find_item(project_id, content_node_id):
    data = graphql("""
        query($contentId: ID!) {
          node(id: $contentId) {
            ... on Issue {
              projectItems(first: 50) {
                nodes { id project { id } }
              }
            }
          }
        }
    """, {"contentId": content_node_id})
    for item in data["node"]["projectItems"]["nodes"]:
        if item["project"]["id"] == project_id:
            return item["id"]
    return None


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


def set_assigned_date(project, item_id):
    field = next(
        (field for field in project["fields"]["nodes"]
         if field.get("name") == "Assigned Date" and field.get("dataType") == "DATE"),
        None,
    )
    if not field:
        print("Field 'Assigned Date' missing, skipping")
        return
    graphql("""
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $date: Date!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
            value: { date: $date }
          }) { projectV2Item { id } }
        }
    """, {
        "projectId": project["id"],
        "itemId": item_id,
        "fieldId": field["id"],
        "date": datetime.now(timezone.utc).strftime("%Y-%m-%d"),
    })
    print(f"Stamped Assigned Date on {item_id}")


def is_guild_member(login):
    team_slug = os.environ.get("GUILD_TEAM_SLUG", "zed-guild")
    response = requests.get(
        f"{GITHUB_API}/orgs/{OWNER}/teams/{team_slug}/members/{login}",
        headers=HEADERS,
    )
    return response.status_code == 204


def main():
    if os.environ.get("GITHUB_EVENT_NAME") != "issues":
        print("Ignoring non-issues event")
        return

    with open(os.environ["GITHUB_EVENT_PATH"]) as event_file:
        event = json.load(event_file)
    if event.get("action") != "assigned":
        print(f"Ignoring issue action: {event.get('action')}")
        return

    assignee = event["assignee"]["login"]
    if not is_guild_member(assignee):
        print(f"{assignee} is not in the Guild team, skipping")
        return

    project = fetch_project(int(os.environ.get("PROJECT_NUMBER", "74")))
    issue = event["issue"]
    item_id = find_item(project["id"], issue["node_id"])
    if not item_id:
        print(f"Issue #{issue['number']} is not on the Guild Board")
        return

    set_single_select_field(project, item_id, "Status", IN_PROGRESS_STATUS)
    set_assigned_date(project, item_id)


if __name__ == "__main__":
    HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    main()
