#!/usr/bin/env python3
"""
Route community PRs to the correct review track on a GitHub Project board,
and surface signal fields (Size, Issue Linked, Contributor, Upvotes) that the
board's views can filter and sort on.

Reads the event payload dispatched by the GitHub Actions workflow and:
- On `labeled`: adds the PR to the board (idempotent), sets the Track
  field to the most specific matching track, and recomputes signal fields.
- On `unlabeled`: re-resolves Track from remaining labels, or clears it
  if no area/platform labels remain (PR stays on the board for visibility).
  Signals are recomputed if the PR is on the board.
- On `edited`: recomputes signals if the PR is on the board (a body edit
  can change which issues the PR closes).
- On `assigned`: if the assignee is a staff team member, sets Status to
  "In Progress (us)".
- On `review_requested`: if current Status is "In Progress (author)",
  flips it to "In Progress (us)" — the author is explicitly asking for
  re-review.
- On `issue_comment.created`: if the commenter is the PR author and
  current Status is "In Progress (author)", flips it to
  "In Progress (us)" — the author is likely signaling they're done.
- On `workflow_dispatch` with a PR number: re-resolves Track and
  recomputes signals for a manually specified PR.
- When run with REFRESH_ALL=1: walks every open PR on the board and
  recomputes its signals. Used by the scheduled refresh workflow as a
  belt-and-suspenders pass against missed webhook events (notably
  intentionally-unsubscribed `synchronize` events that change Size, and
  sidebar edits to linked issues that don't fire a PR webhook).

Review-based status changes (approved → "In Progress (us)", changes
requested → "In Progress (author)") are handled by built-in board
automations, not this script.

Signal fields are written best-effort: if a field or option is missing
from the project, the script logs and skips it rather than failing, so
new fields can be rolled out in the project UI independently of deploys.

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
    """Sync the PR's Track field on the board with its current labels.

    Returns (project, project_item) if the PR ends up on (or stays on)
    the board, so callers can do further per-item work without a second
    project/item lookup. Returns (None, None) otherwise.
    """
    pr_labels = {label["name"] for label in pr.get("labels", [])}
    track_name = resolve_track(pr_labels, load_mapping())

    project = github_fetch_project(project_number)
    project_item = github_find_project_item(project["id"], pr["node_id"])

    if not track_name:
        if project_item:
            github_clear_field(project, project_item, "Track")
            print(f"No track matched, cleared Track on PR #{pr['number']}")
            return project, project_item
        print(
            f"No track matched for labels on PR #{pr['number']}, not on board, nothing to do"
        )
        return None, None

    print(f"Resolved track: {track_name}")

    if not project_item:
        project_item = github_add_to_project(project["id"], pr["node_id"])
    github_set_project_field(project, project_item, "Track", track_name)
    return project, project_item


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


def compute_size_bucket(total_changes):
    """Return the size bucket label for a PR's total (additions + deletions).

    Size is computed from raw additions + deletions; we don't try to strip
    generated files.
    """
    buckets = [(30, "XS"), (150, "S"), (400, "M"), (1000, "L")]
    for threshold, name in buckets:
        if total_changes < threshold:
            return name
    return "XL"


def compute_contributor(pr_labels):
    """Return the Contributor value derived from the PR's labels.

    The auto-labeler is responsible for applying `community champion` and
    `first contribution` based on the author's history; anything else on
    the board is treated as a returning contributor (PRs from staff/bots
    don't reach this function because they're filtered upstream).
    """
    if "community champion" in pr_labels:
        return "Champion"
    if "first contribution" in pr_labels:
        return "First PR"
    return "Returning"


def recompute_signals(pr, project, project_item):
    """Recompute Size, Contributor, Issue Linked, and Upvotes on a board item.

    Best-effort: missing fields/options are logged and skipped so the
    project's field set can be rolled out independently.
    """
    pr_labels = {label["name"] for label in pr.get("labels", [])}
    total_changes = pr.get("additions", 0) + pr.get("deletions", 0)
    author_login = (pr.get("user") or {}).get("login")

    set_field_optional(project, project_item, "Size", compute_size_bucket(total_changes))
    set_field_optional(project, project_item, "Contributor", compute_contributor(pr_labels))
    set_field_optional(project, project_item, "Issue Linked", github_pr_issue_type(pr["node_id"]))
    set_number_field_optional(project, project_item, "Upvotes", github_pr_upvotes(pr["node_id"], author_login))


def refresh_signals_if_on_board(pr, project_number):
    """Recompute signals for a PR only if it's already on the board."""
    project = github_fetch_project(project_number)
    project_item = github_find_project_item(project["id"], pr["node_id"])
    if not project_item:
        print(f"PR #{pr['number']} not on board, skipping signal refresh")
        return
    recompute_signals(pr, project, project_item)


def refresh_all_board_items(project_number):
    """Walk every open PR on the board and recompute its signals.

    Backstop for the daily refresh workflow. Individual PR failures are
    logged and don't abort the rest of the run.
    """
    project = github_fetch_project(project_number)
    processed = skipped = errors = 0
    for item in github_list_project_items(project["id"]):
        if item.get("isArchived"):
            skipped += 1
            continue
        content = item.get("content") or {}
        if content.get("__typename") != "PullRequest":
            skipped += 1
            continue
        if content.get("state") != "OPEN" or content.get("isDraft"):
            skipped += 1
            continue
        label_names = [n["name"] for n in content["labels"]["nodes"]]
        if set(label_names) & SKIP_LABELS:
            skipped += 1
            continue
        pr = {
            "number": content["number"],
            "node_id": content["id"],
            "labels": [{"name": n} for n in label_names],
            "additions": content["additions"],
            "deletions": content["deletions"],
            "user": {"login": (content.get("author") or {}).get("login")},
        }
        print(f"--- Refreshing PR #{pr['number']} ---")
        try:
            recompute_signals(pr, project, item["id"])
            processed += 1
        except Exception as exc:
            print(f"Failed to refresh PR #{pr['number']}: {exc}")
            errors += 1
    print(
        f"Refresh complete: {processed} processed, {skipped} skipped, {errors} errors"
    )


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
    raise RuntimeError("github_graphql: retry loop exited without return")


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
    raise RuntimeError("github_rest_get: retry loop exited without return")


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
                  ... on ProjectV2Field {
                    id
                    name
                    dataType
                  }
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


def github_list_project_items(project_id):
    """Yield every item on a project board, paginating as needed.

    Each item includes `isArchived` so callers can cheaply filter archived
    items out without an extra round-trip per item.
    """
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
                      content {
                        __typename
                        ... on PullRequest {
                          id
                          number
                          state
                          isDraft
                          additions
                          deletions
                          author { login }
                          labels(first: 50) { nodes { name } }
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
        for item in page["nodes"]:
            yield item
        if not page["pageInfo"]["hasNextPage"]:
            return
        cursor = page["pageInfo"]["endCursor"]


def github_pr_upvotes(pr_node_id, author_login):
    """Return the count of unique positive reactors on the PR.

    Counts users (not bots) who left at least one of THUMBS_UP, HEART,
    HOORAY, or ROCKET on the PR itself. Each user is counted once even if
    they reacted with multiple positive emojis. The PR author is excluded
    so self-reactions don't inflate the count.

    Caps at 100 reactors per emoji — above that the exact count stops
    mattering for ranking purposes.
    """
    data = github_graphql(
        """
        query($prId: ID!) {
          node(id: $prId) {
            ... on PullRequest {
              reactionGroups {
                content
                reactors(first: 100) {
                  nodes {
                    __typename
                    ... on User { login }
                  }
                }
              }
            }
          }
        }
        """,
        {"prId": pr_node_id},
    )
    positive_contents = {"THUMBS_UP", "HEART", "HOORAY", "ROCKET"}
    reactors = set()
    for group in data["node"]["reactionGroups"] or []:
        if group["content"] not in positive_contents:
            continue
        for node in group["reactors"]["nodes"]:
            if node.get("__typename") != "User":
                continue
            login = node.get("login")
            if login and login != author_login:
                reactors.add(login)
    return len(reactors)


def github_pr_issue_type(pr_node_id):
    """Return the Issue Linked field value for a PR.

    Reads `closingIssuesReferences` (authoritative source for what GitHub
    will close on merge, covers both `Closes #N` keywords and Development
    sidebar links) and maps the linked issues' types to one of: 'Crash',
    'Bug', 'Feature', 'Docs', or 'No issue'.

    When a PR closes multiple issues with different types, returns the
    most urgent one by the priority Crash > Bug > Feature > Docs.
    """
    data = github_graphql(
        """
        query($prId: ID!) {
          node(id: $prId) {
            ... on PullRequest {
              closingIssuesReferences(first: 20) {
                totalCount
                nodes { issueType { name } }
              }
            }
          }
        }
        """,
        {"prId": pr_node_id},
    )
    refs = data["node"]["closingIssuesReferences"]
    if refs["totalCount"] == 0:
        return "No issue"
    type_names = {
        n["issueType"]["name"] for n in refs["nodes"] if n.get("issueType")
    }
    for priority in ("Crash", "Bug", "Feature", "Docs"):
        if priority in type_names:
            return priority
    # Org-wide guardrails should ensure every issue has a recognized type,
    # so reaching this branch means something slipped through. Log it and
    # fall back so the PR doesn't carry stale data.
    print(
        f"Warning: PR has {refs['totalCount']} linked issue(s) but none with "
        f"a recognized type (saw: {type_names or 'no type set'})"
    )
    return "No issue"


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


def set_field_optional(project, item_id, field_name, option_name):
    """Set a single-select field, logging and skipping if the field or
    option doesn't exist on the project yet.

    Used for signal fields so they can be added in the project UI
    independently of script deploys without crashing the workflow.
    """
    field = None
    for f in project["fields"]["nodes"]:
        if f.get("name") == field_name:
            field = f
            break
    if not field:
        print(f"Field '{field_name}' not on project, skipping")
        return
    option_id = None
    for opt in field.get("options", []):
        if opt["name"] == option_name:
            option_id = opt["id"]
            break
    if not option_id:
        available = [opt["name"] for opt in field.get("options", [])]
        print(
            f"Option '{option_name}' not in field '{field_name}' "
            f"(available: {available}), skipping"
        )
        return
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
            "fieldId": field["id"],
            "optionId": option_id,
        },
    )
    print(f"Set '{field_name}' to '{option_name}'")


def set_number_field_optional(project, item_id, field_name, value):
    """Set a number field, logging and skipping if the field doesn't exist
    on the project yet. Mirror of `set_field_optional` for non-select fields.
    """
    field = None
    for f in project["fields"]["nodes"]:
        if f.get("name") == field_name:
            field = f
            break
    if not field:
        print(f"Field '{field_name}' not on project, skipping")
        return
    github_graphql(
        """
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $value: Float!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId
            itemId: $itemId
            fieldId: $fieldId
            value: { number: $value }
          }) {
            projectV2Item { id }
          }
        }
        """,
        {
            "projectId": project["id"],
            "itemId": item_id,
            "fieldId": field["id"],
            "value": value,
        },
    )
    print(f"Set '{field_name}' to {value}")


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

    if os.environ.get("REFRESH_ALL") == "1":
        refresh_all_board_items(project_number)
        sys.exit(0)

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
        project, project_item = sync_track_from_labels(pr, project_number)
        if project_item:
            recompute_signals(pr, project, project_item)
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
    elif action == "edited":
        refresh_signals_if_on_board(pr, project_number)
    else:
        print(f"Ignoring action: {action}")
