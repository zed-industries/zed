#!/usr/bin/env python3
"""
Assign a labeled contributor issue to the least-busy interested contributor.

When an issue has both a `.contrib/good *` label and an `area:` label, this
script:
1. Fetches Tally form responses to find contributors interested in the issue's areas
2. Queries GitHub for each candidate's current open issue assignment count
3. Assigns the issue to the least-busy candidate (random tiebreak)
4. Adds the issue to a GitHub project board with "Assign" status
5. Notifies the assignee via Slack DM and posts to an activity channel

Errors and notable conditions (no candidates found, API failures) are reported
to the Slack activity channel before the script exits.

Requires:
    requests (pip install requests)

Usage:
    python github-assign-contributor-issue.py <issue_number>

"""

import json
import os
import random
import sys

import requests

GITHUB_API = "https://api.github.com"
TALLY_API = "https://api.tally.so"
SLACK_API = "https://slack.com/api"

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
PROJECT_NUMBER = 83
SLACK_ACTIVITY_CHANNEL_ID = "C0B0JCE8GDC"


def eligible_areas(issue):
    """Returns the list of area names if the issue is eligible for assignment, or None."""
    labels = [label["name"] for label in issue["labels"]]
    assignees = [a["login"] for a in issue["assignees"]]

    contrib_labels = [name for name in labels if name.startswith(".contrib/good ")]
    area_labels = [name for name in labels if name.startswith("area:")]

    if not contrib_labels or not area_labels:
        print("Issue needs both a .contrib/good * label and an area: label, skipping")
        return None

    if assignees:
        print(f"Issue is already assigned to {assignees}, skipping")
        return None

    areas = [label.removeprefix("area:") for label in area_labels]
    print(f"Areas: {areas}")
    return areas


# --- Tally ---


def fetch_tally_contributors(api_key, form_id):
    """Fetch all completed submissions from a Tally form.

    Deduplicates by GitHub username, keeping the latest submission.
    """
    headers = {"Authorization": f"Bearer {api_key}"}
    contributors = {}
    page = 1

    while True:
        response = requests.get(
            f"{TALLY_API}/forms/{form_id}/submissions",
            headers=headers,
            params={"page": page, "limit": 500, "filter": "completed"},
        )
        response.raise_for_status()
        data = response.json()

        field_titles = {}
        for question in data.get("questions", []):
            for field in question.get("fields", []):
                field_titles[field["uuid"]] = field.get("title", "")

        questions = {q["id"]: q for q in data.get("questions", [])}

        for submission in data.get("submissions", []):
            record = parse_submission(submission, questions, field_titles)
            if record:
                contributors[record["github_username"].lower()] = record

        if not data.get("hasMore", False):
            break
        page += 1

    return list(contributors.values())


def parse_submission(submission, questions, field_titles):
    """Parse a single Tally submission into a contributor record.

    Returns a dict with github_username, email (optional), and areas,
    or None if the submission is incomplete.
    """
    github_username = None
    email = None
    areas = []

    for response in submission.get("responses", []):
        try:
            question_title = questions[response["questionId"]]["title"].lower()
            answer = response["answer"]
        except KeyError:
            continue

        try:
            if "github" in question_title:
                github_username = str(answer).strip().lstrip("@")
            elif "email" in question_title:
                email = str(answer).strip().lower()
            elif "area" in question_title:
                for item in answer if isinstance(answer, list) else [answer]:
                    area = field_titles.get(item, item).strip()
                    if area:
                        areas.append(area)
        except (TypeError, AttributeError):
            continue

    if not github_username or not areas:
        return None

    record = {"github_username": github_username, "areas": areas}
    if email:
        record["email"] = email
    return record


def find_candidates(contributors, area_names):
    """Find contributors interested in any of the given areas (case-insensitive)."""
    target = {name.lower() for name in area_names}
    return [c for c in contributors if any(a.lower() in target for a in c["areas"])]


def pick_least_busy(github_headers, candidates):
    """Pick the candidate with the fewest open assignments (random tiebreak)."""
    usernames = [c["github_username"] for c in candidates]
    loads = count_open_assignments(github_headers, usernames)
    for username, count in loads.items():
        print(f"  {username}: {count} open assignments")

    min_load = min(loads.values())
    least_busy = [c for c in candidates if loads[c["github_username"]] == min_load]
    chosen = random.choice(least_busy)
    print(
        f"Selected: {chosen['github_username']} (load: {min_load}, {len(least_busy)} tied)"
    )
    return chosen


# --- GitHub ---


def fetch_issue(headers, issue_number):
    """Fetch issue details from the GitHub API."""
    response = requests.get(
        f"{GITHUB_API}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}",
        headers=headers,
    )
    response.raise_for_status()
    return response.json()


def count_open_assignments(headers, usernames):
    """Count open issues assigned to each user in a single GraphQL request."""
    aliases = [
        f'u{i}: search(query: "repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open assignee:{name}", type: ISSUE) {{ issueCount }}'
        for i, name in enumerate(usernames)
    ]
    query = "query {\n" + "\n".join(aliases) + "\n}"
    data = execute_graphql(headers, query, {})
    return {name: data[f"u{i}"]["issueCount"] for i, name in enumerate(usernames)}


def assign_issue(headers, issue_number, username):
    """Assign a GitHub issue to a user."""
    response = requests.post(
        f"{GITHUB_API}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/assignees",
        headers=headers,
        json={"assignees": [username]},
    )
    response.raise_for_status()


def execute_graphql(headers, query, variables):
    """Execute a GitHub GraphQL query. Raises on HTTP or GraphQL errors."""
    response = requests.post(
        f"{GITHUB_API}/graphql",
        headers=headers,
        json={"query": query, "variables": variables},
    )
    response.raise_for_status()
    result = response.json()
    if "errors" in result:
        raise RuntimeError(f"GraphQL error: {result['errors']}")
    return result["data"]


def fetch_project(headers, project_number):
    """Fetch a GitHub project board's metadata including fields and status options."""
    data = execute_graphql(
        headers,
        """
        query($owner: String!, $number: Int!) {
          organization(login: $owner) {
            projectV2(number: $number) {
              id
              fields(first: 50) {
                nodes {
                  ... on ProjectV2SingleSelectField {
                    id
                    name
                    options { id name }
                  }
                }
              }
            }
          }
        }
        """,
        {"owner": REPO_OWNER, "number": project_number},
    )
    return data["organization"]["projectV2"]


def add_issue_to_project(headers, project_id, issue_node_id):
    """Add an issue to a GitHub project board. Returns the project item ID."""
    data = execute_graphql(
        headers,
        """
        mutation($projectId: ID!, $contentId: ID!) {
          addProjectV2ItemById(input: {projectId: $projectId, contentId: $contentId}) {
            item { id }
          }
        }
        """,
        {"projectId": project_id, "contentId": issue_node_id},
    )
    item_id = data["addProjectV2ItemById"]["item"]["id"]
    print(f"Added issue to project (item: {item_id})")
    return item_id


def set_project_item_status(headers, project, item_id, status_name):
    """Set the Status field on a project item. Hard-fails if the status option is missing."""
    status_field_id = None
    option_id = None
    for field in project["fields"]["nodes"]:
        if field.get("name") == "Status":
            status_field_id = field["id"]
            for option in field.get("options", []):
                if option["name"] == status_name:
                    option_id = option["id"]
                    break
            break

    if not status_field_id or not option_id:
        available = [f.get("name") for f in project["fields"]["nodes"] if f.get("name")]
        raise RuntimeError(
            f"Could not find Status field with '{status_name}' option. "
            f"Fields found: {available}"
        )

    execute_graphql(
        headers,
        """
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId
            itemId: $itemId
            fieldId: $fieldId
            value: { singleSelectOptionId: $optionId }
          }) {
            projectV2Item { id }
          }
        }
        """,
        {
            "projectId": project["id"],
            "itemId": item_id,
            "fieldId": status_field_id,
            "optionId": option_id,
        },
    )
    print(f"Set project status to '{status_name}'")


# --- Slack ---


def slack_post_message(headers, recipient, text):
    """Post a message to a Slack channel or user DM."""
    response = requests.post(
        f"{SLACK_API}/chat.postMessage",
        headers=headers,
        json={"channel": recipient, "text": text},
    )
    response.raise_for_status()
    data = response.json()
    if not data["ok"]:
        raise RuntimeError(f"Slack API error: {data['error']}")


def find_slack_user_id(headers, email):
    """Look up a Slack user ID by email. Returns None if not found."""
    try:
        response = requests.get(
            f"{SLACK_API}/users.lookupByEmail",
            headers=headers,
            params={"email": email},
        )
        response.raise_for_status()
        return response.json()["user"]["id"]
    except (requests.RequestException, KeyError):
        return None


def post_to_activity(slack_headers, message):
    """Best-effort post to the Slack activity channel."""
    try:
        slack_post_message(slack_headers, SLACK_ACTIVITY_CHANNEL_ID, message)
    except Exception as exc:
        print(f"Failed to post to Slack activity channel: {exc}")


def notify_assignment(slack_headers, chosen, issue):
    """DM the chosen contributor and post to the activity channel."""
    issue_number = issue["number"]
    issue_title = issue["title"]
    issue_url = issue["html_url"]
    chosen_username = chosen["github_username"]

    slack_user_id = find_slack_user_id(slack_headers, chosen.get("email"))

    if slack_user_id:
        slack_post_message(
            slack_headers,
            slack_user_id,
            f"\U0001f44b You've been assigned to <{issue_url}|#{issue_number}: {issue_title}>! "
            f"This issue matches your areas of interest. "
            f"Let us know if you have any questions.",
        )

    activity_message = (
        f"\U0001f4cb <{issue_url}|#{issue_number}: {issue_title}> "
        f"assigned to *{chosen_username}*"
    )
    if slack_user_id:
        activity_message += f" (<@{slack_user_id}>)"
    post_to_activity(slack_headers, activity_message)


# --- Main ---


if __name__ == "__main__":
    issue_number = sys.argv[1]

    github_token = os.environ["GITHUB_TOKEN"]
    tally_api_key = os.environ["TALLY_API_KEY"]
    tally_form_id = os.environ["TALLY_FORM_ID"]
    slack_bot_token = os.environ["SLACK_CONTRIBUTOR_ROUTING_BOT_TOKEN"]

    github_headers = {
        "Authorization": f"Bearer {github_token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    slack_headers = {
        "Authorization": f"Bearer {slack_bot_token}",
        "Content-Type": "application/json",
    }

    issue = fetch_issue(github_headers, issue_number)
    if not (areas := eligible_areas(issue)):
        sys.exit(0)

    try:
        contributors = fetch_tally_contributors(tally_api_key, tally_form_id)
        print(f"Found {len(contributors)} contributors in Tally")

        candidates = find_candidates(contributors, areas)
        if not candidates:
            post_to_activity(
                slack_headers,
                f"\u26a0\ufe0f No contributors found for {', '.join(areas)} \u2014 "
                f"<{issue['html_url']}|#{issue_number}: {issue['title']}>",
            )
            print(f"No contributors interested in areas: {areas}")
            sys.exit(0)

        chosen = pick_least_busy(github_headers, candidates)

        assign_issue(github_headers, issue_number, chosen["github_username"])
        print(f"Assigned #{issue_number} to {chosen['github_username']}")

        project = fetch_project(github_headers, PROJECT_NUMBER)
        item_id = add_issue_to_project(github_headers, project["id"], issue["node_id"])
        set_project_item_status(github_headers, project, item_id, "Assigned")

        notify_assignment(slack_headers, chosen, issue)

    except Exception as exc:
        post_to_activity(
            slack_headers,
            f"\u274c Failed to assign contributor for "
            f"<{issue['html_url']}|#{issue_number}: {issue['title']}>: {exc}",
        )
        raise
