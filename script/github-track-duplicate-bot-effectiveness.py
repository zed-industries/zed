#!/usr/bin/env python3
"""
Track the effectiveness of the duplicate-detection bot by classifying issues
into outcome categories on a GitHub Projects v2 board.

Subcommands:
    classify-closed <issue_number> <closer_login> <state_reason>
        Classify a closed issue and add it to the project board.

    classify-open
        Classify open, triaged, bot-commented issues and add them to
        the project board as Noise.

Requires:
    requests (pip install requests)

Environment variables:
    GITHUB_TOKEN     - GitHub App token
    PROJECT_NUMBER   - GitHub Projects v2 board number (default: 76, override for local testing)
"""

import argparse
import functools
import os
import re
import sys

import requests

GITHUB_API = "https://api.github.com"
GRAPHQL_URL = "https://api.github.com/graphql"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
STAFF_TEAM_SLUG = "staff"
BOT_LOGIN = "zed-community-bot[bot]"
BOT_APP_SLUG = "zed-community-bot"
BOT_COMMENT_PREFIX = "This issue appears to be a duplicate of"
BOT_START_DATE = "2026-02-18"
NEEDS_TRIAGE_LABEL = "state:needs triage"
DEFAULT_PROJECT_NUMBER = 76
VALID_CLOSED_AS_VALUES = {"duplicate", "not_planned", "completed"}


def github_api_get(path, params=None):
    url = f"{GITHUB_API}/{path.lstrip('/')}"
    response = requests.get(url, headers=GITHUB_HEADERS, params=params)
    response.raise_for_status()
    return response.json()


def github_search_issues(query):
    """Search issues, returning most recently created first."""
    # not handling pagination on purpose: the oldest issues are on the board already
    params = {"q": query, "sort": "created", "order": "desc", "per_page": 100}
    return github_api_get("/search/issues", params).get("items", [])


def is_staff_member(username):
    """Check if user is an active member of the staff team."""
    try:
        data = github_api_get(
            f"/orgs/{REPO_OWNER}/teams/{STAFF_TEAM_SLUG}/memberships/{username}"
        )
        return data.get("state") == "active"
    except requests.HTTPError as error:
        if error.response.status_code == 404:
            return False
        raise


def fetch_issue(issue_number):
    data = github_api_get(f"/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}")
    return {
        "number": issue_number,
        "node_id": data["node_id"],
        "author": (data.get("user") or {}).get("login", ""),
        "type_name": (data.get("type") or {}).get("name"),
    }


def get_bot_duplicate_comment(issue_number):
    """Get the bot's duplicate-detection comment body from an issue.

    Returns the comment body if found, else None.
    """
    comments_path = f"/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/comments"
    page = 1
    while comments := github_api_get(comments_path, {"per_page": 100, "page": page}):
        for comment in comments:
            author = (comment.get("user") or {}).get("login", "")
            body = comment.get("body", "")
            if author == BOT_LOGIN and body.startswith(BOT_COMMENT_PREFIX):
                return body
        page += 1
    return None


def parse_suggested_issues(comment_body):
    """Extract issue numbers from the bot's comment (lines like '- #12345')."""
    return [int(match) for match in re.findall(r"^- #(\d+)", comment_body, re.MULTILINE)]


def github_api_graphql(query, variables=None):
    """Execute a GitHub GraphQL query. Raises on errors."""
    response = requests.post(
        GRAPHQL_URL,
        headers=GITHUB_HEADERS,
        json={"query": query, "variables": variables or {}},
    )
    response.raise_for_status()
    data = response.json()
    if "errors" in data:
        raise RuntimeError(f"GraphQL errors: {data['errors']}")
    return data["data"]


def get_closed_as_duplicate_of(issue_number):
    """Get the issue number this issue was closed as a duplicate of.

    Uses the timeline to find the most recent MarkedAsDuplicateEvent.
    Returns the original issue number, or None.

    Note: not all "closed as duplicate" issues have a MarkedAsDuplicateEvent.
    If the closer used the "Close as duplicate" button without separately
    marking the duplicate relationship, no event is created and this returns
    None. The caller handles this by flagging the item for manual review.
    """
    data = github_api_graphql(
        """
        query($owner: String!, $repo: String!, $number: Int!) {
          repository(owner: $owner, name: $repo) {
            issue(number: $number) {
              timelineItems(last: 10, itemTypes: [MARKED_AS_DUPLICATE_EVENT]) {
                nodes {
                  ... on MarkedAsDuplicateEvent {
                    canonical { ... on Issue { number } }
                  }
                }
              }
            }
          }
        }
        """,
        {"owner": REPO_OWNER, "repo": REPO_NAME, "number": issue_number},
    )
    nodes = data["repository"]["issue"]["timelineItems"]["nodes"]
    for node in reversed(nodes):
        if original := (node.get("canonical") or {}).get("number"):
            return original
    return None


@functools.lru_cache
def get_project_config():
    """Fetch the project board's ID, field IDs, and option IDs."""
    data = github_api_graphql(
        """
        query($org: String!, $number: Int!) {
          organization(login: $org) {
            projectV2(number: $number) {
              id
              fields(first: 30) {
                nodes {
                  ... on ProjectV2SingleSelectField { id name options { id name } }
                  ... on ProjectV2Field { id name }
                }
              }
            }
          }
        }
        """,
        {"org": REPO_OWNER, "number": PROJECT_NUMBER},
    )
    project = data["organization"]["projectV2"]

    config = {"project_id": project["id"], "fields": {}}
    for field_node in project["fields"]["nodes"]:
        name = field_node.get("name")
        if not name:
            continue
        field_info = {"id": field_node["id"]}
        if "options" in field_node:
            field_info["options"] = {
                option["name"]: option["id"] for option in field_node["options"]
            }
        config["fields"][name] = field_info

    print(f"  Project config loaded: {len(config['fields'])} fields")
    return config


def find_project_item(issue_node_id):
    """Check if an issue is already on our project board.

    Returns the project item ID if found, or None.
    """
    data = github_api_graphql(
        "query($id: ID!) { node(id: $id) { ... on Issue { projectItems(first: 20) { nodes { id project { number } } } } } }",
        {"id": issue_node_id},
    )
    for item in data["node"]["projectItems"]["nodes"]:
        if item["project"]["number"] == PROJECT_NUMBER:
            return item["id"]
    return None


def add_project_item(issue_node_id):
    """Add an issue to the project board. Returns the new item ID."""
    config = get_project_config()
    data = github_api_graphql(
        """
        mutation($projectId: ID!, $contentId: ID!) {
          addProjectV2ItemById(input: {projectId: $projectId, contentId: $contentId}) {
            item { id }
          }
        }
        """,
        {"projectId": config["project_id"], "contentId": issue_node_id},
    )
    return data["addProjectV2ItemById"]["item"]["id"]


def set_field_value(item_id, field_name, value):
    """Set a single field value on a project board item."""
    config = get_project_config()
    field = config["fields"].get(field_name)
    if not field:
        print(f"  Warning: field '{field_name}' not found on project board")
        return

    if "options" in field:
        # single-select field
        option_id = field["options"].get(value)
        if not option_id:
            print(f"  Warning: option '{value}' not found for field '{field_name}'")
            return
        field_value = {"singleSelectOptionId": option_id}
    else:
        # text field
        field_value = {"text": str(value)}

    github_api_graphql(
        """
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $value: ProjectV2FieldValue!) {
          updateProjectV2ItemFieldValue(input: {
            projectId: $projectId
            itemId: $itemId
            fieldId: $fieldId
            value: $value
          }) {
            projectV2Item { id }
          }
        }
        """,
        {
            "projectId": config["project_id"],
            "itemId": item_id,
            "fieldId": field["id"],
            "value": field_value,
        },
    )


def add_or_update_project_item(issue_node_id, outcome, closed_as=None, status="Auto-classified", notes=None):
    """Add an issue to the project board (or update it if already there), setting field values."""
    item_id = find_project_item(issue_node_id)
    if item_id:
        print(f"  Issue already on board, updating (item {item_id})")
    else:
        item_id = add_project_item(issue_node_id)
        print(f"  Added to project board (item {item_id})")

    set_field_value(item_id, "Outcome", outcome)
    set_field_value(item_id, "Status", status)

    if closed_as and closed_as in VALID_CLOSED_AS_VALUES:
        set_field_value(item_id, "Closed as", closed_as)

    if notes:
        set_field_value(item_id, "Notes", notes)

    return item_id


def classify_closed(issue_number, closer_login, state_reason):
    """Classify a closed issue and add/update it on the project board."""
    state_reason = state_reason or "unknown"
    print(f"Classifying closed issue #{issue_number}")
    print(f"  Closer: {closer_login}, state_reason: {state_reason}")

    issue = fetch_issue(issue_number)
    author = issue["author"]
    print(f"  Author: {author}, type: {issue['type_name']}")

    if is_staff_member(author):
        print(f"  Skipping: author '{author}' is a staff member")
        return

    bot_comment = get_bot_duplicate_comment(issue_number)
    bot_commented = bot_comment is not None
    print(f"  Bot commented: {bot_commented}")

    closer_is_author = closer_login == author

    if bot_commented and closer_is_author:
        classify_as_success(issue, state_reason)
    elif bot_commented and not closer_is_author:
        # Only authors, staff, and triagers can close issues, so
        # a non-author closer is always someone with elevated permissions.
        classify_non_author_closed(issue, bot_comment, state_reason)
    elif not bot_commented and state_reason == "duplicate":
        classify_as_missed_opportunity(issue)
    else:
        print("  Skipping: no bot comment and not closed as duplicate")


def classify_as_success(issue, state_reason):
    """Author closed their own issue after the bot commented."""
    if state_reason == "duplicate":
        status = "Auto-classified"
        notes = None
    else:
        # could be closed for an unrelated reason; flag for review
        status = "Needs review"
        notes = f"Author closed as {state_reason}"

    if status == "Auto-classified":
        print(f"  -> Success (closed as {state_reason})")
    else:
        print(f"  -> Possible Success, needs review ({notes})")
    add_or_update_project_item(
        issue["node_id"],
        outcome="Success",
        closed_as=state_reason,
        status=status,
        notes=notes,
    )


def classify_non_author_closed(issue, bot_comment, state_reason):
    """Non-author (staff or triager) closed an issue the bot had commented on."""
    if state_reason == "duplicate":
        classify_as_assist(issue, bot_comment)
    else:
        notes = f"Closed by staff/triager as {state_reason}, not duplicate"
        print(f"  -> Possible Noise, needs review ({notes})")
        add_or_update_project_item(
            issue["node_id"],
            outcome="Noise",
            closed_as=state_reason,
            status="Needs review",
            notes=notes,
        )


def classify_as_assist(issue, bot_comment):
    """Staff member closed as duplicate after the bot commented. Check if the dup matches."""
    suggested = parse_suggested_issues(bot_comment)
    original = None
    try:
        original = get_closed_as_duplicate_of(issue["number"])
    except (requests.RequestException, RuntimeError) as error:
        print(f"  Warning: failed to get the original-for the duplicate issue: {error}")

    if original and suggested:
        if original in suggested:
            status = "Auto-classified"
            notes = None
            print(f"  -> Assist (original #{original} matches bot suggestion)")
        else:
            status = "Needs review"
            suggested_str = ", ".join(f"#{number}" for number in suggested)
            notes = f"Bot suggested {suggested_str}; closed as dup of #{original}"
            print(f"  -> Possible Assist, needs review ({notes})")
    else:
        # couldn't determine original or no suggestions parsed
        status = "Needs review"
        if not original:
            notes = "Could not determine original issue from timeline"
        else:
            notes = f"Closed as dup of #{original}; could not parse bot suggestions"
        print(f"  -> Possible Assist, needs review ({notes})")

    add_or_update_project_item(
        issue["node_id"], outcome="Assist", closed_as="duplicate", status=status, notes=notes)


def classify_as_missed_opportunity(issue):
    """Issue closed as duplicate but the bot never commented."""
    print("  -> Missed opportunity")
    add_or_update_project_item(
        issue["node_id"], outcome="Missed opportunity", closed_as="duplicate", status="Auto-classified")


def classify_open():
    """Classify open, triaged, bot-commented issues as Noise."""
    print("Classifying open issues")

    query = (
        f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open "
        f"commenter:app/{BOT_APP_SLUG} "
        f'-label:"{NEEDS_TRIAGE_LABEL}" '
        f"created:>={BOT_START_DATE}"
    )
    print(f"  Search query: {query}")

    results = github_search_issues(query)
    print(f"  Found {len(results)} candidate issues")

    added, skipped, errors = 0, 0, 0
    for item in results:
        number = item["number"]
        try:
            type_name = (item.get("type") or {}).get("name")
            author = (item.get("user") or {}).get("login", "")
            node_id = item["node_id"]

            skip_reason = (
                f"type is {type_name}" if type_name not in ("Bug", "Crash")
                else f"author {author} is staff" if is_staff_member(author)
                else "already on the board" if find_project_item(node_id)
                else "no bot duplicate comment found" if not get_bot_duplicate_comment(number)
                else None
            )
            if skip_reason:
                print(f"  #{number}: skipping, {skip_reason}")
                skipped += 1
                continue

            print(f"  #{number}: adding as Noise")
            add_or_update_project_item(node_id, outcome="Noise", status="Auto-classified")
            added += 1
        except Exception as error:  # broad catch: one issue failing shouldn't stop the sweep
            print(f"  #{number}: error processing issue, skipping: {error}")
            errors += 1

    print(f"  Done: added {added}, skipped {skipped}, errors {errors}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Track duplicate bot effectiveness on a GitHub project board.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    classify_parser = subparsers.add_parser(
        "classify-closed",
        help="Classify a closed issue and add it to the project board.",
    )
    classify_parser.add_argument("issue_number", type=int)
    classify_parser.add_argument("closer_login")
    classify_parser.add_argument("state_reason")

    subparsers.add_parser(
        "classify-open",
        help="Classify open, triaged, bot-commented issues as Noise.",
    )

    args = parser.parse_args()

    GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN", "")
    if not GITHUB_TOKEN:
        print("Error: GITHUB_TOKEN environment variable is required")
        sys.exit(1)

    raw_project_number = os.environ.get("PROJECT_NUMBER", "")
    if raw_project_number:
        try:
            PROJECT_NUMBER = int(raw_project_number)
        except ValueError:
            print(f"Error: PROJECT_NUMBER must be an integer, got '{raw_project_number}'")
            sys.exit(1)
    else:
        PROJECT_NUMBER = DEFAULT_PROJECT_NUMBER

    GITHUB_HEADERS = {
        "Authorization": f"token {GITHUB_TOKEN}",
        "Accept": "application/vnd.github+json",
    }

    if args.command == "classify-closed":
        classify_closed(args.issue_number, args.closer_login, args.state_reason)
    elif args.command == "classify-open":
        classify_open()
