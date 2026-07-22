#!/usr/bin/env python3
"""Zed Guild Board (#74): cleanup, stale-nudge, shipped-weekly, repro/assignment events."""

import json
import os
import sys
import time
from datetime import datetime, timedelta, timezone

import requests

GITHUB_API = "https://api.github.com"
OWNER = "zed-industries"
REPO = "zed"
MAX_RETRIES = 3
RETRY_DELAY = 5

SHIPPED_STATUS = "Shipped by Guild"
IN_PROGRESS_STATUS = "In Progress"
STALE_DAYS = 14

# Labels that exist in zed-industries/zed as of 2026-06-24
REPRO_LABELS = {
    "state:reproducible": "Reproduced",
    "state:unactionable": "Cannot Reproduce",
}


def graphql(query, variables):
    for attempt in range(MAX_RETRIES + 1):
        r = requests.post(
            f"{GITHUB_API}/graphql",
            headers=HEADERS,
            json={"query": query, "variables": variables},
        )
        if r.status_code in (502, 503, 504) and attempt < MAX_RETRIES:
            time.sleep(RETRY_DELAY)
            continue
        r.raise_for_status()
        data = r.json()
        if "errors" in data:
            raise RuntimeError(f"GraphQL: {data['errors']}")
        return data["data"]


def rest_post(path, body):
    r = requests.post(f"{GITHUB_API}/{path}", headers=HEADERS, json=body)
    r.raise_for_status()
    return r.json()


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
        print(
            f"Could not fetch project #{number}. "
            "Check that your token has read:project (classic) or "
            "Organization Projects: Read and write (fine-grained) scope."
        )
        sys.exit(1)
    return project


def list_project_items(project_id):
    cursor = None
    while True:
        data = graphql("""
            query($pid: ID!, $cursor: String) {
              node(id: $pid) {
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
                          ... on ProjectV2ItemFieldDateValue {
                            field { ... on ProjectV2Field { name } }
                            date
                          }
                        }
                      }
                      content {
                        __typename
                        ... on Issue {
                          id number state closed closedAt
                          assignees(first: 10) { nodes { login } }
                          labels(first: 30) { nodes { name } }
                          timelineItems(last: 1, itemTypes: [CROSS_REFERENCED_EVENT]) {
                            nodes {
                              ... on CrossReferencedEvent {
                                source { __typename ... on PullRequest { number state } }
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
        """, {"pid": project_id, "cursor": cursor})
        page = data["node"]["items"]
        yield from page["nodes"]
        if not page["pageInfo"]["hasNextPage"]:
            return
        cursor = page["pageInfo"]["endCursor"]


def set_field(project, item_id, field_name, option_name):
    field = next((f for f in project["fields"]["nodes"] if f.get("name") == field_name), None)
    if not field:
        print(f"Field '{field_name}' missing, skipping")
        return
    option = next((o for o in field.get("options", []) if o["name"] == option_name), None)
    if not option:
        print(f"Option '{option_name}' missing in '{field_name}', skipping")
        return
    graphql("""
        mutation($pid: ID!, $iid: ID!, $fid: ID!, $oid: String!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $pid, itemId: $iid, fieldId: $fid,
            value: { singleSelectOptionId: $oid }
          }) { projectV2Item { id } }
        }
    """, {"pid": project["id"], "iid": item_id, "fid": field["id"], "oid": option["id"]})
    print(f"Set '{field_name}' = '{option_name}' on {item_id}")


def add_comment(issue_number, body):
    rest_post(f"repos/{OWNER}/{REPO}/issues/{issue_number}/comments", {"body": body})
    print(f"Commented on #{issue_number}")


def is_guild_member(login):
    slug = os.environ.get("GUILD_TEAM_SLUG", "zed-guild")
    r = requests.get(
        f"{GITHUB_API}/orgs/{OWNER}/teams/{slug}/members/{login}",
        headers=HEADERS,
    )
    return r.status_code == 204


def find_item(project_id, content_node_id):
    data = graphql("""
        query($cid: ID!) {
          node(id: $cid) {
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


def run_cleanup(project_number):
    project = fetch_project(project_number)
    shipped = 0
    for item in list_project_items(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue":
            continue
        if content.get("state") != "CLOSED":
            continue
        current_status = next(
            (fv.get("name") for fv in item.get("fieldValues", {}).get("nodes", [])
             if fv.get("field", {}).get("name") == "Status"), None)
        if current_status == SHIPPED_STATUS:
            continue
        assignees = [a["login"] for a in content.get("assignees", {}).get("nodes", [])]
        if not any(is_guild_member(a) for a in assignees):
            continue
        set_field(project, item["id"], "Status", SHIPPED_STATUS)
        shipped += 1
    print(f"Cleanup: {shipped} marked shipped")


def run_stale_nudge(project_number):
    project = fetch_project(project_number)
    now = datetime.now(timezone.utc)
    nudged = 0
    for item in list_project_items(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue" or content.get("state") != "OPEN":
            continue
        assignees = [a["login"] for a in content.get("assignees", {}).get("nodes", [])]
        if not assignees:
            continue
        has_pr = any(
            (tl.get("source") or {}).get("__typename") == "PullRequest"
            and (tl.get("source") or {}).get("state") == "OPEN"
            for tl in content.get("timelineItems", {}).get("nodes", []))
        if has_pr:
            continue
        assigned_date = next(
            (fv.get("date") for fv in item.get("fieldValues", {}).get("nodes", [])
             if fv.get("field", {}).get("name") == "Assigned Date"), None)
        if not assigned_date:
            continue
        days = (now - datetime.fromisoformat(assigned_date).replace(tzinfo=timezone.utc)).days
        if days < STALE_DAYS:
            continue
        assignee_list = ", ".join(f"@{a}" for a in assignees)
        add_comment(content["number"],
            f"Hey {assignee_list}, this issue has been assigned for "
            f"{days} days with no linked PR yet. If you're still working on it, "
            f"no action needed. If you're no longer planning to pick this up, "
            f"feel free to unassign yourself so another Guild member can grab it. "
            f"(This check runs once every two weeks.)")
        nudged += 1
    print(f"Stale nudge: {nudged} issues nudged")


def run_event(project_number):
    event_name = os.environ["GITHUB_EVENT_NAME"]
    with open(os.environ["GITHUB_EVENT_PATH"]) as f:
        event = json.load(f)
    if event_name != "issues":
        print(f"Ignoring event: {event_name}")
        return
    action = event["action"]
    issue = event["issue"]
    if action == "labeled":
        handle_label(event, issue, project_number)
    elif action == "assigned":
        handle_assignment(event, issue, project_number)


def run_shipped_weekly(project_number):
    project = fetch_project(project_number)
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
            (fv.get("name") for fv in item.get("fieldValues", {}).get("nodes", [])
             if fv.get("field", {}).get("name") == "Status"), None)
        if status != SHIPPED_STATUS:
            continue
        closed_at = content.get("closedAt")
        if not closed_at:
            continue
        closed_dt = datetime.fromisoformat(closed_at.replace("Z", "+00:00"))
        if closed_dt < week_ago:
            continue
        assignees = [a["login"] for a in content.get("assignees", {}).get("nodes", [])]
        shipped.append({
            "number": content["number"],
            "title": content["title"],
            "assignees": assignees,
        })
    if not shipped:
        print("Nothing shipped this week, skipping discussion post.")
        return
    week_str = now.strftime("%b %d, %Y")
    title = f"Shipped by the Guild - Week of {week_str}"
    lines = [f"Here's what the Zed Guild shipped this week:\n"]
    for item in shipped:
        credit = ", ".join(f"@{a}" for a in item["assignees"]) if item["assignees"] else "unassigned"
        lines.append(f"- #{item['number']} {item['title']} ({credit})")
    lines.append(f"\n{len(shipped)} issue{'s' if len(shipped) != 1 else ''} shipped. Nice work.")
    body = "\n".join(lines)
    category_id = os.environ.get("DISCUSSION_CATEGORY_ID")
    if not category_id:
        print("DISCUSSION_CATEGORY_ID not set, printing instead:\n")
        print(f"# {title}\n\n{body}")
        return
    repo_id = graphql("""
        query($owner: String!, $name: String!) {
          repository(owner: $owner, name: $name) { id }
        }
    """, {"owner": OWNER, "name": REPO})["repository"]["id"]
    graphql("""
        mutation($repoId: ID!, $catId: ID!, $title: String!, $body: String!) {
          createDiscussion(input: {
            repositoryId: $repoId, categoryId: $catId,
            title: $title, body: $body
          }) { discussion { url } }
        }
    """, {"repoId": repo_id, "catId": category_id, "title": title, "body": body})
    print(f"Posted discussion: {title}")


def handle_label(event, issue, project_number):
    label_name = event["label"]["name"]
    if label_name not in REPRO_LABELS:
        return
    project = fetch_project(project_number)
    item_id = find_item(project["id"], issue["node_id"])
    if not item_id:
        print(f"Issue #{issue['number']} not on board, skipping repro tracking")
        return
    repro_value = REPRO_LABELS[label_name]
    set_field(project, item_id, "Repro Status", repro_value)
    print(f"Repro by {event['sender']['login']}: {repro_value} on #{issue['number']}")


def handle_assignment(event, issue, project_number):
    assignee = event["assignee"]["login"]
    if not is_guild_member(assignee):
        print(f"{assignee} not in guild team, skipping")
        return
    project = fetch_project(project_number)
    item_id = find_item(project["id"], issue["node_id"])
    if not item_id:
        print(f"Issue #{issue['number']} not on board")
        return
    set_field(project, item_id, "Status", IN_PROGRESS_STATUS)
    date_field = next(
        (f for f in project["fields"]["nodes"]
         if f.get("name") == "Assigned Date" and f.get("dataType") == "DATE"), None)
    if date_field:
        graphql("""
            mutation($pid: ID!, $iid: ID!, $fid: ID!, $date: Date!) {
              updateProjectV2ItemFieldValue(input: {
                projectId: $pid, itemId: $iid, fieldId: $fid,
                value: { date: $date }
              }) { projectV2Item { id } }
            }
        """, {"pid": project["id"], "iid": item_id, "fid": date_field["id"],
              "date": datetime.now(timezone.utc).strftime("%Y-%m-%d")})
        print(f"Stamped Assigned Date on #{issue['number']}")


if __name__ == "__main__":
    HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    project_number = int(os.environ.get("PROJECT_NUMBER", "74"))
    mode = os.environ.get("MODE", "event")
    {"cleanup": run_cleanup, "stale-nudge": run_stale_nudge, "event": run_event,
     "shipped-weekly": run_shipped_weekly}.get(
        mode, lambda _: (print(f"Unknown mode: {mode}"), sys.exit(1))
    )(project_number)
