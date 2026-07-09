#!/usr/bin/env python3
"""
Automation for the Guild project board (#74).

GUILD_MODE selects behavior:

- event: react to a single issue webhook.
    assigned    guild member -> Status "In Progress", or a Slack heads-up if
                the issue isn't on the board.
    unassigned  guild member off an "In Progress" issue -> move back to a
                To-Do column by Type (e.g. Bugs go back to Bug Bashers column) + Slack.
    created     (issue_comment) guild assignee comments on an issue that has a
                pending check-in -> Slack (fires on each such comment).
- stale: nudge issues assigned to a guild member with no linked PR once the
    assignee goes quiet; a reply resets the clock, so nudges recur after renewed
    silence, and the assignment is cleared after a further grace period without a
    reply. The "guild hold" label pauses both nudging and clearing.
- weekly: Slack digest of board issues recently closed by a merged PR authored
    by a guild member.

Requires: requests
"""

import html
import json
import os
import random
import time
from datetime import datetime, timedelta, timezone
from functools import lru_cache

import requests

RETRYABLE_STATUS_CODES = {502, 503, 504}
MAX_RETRIES = 3
RETRY_DELAY_SECONDS = 5

GITHUB_API_URL = "https://api.github.com"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
# Cohort members are outside collaborators on the repo holding this custom
# repository role, rather than members of an org team. Rotating the cohort is
# then just adding/removing collaborators, with no org seats involved.
GUILD_ROLE_NAME = "Guild Assign issues/PRs"

STATUS_FIELD = "Status"
STATUS_IN_PROGRESS = "In Progress"
STATUS_BUG_BASHERS = "Bug Bashers"
STATUS_SHIP_FEATURE = "Ship a New Feature"

# Which to-do column to move issues of a given type to.
STATUS_FOR_TYPE = {
    "Feature": STATUS_SHIP_FEATURE,
    "Bug": STATUS_BUG_BASHERS,
    "Docs": STATUS_BUG_BASHERS,
    "Crash": STATUS_BUG_BASHERS,
}

# How many days to wait after assignment before posting a check-in comment.
CHECK_IN_AFTER_DAYS = 14
# Clear the assignment after this many days of no reply to check-in.
AUTO_CLEAR_AFTER_DAYS = 7
# For the slack summaries of what's been recently shipped by the Guild.
SHIPPED_WINDOW_DAYS = 7

# Applying this label to an issue pauses the stale sweep for it (no nudges, no
# auto-clear of the assignee) until a human removes the label.
NUDGE_HOLD_LABEL = "guild hold"

# Hidden in the rendered issue; lets later runs find the bot's own check-in.
CHECK_IN_MARKER = "<!-- zedgar:guild-check-in -->"

CHECK_IN_BODY = (
    "{marker}\n"
    "Hey @{assignee}, checking in to see if you're still actively working on "
    "this issue. No worries if you no longer have the time. If that's the case, "
    "do you mind unassigning yourself from the issue so another contributor can "
    "work on it? If you are still working on it, drop a comment and let us know "
    "how we can help! Otherwise, the bot will clear the assignment in "
    "{clear_days} days from now."
)

ZEDGAR_QUIPS = [
    "Deep into the backlog peering...",
    "Once upon a board so dreary...",
    "A tell-tale ping beneath the board.",
]


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
    url = f"{GITHUB_API_URL}/{path}"
    for attempt in range(MAX_RETRIES + 1):
        response = requests.request(
            method, url, headers=GITHUB_HEADERS, json=body, timeout=30
        )
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
            break
        results.extend(batch)
        if len(batch) < 100:
            break
        page += 1
    return results


@lru_cache(maxsize=None)
def is_guild_member(username):
    response = requests.get(
        f"{GITHUB_API_URL}/repos/{REPO_OWNER}/{REPO_NAME}/collaborators/{username}/permission",
        headers=GITHUB_HEADERS,
        timeout=30,
    )
    # 404 means the user isn't a collaborator on the repo at all.
    if response.status_code == 404:
        return False
    response.raise_for_status()
    # role_name is the effective (highest) role for the user. For a cohort of
    # outside collaborators whose only grant is this custom role, that is the
    # custom role's name; built-in roles come back lowercased and won't match.
    role_name = response.json().get("role_name") or ""
    return role_name.lower() == GUILD_ROLE_NAME.lower()


def issue_comments(issue_number):
    return github_rest_get_paginated(
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/comments"
    )


def latest_assignment_time(issue_number, assignee):
    events = github_rest_get_paginated(
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/timeline"
    )
    times = [
        parse_dt(event["created_at"])
        for event in events
        if event.get("event") == "assigned"
        and (event.get("assignee") or {}).get("login") == assignee
    ]
    if not times:
        raise RuntimeError(
            f"No assignment event for {assignee} on issue #{issue_number}"
        )
    return max(times)


def issue_closing_prs(issue_node_id, include_closed_prs=False):
    # Caps at the first 20 closing PRs: callers only test presence or scan for a
    # single guild-authored merge, so an issue exceeding this bound is not worth
    # paginating.
    data = github_graphql(
        """
        query($issueId: ID!, $includeClosedPrs: Boolean!) {
          node(id: $issueId) {
            ... on Issue {
              closedByPullRequestsReferences(first: 20, includeClosedPrs: $includeClosedPrs) {
                nodes { merged author { login } }
              }
            }
          }
        }
        """,
        {"issueId": issue_node_id, "includeClosedPrs": include_closed_prs},
    )
    node = data.get("node") or {}
    refs = node.get("closedByPullRequestsReferences") or {}
    return [
        {"merged": pr["merged"], "author": (pr.get("author") or {}).get("login")}
        for pr in refs.get("nodes", [])
    ]


def fetch_project(project_number):
    data = github_graphql(
        """
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
        """,
        {"owner": REPO_OWNER, "number": project_number},
    )
    project = data["organization"]["projectV2"]
    if not project:
        raise RuntimeError(f"Project #{project_number} not found in {REPO_OWNER}")
    return project


def find_project_item(project_id, content_node_id):
    # Also fetches each item's single-select values, so callers that need the
    # issue's Status (the unassignment handler) don't have to re-query the item.
    data = github_graphql(
        """
        query($contentId: ID!) {
          node(id: $contentId) {
            ... on Issue {
              projectItems(first: 50) {
                nodes {
                  id
                  project { id }
                  fieldValues(first: 20) {
                    nodes {
                      ... on ProjectV2ItemFieldSingleSelectValue {
                        field { ... on ProjectV2SingleSelectField { name } }
                        name
                      }
                    }
                  }
                }
              }
            }
          }
        }
        """,
        {"contentId": content_node_id},
    )
    node = data.get("node") or {}
    for item in (node.get("projectItems") or {}).get("nodes", []):
        if item["project"]["id"] == project_id:
            return item
    return None


def set_project_field(project, item_id, field_name, option_name):
    field = next(
        (f for f in project["fields"]["nodes"] if f.get("name") == field_name), None
    )
    if not field:
        raise RuntimeError(f"Field '{field_name}' not found on board")
    option_id = next(
        (o["id"] for o in field.get("options", []) if o["name"] == option_name), None
    )
    if not option_id:
        raise RuntimeError(f"Option '{option_name}' not found in field '{field_name}'")
    github_graphql(
        """
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
            value: { singleSelectOptionId: $optionId }
          }) { projectV2Item { id } }
        }
        """,
        {
            "projectId": project["id"],
            "itemId": item_id,
            "fieldId": field["id"],
            "optionId": option_id,
        },
    )


def list_project_issues(project_id):
    cursor = None
    while True:
        data = github_graphql(
            """
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
                          id number title url state closedAt
                          issueType { name }
                          assignees(first: 10) { nodes { login } }
                          labels(first: 20) { nodes { name } }
                        }
                      }
                    }
                  }
                }
              }
            }
            """,
            {"projectId": project_id, "cursor": cursor},
        )
        page = data["node"]["items"]
        yield from page["nodes"]
        if not page["pageInfo"]["hasNextPage"]:
            return
        cursor = page["pageInfo"]["endCursor"]


def post_comment(issue_number, body):
    github_rest_request(
        "POST", f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/comments", {"body": body}
    )


def remove_assignees(issue_number, assignees):
    github_rest_request(
        "DELETE",
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/assignees",
        {"assignees": assignees},
    )


def escape_slack(text):
    # Escape Slack's control characters (&, <, >) so free text like issue/PR
    # titles and comment bodies renders literally instead of being interpreted
    # as a link or @-mention. send_slack can't do this wholesale because its own
    # messages legitimately contain <url|text> links and *bold* markup.
    # quote=False keeps it to Slack's three characters (no " or ' escaping).
    return html.escape(text or "", quote=False)


def slack_link(url, text):
    # The single way to embed a titled link, so the title is always escaped
    # without every caller having to remember to do it.
    return f"<{url}|{escape_slack(text)}>"


def send_slack(text):
    webhook = os.environ.get("SLACK_WEBHOOK_GUILD_INTERNAL")
    if not webhook:
        raise RuntimeError("SLACK_WEBHOOK_GUILD_INTERNAL is not set")
    message = f"{random.choice(ZEDGAR_QUIPS)} {text}"
    response = requests.post(
        webhook,
        json={
            "text": message,
            "blocks": [{"type": "section", "text": {"type": "mrkdwn", "text": message}}],
        },
        timeout=30,
    )
    if response.status_code != 200:
        raise RuntimeError(
            f"Slack webhook returned {response.status_code}: {response.text[:200]}"
        )


def parse_dt(value):
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def item_status(item):
    for field_value in (item.get("fieldValues") or {}).get("nodes", []):
        if field_value.get("field", {}).get("name") == STATUS_FIELD:
            return field_value.get("name")
    return None


def latest_check_in_time(comments):
    times = [
        parse_dt(comment["created_at"])
        for comment in comments
        if CHECK_IN_MARKER in (comment.get("body") or "")
    ]
    return max(times, default=None)


def handle_assignment(issue, assignee_login, project_number):
    if not is_guild_member(assignee_login):
        return

    project = fetch_project(project_number)
    item = find_project_item(project["id"], issue["node_id"])
    if not item:
        send_slack(
            f"Heads-up: @{assignee_login} was assigned to issue #{issue['number']} "
            f"{slack_link(issue['html_url'], issue['title'])}, which isn't on the Guild board. "
            "Possible mistake — if it's meant to be Guild work, add it to Bug Bashers "
            "or Ship a New Feature; otherwise no action needed."
        )
        return

    set_project_field(project, item["id"], STATUS_FIELD, STATUS_IN_PROGRESS)


def handle_unassignment(issue, removed_login, sender, project_number):
    # Our own auto-clear also unassigns; don't double-handle that.
    if (sender or {}).get("type") == "Bot":
        return
    if not is_guild_member(removed_login):
        return

    project = fetch_project(project_number)
    item = find_project_item(project["id"], issue["node_id"])
    if not item or item_status(item) != STATUS_IN_PROGRESS:
        return

    sender_login = (sender or {}).get("login")
    who = (
        "unassigned themselves from"
        if sender_login == removed_login
        else f"was unassigned (by @{sender_login}) from"
    )
    status = STATUS_FOR_TYPE.get((issue.get("type") or {}).get("name") or "")
    link = slack_link(issue["html_url"], issue["title"])
    if status:
        set_project_field(project, item["id"], STATUS_FIELD, status)
        send_slack(
            f"@{removed_login} {who} In Progress issue #{issue['number']} "
            f"{link}. I moved it back to *{status}*, "
            "so it's up for grabs again — no action needed."
        )
    else:
        send_slack(
            f"@{removed_login} {who} In Progress issue #{issue['number']} "
            f"{link}. I couldn't tell its Type, so it's "
            "still In Progress — please move it to Bug Bashers or Ship a New Feature so it "
            "can be picked up again, then set the :done-checkmark: emoji on this message when done."
        )


def handle_comment(issue, comment):
    commenter = (comment.get("user") or {}).get("login")
    if not commenter or CHECK_IN_MARKER in (comment.get("body") or ""):
        return

    assignees = [a["login"] for a in (issue.get("assignees") or [])]
    if commenter not in assignees or not is_guild_member(commenter):
        return

    check_in_at = latest_check_in_time(issue_comments(issue["number"]))
    if check_in_at is None or parse_dt(comment["created_at"]) <= check_in_at:
        return

    reply = " ".join((comment.get("body") or "").split())
    if len(reply) > 500:
        reply = reply[:500] + "…"
    send_slack(
        f"@{commenter} replied to the check-in on In Progress issue #{issue['number']} "
        f"{slack_link(issue['html_url'], issue['title'])}:\n"
        f"> {escape_slack(reply)}\n"
        f"Follow up if they need help, or add the `{NUDGE_HOLD_LABEL}` label to the "
        "issue to pause check-ins on it."
    )


def run_event(project_number):
    event_name = os.environ.get("GITHUB_EVENT_NAME", "")
    with open(os.environ["GITHUB_EVENT_PATH"]) as f:
        event = json.load(f)

    if event_name == "issues":
        action = event["action"]
        issue = event["issue"]
        if action == "assigned":
            handle_assignment(issue, event["assignee"]["login"], project_number)
        elif action == "unassigned":
            handle_unassignment(
                issue, event["assignee"]["login"], event.get("sender"), project_number
            )
    elif event_name == "issue_comment" and "pull_request" not in event["issue"]:
        handle_comment(event["issue"], event["comment"])


def run_stale(project_number):
    project = fetch_project(project_number)
    checked_in = cleared = 0

    for item in list_project_issues(project["id"]):
        if item.get("isArchived"):
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "Issue":
            continue
        if item_status(item) != STATUS_IN_PROGRESS:
            continue

        assignees = [a["login"] for a in content["assignees"]["nodes"]]
        guild_assignees = [a for a in assignees if is_guild_member(a)]
        if not guild_assignees:
            continue

        labels = [label["name"] for label in (content.get("labels") or {}).get("nodes", [])]
        if NUDGE_HOLD_LABEL in labels:
            continue

        issue_number = content["number"]
        if issue_closing_prs(content["id"]):
            continue

        comments = issue_comments(issue_number)
        assignee = guild_assignees[0]

        # One pass over the comments for both the latest check-in and the
        # assignee's latest comment. Measuring from the assignee's last activity
        # (their assignment or a later comment) means a reply resets the clock, so
        # a nudge only recurs after renewed silence (and a check-in from a prior
        # assignment stint predates this, so it is naturally ignored).
        last_activity = latest_assignment_time(issue_number, assignee)
        check_in_at = None
        for comment in comments:
            created_at = parse_dt(comment["created_at"])
            if CHECK_IN_MARKER in (comment.get("body") or ""):
                if check_in_at is None or created_at > check_in_at:
                    check_in_at = created_at
            elif (comment.get("user") or {}).get("login") == assignee:
                if created_at > last_activity:
                    last_activity = created_at

        if check_in_at is None or check_in_at < last_activity:
            # No outstanding nudge since the assignee last engaged; nudge once
            # they have stayed quiet for the check-in window.
            if (NOW - last_activity).days < CHECK_IN_AFTER_DAYS:
                continue
            post_comment(
                issue_number,
                CHECK_IN_BODY.format(
                    marker=CHECK_IN_MARKER,
                    assignee=assignee,
                    clear_days=AUTO_CLEAR_AFTER_DAYS,
                ),
            )
            checked_in += 1
            continue

        # We nudged and the assignee stayed quiet; clear once the grace period
        # lapses, otherwise leave them the rest of it.
        if (NOW - check_in_at).days < AUTO_CLEAR_AFTER_DAYS:
            continue

        remove_assignees(issue_number, guild_assignees)
        status = STATUS_FOR_TYPE.get((content.get("issueType") or {}).get("name") or "")
        link = slack_link(content["url"], content["title"])
        if status:
            set_project_field(project, item["id"], STATUS_FIELD, status)
            send_slack(
                f"Issue #{issue_number} {link} went quiet with @{assignee} "
                "(no linked PR, no reply to my check-in), so I cleared the assignment and "
                f"moved it back to *{status}*. It's up for grabs again — no action needed."
            )
        else:
            send_slack(
                f"Issue #{issue_number} {link} went quiet with @{assignee} "
                "(no linked PR, no reply to my check-in), so I cleared the assignment. I "
                "couldn't tell its Type — please move it to Bug Bashers or Ship a New "
                "Feature so it can be picked up again, then set the :done-checkmark: emoji "
                "on this message when done."
            )
        cleared += 1

    print(f"Stale sweep: {checked_in} checked in, {cleared} cleared")


def run_weekly(project_number):
    project = fetch_project(project_number)
    cutoff = NOW - timedelta(days=SHIPPED_WINDOW_DAYS)
    shipped = []

    for item in list_project_issues(project["id"]):
        content = item.get("content") or {}
        if content.get("__typename") != "Issue" or content.get("state") != "CLOSED":
            continue
        if parse_dt(content["closedAt"]) < cutoff:
            continue
        for pr in issue_closing_prs(content["id"], include_closed_prs=True):
            if pr["merged"] and pr["author"] and is_guild_member(pr["author"]):
                shipped.append((pr["author"], content))
                break

    if not shipped:
        send_slack("A quiet week on the Guild board — nothing shipped. Onward.")
        return

    lines = "\n".join(
        f"• @{author} — {slack_link(content['url'], content['title'])} (#{content['number']})"
        for author, content in shipped
    )
    send_slack(f"Here's what the Guild shipped this week:\n{lines}")


if __name__ == "__main__":
    GITHUB_HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }

    NOW = datetime.now(timezone.utc)

    project_number = int(os.environ["PROJECT_NUMBER"])
    mode = os.environ["GUILD_MODE"]

    if mode == "event":
        run_event(project_number)
    elif mode == "stale":
        run_stale(project_number)
    elif mode == "weekly":
        run_weekly(project_number)
    else:
        raise RuntimeError(f"Unknown GUILD_MODE: {mode}")
