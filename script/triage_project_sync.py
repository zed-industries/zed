#!/usr/bin/env python3
"""
triage_project_sync.py
======================

Sync triage state from `zed-industries/zed` issues into the
"Zed weekly triage" project (#84).

Auto-derives `Status`, `Stale since`, `Aged?`, `Intake week` from issue labels
+ comment activity + assignees. Mutates the project to
reflect the derived state.

The labels and the issue thread are the source of truth. The project is a
*derived view* — manual edits to the synced fields will be overwritten on the
next sync.

Modes
-----
    --issue N        Sync a single issue. Used by GH Actions on issue events.
    --all            Sync every item currently in the project. Used by daily
                     cron as a safety net.
    --dry-run        Compute derivations and log them, but don't mutate the
                     project. Safe for local testing / first deploy.

Auth
----
Reads `GITHUB_TOKEN` from env. For production, this is an installation token
from the `ZED_COMMUNITY_BOT_APP_ID` GitHub App, scoped to
`owner: zed-industries`, with `Organization Projects: Read and write`.

For local `--dry-run` testing, a personal token with `repo, read:org,
read:project` is sufficient.

Idempotency / safety
--------------------
- Every run re-derives all fields from current issue state. Running twice
  produces the same result as once.
- Failures on a single issue (in `--all` mode) are logged and the run
  continues. One bad item doesn't poison the batch.
- `--dry-run` makes no GraphQL mutations and no REST writes.

Dependencies
------------
    pip install requests
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone

import requests

# ---------------------------------------------------------------------------
# Constants

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
REPO = f"{REPO_OWNER}/{REPO_NAME}"

PROJECT_NUMBER = 84
PROJECT_OWNER = REPO_OWNER

STAFF_TEAM_SLUG = "staff"

# Status names. MUST match the option names configured in project #84.
# (Casing matters — GH Projects single-select option matching is case-sensitive.)
STATUS_NEEDS_LABELS = "Needs labels"
STATUS_NEEDS_REPRO_ATTEMPT = "Needs repro attempt"
STATUS_NEEDS_ASK = "Needs ask"
STATUS_USER_REPLIED = "User replied (review)"
STATUS_AWAITING_USER = "Awaiting user"
STATUS_RESPONDED_NO_REPRO = "Responded, no repro"
STATUS_AWAITING_EXTERNAL_REPRO = "Awaiting external repro"  # not auto-set; placeholder
STATUS_REPRODUCIBLE = "Reproducible"
STATUS_HANDOFF = "Handoff"
STATUS_HANDOFF_INCOMPLETE = "Handoff (incomplete)"
STATUS_CLAIMED_COMMUNITY = "Claimed by community"
STATUS_CLOSED = "Closed"
STATUS_UNKNOWN = "Unknown"

# Aging thresholds (days) per spec.
SUBSTANTIVE_COMMENT_MIN_LEN = 50
AGE_THRESHOLDS_DAYS = {
    STATUS_NEEDS_LABELS: 7,
    STATUS_NEEDS_REPRO_ATTEMPT: 7,
    STATUS_AWAITING_USER: 14,
    STATUS_USER_REPLIED: 3,
    # Needs ask is handled explicitly in derive_aged (always flagged), so
    # it doesn't need a threshold here.
}

TERMINAL_OR_RESTING_STATUSES = {
    STATUS_REPRODUCIBLE,
    STATUS_HANDOFF,
    STATUS_CLOSED,
    STATUS_RESPONDED_NO_REPRO,
    STATUS_CLAIMED_COMMUNITY,
}

# Issue types that aren't triage work items — administrative collections,
# dashboards, and trackers. The sync detects these and skips field updates;
# they remain in the project (auto-add put them there) but with empty fields,
# invisible in any status-filtered view. Manually remove them in the UI if
# they're cluttering the all-items list.
SKIP_ISSUE_TYPES = {"Meta", "Tracking"}

REST_API = "https://api.github.com"
GRAPHQL_API = "https://api.github.com/graphql"

NOW = datetime.now(timezone.utc)


# ---------------------------------------------------------------------------
# Logging


def log(msg: str, level: str = "INFO") -> None:
    ts = datetime.now(timezone.utc).strftime("%H:%M:%S")
    print(f"[{ts}] [{level}] {msg}", file=sys.stderr, flush=True)


# ---------------------------------------------------------------------------
# Auth


def get_token() -> str:
    token = os.environ.get("GITHUB_TOKEN", "").strip()
    if not token:
        sys.exit("ERROR: GITHUB_TOKEN env var is required")
    return token


_TOKEN: str | None = None


def headers_rest() -> dict[str, str]:
    return {
        "Authorization": f"Bearer {_TOKEN}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }


def headers_graphql() -> dict[str, str]:
    return {"Authorization": f"Bearer {_TOKEN}", "Content-Type": "application/json"}


# ---------------------------------------------------------------------------
# REST


def rest_get(path: str, params: dict | None = None, retries: int = 3) -> dict | list:
    url = f"{REST_API}/{path.lstrip('/')}"
    last_err: Exception | None = None
    for attempt in range(retries):
        try:
            r = requests.get(url, headers=headers_rest(), params=params, timeout=30)
            if r.status_code == 200:
                return r.json()
            if r.status_code in (429, 502, 503, 504):
                wait = 2**attempt * 2
                log(f"REST {r.status_code} on {path}; retry in {wait}s", "WARN")
                time.sleep(wait)
                continue
            log(f"REST GET {path} failed: {r.status_code} {r.text[:200]}", "ERROR")
            r.raise_for_status()
        except requests.RequestException as e:
            last_err = e
            wait = 2**attempt * 2
            log(f"REST GET {path} threw {e}; retry in {wait}s", "WARN")
            time.sleep(wait)
    raise RuntimeError(f"REST GET {path} failed after {retries} retries: {last_err}")


def rest_get_paginated(path: str, params: dict | None = None, max_pages: int = 20) -> list:
    p = dict(params or {})
    p["per_page"] = 100
    out: list = []
    for page in range(1, max_pages + 1):
        p["page"] = page
        items = rest_get(path, p)
        if not items:
            break
        if not isinstance(items, list):
            log(f"REST {path} page {page} returned non-list", "WARN")
            break
        out.extend(items)
        if len(items) < 100:
            break
    return out


# ---------------------------------------------------------------------------
# GraphQL


def graphql(query: str, variables: dict | None = None, retries: int = 3) -> dict:
    payload = {"query": query, "variables": variables or {}}
    last_err: Exception | None = None
    for attempt in range(retries):
        try:
            r = requests.post(GRAPHQL_API, headers=headers_graphql(), json=payload, timeout=30)
            if r.status_code == 200:
                data = r.json()
                if "errors" in data:
                    log(f"GraphQL errors: {json.dumps(data['errors'])[:400]}", "ERROR")
                    raise RuntimeError("GraphQL returned errors")
                return data["data"]
            if r.status_code in (429, 502, 503, 504):
                wait = 2**attempt * 2
                log(f"GraphQL {r.status_code}; retry in {wait}s", "WARN")
                time.sleep(wait)
                continue
            log(f"GraphQL HTTP {r.status_code}: {r.text[:300]}", "ERROR")
            r.raise_for_status()
        except requests.RequestException as e:
            last_err = e
            wait = 2**attempt * 2
            log(f"GraphQL threw {e}; retry in {wait}s", "WARN")
            time.sleep(wait)
    raise RuntimeError(f"GraphQL failed after {retries} retries: {last_err}")


# ---------------------------------------------------------------------------
# Issue data fetch


@dataclass
class IssueData:
    number: int
    node_id: str
    title: str
    state: str  # "open" / "closed"
    closed_at: datetime | None
    created_at: datetime
    reporter: str
    assignees: list[str]
    labels: list[str]
    issue_type: str | None  # e.g. "Bug", "Crash", "Meta", "Tracking", or None
    is_pull_request: bool
    comments: list[dict]


def parse_dt(s: str | None) -> datetime | None:
    if not s:
        return None
    return datetime.fromisoformat(s.replace("Z", "+00:00"))


def fetch_issue(number: int) -> IssueData:
    issue = rest_get(f"repos/{REPO}/issues/{number}")
    if not isinstance(issue, dict):
        raise RuntimeError(f"unexpected response for issue {number}")
    comments = rest_get_paginated(f"repos/{REPO}/issues/{number}/comments")
    created_at = parse_dt(issue["created_at"])
    if created_at is None:
        raise RuntimeError(f"issue {number} has no created_at")
    issue_type = None
    if isinstance(issue.get("type"), dict):
        issue_type = issue["type"].get("name")
    return IssueData(
        number=number,
        node_id=issue["node_id"],
        title=issue["title"],
        state=issue["state"],
        closed_at=parse_dt(issue.get("closed_at")),
        created_at=created_at,
        reporter=issue["user"]["login"],
        assignees=[a["login"] for a in (issue.get("assignees") or [])],
        labels=[l["name"] for l in issue["labels"]],
        issue_type=issue_type,
        is_pull_request="pull_request" in issue,
        comments=comments,
    )


# ---------------------------------------------------------------------------
# Staff team


_STAFF: set[str] | None = None


def fetch_staff() -> set[str]:
    global _STAFF
    if _STAFF is not None:
        return _STAFF
    members = rest_get_paginated(f"orgs/{REPO_OWNER}/teams/{STAFF_TEAM_SLUG}/members")
    _STAFF = {m["login"] for m in members}
    log(f"loaded {len(_STAFF)} staff members")
    return _STAFF


def is_bot(user: dict) -> bool:
    return user.get("type") == "Bot" or user.get("login", "").endswith("[bot]")


def is_substantive_staff_comment(comment: dict, staff: set[str]) -> bool:
    user = comment.get("user", {})
    if user.get("login") not in staff or is_bot(user):
        return False
    body = comment.get("body") or ""
    if len(body) >= SUBSTANTIVE_COMMENT_MIN_LEN:
        return True
    # Cheap attachment heuristic: looks for media tokens or attachment hosts.
    if any(
        m in body
        for m in (
            "user-attachments/assets",
            ".png",
            ".jpg",
            ".jpeg",
            ".gif",
            ".mp4",
            ".webm",
            ".mov",
        )
    ):
        return True
    return False


def latest_reporter_activity(issue: IssueData) -> datetime:
    times = [issue.created_at]
    for c in issue.comments:
        if c["user"]["login"] == issue.reporter:
            t = parse_dt(c["created_at"])
            if t:
                times.append(t)
    return max(times)


# ---------------------------------------------------------------------------
# Derivation rules
# (Mirrors the spec's R0-R6 cascade. Keep in sync with
# spec.md → "Status derivation rules".)


def derive_status(issue: IssueData, staff: set[str]) -> tuple[str, str, str]:
    """Returns (status, rule_id, why)."""
    L = set(issue.labels)

    if issue.closed_at is not None:
        return STATUS_CLOSED, "R1", "issue is closed"

    if "state:claimed by community" in L:
        return STATUS_CLAIMED_COMMUNITY, "R0", "state:claimed by community label"

    if "state:reproducible" in L:
        if issue.assignees:
            return STATUS_REPRODUCIBLE, "R2a", f"reproducible, assignee={','.join(issue.assignees)}"
        # R2b vs R2c: any substantive staff comment in the thread?
        substantive = None
        for c in issue.comments:
            if is_substantive_staff_comment(c, staff):
                substantive = c
        if substantive:
            return (
                STATUS_HANDOFF,
                "R2b",
                f"reproducible, no assignee, staff context @ {substantive['created_at']} "
                f"({len(substantive['body'])} chars by @{substantive['user']['login']})",
            )
        return (
            STATUS_HANDOFF_INCOMPLETE,
            "R2c",
            "reproducible, no assignee, no substantive staff comment — close the loop",
        )

    # R4 (state:needs info) and R5 (state:needs repro) intentionally come
    # before R3 (state:needs triage). Per the team's actual practice,
    # state:needs triage is often left on while triage is in progress; only
    # when no other state label is more specific should we treat the issue
    # as "needs initial labels."
    if "state:needs info" in L:
        # R4 splits into three sub-cases based on whether we've actually
        # asked anything (substantive staff comment) and whether the reporter
        # or a third-party has responded.
        substantive_staff = None
        for c in issue.comments:
            if is_substantive_staff_comment(c, staff):
                substantive_staff = c
        if substantive_staff is None:
            # state:needs info applied without an actual question to the user.
            # Runbook violation — we owe the reporter a comment explaining
            # what info we need.
            return (
                STATUS_NEEDS_ASK,
                "R4c",
                "state:needs info present but no substantive staff comment exists — we haven't asked anything",
            )
        last_comment = issue.comments[-1] if issue.comments else None
        if last_comment is not None:
            author = last_comment["user"]["login"]
            non_staff = author not in staff and not is_bot(last_comment["user"])
            if non_staff:
                ct = parse_dt(last_comment["created_at"])
                st = parse_dt(substantive_staff["created_at"])
                if ct and st and ct > st:
                    relation = "reporter" if author == issue.reporter else "third-party"
                    return (
                        STATUS_USER_REPLIED,
                        "R4b",
                        f"{relation} (@{author}) replied {ct.isoformat()} after substantive staff @ {st.isoformat()}",
                    )
        return (
            STATUS_AWAITING_USER,
            "R4a",
            f"substantive staff comment @ {substantive_staff['created_at']}, no non-staff reply since",
        )

    if "state:needs repro" in L:
        cutoff = latest_reporter_activity(issue)
        for c in reversed(issue.comments):
            ct = parse_dt(c["created_at"])
            if ct and ct > cutoff and is_substantive_staff_comment(c, staff):
                return (
                    STATUS_RESPONDED_NO_REPRO,
                    "R5b",
                    f"staff comment {len(c['body'])} chars by @{c['user']['login']} @ {c['created_at']}",
                )
        return STATUS_NEEDS_REPRO_ATTEMPT, "R5a", "no substantive staff comment after reporter's last activity"

    # R3 (state:needs triage) is checked LAST among recognized state labels.
    # If state:needs triage is the only state label, the issue genuinely needs
    # initial labeling. If any other state label is also present, that state
    # has already been matched above and won.
    if "state:needs triage" in L:
        return STATUS_NEEDS_LABELS, "R3", "state:needs triage label present (no other state:* matched)"

    return STATUS_UNKNOWN, "R6", f"open with no recognized state label (labels: {sorted(L) or '<none>'})"


def derive_stale_since(
    issue: IssueData, status: str, staff: set[str]
) -> datetime | None:
    """Returns the timestamp anchor used to measure aging, or None."""
    if status in TERMINAL_OR_RESTING_STATUSES or status == STATUS_UNKNOWN:
        return None
    if status == STATUS_NEEDS_LABELS:
        return issue.created_at
    if status == STATUS_NEEDS_REPRO_ATTEMPT:
        return latest_reporter_activity(issue)
    if status == STATUS_NEEDS_ASK:
        # Anchor on issue creation — measures how long the runbook violation
        # has gone unaddressed. Aging threshold is 0 (always flagged).
        return issue.created_at
    if status == STATUS_AWAITING_USER:
        # Anchor on the most recent SUBSTANTIVE staff comment (the actual
        # "ask"), consistent with R4's substantive-comment requirement.
        substantive_staff = None
        for c in issue.comments:
            if is_substantive_staff_comment(c, staff):
                substantive_staff = c
        return parse_dt(substantive_staff["created_at"]) if substantive_staff else issue.created_at
    if status == STATUS_USER_REPLIED:
        last_non_staff = None
        for c in issue.comments:
            u = c["user"]
            if u["login"] not in staff and not is_bot(u):
                last_non_staff = c
        return parse_dt(last_non_staff["created_at"]) if last_non_staff else None
    if status == STATUS_HANDOFF_INCOMPLETE:
        # Spec: when state:reproducible was applied. Approximation for v0:
        # issue.created_at as a weak proxy. Replacing with timeline event lookup
        # is a "parked" item.
        return issue.created_at
    return None


def derive_aged(status: str, stale_since: datetime | None) -> tuple[str, str]:
    """Returns ('Yes' | 'No', why)."""
    if status == STATUS_HANDOFF_INCOMPLETE:
        return "Yes", "always-flagged for loop closure"
    if status == STATUS_NEEDS_ASK:
        return "Yes", "always-flagged: state:needs info applied without a substantive staff comment"
    if status in TERMINAL_OR_RESTING_STATUSES or status == STATUS_UNKNOWN:
        return "No", "terminal/resting"
    if not stale_since:
        return "No", "no stale_since (status not aged-tracked)"
    if status not in AGE_THRESHOLDS_DAYS:
        return "No", f"status {status} not aged-tracked"
    age = NOW - stale_since
    threshold = AGE_THRESHOLDS_DAYS[status]
    if age > timedelta(days=threshold):
        return "Yes", f"{status} for {age.days}d (>{threshold}d)"
    return "No", f"{status} for {age.days}d (≤{threshold}d)"


# ---------------------------------------------------------------------------
# Project schema cache
# Discovered at runtime by name so the script doesn't break if field IDs
# change (e.g., project recreated). Project number is stable config.


_PROJECT_SCHEMA: dict | None = None


def fetch_project_schema() -> dict:
    """Returns {'id', 'fields_by_name'} where fields_by_name maps name → field dict."""
    global _PROJECT_SCHEMA
    if _PROJECT_SCHEMA is not None:
        return _PROJECT_SCHEMA
    query = """
    query($owner: String!, $number: Int!) {
      organization(login: $owner) {
        projectV2(number: $number) {
          id
          fields(first: 30) {
            nodes {
              __typename
              ... on ProjectV2Field { id name dataType }
              ... on ProjectV2SingleSelectField {
                id name dataType options { id name }
              }
              ... on ProjectV2IterationField {
                id name dataType
                configuration {
                  duration startDay
                  iterations { id title startDate duration }
                  completedIterations { id title startDate duration }
                }
              }
            }
          }
        }
      }
    }
    """
    data = graphql(query, {"owner": PROJECT_OWNER, "number": PROJECT_NUMBER})
    proj = data["organization"]["projectV2"]
    if not proj:
        sys.exit(f"ERROR: project #{PROJECT_NUMBER} not found in {PROJECT_OWNER}")
    fields_by_name = {f["name"]: f for f in proj["fields"]["nodes"]}
    required = ["Status", "Intake week", "Stale since", "Aged?"]
    missing = [n for n in required if n not in fields_by_name]
    if missing:
        sys.exit(f"ERROR: project missing required fields: {missing}")
    _PROJECT_SCHEMA = {"id": proj["id"], "fields_by_name": fields_by_name}
    log(f"loaded project schema: id={proj['id']}, fields={list(fields_by_name)}")
    return _PROJECT_SCHEMA


def status_option_id(status_name: str) -> str | None:
    schema = fetch_project_schema()
    for opt in schema["fields_by_name"]["Status"]["options"]:
        if opt["name"] == status_name:
            return opt["id"]
    return None


def aged_option_id(value: str) -> str | None:
    schema = fetch_project_schema()
    for opt in schema["fields_by_name"]["Aged?"]["options"]:
        if opt["name"] == value:
            return opt["id"]
    return None


def iteration_id_for_date(d: datetime) -> str | None:
    schema = fetch_project_schema()
    field = schema["fields_by_name"]["Intake week"]
    cfg = field["configuration"]
    iterations = list(cfg.get("iterations") or []) + list(cfg.get("completedIterations") or [])
    for it in iterations:
        start = parse_dt(it["startDate"] + "T00:00:00+00:00")
        if start is None:
            continue
        end = start + timedelta(days=int(it["duration"]))
        if start <= d < end:
            return it["id"]
    return None


# ---------------------------------------------------------------------------
# Project item lookup / mutation


def get_project_item_id(issue_node_id: str) -> str | None:
    """Returns the ProjectV2Item.id for the issue in our project, or None."""
    schema = fetch_project_schema()
    project_id = schema["id"]
    query = """
    query($issueId: ID!) {
      node(id: $issueId) {
        ... on Issue {
          projectItems(first: 100) {
            pageInfo { hasNextPage }
            nodes { id project { id } }
          }
        }
      }
    }
    """
    data = graphql(query, {"issueId": issue_node_id})
    node = data["node"]
    if not node:
        return None
    items_block = node["projectItems"]
    for item in items_block["nodes"]:
        if item["project"]["id"] == project_id:
            return item["id"]
    if items_block["pageInfo"]["hasNextPage"]:
        # Issue is on >100 projects; very unlikely. Log + return None.
        log(f"issue {issue_node_id} on >100 projects, can't find ours in first page", "WARN")
    return None


def add_to_project(issue_node_id: str) -> str:
    schema = fetch_project_schema()
    mutation = """
    mutation($projectId: ID!, $issueId: ID!) {
      addProjectV2ItemById(input: { projectId: $projectId, contentId: $issueId }) {
        item { id }
      }
    }
    """
    data = graphql(mutation, {"projectId": schema["id"], "issueId": issue_node_id})
    return data["addProjectV2ItemById"]["item"]["id"]




def update_single_select(item_id: str, field_id: str, option_id: str, dry_run: bool) -> None:
    if dry_run:
        log(f"  [DRY] single-select field={field_id} option={option_id} on item={item_id}")
        return
    schema = fetch_project_schema()
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
      updateProjectV2ItemFieldValue(input: {
        projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
        value: { singleSelectOptionId: $optionId }
      }) { projectV2Item { id } }
    }
    """
    graphql(
        mutation,
        {
            "projectId": schema["id"],
            "itemId": item_id,
            "fieldId": field_id,
            "optionId": option_id,
        },
    )


def update_date(item_id: str, field_id: str, date_iso: str, dry_run: bool) -> None:
    if dry_run:
        log(f"  [DRY] date field={field_id} value={date_iso} on item={item_id}")
        return
    schema = fetch_project_schema()
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $date: Date!) {
      updateProjectV2ItemFieldValue(input: {
        projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
        value: { date: $date }
      }) { projectV2Item { id } }
    }
    """
    graphql(
        mutation,
        {"projectId": schema["id"], "itemId": item_id, "fieldId": field_id, "date": date_iso},
    )


def update_iteration(item_id: str, field_id: str, iteration_id: str, dry_run: bool) -> None:
    if dry_run:
        log(f"  [DRY] iteration field={field_id} value={iteration_id} on item={item_id}")
        return
    schema = fetch_project_schema()
    mutation = """
    mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $iterId: String!) {
      updateProjectV2ItemFieldValue(input: {
        projectId: $projectId, itemId: $itemId, fieldId: $fieldId,
        value: { iterationId: $iterId }
      }) { projectV2Item { id } }
    }
    """
    graphql(
        mutation,
        {
            "projectId": schema["id"],
            "itemId": item_id,
            "fieldId": field_id,
            "iterId": iteration_id,
        },
    )


# ---------------------------------------------------------------------------
# Sync


def sync_issue(number: int, dry_run: bool = False) -> None:
    """Sync a single issue. Adds to project if missing, then updates fields.

    Idempotent — running twice with the same issue state has no effect after
    the first run.
    """
    log(f"sync #{number} (dry_run={dry_run})")
    issue = fetch_issue(number)

    if issue.is_pull_request:
        log(f"  #{number} is a PR; skipping (project tracks issues)")
        return

    # Skip administrative issue types (Meta, Tracking, etc.). These are
    # collections / dashboards, not triage work. The script doesn't have
    # permission to remove items from the project (intentional — narrows blast
    # radius). Existing Meta/Tracking items in the project should be removed
    # manually one-time; new ones get auto-added by the project's auto-add
    # workflow but the sync below skips them, so they sit with no Status /
    # Aged? / Stale since fields set and don't appear in any status-filtered
    # view.
    if issue.issue_type in SKIP_ISSUE_TYPES:
        log(f"  #{number} is type={issue.issue_type}; not a triage item, skipping fields")
        return

    staff = fetch_staff()

    status, rule, why = derive_status(issue, staff)
    stale_since = derive_stale_since(issue, status, staff)
    aged, aged_why = derive_aged(status, stale_since)
    intake_iter_id = iteration_id_for_date(issue.created_at)

    log(f"  status={status} ({rule}: {why})")
    log(f"  stale_since={stale_since.isoformat() if stale_since else 'none'}")
    log(f"  aged={aged} ({aged_why})")
    log(f"  intake_iteration_id={intake_iter_id or 'none (created_at outside iteration range)'}")

    schema = fetch_project_schema()
    item_id = get_project_item_id(issue.node_id)
    if not item_id:
        if dry_run:
            log("  [DRY] would add to project (item not yet present)")
            return
        item_id = add_to_project(issue.node_id)
        log(f"  added to project as item={item_id}")

    # Status (always set)
    sid = status_option_id(status)
    if not sid:
        log(f"  ERROR: no Status option named '{status}' in project; skipping status update", "ERROR")
    else:
        update_single_select(
            item_id, schema["fields_by_name"]["Status"]["id"], sid, dry_run
        )

    # Aged? (always set)
    aged_id = aged_option_id(aged)
    if not aged_id:
        log(f"  ERROR: no Aged? option named '{aged}'; skipping", "ERROR")
    else:
        update_single_select(
            item_id, schema["fields_by_name"]["Aged?"]["id"], aged_id, dry_run
        )

    # Stale since (only set when meaningful)
    if stale_since:
        update_date(
            item_id,
            schema["fields_by_name"]["Stale since"]["id"],
            stale_since.date().isoformat(),
            dry_run,
        )

    # Intake week (only set when an iteration covers the created_at)
    if intake_iter_id:
        update_iteration(
            item_id,
            schema["fields_by_name"]["Intake week"]["id"],
            intake_iter_id,
            dry_run,
        )


def sync_all(dry_run: bool = False) -> None:
    """Sync every item currently in the project. Cron mode."""
    log("fetching all project items…")
    cursor: str | None = None
    total = 0
    failed = 0
    while True:
        query = """
        query($owner: String!, $number: Int!, $cursor: String) {
          organization(login: $owner) {
            projectV2(number: $number) {
              items(first: 100, after: $cursor) {
                pageInfo { hasNextPage endCursor }
                nodes {
                  id
                  content {
                    __typename
                    ... on Issue { number }
                    ... on PullRequest { number }
                  }
                }
              }
            }
          }
        }
        """
        data = graphql(
            query, {"owner": PROJECT_OWNER, "number": PROJECT_NUMBER, "cursor": cursor}
        )
        items_block = data["organization"]["projectV2"]["items"]
        for item in items_block["nodes"]:
            content = item.get("content")
            if not content:
                continue
            if content["__typename"] != "Issue":
                continue
            num = content["number"]
            try:
                sync_issue(num, dry_run=dry_run)
            except Exception as e:
                log(f"sync #{num} failed: {e}", "ERROR")
                failed += 1
            total += 1
        if not items_block["pageInfo"]["hasNextPage"]:
            break
        cursor = items_block["pageInfo"]["endCursor"]
    log(f"done: synced {total} items, {failed} failed")


# ---------------------------------------------------------------------------
# Main


def main() -> int:
    global _TOKEN

    ap = argparse.ArgumentParser(description=__doc__)
    grp = ap.add_mutually_exclusive_group(required=True)
    grp.add_argument("--issue", type=int, help="sync a single issue by number")
    grp.add_argument("--all", action="store_true", help="sync every project item")
    ap.add_argument("--dry-run", action="store_true", help="compute but don't mutate")
    args = ap.parse_args()

    _TOKEN = get_token()

    if args.issue:
        sync_issue(args.issue, dry_run=args.dry_run)
    elif args.all:
        sync_all(dry_run=args.dry_run)

    return 0


if __name__ == "__main__":
    sys.exit(main())
