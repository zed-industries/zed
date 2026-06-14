#!/usr/bin/env python3
"""
Comment on newly opened issues with possible duplicates and triage hints.

This script is run by a GitHub Actions workflow when a new issue is opened. It:
1. Checks eligibility (bug/crash type or untyped, non-staff author)
2. Detects relevant areas using Claude + the area label taxonomy
3. Parses known "duplicate magnets" from tracking issue #46355
4. Searches for similar issues — open (last 60 days) and recently closed (last 30 days)
5. Asks Claude to sort open candidates into likely and possible duplicates, and
   surface recently closed issues that may be useful triage context
6. Posts a comment if anything is found: a user-facing duplicate alert for likely
   duplicates, and/or a collapsed triager-facing section for possible duplicates
   and recently closed related issues

Requires:
    requests (pip install requests)

Usage:
    python github-check-new-issue-for-duplicates.py <issue_number>

Environment variables:
    GITHUB_TOKEN       - GitHub token (org members: read, issues: read & write)
    ANTHROPIC_API_KEY  - Anthropic API key for Claude

"""

import argparse
import json
import os
import re
import sys
import time
from datetime import datetime, timedelta

import requests

GITHUB_API = "https://api.github.com"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
TRACKING_ISSUE_NUMBER = 46355
STAFF_TEAM_SLUG = "staff"

# area prefixes to collapse in taxonomy (show summary instead of all sub-labels)
PREFIXES_TO_COLLAPSE = ["languages", "parity", "tooling"]

# stopwords to filter from title keyword searches (short words handled by len > 2 filter)
STOPWORDS = {
    "after", "all", "also", "and", "any", "but", "can't", "does", "doesn't",
    "don't", "for", "from", "have", "just", "not", "only", "some", "that",
    "the", "this", "when", "while", "with", "won't", "work", "working", "zed",
}

# HTTP statuses we'll retry on for GET requests
TRANSIENT_HTTP_STATUSES = {429, 500, 502, 503, 504}


def log(message):
    """Print to stderr so it doesn't interfere with JSON output on stdout."""
    print(message, file=sys.stderr)


def github_api_get(path, params=None):
    """Fetch JSON from the GitHub API, retrying transient failures. Raises on non-2xx status."""
    url = f"{GITHUB_API}/{path.lstrip('/')}"
    for attempt in range(3):
        try:
            response = requests.get(url, headers=GITHUB_HEADERS, params=params)
            response.raise_for_status()
            return response.json()
        except requests.RequestException as e:
            transient = isinstance(e, (requests.ConnectionError, requests.Timeout)) or (
                isinstance(e, requests.HTTPError) and e.response.status_code in TRANSIENT_HTTP_STATUSES
            )
            if not transient or attempt == 2:
                raise
            wait = 2 ** attempt
            log(f"  Transient GitHub API error ({e}); retrying in {wait}s")
            time.sleep(wait)


def github_search_issues(query, per_page=15):
    """Search issues, returning most recently created first."""
    params = {"q": query, "sort": "created", "order": "desc", "per_page": per_page}
    return github_api_get("/search/issues", params).get("items", [])


def check_team_membership(org, team_slug, username):
    """Check if user is an active member of a team."""
    try:
        data = github_api_get(f"/orgs/{org}/teams/{team_slug}/memberships/{username}")
        return data.get("state") == "active"
    except requests.HTTPError as e:
        if e.response.status_code == 404:
            return False
        raise


def post_comment(issue_number: int, body):
    url = f"{GITHUB_API.rstrip('/')}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}/comments"
    response = requests.post(url, headers=GITHUB_HEADERS, json={"body": body})
    response.raise_for_status()
    log(f"  Posted comment on #{issue_number}")


def build_comment(likely_duplicates, possible_duplicates, related_closed_issues):
    """Compose the full comment body. Returns empty string if there's nothing to post.

    The comment has two sections, each optional:
    - User-facing duplicate alert, rendered when likely_duplicates is non-empty.
    - Collapsed triage context, rendered when there are possible duplicates or
      related closed issues to surface for triagers.
    """
    sections = []

    if likely_duplicates:
        match_list = "\n".join(f"- #{m['number']}" for m in likely_duplicates)
        explanations = "\n\n".join(
            f"**#{m['number']}:** {m['explanation']}\n\n**Shared root cause:** {m['shared_root_cause']}"
            for m in likely_duplicates
        )
        sections.append(f"""This issue appears to be a duplicate of:

{match_list}

**If this is indeed a duplicate:**
Please close this issue and subscribe to the linked issue for updates (select "Close as not planned" → "Duplicate")

**If this is a different issue:**
No action needed. A maintainer will review this shortly.

<details>
<summary>Why were these issues selected?</summary>

{explanations}

</details>""")

    if possible_duplicates or related_closed_issues:
        parts = []
        if possible_duplicates:
            lines = [
                f"- #{m['number']} — {m['explanation']}\n"
                f"  - Possible shared root cause: {m['shared_root_cause']}"
                for m in possible_duplicates
            ]
            parts.append("**Possibly related open issues:**\n\n" + "\n".join(lines))
        if related_closed_issues:
            # state_reason is shown only for "duplicate" (the close type is otherwise
            # already visible from GitHub's icon next to the issue number on render).
            lines = [
                f"- #{m['number']}"
                f"{' (closed as duplicate)' if m['state_reason'] == 'duplicate' else ''}"
                f" — {m['explanation']}"
                for m in related_closed_issues
            ]
            parts.append("**Recently closed, possibly the same bug:**\n\n" + "\n".join(lines))
        body = "\n\n".join(parts)
        sections.append(f"""<details>
<summary>Additional recent context for triagers</summary>

{body}

</details>""")

    if not sections:
        return ""

    sections.append("---\n<sub>This is an automated analysis and might be incorrect.</sub>")
    return "\n\n".join(sections)


def call_claude(api_key, system, user_content, max_tokens=1024):
    """Send a message to Claude and return the text response. Raises on non-2xx status."""
    response = requests.post(
        "https://api.anthropic.com/v1/messages",
        headers={
            "x-api-key": api_key,
            "anthropic-version": "2023-06-01",
            "content-type": "application/json",
        },
        json={
            "model": "claude-sonnet-4-20250514",
            "max_tokens": max_tokens,
            "temperature": 0.0,
            "system": system,
            "messages": [{"role": "user", "content": user_content}],
        },
    )
    response.raise_for_status()
    data = response.json()

    usage = data.get("usage", {})
    log(f"  Token usage - Input: {usage.get('input_tokens', 'N/A')}, Output: {usage.get('output_tokens', 'N/A')}")

    content = data.get("content", [])
    if content and content[0].get("type") == "text":
        return content[0].get("text") or ""
    return ""


def fetch_issue(issue_number: int):
    """Fetch issue from GitHub and return as a dict."""
    log(f"Fetching issue #{issue_number}")

    issue_data = github_api_get(f"/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}")
    issue = {
        "number": issue_number,
        "title": issue_data["title"],
        "body": issue_data.get("body") or "",
        "author": (issue_data.get("user") or {}).get("login") or "",
        "type": (issue_data.get("type") or {}).get("name"),
    }

    log(f"  Title: {issue['title']}\n  Type: {issue['type']}\n  Author: {issue['author']}")
    return issue


def should_skip(issue):
    """Check if issue should be skipped in duplicate detection process."""
    if issue["type"] and issue["type"] not in ["Bug", "Crash"]:
        log(f"  Skipping: issue type '{issue['type']}' is not blank and not a bug/crash report")
        return True

    if issue["author"] and check_team_membership(REPO_OWNER, STAFF_TEAM_SLUG, issue["author"]):
        log(f"  Skipping: author '{issue['author']}' is a {STAFF_TEAM_SLUG} member")
        return True

    return False


def fetch_area_labels():
    """Fetch area:* labels from the repository. Returns list of {name, description} dicts."""
    log("Fetching area labels")

    labels = []
    page = 1
    while page_labels := github_api_get(
        f"/repos/{REPO_OWNER}/{REPO_NAME}/labels",
        params={"per_page": 100, "page": page},
    ):
        labels.extend(page_labels)
        page += 1

    # label["name"][5:] removes the "area:" prefix
    area_labels = [
        {"name": label["name"][5:], "description": label.get("description") or ""}
        for label in labels
        if label["name"].startswith("area:")
    ]

    log(f"  Found {len(area_labels)} area labels")
    return area_labels


def format_taxonomy_for_claude(area_labels):
    """Format area labels into a string for Claude, collapsing certain prefixes."""
    lines = set()

    for area in area_labels:
        name = area["name"]
        collapsible_prefix = next(
            (p for p in PREFIXES_TO_COLLAPSE if name.startswith(f"{p}/")), None)

        if collapsible_prefix:
            lines.add(f"- {collapsible_prefix}/* (multiple specific sub-labels exist)")
        else:
            desc = area["description"]
            lines.add(f"- {name}: {desc}" if desc else f"- {name}")

    return "\n".join(sorted(lines))


def detect_areas(anthropic_key, issue, area_labels):
    """Use Claude to detect which area labels apply to the issue.

    Claude may ignore the format instruction or hallucinate names, so the response
    is validated against the canonical set of area labels.
    """
    log("Detecting areas with Claude")

    taxonomy = format_taxonomy_for_claude(area_labels)
    valid_areas = {label["name"] for label in area_labels}

    system_prompt = """You analyze GitHub issues to identify which area labels apply.

Decide the area from the user's stated symptom and reproduction steps. Issue bodies routinely
contain pasted log output, crash dumps, stack traces, settings files, and template headers like
"Attach Zed log file" or "Relevant Zed settings" — these are evidence about the symptom and
should not push you toward labels like "logging" or "settings" unless the bug itself is about
how that subsystem works.

Respond with ONLY a comma-separated list of matching area names. No prose, no explanation,
no markdown, no preamble — just the names.

- Output at most 3 areas, ranked by relevance
- Use exact area names from the taxonomy
- If no areas clearly match, respond with: none
- For languages/*, tooling/*, or parity/*, use the specific sub-label (e.g., "languages/rust",
  tooling/eslint, parity/vscode)

Examples of valid responses (each line is a complete response on its own):
  editor, parity/vim
  ai, ai/agent panel
  none
"""

    user_content = f"""## Area Taxonomy
{taxonomy}

# Issue Title
{issue['title']}

# Issue Body
{issue['body'][:4000]}"""

    response = call_claude(anthropic_key, system_prompt, user_content, max_tokens=100).strip()
    log(f"  Detected areas: {response}")

    if response.lower() == "none":
        return []

    valid, dropped = [], []
    for area in response.split(","):
        area = area.strip()
        (valid if area in valid_areas else dropped).append(area)
    if dropped:
        log(f"  Dropped {len(dropped)} unknown area(s) from Claude response: {dropped}")
    return valid


def parse_duplicate_magnets():
    """Parse known duplicate magnets from tracking issue #46355.

    Returns a list of magnets sorted by duplicate count (most duplicated first).
    Magnets only have number, areas, and dupe_count — use enrich_magnets() to fetch
    title and body_preview for the ones you need.
    """
    log(f"Parsing duplicate magnets from #{TRACKING_ISSUE_NUMBER}")

    issue_data = github_api_get(f"/repos/{REPO_OWNER}/{REPO_NAME}/issues/{TRACKING_ISSUE_NUMBER}")
    body = issue_data.get("body") or ""

    # parse the issue body
    # format: ## area_name
    #         -   [N dupes] https://github.com/zed-industries/zed/issues/NUMBER
    magnets = {}  # number -> {number, areas, dupe_count}
    current_area = None

    for line in body.split("\n"):
        # check for area header
        if line.startswith("## "):
            current_area = line[3:].strip()
            continue

        if not current_area or not line.startswith("-") or "/issues/" not in line:
            continue

        # parse: -   [N dupes] https://github.com/.../issues/NUMBER
        try:
            dupe_count = int(line.split("[")[1].split()[0])
            number = int(line.split("/issues/")[1].split()[0].rstrip(")"))
        except (ValueError, IndexError):
            continue

        # skip "(unlabeled)": these magnets should match everything
        is_unlabeled = current_area == "(unlabeled)"

        if number in magnets:
            if not is_unlabeled:
                magnets[number]["areas"].append(current_area)
        else:
            magnets[number] = {
                "number": number,
                "areas": [] if is_unlabeled else [current_area],
                "dupe_count": dupe_count,
            }

    magnet_list = sorted(magnets.values(), key=lambda m: m["dupe_count"], reverse=True)
    log(f"  Parsed {len(magnet_list)} duplicate magnets")
    return magnet_list


def enrich_magnets(magnets):
    """Fetch title and body_preview for magnets from the API."""
    log(f"  Fetching details for {len(magnets)} magnets")
    for magnet in magnets:
        data = github_api_get(f"/repos/{REPO_OWNER}/{REPO_NAME}/issues/{magnet['number']}")
        magnet["title"] = data["title"]
        magnet["body_preview"] = (data.get("body") or "")[:1000]


def areas_match(detected, magnet_area):
    """Check if detected area matches magnet area. Matches broadly across hierarchy levels."""
    return (
        detected == magnet_area
        or magnet_area.startswith(f"{detected}/")
        or detected.startswith(f"{magnet_area}/")
    )


def filter_magnets_by_areas(magnets, detected_areas):
    """Filter magnets based on detected areas."""
    if not detected_areas:
        return magnets

    detected_set = set(detected_areas)

    def matches(magnet):
        # unlabeled magnets (empty areas) match everything
        if not magnet["areas"]:
            return True
        return any(
            areas_match(detected, magnet_area)
            for detected in detected_set
            for magnet_area in magnet["areas"]
        )

    return list(filter(matches, magnets))


def search_for_similar_issues(issue, detected_areas, max_searches_per_state=6):
    """Search for similar issues — both open and recently closed.

    Runs two passes:
    - Open issues: title keywords / error pattern unrestricted, area searches last 60 days.
    - Closed issues: closed within the last 30 days (across all query types).

    max_searches_per_state caps queries per state to keep token usage and context size bounded.
    """
    log("Searching for similar issues")

    sixty_days_ago = (datetime.now() - timedelta(days=60)).strftime("%Y-%m-%d")
    thirty_days_ago = (datetime.now() - timedelta(days=30)).strftime("%Y-%m-%d")

    title_keywords = [word for word in issue["title"].split() if word.lower() not in STOPWORDS and len(word) > 2]
    keywords_query = " ".join(title_keywords) if title_keywords else None

    # error pattern search: capture 5–90 chars after keyword, colon optional
    error_pattern = r"(?i:\b(?:error|panicked|panic|failed)\b)\s*([^\n]{5,90})"
    error_match = re.search(error_pattern, issue["body"])
    error_snippet = error_match.group(1).strip() if error_match else None

    def build_queries(base, area_window=None):
        queries = []
        if keywords_query:
            queries.append(("title_keywords", f"{base} {keywords_query}"))
        for area in detected_areas:
            area_q = f'{base} label:"area:{area}"'
            if area_window:
                area_q += f" created:>{area_window}"
            queries.append(("area_label", area_q))
        if error_snippet:
            queries.append(("error_pattern", f'{base} in:body "{error_snippet}"'))
        return queries

    open_queries = build_queries(
        f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open",
        area_window=sixty_days_ago,
    )
    # closed pass: filter by close date so we catch issues closed recently regardless of
    # when they were opened. closed:> already restricts the result set, so the per-query
    # area window is unnecessary.
    closed_queries = build_queries(
        f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:closed closed:>{thirty_days_ago}",
    )

    seen_issues = {}
    for state_label, queries in (
        ("open", open_queries[:max_searches_per_state]),
        ("closed", closed_queries[:max_searches_per_state]),
    ):
        for search_type, query in queries:
            log(f"  Search ({state_label} / {search_type}): {query}")
            try:
                results = github_search_issues(query, per_page=15)
                for item in results:
                    number = item["number"]
                    if number != issue["number"] and number not in seen_issues:
                        body = item.get("body") or ""
                        seen_issues[number] = {
                            "number": number,
                            "title": item["title"],
                            "state": item.get("state", ""),
                            "state_reason": item.get("state_reason"),
                            "created_at": item.get("created_at", ""),
                            "body_preview": body[:1000],
                            "source": search_type,
                        }
            except requests.RequestException as e:
                log(f"  Search failed: {e}")

    similar_issues = list(seen_issues.values())
    log(f"  Found {len(similar_issues)} similar issues")
    return similar_issues


def analyze_duplicates(anthropic_key, issue, magnets, search_results):
    """Use Claude to identify duplicates (open) and surface related closed issues.

    Returns (likely_duplicates, possible_duplicates, related_closed_issues).
    """
    top_magnets = magnets[:10]
    magnet_numbers = {m["number"] for m in top_magnets}

    open_results = [r for r in search_results if r["state"] == "open" and r["number"] not in magnet_numbers]
    closed_results = [r for r in search_results if r["state"] == "closed" and r["number"] not in magnet_numbers]

    if not top_magnets and not open_results and not closed_results:
        return [], [], []

    log("Analyzing candidates with Claude")
    log(f"  Candidate pool: {len(top_magnets)} magnets, {len(open_results)} open search results, "
        f"{len(closed_results)} closed search results (will pass {min(len(closed_results), 5)} closed)")
    enrich_magnets(top_magnets)

    closed_candidates_for_claude = closed_results[:5]
    if closed_candidates_for_claude:
        log(f"  Closed candidates given to proposer: {[r['number'] for r in closed_candidates_for_claude]}")

    candidates = [
        {"number": m["number"], "title": m["title"], "body_preview": m["body_preview"],
         "state": "open", "state_reason": None, "source": "known_duplicate_magnet"}
        for m in top_magnets
    ] + [
        {"number": r["number"], "title": r["title"], "body_preview": r["body_preview"],
         "state": r["state"], "state_reason": r["state_reason"], "source": "search_result"}
        for r in open_results[:10] + closed_candidates_for_claude
    ]

    system_prompt = """You analyze GitHub issues to (a) identify duplicates among OPEN candidates
and (b) surface recently CLOSED candidates that are useful triage context.

Each candidate has a "state" field ("open" or "closed"), and closed candidates carry a
"state_reason" ("completed", "not_planned", or "duplicate").

# (a) Duplicates — OPEN candidates only

A duplicate means: caused by the SAME BUG in the code, not just similar symptoms.

CRITICAL DISTINCTION — shared symptoms vs shared root cause:
- "models missing", "can't sign in", "editor hangs", "venv not detected" are SYMPTOMS that many
  different bugs can produce. Two reports of the same symptom are NOT duplicates unless you can
  identify a specific shared root cause.
- A duplicate means: if a developer fixed the existing issue, the new issue would also be fixed.
- If the issues just happen to be in the same feature area, or describe similar-sounding problems
  with different specifics (different error messages, different triggers, different platforms,
  different configurations), they are NOT duplicates.

Sort duplicates into two buckets:
- "likely_duplicates": Almost certainly the same bug. You can name a specific shared root cause, and
  the reproduction steps / error messages / triggers are consistent.
- "possible_duplicates": Likely the same bug based on specific technical details, but some
  uncertainty remains.
- Do NOT include issues that merely share symptoms, affect the same feature area, or sound similar
  at a surface level.

Examples of things that are NOT duplicates:
- Two issues about "Copilot models not showing" — one caused by a Zed update breaking the model list,
  the other caused by the user's plan not including those models.
- Two issues about "Zed hangs" — one triggered by network drives, the other by large projects.
- Two issues about "can't sign in" — one caused by a missing system package, the other by a server-side error.

For OPEN duplicates (either bucket), false positives are MUCH worse than false negatives — they
waste the time of both the issue author and the maintainers. When in doubt, omit.

# (b) Closed candidates that may be the same bug — CLOSED candidates only

The goal is NOT a "related reading" list. The goal is to surface closed issues where the
new issue is plausibly the SAME bug — a duplicate that just happens to be filed against a
closed predecessor instead of an open one. Empty is preferable to weak filler — triagers
lose trust in this section quickly if it's stretched. The same false-positives-are-worse
asymmetry as for duplicates applies here.

The bar: a triager reading this should be able to act — ask the reporter to retest a fix,
point at a known design decision that already declined this request, or point at the
canonical bug this is a duplicate of. "Useful context" or "shared area" is NOT a reason
to include.

Omit a candidate if ANY of these apply (in observed practice, almost everything does):

1. Self-contradiction. If you find yourself writing "while focused on X rather than Y",
   "although this is about A, the new issue is about B", "this issue focuses on... rather
   than...", or any acknowledgment that the candidate isn't on the same topic — STOP.
   You've already made the case for omitting it.

2. Fabricated specifics. Every concrete claim about the candidate (its trigger, its scope,
   its conditions) must be visible in the candidate's title or body preview. Specifics
   like "when X happens", "under Y conditions", "specifically affecting Z" that aren't
   supported by the candidate's actual text mean you're inventing details to fit the new
   issue. Omit.

3. Weasel phrases. Paraphrases of these all indicate you don't have a real claim:
   "may indicate similar...", "could provide context for...", "shows / demonstrates recent
   attention to...", "indicates the team has considered...", "demonstrates a pattern
   of...", "may provide useful context...". STOP and omit.

4. Retest by default. The "reporter may need to retest on the latest build" framing only
   applies when the candidate's symptom is literally the same as the new issue's. It is
   NOT a default justification for "this was a recent fix in roughly the same area."

5. Same area / feature, different mechanism. Examples to omit:
   - "ARM compile failure" alongside "ARM runtime perf" — same area, different mechanism.
   - "Worktree path bug" alongside "worktree display label confusion" — same feature,
     unrelated.

6. Vague catch-all candidate. A closed issue like "Zed is slow" / "performance" / "agent
   panel UX" that could be cited next to almost any new bug is filler. If you'd reuse the
   same closed issue across many unrelated new issues, omit.

7. Label or single-keyword overlap. A closed issue whose only connection is a shared
   area:* label or one shared keyword is not relevant.

Worth surfacing — strict examples:
- A recently fixed ("completed") issue with the SAME specific trigger as the new issue —
  triager can ask the reporter to retest on the latest build.
- A cluster of "not_planned" closures about the EXACT same request — known design choice
  the triager can point to.
- A previously triaged "duplicate" pointing at the same canonical issue, or sharing the
  same specific mechanism.

Count: typically 0 or 1. Never more than 2 unless there's an obvious cluster of identical
"not_planned" reports. 0 is a normal outcome.

# Output format

Output only valid JSON (no markdown code blocks):
{
  "likely_duplicates": [
    {
      "number": 12345,
      "shared_root_cause": "The specific bug/root cause shared by both issues",
      "explanation": "Brief explanation with concrete evidence from both issues"
    }
  ],
  "possible_duplicates": [
    {
      "number": 12345,
      "shared_root_cause": "The specific bug/root cause shared by both issues",
      "explanation": "Brief explanation with concrete evidence from both issues"
    }
  ],
  "related_closed_issues": [
    {
      "number": 12345,
      "explanation": "Brief explanation of why this is useful triage context"
    }
  ]
}

Return empty arrays where nothing relevant is found."""

    user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:3000]}

## Existing Issues to Compare
{json.dumps(candidates, indent=2)}"""

    response = call_claude(anthropic_key, system_prompt, user_content, max_tokens=2048)

    # Claude sometimes wraps JSON in a ```json ... ``` fence despite the prompt forbidding it
    fence = re.match(r"^\s*```(?:json)?\s*\n?(.*?)\n?```\s*$", response, re.DOTALL)
    if fence:
        response = fence.group(1)

    try:
        data = json.loads(response)
    except json.JSONDecodeError as e:
        log(f"  Failed to parse Claude response as JSON: {e}")
        log(f"  Raw response:\n{response}")
        sys.exit(1)

    likely = data.get("likely_duplicates", [])
    possible = data.get("possible_duplicates", [])
    closed = data.get("related_closed_issues", [])

    # Claude occasionally places a closed candidate in the duplicate buckets, or vice
    # versa. Enforce that each match lives in the bucket consistent with the canonical
    # state of the candidate we passed in.
    candidate_states = {c["number"]: c["state"] for c in candidates}

    def filter_by_state(items, expected_state, label):
        kept, wrong = [], []
        for m in items:
            (kept if candidate_states.get(m["number"]) == expected_state else wrong).append(m)
        if wrong:
            log(f"  Dropped {len(wrong)} from {label} with wrong/unknown state: {[m['number'] for m in wrong]}")
        return kept

    likely = filter_by_state(likely, "open", "likely_duplicates")
    possible = filter_by_state(possible, "open", "possible_duplicates")
    closed = filter_by_state(closed, "closed", "related_closed_issues")

    # Avoid showing the same issue in both the user-facing alert and the triage section.
    likely_numbers = {m["number"] for m in likely}
    overlap = [m["number"] for m in possible if m["number"] in likely_numbers]
    if overlap:
        log(f"  Dropped {len(overlap)} from possible_duplicates already in likely_duplicates: {overlap}")
    possible = [m for m in possible if m["number"] not in likely_numbers]

    log(f"  Found {len(likely) + len(possible) + len(closed)} potential matches")
    return likely, possible, closed


CRITIQUE_SYSTEM_PROMPT = """You are evaluating ONE recently closed GitHub issue to decide whether a triager looking
at a brand-new bug report would find it useful to be told about that closed issue.

There is no slate to fill. There is no quota. You will be shown exactly one candidate.
The default verdict is OMIT. Zero is the expected outcome for most candidates.

A candidate is worth surfacing ONLY if the new issue is plausibly the SAME BUG as the
closed one — a duplicate that happens to be filed against a closed predecessor. Concretely,
the legitimate cases are exactly three:

- The candidate was closed as "completed" (a fix shipped) AND the new issue has the same
  specific trigger / symptom. The triager will ask the reporter to retest.
- The candidate was closed as "not_planned" AND the new issue is the EXACT same request
  (a feature decision the team already declined). The triager will point at it.
- The candidate was closed as "duplicate" AND it pointed at the same canonical bug the new
  issue describes, or it shares the same specific mechanism.

"Same broad area", "similar-sounding symptom", or "recent attention to this subsystem" are
NOT reasons to include. Omit them.

Return "omit" if ANY of the following apply (in observed practice, almost everything does):

1. Self-contradiction. If your reasoning includes "while focused on X rather than Y",
   "although this is about A, the new issue is about B", "this issue focuses on... rather
   than...", or any acknowledgment the candidate is on a different topic — you've already
   decided to omit.
2. Fabricated specifics. Every concrete claim about the candidate (its trigger, scope,
   conditions) must be visible in the candidate's title or body preview. If you find
   yourself describing the candidate using details that aren't in its text, you're
   inventing details to fit the new issue. Omit.
3. Weasel phrases. Paraphrases of "may indicate similar...", "could provide context
   for...", "shows / demonstrates recent attention to...", "indicates the team has
   considered...", "demonstrates a pattern of...", "may provide useful context..." —
   these mean you don't have a real claim. Omit.
4. Retest by default. The "reporter may need to retest on the latest build" framing only
   applies when the closed issue's symptom is LITERALLY the same as the new issue's. "This
   was a recent fix in roughly the same area" is not enough.
5. Same area / feature, different mechanism. Same area label but different bug, different
   code path, different trigger. Omit.
6. Vague catch-all candidate. A closed issue like "Zed is slow" / "performance" / "agent
   panel UX" that you could cite next to many unrelated new bugs. Omit.
7. Label or single-keyword overlap. Only connection is a shared area:* label or one shared
   keyword. Omit.

Output only valid JSON (no markdown code blocks):
{
  "verdict": "include" | "omit",
  "rule_violated": null | 1 | 2 | 3 | 4 | 5 | 6 | 7,
  "rationale": "one concise sentence explaining the verdict"
}

When "verdict" is "include", "rule_violated" must be null.
When "verdict" is "omit", "rule_violated" should be the most relevant rule number, or null
if the candidate is simply too unrelated for any rule to specifically apply."""


def critique_closed_candidates(anthropic_key, issue, proposed, search_results):
    """Run a strict per-candidate critique pass over the proposer's closed candidates.

    For each proposed match, call Claude with only the new issue and that single candidate
    (blind to the proposer's rationale) and ask for a yes/no verdict. Default is omit.
    Returns the subset of `proposed` that passes critique.
    """
    if not proposed:
        log("  Critique: proposer surfaced 0 closed candidates; skipping")
        return []

    log(f"  Critique: proposer surfaced {len(proposed)} closed candidate(s): "
        f"{[m['number'] for m in proposed]}")

    results_by_number = {r["number"]: r for r in search_results}
    kept = []
    for match in proposed:
        number = match["number"]
        candidate = results_by_number.get(number)
        if candidate is None:
            # Should not happen — analyze_duplicates only emits numbers from candidates it
            # was given — but be defensive rather than crash the bot.
            log(f"  Critique: dropping #{number} — candidate context not found")
            continue

        state_reason = candidate.get("state_reason") or "unknown"
        user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:3000]}

## Closed Candidate #{number}
**Title:** {candidate.get('title', '')}
**State reason:** {state_reason}

**Body preview:**
{candidate.get('body_preview', '')}"""

        log(f"  Critique: evaluating #{number}")
        try:
            response = call_claude(anthropic_key, CRITIQUE_SYSTEM_PROMPT, user_content, max_tokens=300)
        except requests.RequestException as e:
            # If the critique call fails, prefer omitting the candidate over posting noise.
            log(f"  Critique: API call failed for #{number} ({e}); omitting candidate")
            continue

        fence = re.match(r"^\s*```(?:json)?\s*\n?(.*?)\n?```\s*$", response, re.DOTALL)
        if fence:
            response = fence.group(1)

        try:
            verdict_data = json.loads(response)
        except json.JSONDecodeError as e:
            log(f"  Critique: failed to parse verdict for #{number} ({e}); omitting candidate")
            log(f"    Raw response: {response}")
            continue

        verdict = verdict_data.get("verdict")
        rule = verdict_data.get("rule_violated")
        rationale = verdict_data.get("rationale", "")

        if verdict == "include":
            log(f"  Critique: keeping #{number} — {rationale}")
            kept.append(match)
        else:
            rule_str = f"rule {rule}" if rule else "no specific rule"
            log(f"  Critique: omitting #{number} ({rule_str}) — {rationale}")

    log(f"  Critique: kept {len(kept)} of {len(proposed)} closed candidates")
    return kept


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Identify potential duplicate issues")
    parser.add_argument("issue_number", type=int, help="Issue number to analyze")
    parser.add_argument("--dry-run", action="store_true", help="Skip posting comment, just log what would be posted")
    args = parser.parse_args()

    github_token = os.environ.get("GITHUB_TOKEN")
    anthropic_key = os.environ.get("ANTHROPIC_API_KEY")

    if not github_token:
        log("Error: GITHUB_TOKEN not set")
        sys.exit(1)
    if not anthropic_key:
        log("Error: ANTHROPIC_API_KEY not set")
        sys.exit(1)

    GITHUB_HEADERS = {
        "Authorization": f"Bearer {github_token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }

    issue = fetch_issue(args.issue_number)
    if should_skip(issue):
        print(json.dumps({"skipped": True}))
        sys.exit(0)

    # detect areas
    detected_areas = detect_areas(anthropic_key, issue, fetch_area_labels())

    # search for potential duplicates and related closed issues
    all_magnets = parse_duplicate_magnets()
    relevant_magnets = filter_magnets_by_areas(all_magnets, detected_areas)
    search_results = search_for_similar_issues(issue, detected_areas)

    # analyze candidates
    likely_duplicates, possible_duplicates, related_closed_issues = analyze_duplicates(
        anthropic_key, issue, relevant_magnets, search_results
    )

    # second-pass critique: prompt iteration on the proposer hit a ceiling around 30% noise.
    # Re-evaluate each proposed closed candidate in isolation with a stricter prompt that
    # has no slate to fill and is blind to the proposer's rationale.
    related_closed_issues = critique_closed_candidates(
        anthropic_key, issue, related_closed_issues, search_results
    )

    # resolve close reason from our search results (the source of truth) so we don't depend
    # on Claude to faithfully echo it back
    results_by_number = {r["number"]: r for r in search_results}
    for m in related_closed_issues:
        m["state_reason"] = results_by_number[m["number"]]["state_reason"]

    comment_body = build_comment(likely_duplicates, possible_duplicates, related_closed_issues)
    commented = False

    if comment_body:
        if args.dry_run:
            log("Dry run - would post comment:\n" + "-" * 40 + "\n" + comment_body + "\n" + "-" * 40)
        else:
            log("Posting comment")
            try:
                post_comment(issue["number"], comment_body)
                commented = True
            except requests.RequestException as e:
                log(f"  Failed to post comment: {e}")
                log(f"  Comment we were trying to post:\n{comment_body}")
                sys.exit(1)

    print(json.dumps({
        "skipped": False,
        "issue": {
            "number": issue["number"],
            "title": issue["title"],
            "author": issue["author"],
            "type": issue["type"],
        },
        "detected_areas": detected_areas,
        "magnets_count": len(relevant_magnets),
        "search_results_count": len(search_results),
        "likely_duplicates": likely_duplicates,
        "possible_duplicates": possible_duplicates,
        "related_closed_issues": related_closed_issues,
        "commented": commented,
    }))
