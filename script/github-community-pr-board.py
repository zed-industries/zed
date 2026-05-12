#!/usr/bin/env python3
"""
Route community PRs to the correct review track on a GitHub Project board.

Reads the event payload dispatched by the GitHub Actions workflow and:
- On `labeled`: adds the PR to the board (idempotent) and sets the Track
  field to the most specific matching track.
- On `unlabeled`: re-resolves Track from remaining labels, or clears it
  if no area/platform labels remain (PR stays on the board for visibility).
- On `assigned`: if the assignee is a staff team member, sets Status to
  "In Progress (us)".
- On `review_requested`: if current Status is "In Progress (author)",
  flips it to "In Progress (us)" — the author is explicitly asking for
  re-review.
- On `issue_comment.created`: if the commenter is the PR author and
  current Status is "In Progress (author)", flips it to
  "In Progress (us)" — the author is likely signaling they're done.
- On `workflow_dispatch`: re-resolves Track for a manually specified PR.

Review-based status changes (approved → "In Progress (us)", changes
requested → "In Progress (author)") are handled by built-in board
automations, not this script.

Requires:
    requests (pip install requests)

Usage (called by the workflow, not directly):
    python github-community-pr-board.py
"""

import json
import os
import sys
import time
from pathlib import Path

import requests

RETRYABLE_STATUS_CODES = {502, 503, 504}
MAX_RETRIES = 3
RETRY_DELAY_SECONDS = 5

GITHUB_API_URL = "https://api.github.com"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
STAFF_TEAM_SLUG = "staff"

SKIP_LABELS = {"staff", "bot"}


STATUS_IN_PROGRESS_US = "In Progress (us)"
STATUS_IN_PROGRESS_AUTHOR = (
    "In Progress (author)"  # set by built-in board automation, read by this script
)

MAPPING_PATH = Path(__file__).parent / "community-pr-track-mapping.json"


def sync_track_from_labels(pr, project_number):
    """Sync the PR's Track field on the board with its current labels."""
    pr_labels = {label["name"] for label in pr.get("labels", [])}
    track_name = resolve_track(pr_labels, load_mapping())

    project = github_fetch_project(project_number)
    project_item = github_find_project_item(project["id"], pr["node_id"])

    if not track_name:
        if project_item:
            github_clear_field(project, project_item, "Track")
            print(f"No track matched, cleared Track on PR #{pr['number']}")
        else:
            print(
                f"No track matched for labels on PR #{pr['number']}, not on board, nothing to do"
            )
        return

    print(f"Resolved track: {track_name}")

    if not project_item:
        project_item = github_add_to_project(project["id"], pr["node_id"])
    github_set_project_field(project, project_item, "Track", track_name)


def set_progress_status_on_assignment(pr, assignee_login, project_number):
    """Set Status to 'In Progress (us)' when a staff member self-assigns."""
    if not github_is_staff_member(assignee_login):
        print(f"Assignee '{assignee_login}' is not a staff member, skipping")
        return

    project = github_fetch_project(project_number)
    item_id = github_find_project_item(project["id"], pr["node_id"])
    if not item_id:
        print(f"PR #{pr['number']} not on board, skipping assignment status update")
        return

    github_set_project_field(project, item_id, "Status", STATUS_IN_PROGRESS_US)


def return_to_reviewer(pr, project_number, reason):
    """Flip Status from 'In Progress (author)' to 'In Progress (us)'.

    Called when the author signals they're ready for re-review, either
    by re-requesting review or by commenting on the PR.
    """
    project = github_fetch_project(project_number)
    item_id = github_find_project_item(project["id"], pr["node_id"])
    if not item_id:
        print(f"PR #{pr['number']} not on board, skipping")
        return

    current_status = github_get_field_value(item_id, "Status")
    if current_status == STATUS_IN_PROGRESS_AUTHOR:
        print(
            f"{reason}, flipping status from '{current_status}' to '{STATUS_IN_PROGRESS_US}'"
        )
        github_set_project_field(project, item_id, "Status", STATUS_IN_PROGRESS_US)
    else:
        print(f"Current status is '{current_status}', not flipping ({reason})")


def load_mapping(path=MAPPING_PATH):
    """Load the Track-to-labels mapping from the JSON file."""
    with open(path) as f:
        data = json.load(f)
    return data["tracks"]


def resolve_track(pr_labels, tracks):
    """Return the name of the most specific track matching the PR's labels.

    Tracks are checked in order; the first match wins (most specific first).
    """
    for track in tracks:
        if pr_labels & set(track["labels"]):
            return track["name"]
    return None


def github_graphql(query, variables):
    """Execute a GitHub GraphQL query. Retries on transient server errors."""
    for attempt in range(MAX_RETRIES + 1):
        response = requests.post(
            f"{GITHUB_API_URL}/graphql",
            headers=GITHUB_HEADERS,
            json={"query": query, "variables": variables},
        )
        if response.status_code in RETRYABLE_STATUS_CODES and attempt < MAX_RETRIES:
            print(
                f"GitHub API returned {response.status_code}, retrying in {RETRY_DELAY_SECONDS}s (attempt {attempt + 1}/{MAX_RETRIES})..."
            )
            time.sleep(RETRY_DELAY_SECONDS)
            continue
        response.raise_for_status()
        result = response.json()
        if "errors" in result:
            raise RuntimeError(f"GraphQL error: {result['errors']}")
        return result["data"]


def github_rest_get(path):
    """GET from the GitHub REST API. Retries on transient server errors."""
    for attempt in range(MAX_RETRIES + 1):
        response = requests.get(f"{GITHUB_API_URL}/{path}", headers=GITHUB_HEADERS)
        if response.status_code in RETRYABLE_STATUS_CODES and attempt < MAX_RETRIES:
            print(
                f"GitHub API returned {response.status_code}, retrying in {RETRY_DELAY_SECONDS}s (attempt {attempt + 1}/{MAX_RETRIES})..."
            )
            time.sleep(RETRY_DELAY_SECONDS)
            continue
        response.raise_for_status()
        return response.json()


def github_is_staff_member(username):
    """Check if a user is a member of the staff team."""
    try:
        response = requests.get(
            f"{GITHUB_API_URL}/orgs/{REPO_OWNER}/teams/{STAFF_TEAM_SLUG}/members/{username}",
            headers=GITHUB_HEADERS,
        )
        if response.status_code == 204:
            return True
        if response.status_code == 404:
            return False
        print(
            f"Warning: unexpected status {response.status_code} checking staff membership for '{username}'"
        )
        return False
    except requests.RequestException as exc:
        print(f"Warning: failed to check staff membership for '{username}': {exc}")
        return False


def github_fetch_pr(pr_number):
    """Fetch a PR by number via the REST API."""
    return github_rest_get(f"repos/{REPO_OWNER}/{REPO_NAME}/pulls/{pr_number}")


def github_fetch_project(project_number):
    """Fetch a GitHub project board's metadata including fields and their options."""
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


def github_add_to_project(project_id, content_node_id):
    """Add a PR to the project board. Returns the new project item ID."""
    data = github_graphql(
        """
        mutation($projectId: ID!, $contentId: ID!) {
          addProjectV2ItemById(input: {projectId: $projectId, contentId: $contentId}) {
            item { id }
          }
        }
        """,
        {"projectId": project_id, "contentId": content_node_id},
    )
    item_id = data["addProjectV2ItemById"]["item"]["id"]
    print(f"Added PR to board (item: {item_id})")
    return item_id


def github_find_project_item(project_id, content_node_id):
    """Find a PR's item ID on the project board, or None if not present.

    Uses a read-only query so it won't add the PR as a side effect.
    """
    data = github_graphql(
        """
        query($contentId: ID!) {
          node(id: $contentId) {
            ... on PullRequest {
              projectItems(first: 50) {
                nodes {
                  id
                  project { id }
                }
              }
            }
          }
        }
        """,
        {"contentId": content_node_id},
    )
    for item in data["node"]["projectItems"]["nodes"]:
        if item["project"]["id"] == project_id:
            return item["id"]
    return None


def github_set_project_field(project, item_id, field_name, option_name):
    """Set a single-select field on a project item."""
    field_id = None
    option_id = None
    for field in project["fields"]["nodes"]:
        if field.get("name") == field_name:
            field_id = field["id"]
            for option in field.get("options", []):
                if option["name"] == option_name:
                    option_id = option["id"]
                    break
            break

    if not field_id:
        available = [f["name"] for f in project["fields"]["nodes"] if "name" in f]
        raise RuntimeError(
            f"Field '{field_name}' not found on project. Available: {available}"
        )
    if not option_id:
        available = [
            opt["name"]
            for f in project["fields"]["nodes"]
            if f.get("name") == field_name
            for opt in f.get("options", [])
        ]
        raise RuntimeError(
            f"Option '{option_name}' not found in field '{field_name}'. "
            f"Available: {available}"
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
            "fieldId": field_id,
            "optionId": option_id,
        },
    )
    print(f"Set '{field_name}' to '{option_name}'")


def github_clear_field(project, item_id, field_name):
    """Clear a single-select field on a project item."""
    field_id = None
    for field in project["fields"]["nodes"]:
        if field.get("name") == field_name:
            field_id = field["id"]
            break

    if not field_id:
        available = [f["name"] for f in project["fields"]["nodes"] if "name" in f]
        raise RuntimeError(
            f"Field '{field_name}' not found on project. Available: {available}"
        )

    github_graphql(
        """
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!) {
          clearProjectV2ItemFieldValue(input: {
            projectId: $projectId
            itemId: $itemId
            fieldId: $fieldId
          }) {
            projectV2Item { id }
          }
        }
        """,
        {
            "projectId": project["id"],
            "itemId": item_id,
            "fieldId": field_id,
        },
    )
    print(f"Cleared '{field_name}'")


def github_get_field_value(item_id, field_name):
    """Read the current value of a single-select field on a project item."""
    data = github_graphql(
        """
        query($itemId: ID!) {
          node(id: $itemId) {
            ... on ProjectV2Item {
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
        """,
        {"itemId": item_id},
    )
    for field_value in data["node"]["fieldValues"]["nodes"]:
        if field_value.get("field", {}).get("name") == field_name:
            return field_value.get("name")
    return None


if __name__ == "__main__":
    GITHUB_HEADERS = {
        "Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }

    project_number = int(os.environ["PROJECT_NUMBER"])
    manual_pr_number = os.environ.get("MANUAL_PR_NUMBER")

    if manual_pr_number:
        pr = github_fetch_pr(manual_pr_number)
        action = "labeled"
        event = {}
        print(f"Manual dispatch for PR #{manual_pr_number}")
    else:
        event_name = os.environ["GITHUB_EVENT_NAME"]
        with open(os.environ["GITHUB_EVENT_PATH"]) as f:
            event = json.load(f)

        if event_name in ("pull_request", "pull_request_target"):
            pr = event["pull_request"]
            action = event["action"]
        elif event_name == "issue_comment":
            issue = event["issue"]
            if "pull_request" not in issue:
                print("Comment is on an issue, not a PR, skipping")
                sys.exit(0)
            commenter = event["comment"]["user"]["login"]
            pr_author = issue["user"]["login"]
            if commenter != pr_author:
                print(
                    f"Commenter '{commenter}' is not PR author '{pr_author}', skipping"
                )
                sys.exit(0)
            pr = github_fetch_pr(issue["number"])
            action = "author_commented"
        else:
            print(f"Unexpected event: {event_name}")
            sys.exit(0)

    pr_labels = {label["name"] for label in pr.get("labels", [])}
    if pr_labels & SKIP_LABELS:
        print(f"Skipping PR #{pr['number']} (has {pr_labels & SKIP_LABELS} label)")
        sys.exit(0)

    if pr.get("draft"):
        print(f"Skipping draft PR #{pr['number']}")
        sys.exit(0)

    print(f"Processing PR #{pr['number']}: action={action}")

    if action in ("labeled", "unlabeled"):
        sync_track_from_labels(pr, project_number)
    elif action == "assigned":
        assignee_login = event.get("assignee", {}).get("login")
        if not assignee_login:
            print("No assignee login in event payload, skipping")
        else:
            set_progress_status_on_assignment(pr, assignee_login, project_number)
    elif action == "review_requested":
        return_to_reviewer(pr, project_number, "Author re-requested review")
    elif action == "author_commented":
        return_to_reviewer(pr, project_number, "Author commented on PR")
    else:
        print(f"Ignoring action: {action}")
