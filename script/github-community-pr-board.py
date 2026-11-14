#!/usr/bin/env python3
"""
Route community PRs to the correct review track on a GitHub Project board,
and surface signal fields (Size, Issue Linked, Contributor, Upvotes, Last
push, Merge conflicts, Opened with no self-review) that the board's views
can filter and sort on.

Reads the event payload dispatched by the GitHub Actions workflow and:
- On `labeled` or `reopened`: adds the PR to the board (idempotent), sets the
  Track field to the most specific matching track, and recomputes signals.
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
  belt-and-suspenders pass against changes that don't reach the
  event-driven workflow (notably intentionally-unsubscribed `synchronize`
  events that change Size, Last push and Merge conflicts, pushes to main
  that change Merge conflicts, reactions, comments and reviews that change
  Upvotes, and sidebar edits to linked issues that don't fire a PR webhook).

Review-based status changes (approved → "In Progress (us)", changes
requested → "In Progress (author)") are handled by built-in board
automations, not this script.

Signal fields are written only when their value changed, and best-effort:
if a field or option is missing from the project, the script logs and
skips it rather than failing, so new fields can be rolled out in the
project UI independently of deploys.

Requires:
    requests (pip install requests)

Usage (called by the workflow, not directly):
    python github-community-pr-board.py
"""

import json
import os
import sys
import time
from itertools import batched
from pathlib import Path

from github_helpers import (
    add_github_project_item,
    fetch_github_project,
    github_graphql,
    github_rest_api,
    is_staff_member,
    set_github_project_field,
)

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
REPO_FULL_NAME = f"{REPO_OWNER}/{REPO_NAME}"

SKIP_LABELS = {"staff", "bot"}


STATUS_IN_PROGRESS_US = "In Progress (us)"
STATUS_IN_PROGRESS_AUTHOR = (
    "In Progress (author)"  # set by built-in board automation, read by this script
)

MAPPING_PATH = Path(__file__).parent / "community-pr-track-mapping.json"

MERGE_CONFLICTS_FIELD = "Merge conflicts"
HAS_CONFLICTS_OPTION = "❌"
NO_SELF_REVIEW_LABEL = "no self-review"
NO_SELF_REVIEW_OPTION = "AI?"
POSITIVE_REACTIONS = {"THUMBS_UP", "HEART", "HOORAY", "ROCKET"}
BOTS_WITH_USER_ACCOUNTS = {"zed-industries-bot"}
# GitHub answers 502 to batches of 25.
BOARD_ITEMS_PER_QUERY = 10
MAX_NODE_IDS_PER_QUERY = 100
# GitHub forgets mergeability whenever main moves, and only computes it again
# once asked, so a recheck asks, waits, and reads again.
MERGEABILITY_RECHECK_DELAY_SECONDS = 15
MERGEABILITY_RECHECK_ROUNDS = 3

CONNECTION_NODE_SELECTIONS = {
    "reactions": "content user { login }",
    "comments": "author { __typename login }",
    "reviews": "author { __typename login }",
}
# %-formatting, since every GraphQL brace would need doubling in an f-string.
BOARD_ITEM_FRAGMENT = """
fragment BoardItem on ProjectV2Item {
  id
  isArchived
  fieldValues(first: 30) {
    nodes {
      ... on ProjectV2ItemFieldSingleSelectValue { name field { ... on ProjectV2FieldCommon { name } } }
      ... on ProjectV2ItemFieldNumberValue { number field { ... on ProjectV2FieldCommon { name } } }
      ... on ProjectV2ItemFieldDateValue { date field { ... on ProjectV2FieldCommon { name } } }
    }
  }
  content {
    ... on PullRequest {
      id
      number
      state
      additions
      deletions
      mergeable
      author { login }
      labels(first: 50) { nodes { name } }
      commits(last: 1) { nodes { commit { committedDate } } }
      reactions(first: 100) { nodes { %(reactions)s } pageInfo { hasNextPage endCursor } }
      comments(first: 100) { nodes { %(comments)s } pageInfo { hasNextPage endCursor } }
      reviews(first: 100) { nodes { %(reviews)s } pageInfo { hasNextPage endCursor } }
      closingIssuesReferences(first: 20) {
        nodes {
          __typename
          id
          repository { nameWithOwner }
          issueType { name }
          reactions(first: 100) { nodes { %(reactions)s } pageInfo { hasNextPage endCursor } }
          comments(first: 100) { nodes { %(comments)s } pageInfo { hasNextPage endCursor } }
        }
      }
    }
  }
}
""" % CONNECTION_NODE_SELECTIONS


def sync_track_from_labels(pr, project_number):
    """Sync the PR's Track field on the board with its current labels.

    Returns (project, item) if the PR ends up on (or stays on) the board, so
    callers can refresh its signals without fetching it again. Returns
    (None, None) otherwise.
    """
    pr_labels = {label["name"] for label in pr.get("labels", [])}
    track_name = resolve_track(pr_labels, load_mapping())

    project = fetch_github_project(project_number)
    item_id = github_find_project_item_id(project["id"], pr["node_id"])
    if not item_id:
        if not track_name:
            print(
                f"No track matched for labels on PR #{pr['number']}, not on board, nothing to do"
            )
            return None, None
        item_id = add_github_project_item(project["id"], pr["node_id"])

    [item] = github_fetch_board_items([item_id])
    current_track = current_field_values(item).get("Track")
    # Case-insensitive like the write, since the mapping is maintained by hand.
    if (current_track or "").casefold() != (track_name or "").casefold():
        set_github_project_field(project, item_id, "Track", track_name)
    return project, item


def set_progress_status_on_assignment(pr, assignee_login, project_number):
    """Set Status to 'In Progress (us)' when a staff member self-assigns."""
    if not is_staff_member(assignee_login):
        print(f"Assignee '{assignee_login}' is not a staff member, skipping")
        return

    project = fetch_github_project(project_number)
    item_id = github_find_project_item_id(project["id"], pr["node_id"])
    if not item_id:
        print(f"PR #{pr['number']} not on board, skipping assignment status update")
        return

    set_github_project_field(project, item_id, "Status", STATUS_IN_PROGRESS_US)


def return_to_reviewer(pr, project_number, reason):
    """Flip Status from 'In Progress (author)' to 'In Progress (us)'.

    Called when the author signals they're ready for re-review, either
    by re-requesting review or by commenting on the PR.
    """
    project = fetch_github_project(project_number)
    item_id = github_find_project_item_id(project["id"], pr["node_id"])
    if not item_id:
        print(f"PR #{pr['number']} not on board, skipping")
        return

    current_status = github_get_field_value(item_id, "Status")
    if current_status == STATUS_IN_PROGRESS_AUTHOR:
        print(
            f"{reason}, flipping status from '{current_status}' to '{STATUS_IN_PROGRESS_US}'"
        )
        set_github_project_field(project, item_id, "Status", STATUS_IN_PROGRESS_US)
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

    The auto-labeler is responsible for applying `guild`, `community champion`
    and `first contribution` based on the author's membership/history; anything
    else on the board is treated as a returning contributor (PRs from staff/bots
    don't reach this function because they're filtered upstream).

    Guild takes precedence over Champion because current-cohort guild members
    are frequently also community champions, and their guild status is the more
    relevant signal while the cohort is active.
    """
    if "guild" in pr_labels:
        return "Guild"
    if "community champion" in pr_labels:
        return "Champion"
    if "first contribution" in pr_labels:
        return "First PR"
    return "Returning"


def closing_issues_in_repository(pr):
    return [
        issue
        for issue in pr["closingIssuesReferences"]["nodes"]
        if issue["repository"]["nameWithOwner"] == REPO_FULL_NAME
    ]


def linked_issue_type(pr):
    """Return the most significant type among the repository's issues the PR closes."""
    closing_issues = closing_issues_in_repository(pr)
    if not closing_issues:
        return "No issue"

    type_names = {
        issue["issueType"]["name"]
        for issue in closing_issues
        if issue.get("issueType")
    }
    for type_name in ("Crash", "Bug", "Feature", "Docs"):
        if type_name in type_names:
            return type_name

    print(
        f"Warning: PR has {len(closing_issues)} linked issue(s) but none with "
        f"a recognized type (saw: {type_names or 'no type set'})"
    )
    return "No issue"


def github_connection_page(subject, connection_name, cursor):
    data = github_graphql(
        f"""
        query($id: ID!, $cursor: String!) {{
          node(id: $id) {{
            ... on {subject['__typename']} {{
              page: {connection_name}(first: 100, after: $cursor) {{
                nodes {{ {CONNECTION_NODE_SELECTIONS[connection_name]} }}
                pageInfo {{ hasNextPage endCursor }}
              }}
            }}
          }}
        }}
        """,
        {"id": subject["id"], "cursor": cursor},
    )
    return data["node"]["page"]


def connection_nodes(subject, connection_name):
    connection = subject[connection_name]
    while True:
        yield from connection["nodes"]
        if not connection["pageInfo"]["hasNextPage"]:
            return
        connection = github_connection_page(
            subject, connection_name, connection["pageInfo"]["endCursor"]
        )


def engaged_users(pr):
    for subject in [pr, *closing_issues_in_repository(pr)]:
        for reaction in connection_nodes(subject, "reactions"):
            if reaction["content"] in POSITIVE_REACTIONS:
                yield reaction["user"]
        for comment in connection_nodes(subject, "comments"):
            yield comment["author"]
    for review in connection_nodes(pr, "reviews"):
        yield review["author"]


def is_bot(user):
    login = user["login"].lower()
    return (
        user.get("__typename") == "Bot"
        or login.endswith("[bot]")
        or login in BOTS_WITH_USER_ACCOUNTS
    )


def count_interested_people(pr):
    author_login = (pr["author"] or {}).get("login")
    interested_logins = {
        user["login"]
        for user in engaged_users(pr)
        if user
        and user["login"] != author_login
        and not is_bot(user)
        and not is_staff_member(user["login"])
    }
    return len(interested_logins)


def merge_conflicts_value(mergeable):
    return HAS_CONFLICTS_OPTION if mergeable == "CONFLICTING" else None


def compute_signals(pr):
    pr_labels = {label["name"] for label in pr["labels"]["nodes"]}
    signals = {
        "Size": compute_size_bucket(pr["additions"] + pr["deletions"]),
        "Contributor": compute_contributor(pr_labels),
        "Issue Linked": linked_issue_type(pr),
        # Counts people, not reactions, despite the field's name.
        "Upvotes": count_interested_people(pr),
        # The committer date moves with rebases and merges from main.
        "Last push": pr["commits"]["nodes"][0]["commit"]["committedDate"][:10],
        "Opened with no self-review": (
            NO_SELF_REVIEW_OPTION if NO_SELF_REVIEW_LABEL in pr_labels else None
        ),
    }
    # Left out while GitHub is still computing mergeability, so the current value stays.
    if pr["mergeable"] != "UNKNOWN":
        signals[MERGE_CONFLICTS_FIELD] = merge_conflicts_value(pr["mergeable"])
    return signals


def current_field_values(item):
    return {
        node["field"]["name"]: node.get("name", node.get("number", node.get("date")))
        for node in item["fieldValues"]["nodes"]
        if "field" in node
    }


def write_changed_fields(project, item_id, values, current_values):
    for field_name, value in values.items():
        if current_values.get(field_name) != value:
            set_github_project_field(
                project, item_id, field_name, value, missing_ok=True
            )


def refresh_signals(project, item):
    pr = item["content"]
    print(f"--- Refreshing signals on PR #{pr['number']} ---")
    write_changed_fields(
        project, item["id"], compute_signals(pr), current_field_values(item)
    )


def refresh_signals_if_on_board(pr, project_number):
    project = fetch_github_project(project_number)
    item_id = github_find_project_item_id(project["id"], pr["node_id"])
    if not item_id:
        print(f"PR #{pr['number']} not on board, skipping signal refresh")
        return
    [item] = github_fetch_board_items([item_id])
    refresh_signals(project, item)


def recheck_mergeability(project, mergeability_rechecks):
    """Write Merge conflicts on items whose mergeability was still UNKNOWN on
    the first read. Returns the number of items that failed.
    """
    failed = 0
    pending_rechecks = mergeability_rechecks
    for _ in range(MERGEABILITY_RECHECK_ROUNDS):
        time.sleep(MERGEABILITY_RECHECK_DELAY_SECONDS)
        still_unknown = []
        for batch in batched(pending_rechecks, MAX_NODE_IDS_PER_QUERY):
            try:
                mergeable_by_pr_id = github_fetch_mergeable_by_pr_id(
                    [recheck["pr_id"] for recheck in batch]
                )
            except Exception as error:
                print(f"Failed to re-read mergeability of {len(batch)} PR(s): {error}")
                failed += len(batch)
                continue
            for recheck in batch:
                mergeable = mergeable_by_pr_id.get(recheck["pr_id"], "UNKNOWN")
                if mergeable == "UNKNOWN":
                    still_unknown.append(recheck)
                    continue
                try:
                    write_changed_fields(
                        project,
                        recheck["item_id"],
                        {MERGE_CONFLICTS_FIELD: merge_conflicts_value(mergeable)},
                        {MERGE_CONFLICTS_FIELD: recheck["merge_conflicts"]},
                    )
                except Exception as error:
                    print(f"Failed to write Merge conflicts on PR #{recheck['number']}: {error}")
                    failed += 1
        pending_rechecks = still_unknown
        if not pending_rechecks:
            break
    if pending_rechecks:
        print(f"Mergeability still unknown for {len(pending_rechecks)} PR(s), left as they were")
    return failed


def refresh_all_board_items(project_number):
    """Walk every open PR on the board and refresh its signals.

    Backstop for the daily refresh workflow. A failure on one PR doesn't
    stop the others, but fails the run at the end.
    """
    project = fetch_github_project(project_number)
    listed_items = []
    skipped = 0
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
        label_names = [label["name"] for label in content["labels"]["nodes"]]
        if set(label_names) & SKIP_LABELS:
            skipped += 1
            continue
        listed_items.append(item)

    processed = failed = 0
    mergeability_rechecks = []
    for batch in batched(listed_items, BOARD_ITEMS_PER_QUERY):
        try:
            items = github_fetch_board_items([item["id"] for item in batch])
        except Exception as error:
            # One broken item fails the whole query, so they're fetched one by one.
            print(f"Failed to fetch {len(batch)} items at once, retrying one by one: {error}")
            items = [None] * len(batch)
        for listed_item, item in zip(batch, items):
            try:
                if item is None:
                    [item] = github_fetch_board_items([listed_item["id"]])
                pr = item["content"]
                # Closed since the board was listed, and archived along with it.
                if item["isArchived"] or pr["state"] != "OPEN":
                    skipped += 1
                    continue
                refresh_signals(project, item)
                processed += 1
                if pr["mergeable"] == "UNKNOWN":
                    mergeability_rechecks.append({
                        "item_id": item["id"],
                        "pr_id": pr["id"],
                        "number": pr["number"],
                        "merge_conflicts": current_field_values(item).get(
                            MERGE_CONFLICTS_FIELD
                        ),
                    })
            except Exception as error:
                print(f"Failed to refresh PR #{listed_item['content']['number']}: {error}")
                failed += 1
    if mergeability_rechecks:
        failed += recheck_mergeability(project, mergeability_rechecks)
    print(
        f"Refresh complete: {processed} processed, {skipped} skipped, {failed} failed"
    )
    if failed:
        raise RuntimeError(f"Failed to refresh {failed} PR(s), see above")


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


def github_fetch_pr(pr_number):
    """Fetch a PR by number via the REST API."""
    return github_rest_api("GET", f"repos/{REPO_OWNER}/{REPO_NAME}/pulls/{pr_number}")


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
                          number
                          state
                          isDraft
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


def github_fetch_board_items(item_ids):
    data = github_graphql(
        BOARD_ITEM_FRAGMENT
        + """
        query($ids: [ID!]!) {
          nodes(ids: $ids) { ...BoardItem }
        }
        """,
        {"ids": item_ids},
    )
    return data["nodes"]


def github_fetch_mergeable_by_pr_id(pr_ids):
    data = github_graphql(
        """
        query($ids: [ID!]!) {
          nodes(ids: $ids) { ... on PullRequest { id mergeable } }
        }
        """,
        {"ids": pr_ids},
        partial_errors_ok=True,
    )
    # PRs GitHub can no longer return come back as null.
    return {node["id"]: node["mergeable"] for node in data["nodes"] if node}


def github_find_project_item_id(project_id, content_node_id):
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

    if pr.get("state") != "open":
        print(f"Skipping PR #{pr['number']} (state: {pr.get('state')})")
        sys.exit(0)

    print(f"Processing PR #{pr['number']}: action={action}")

    if action in ("labeled", "unlabeled", "reopened"):
        project, item = sync_track_from_labels(pr, project_number)
        if item:
            refresh_signals(project, item)
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
