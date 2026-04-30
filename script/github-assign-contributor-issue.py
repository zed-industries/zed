#!/usr/bin/env python3
"""
Assign a labeled contributor issue to the least-busy interested contributor.

When an issue has both a `.contrib/good *` label and an `area:` label, this
script:
1. Fetches Tally form responses to find contributors interested in the issue's areas
2. Queries GitHub for each candidate's current open issue assignment count
3. Assigns the issue to the least-busy candidate (random tiebreak)
4. Adds the issue to a GitHub project board with "Assigned" status
5. Notifies the assignee via Slack DM and posts to an activity channel

Errors and notable conditions (no candidates found, API failures) are reported
to the Slack activity channel before the script exits.

Requires:
    requests (pip install requests)

Usage:
    python github-assign-contributor-issue.py <issue_number>

"""

import os
import random
import sys

import requests

GITHUB_API_URL = "https://api.github.com"
TALLY_API_URL = "https://api.tally.so"
SLACK_API_URL = "https://slack.com/api"

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
GITHUB_PROJECT_NUMBER = 83
SLACK_ACTIVITY_CHANNEL_ID = "C0B0JCE8GDC"


def tally_fetch_contributors(api_key, form_id):
    """Fetch all completed submissions from a Tally form.

    Deduplicates by GitHub username, keeping the latest submission.
    """
    headers = {"Authorization": f"Bearer {api_key}"}
    contributors = {}
    page = 1

    while True:
        response = requests.get(
            f"{TALLY_API_URL}/forms/{form_id}/submissions",
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
            record = tally_parse_submission(submission, questions, field_titles)
            if not record:
                continue
            key = record["github_username"].lower()
            if (
                key not in contributors
                or submission["submittedAt"] > contributors[key]["_submitted_at"]
            ):
                record["_submitted_at"] = submission["submittedAt"]
                contributors[key] = record

        if not data.get("hasMore", False):
            break
        page += 1

    return list(contributors.values())


def tally_parse_submission(submission, questions, field_titles):
    """Parse a single Tally submission into a contributor record.

    Returns a dict with github_username, email, and areas,
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

    return {
        "github_username": github_username,
        "email": email,
        "areas": areas,
    }


def eligible_areas(issue):
    """Returns the list of area names if the issue is eligible for assignment, or None."""
    labels = [label["name"] for label in issue["labels"]]
    assignees = [a["login"] for a in issue["assignees"]]

    has_contrib = any(name.startswith(".contrib/good ") for name in labels)
    area_labels = [name for name in labels if name.startswith("area:")]

    if not has_contrib or not area_labels:
        print("Issue needs both a .contrib/good * label and an area: label, skipping")
        return None

    if assignees:
        print(f"Issue is already assigned to {assignees}, skipping")
        return None

    areas = [label.removeprefix("area:") for label in area_labels]
    print(f"Areas: {areas}")
    return areas


def find_candidates(contributors, area_names):
    """Find contributors interested in any of the given areas (case-insensitive)."""
    target = {name.lower() for name in area_names}
    return [c for c in contributors if {a.lower() for a in c["areas"]} & target]


def pick_least_busy(candidates):
    """Pick the candidate with the fewest open assignments (random tiebreak)."""
    usernames = [c["github_username"] for c in candidates]
    loads = github_count_open_assignments(usernames)
    for username, count in loads.items():
        print(f"  {username}: {count} open assignments")

    min_load = min(loads.values())
    least_busy = [c for c in candidates if loads[c["github_username"]] == min_load]
    chosen = random.choice(least_busy)
    print(
        f"Selected: {chosen['github_username']} (load: {min_load}, {len(least_busy)} tied)"
    )
    return chosen


def github_get(path):
    """GET from the GitHub REST API."""
    response = requests.get(f"{GITHUB_API_URL}/{path}", headers=GITHUB_HEADERS)
    response.raise_for_status()
    return response.json()


def github_post(path, json):
    """POST to the GitHub REST API."""
    response = requests.post(
        f"{GITHUB_API_URL}/{path}", headers=GITHUB_HEADERS, json=json
    )
    response.raise_for_status()
    return response.json()


def github_graphql(query, variables):
    """Execute a GitHub GraphQL query. Raises on HTTP or GraphQL errors."""
    response = requests.post(
        f"{GITHUB_API_URL}/graphql",
        headers=GITHUB_HEADERS,
        json={"query": query, "variables": variables},
    )
    response.raise_for_status()
    result = response.json()
    if "errors" in result:
        raise RuntimeError(f"GraphQL error: {result['errors']}")
    return result["data"]


def github_fetch_issue(issue_number):
    """Fetch issue details from the GitHub API."""
    return github_get(f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}")


def github_count_open_assignments(usernames):
    """Count open issues assigned to each user in a single GraphQL request."""
    aliases = [
        f'u{i}: search(query: "repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open assignee:{name}", type: ISSUE) {{ issueCount }}'
        for i, name in enumerate(usernames)
    ]
    query = "query {\n" + "\n".join(aliases) + "\n}"
    data = github_graphql(query, {})
    return {name: data[f"u{i}"]["issueCount"] for i, name in enumerate(usernames)}


def github_assign_issue(issue_number, username):
    """Assign a GitHub issue to a user."""
    github_post(
        f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/assignees",
        json={"assignees": [username]},
    )


def github_fetch_project(project_number):
    """Fetch a GitHub project board's metadata including fields and status options."""
    data = github_graphql(
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


def github_add_issue_to_project(project_id, issue_node_id):
    """Add an issue to a GitHub project board. Returns the project item ID."""
    data = github_graphql(
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


def github_set_project_item_status(project, item_id, status_name):
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
        available = [f["name"] for f in project["fields"]["nodes"] if "name" in f]
        raise RuntimeError(
            f"Could not find Status field with '{status_name}' option. "
            f"Fields found: {available}"
        )

    github_graphql(
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


def slack_post_message(recipient, text):
    """Post a message to a Slack channel or user DM."""
    response = requests.post(
        f"{SLACK_API_URL}/chat.postMessage",
        headers=SLACK_HEADERS,
        json={"channel": recipient, "text": text},
    )
    response.raise_for_status()
    data = response.json()
    if not data["ok"]:
        raise RuntimeError(f"Slack API error: {data['error']}")


def slack_find_user_id(email):
    """Look up a Slack user ID by email. Returns None if not found."""
    try:
        response = requests.get(
            f"{SLACK_API_URL}/users.lookupByEmail",
            headers=SLACK_HEADERS,
            params={"email": email},
        )
        response.raise_for_status()
        return response.json()["user"]["id"]
    except (requests.RequestException, KeyError):
        return None


def slack_notify_on_assignment(chosen, issue):
    """DM the chosen contributor and post to the activity channel."""
    issue_number = issue["number"]
    issue_title = issue["title"]
    issue_url = issue["html_url"]
    chosen_username = chosen["github_username"]

    slack_user_id = slack_find_user_id(chosen.get("email"))

    if slack_user_id:
        slack_post_message(
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
    slack_post_message(SLACK_ACTIVITY_CHANNEL_ID, activity_message)


if __name__ == "__main__":
    issue_number = sys.argv[1]

    GITHUB_HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    SLACK_HEADERS = {
        "Authorization": f"Bearer {os.environ['SLACK_BOT_TOKEN']}",
        "Content-Type": "application/json",
    }

    issue = github_fetch_issue(issue_number)
    if not (areas := eligible_areas(issue)):
        sys.exit(0)

    try:
        contributors = tally_fetch_contributors(
            os.environ["TALLY_API_KEY"], os.environ["TALLY_FORM_ID"]
        )
        print(f"Found {len(contributors)} contributors in Tally")

        candidates = find_candidates(contributors, areas)
        if not candidates:
            slack_post_message(
                SLACK_ACTIVITY_CHANNEL_ID,
                f"\u26a0\ufe0f No contributors found for {', '.join(areas)} \u2014 "
                f"<{issue['html_url']}|#{issue_number}: {issue['title']}>",
            )
            print(f"No contributors interested in areas: {areas}")
            sys.exit(0)

        chosen = pick_least_busy(candidates)
        github_assign_issue(issue_number, chosen["github_username"])
        print(f"Assigned #{issue_number} to {chosen['github_username']}")

        project = github_fetch_project(GITHUB_PROJECT_NUMBER)
        item_id = github_add_issue_to_project(project["id"], issue["node_id"])
        github_set_project_item_status(project, item_id, "Assigned")

        slack_notify_on_assignment(chosen, issue)

    except Exception as exc:
        slack_post_message(
            SLACK_ACTIVITY_CHANNEL_ID,
            f"\u274c Failed to assign contributor for "
            f"<{issue['html_url']}|#{issue_number}: {issue['title']}>: {exc}",
        )
        raise
