#!/usr/bin/env python3
"""
Comment on newly opened issues that might be duplicates of an existing issue.

This script is run by a GitHub Actions workflow when a new bug or crash report
is opened. It:
1. Checks eligibility (must be bug/crash type, non-staff author)
2. Detects relevant areas using Claude + the area label taxonomy
3. Parses known "duplicate magnets" from tracking issue #46355
4. Searches for similar recent issues by title keywords, area labels, and error patterns
5. Asks Claude to analyze potential duplicates (magnets + search results)
6. Posts a comment on the issue if high-confidence duplicates are found

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


def log(message):
    """Print to stderr so it doesn't interfere with JSON output on stdout."""
    print(message, file=sys.stderr)


def github_api_get(path, params=None):
    """Fetch JSON from the GitHub API. Raises on non-2xx status."""
    url = f"{GITHUB_API}/{path.lstrip('/')}"
    response = requests.get(url, headers=GITHUB_HEADERS, params=params)
    response.raise_for_status()
    return response.json()


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


def build_duplicate_comment(matches):
    """Build the comment body for potential duplicates."""
    match_list = "\n".join(f"- #{m['number']}" for m in matches)
    explanations = "\n\n".join(
        f"**#{m['number']}:** {m['explanation']}\n\n**Shared root cause:** {m['shared_root_cause']}"
        if m.get('shared_root_cause')
        else f"**#{m['number']}:** {m['explanation']}"
        for m in matches
    )

    return f"""This issue appears to be a duplicate of:

{match_list}

**If this is indeed a duplicate:**
Please close this issue and subscribe to the linked issue for updates (select "Close as not planned" → "Duplicate")

**If this is a different issue:**
No action needed. A maintainer will review this shortly.

<details>
<summary>Why were these issues selected?</summary>

{explanations}

</details>

---
<sub>This is an automated analysis and might be incorrect.</sub>"""


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
    if issue["type"] not in ["Bug", "Crash"]:
        log(f"  Skipping: issue type '{issue['type']}' is not a bug/crash report")
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


def detect_areas(anthropic_key, issue, taxonomy):
    """Use Claude to detect relevant areas for the issue."""
    log("Detecting areas with Claude")

    system_prompt = """You analyze GitHub issues to identify which area labels apply.

Given an issue and a taxonomy of areas, output ONLY a comma-separated list of matching area names.
- Output at most 3 areas, ranked by relevance
- Use exact area names from the taxonomy
- If no areas clearly match, output: none
- For languages/*, tooling/*, or parity/*, use the specific sub-label (e.g., "languages/rust",
tooling/eslint, parity/vscode)

Example outputs:
- "editor, parity/vim"
- "ai, ai/agent panel"
- "none"
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
    return [area.strip() for area in response.split(",")]


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


def search_for_similar_issues(issue, detected_areas, max_searches=6):
    """Search for similar issues that might be duplicates.

    Searches by title keywords, area labels (last 60 days), and error patterns.
    max_searches caps the total number of queries to keep token usage and context size under control.
    """
    log("Searching for similar issues")

    sixty_days_ago = (datetime.now() - timedelta(days=60)).strftime("%Y-%m-%d")
    base_query = f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open"
    seen_issues = {}
    queries = []

    title_keywords = [word for word in issue["title"].split() if word.lower() not in STOPWORDS and len(word) > 2]

    if title_keywords:
        keywords_query = " ".join(title_keywords)
        queries.append(("title_keywords", f"{base_query} {keywords_query}"))

    for area in detected_areas:
        queries.append(("area_label", f'{base_query} label:"area:{area}" created:>{sixty_days_ago}'))

    # error pattern search: capture 5–90 chars after keyword, colon optional
    error_pattern = r"(?i:\b(?:error|panicked|panic|failed)\b)\s*([^\n]{5,90})"
    match = re.search(error_pattern, issue["body"])
    if match:
        error_snippet = match.group(1).strip()
        queries.append(("error_pattern", f'{base_query} in:body "{error_snippet}"'))

    for search_type, query in queries[:max_searches]:
        log(f"  Search ({search_type}): {query}")
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
    """Use Claude to analyze potential duplicates."""
    log("Analyzing duplicates with Claude")

    top_magnets = magnets[:10]
    enrich_magnets(top_magnets)
    magnet_numbers = {m["number"] for m in top_magnets}

    candidates = [
        {"number": m["number"], "title": m["title"], "body_preview": m["body_preview"], "source": "known_duplicate_magnet"}
        for m in top_magnets
    ] + [
        {"number": r["number"], "title": r["title"], "body_preview": r["body_preview"], "source": "search_result"}
        for r in search_results[:10]
        if r["number"] not in magnet_numbers
    ]

    if not candidates:
        return [], "No candidates to analyze"

    system_prompt = """You analyze GitHub issues to identify potential duplicates.

Given a new issue and a list of existing issues, identify which existing issues are duplicates — meaning
they are caused by the SAME BUG in the code, not just similar symptoms.

CRITICAL DISTINCTION — shared symptoms vs shared root cause:
- "models missing", "can't sign in", "editor hangs", "venv not detected" are SYMPTOMS that many
  different bugs can produce. Two reports of the same symptom are NOT duplicates unless you can
  identify a specific shared root cause.
- A duplicate means: if a developer fixed the existing issue, the new issue would also be fixed.
- If the issues just happen to be in the same feature area, or describe similar-sounding problems
  with different specifics (different error messages, different triggers, different platforms, different
  configurations), they are NOT duplicates.

For each potential duplicate, assess confidence:
- "high": Almost certainly the same bug. You can name a specific shared root cause, and the
  reproduction steps / error messages / triggers are consistent.
- "medium": Likely the same bug based on specific technical details, but some uncertainty remains.
- Do NOT include issues that merely share symptoms, affect the same feature area, or sound similar
  at a surface level.

Examples of things that are NOT duplicates:
- Two issues about "Copilot models not showing" — one caused by a Zed update breaking the model list,
  the other caused by the user's plan not including those models.
- Two issues about "Zed hangs" — one triggered by network drives, the other by large projects.
- Two issues about "can't sign in" — one caused by a missing system package, the other by a server-side error.

Output only valid JSON (no markdown code blocks) with this structure:
{
  "matches": [
    {
      "number": 12345,
      "confidence": "high|medium",
      "shared_root_cause": "The specific bug/root cause shared by both issues",
      "explanation": "Brief explanation with concrete evidence from both issues"
    }
  ],
  "summary": "One sentence summary of findings"
}

When in doubt, return an empty matches array. A false positive (flagging a non-duplicate) is much
worse than a false negative (missing a real duplicate), because it wastes the time of both the
issue author and the maintainers.

Return empty matches array if none found or if you can only identify shared symptoms."""

    user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:3000]}

## Existing Issues to Compare
{json.dumps(candidates, indent=2)}"""

    response = call_claude(anthropic_key, system_prompt, user_content, max_tokens=2048)

    try:
        data = json.loads(response)
    except json.JSONDecodeError as e:
        log(f"  Failed to parse response: {e}")
        log(f"  Raw response: {response}")
        return [], "Failed to parse analysis"

    matches = data.get("matches", [])
    summary = data.get("summary", "Analysis complete")
    log(f"  Found {len(matches)} potential matches")
    return matches, summary


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
    taxonomy = format_taxonomy_for_claude(fetch_area_labels())
    detected_areas = detect_areas(anthropic_key, issue, taxonomy)

    # search for potential duplicates
    all_magnets = parse_duplicate_magnets()
    relevant_magnets = filter_magnets_by_areas(all_magnets, detected_areas)
    search_results = search_for_similar_issues(issue, detected_areas)

    # analyze potential duplicates
    if relevant_magnets or search_results:
        matches, summary = analyze_duplicates(anthropic_key, issue, relevant_magnets, search_results)
    else:
        matches, summary = [], "No potential duplicates to analyze"

    # post comment if high-confidence matches found
    high_confidence_matches = [m for m in matches if m["confidence"] == "high"]
    commented = False

    if high_confidence_matches:
        comment_body = build_duplicate_comment(high_confidence_matches)
        if args.dry_run:
            log("Dry run - would post comment:\n" + "-" * 40 + "\n" + comment_body + "\n" + "-" * 40)
        else:
            log("Posting comment for high-confidence match(es)")
            try:
                post_comment(issue["number"], comment_body)
                commented = True
            except requests.RequestException as e:
                log(f"  Failed to post comment: {e}")

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
        "matches": matches,
        "summary": summary,
        "commented": commented,
    }))
