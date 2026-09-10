#!/usr/bin/env python3
"""
Comment on newly opened issues with possible duplicates and triage hints.

This script is run by a GitHub Actions workflow when a new issue is opened. It:
1. Checks eligibility (bug/crash type or untyped, non-staff author)
2. Detects relevant areas using Claude + the area label taxonomy
3. Parses known "duplicate magnets" from tracking issue #46355
4. Searches for similar issues — open (including long-lived, recently active issues) and recently
   closed (last 90 days) — and Discussions (feature requests / open-ended topics)
5. Asks Claude to sort open candidates into likely and possible duplicates, surface
   recently closed issues that may be useful triage context, and flag discussions the
   issue may duplicate
6. Posts a comment if anything is found: a user-facing alert for likely duplicate issues
   or discussions, and/or a collapsed triager-facing section for possible duplicates,
   recently closed related issues, and possibly-related discussions

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
from github_helpers import github_graphql, github_rest_api, post_github_comment

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
TRACKING_ISSUE_NUMBER = 46355
STAFF_TEAM_SLUG = "staff"
CLAUDE_MODEL = "claude-sonnet-4-6"

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


def github_search_issues(query, per_page=50, sort=None):
    """Search issues, using GitHub's relevance ordering unless a sort is specified."""
    params = {"q": query, "per_page": per_page}
    if sort:
        params.update({"sort": sort, "order": "desc"})
    return github_rest_api("GET", "search/issues", params=params).get("items", [])


def check_team_membership(org, team_slug, username):
    """Check if user is an active member of a team."""
    try:
        data = github_rest_api(
            "GET", f"orgs/{org}/teams/{team_slug}/memberships/{username}"
        )
        return data.get("state") == "active"
    except requests.HTTPError as e:
        if e.response.status_code == 404:
            return False
        raise


def format_candidate_reference(match):
    candidate = match["candidate"]
    if candidate["kind"] == "discussion":
        return f"[Discussion #{candidate['number']}]({candidate['url']})"
    return f"#{candidate['number']}"


def build_comment(likely_matches, possible_matches, related_closed_candidates):
    """Compose the full comment body. Returns empty string if there's nothing to post."""
    sections = []
    likely_issues = [m for m in likely_matches if m["candidate"]["kind"] == "issue"]
    likely_discussions = [m for m in likely_matches if m["candidate"]["kind"] == "discussion"]

    if likely_issues:
        match_list = "\n".join(f"- {format_candidate_reference(m)}" for m in likely_issues)
        explanations = "\n\n".join(
            f"**{format_candidate_reference(m)}:** {m['explanation']}\n\n"
            f"**Shared root cause:** {m['shared_root_cause']}"
            for m in likely_issues
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

    if likely_discussions:
        match_list = "\n".join(f"- {format_candidate_reference(m)}" for m in likely_discussions)
        explanations = "\n\n".join(
            f"**{format_candidate_reference(m)}:** {m['explanation']}" for m in likely_discussions
        )
        sections.append(f"""This looks like it may already be covered by an existing discussion:

{match_list}

Zed tracks feature requests and open-ended topics in Discussions rather than Issues. **If your report is covered there, please close this issue as a duplicate (select "Close as not planned" → "Duplicate") and continue in the discussion** so the conversation stays in one place.

<details>
<summary>Why were these selected?</summary>

{explanations}

</details>""")

    possible_issues = [m for m in possible_matches if m["candidate"]["kind"] == "issue"]
    possible_discussions = [m for m in possible_matches if m["candidate"]["kind"] == "discussion"]
    if possible_matches or related_closed_candidates:
        parts = []
        if possible_issues:
            lines = [
                f"- {format_candidate_reference(m)} — {m['explanation']}\n"
                f"  - Possible shared root cause: {m['shared_root_cause']}"
                for m in possible_issues
            ]
            parts.append("**Possibly related open issues:**\n\n" + "\n".join(lines))
        if related_closed_candidates:
            lines = [
                f"- {format_candidate_reference(m)}"
                f"{' (closed as duplicate)' if m['candidate'].get('state_reason') == 'duplicate' else ''}"
                f" — {m['explanation']}"
                for m in related_closed_candidates
            ]
            parts.append("**Recently closed, possibly the same bug:**\n\n" + "\n".join(lines))
        if possible_discussions:
            lines = [
                f"- {format_candidate_reference(m)} — {m['explanation']}"
                for m in possible_discussions
            ]
            parts.append("**Possibly related discussions:**\n\n" + "\n".join(lines))
        body = "\n\n".join(parts)
        sections.append(f"""<details>
<summary>Additional recent context for triagers</summary>

{body}

</details>""")

    if not sections:
        return ""

    sections.append("---\n<sub>This is an automated analysis and might be incorrect.</sub>")
    return "\n\n".join(sections)


def _claude_request(api_key, payload):
    """POST to the Claude Messages API, raise on non-2xx, log token usage, return parsed data."""
    response = requests.post(
        "https://api.anthropic.com/v1/messages",
        headers={
            "x-api-key": api_key,
            "anthropic-version": "2023-06-01",
            "content-type": "application/json",
        },
        json={"model": CLAUDE_MODEL, "temperature": 0.0, **payload},
    )
    response.raise_for_status()
    data = response.json()

    usage = data.get("usage", {})
    log(f"  Token usage - Input: {usage.get('input_tokens', 'N/A')}, Output: {usage.get('output_tokens', 'N/A')}")
    return data


def call_claude(api_key, system_prompt, user_content, max_tokens=1024):
    """Send a message to Claude and return the text response. Raises on non-2xx status."""
    data = _claude_request(api_key, {
        "max_tokens": max_tokens,
        "system": system_prompt,
        "messages": [{"role": "user", "content": user_content}],
    })

    content = data.get("content", [])
    if content and content[0].get("type") == "text":
        return content[0].get("text") or ""
    return ""


def call_claude_tool(api_key, system_prompt, user_content, tool, max_tokens=1024):
    """Call Claude, forcing it to invoke `tool`, and return the structured input dict.

    Forcing a tool call makes the API emit schema-shaped JSON via its tool-use mechanism
    instead of free-form text we'd have to parse out of prose or markdown fences. Raises on
    non-2xx status, or if no tool_use block is returned.
    """
    data = _claude_request(api_key, {
        "max_tokens": max_tokens,
        "system": system_prompt,
        "messages": [{"role": "user", "content": user_content}],
        "tools": [tool],
        "tool_choice": {"type": "tool", "name": tool["name"]},
    })

    if data.get("stop_reason") == "max_tokens":
        log("  Warning: response hit max_tokens; structured output may be truncated")

    for block in data.get("content", []):
        if block.get("type") == "tool_use":
            return block.get("input") or {}
    raise ValueError(f"Claude returned no tool_use block for tool '{tool['name']}'")


def fetch_issue(issue_number: int):
    """Fetch issue from GitHub and return as a dict."""
    log(f"Fetching issue #{issue_number}")

    issue_data = github_rest_api(
        "GET", f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}"
    )
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
    while page_labels := github_rest_api(
        "GET",
        f"repos/{REPO_OWNER}/{REPO_NAME}/labels",
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


SEARCH_QUERY_TOOL = {
    "name": "report_search_queries",
    "description": "Report concise GitHub search queries for finding duplicate reports.",
    "input_schema": {
        "type": "object",
        "properties": {
            "queries": {
                "type": "array",
                "items": {"type": "string"},
                "minItems": 1,
                "maxItems": 3,
            },
        },
        "required": ["queries"],
    },
}


def generate_search_queries(anthropic_key, issue):
    """Use Claude to derive a few concise searches from the issue's title and body."""
    log("Generating search queries with Claude")
    system_prompt = """Generate 1-3 concise GitHub search queries for finding earlier reports of the
same underlying bug or request. Each query should contain 2-5 terms that are likely to appear in a
canonical issue's title or body. Use the report body as well as its title, include distinctive terms
such as error codes or requested mechanisms, and vary vocabulary when useful. Do not include GitHub
qualifiers such as repo:, is:, label:, or in:, and do not wrap terms in quotes."""
    user_content = f"""# Issue Title
{issue['title']}

# Issue Body
{issue['body'][:4000]}"""
    try:
        response = call_claude_tool(
            anthropic_key,
            system_prompt,
            user_content,
            SEARCH_QUERY_TOOL,
            max_tokens=300,
        )
    except (requests.RequestException, ValueError) as error:
        log(f"  Search query generation failed ({error}); falling back to title and area searches")
        return []

    queries = []
    for query in response.get("queries", []):
        if not isinstance(query, str) or re.search(
            r"\b(?:repo|is|label|state|created|updated|closed|in):", query, re.IGNORECASE
        ):
            continue
        terms = re.findall(r"[A-Za-z0-9][A-Za-z0-9_./+#'-]*", query)
        normalized = " ".join(terms[:5])
        if normalized and normalized not in queries:
            queries.append(normalized)

    log(f"  Generated search queries: {queries}")
    return queries


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

    issue_data = github_rest_api(
        "GET", f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{TRACKING_ISSUE_NUMBER}"
    )
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
    """Fetch details for magnets and normalize them as candidates."""
    log(f"  Fetching details for {len(magnets)} magnets")
    for magnet in magnets:
        data = github_rest_api(
            "GET", f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{magnet['number']}"
        )
        magnet.update({
            "key": f"issue:{magnet['number']}",
            "kind": "issue",
            "title": data["title"],
            "url": data["html_url"],
            "state": data["state"],
            "state_reason": data.get("state_reason"),
            "body_preview": (data.get("body") or "")[:3000],
            "source": "known_duplicate_magnet",
        })


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


def filter_author_referenced_candidates(issue, candidates):
    text = f"{issue['title']}\n{issue['body']}"
    discussion_shorthand_pattern = r"\bdiscussion\s+#(\d+)\b"
    discussion_numbers = re.findall(discussion_shorthand_pattern, text, re.IGNORECASE)
    text_without_discussion_shorthand = re.sub(
        discussion_shorthand_pattern, "", text, flags=re.IGNORECASE
    )
    referenced_keys = {
        f"issue:{number}" for number in re.findall(r"#(\d+)\b", text_without_discussion_shorthand)
    }
    referenced_keys.update(f"discussion:{number}" for number in discussion_numbers)

    resource_pattern = rf"https?://github\.com/{REPO_OWNER}/{REPO_NAME}/(issues|discussions)/(\d+)\b"
    for resource, number in re.findall(resource_pattern, text, re.IGNORECASE):
        kind = "issue" if resource.lower() == "issues" else "discussion"
        referenced_keys.add(f"{kind}:{number}")

    omitted = [candidate["key"] for candidate in candidates if candidate["key"] in referenced_keys]
    if omitted:
        log(f"  Omitted candidates already referenced by the author: {omitted}")
    return [candidate for candidate in candidates if candidate["key"] not in referenced_keys]


def rank_search_candidates(candidates):
    def rank(candidate):
        matched_searches = candidate["matched_searches"]
        return (
            len(matched_searches) > 1,
            "semantic_query" in matched_searches,
            "title_keywords" in matched_searches,
            "error_pattern" in matched_searches,
            "popular_area" in matched_searches,
            len(matched_searches),
            -candidate.get("best_match_rank", 1000),
            candidate.get("updated_at", ""),
        )

    return sorted(candidates, key=rank, reverse=True)


def select_search_candidates(candidates, limit):
    """Select candidates while preserving capacity for each retrieval channel."""
    selected = []
    selected_keys = set()
    reserved = (
        ("semantic_query", max(2, limit // 3)),
        ("error_pattern", max(1, limit // 10)),
        ("title_keywords", max(1, limit // 6)),
        ("popular_area", max(1, limit // 5)),
        ("area_label", max(1, limit // 6)),
    )

    def add(candidate):
        if candidate["key"] not in selected_keys and len(selected) < limit:
            selected.append(candidate)
            selected_keys.add(candidate["key"])

    for search_type, quota in reserved:
        matching = [candidate for candidate in candidates if search_type in candidate["matched_searches"]]
        for candidate in matching[:quota]:
            add(candidate)

    for candidate in candidates:
        add(candidate)

    return selected


def extract_error_snippet(body):
    match = re.search(
        r"(?i:(?:\berror\b|\bfailed\b)[ \t]*:[ \t]+|\bpanicked at[ \t]+)([^\r\n]{5,90})",
        body,
    )
    if not match:
        return None
    snippet = match.group(1).strip()
    if snippet.startswith(("#", "<", "```")):
        return None
    return snippet


def search_for_similar_issues(issue, detected_areas, search_queries, max_searches_per_state=12):
    """Search for similar open issues and issues closed within the last 90 days."""
    log("Searching for similar issues")

    ninety_days_ago = (datetime.now() - timedelta(days=90)).strftime("%Y-%m-%d")

    title_keywords = [word for word in issue["title"].split() if word.lower() not in STOPWORDS and len(word) > 2]
    keywords_query = " ".join(title_keywords) if title_keywords else None

    error_snippet = extract_error_snippet(issue["body"])

    def build_queries(base):
        queries = [("semantic_query", f"{base} {query}") for query in search_queries]
        if keywords_query and keywords_query not in search_queries:
            queries.append(("title_keywords", f"{base} {keywords_query}"))
        if error_snippet:
            queries.append(("error_pattern", f'{base} in:body "{error_snippet}"'))
        queries.extend(("area_label", f'{base} label:"area:{area}"') for area in detected_areas)
        return queries

    open_queries = build_queries(f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open")
    open_queries.extend(
        ("popular_area", f'repo:{REPO_OWNER}/{REPO_NAME} is:issue is:open label:"area:{area}"')
        for area in detected_areas
    )
    # closed pass: filter by close date so we catch issues closed recently regardless of
    # when they were opened. closed:> already restricts the result set, so the per-query
    # area window is unnecessary.
    closed_queries = build_queries(
        f"repo:{REPO_OWNER}/{REPO_NAME} is:issue is:closed closed:>{ninety_days_ago}"
    )

    seen_issues = {}
    for state_label, queries in (
        ("open", open_queries[:max_searches_per_state]),
        ("closed", closed_queries[:max_searches_per_state]),
    ):
        for search_type, query in queries:
            log(f"  Search ({state_label} / {search_type}): {query}")
            try:
                if search_type == "popular_area":
                    sort = "reactions"
                elif state_label == "open" and search_type == "area_label":
                    sort = "updated"
                else:
                    sort = None
                results = github_search_issues(query, per_page=50, sort=sort)
                for result_rank, item in enumerate(results):
                    number = item["number"]
                    if number == issue["number"]:
                        continue
                    existing = seen_issues.get(number)
                    if existing:
                        if search_type not in existing["matched_searches"]:
                            existing["matched_searches"].append(search_type)
                        if search_type not in ("area_label", "popular_area"):
                            existing["best_match_rank"] = min(existing["best_match_rank"], result_rank)
                        continue
                    body = item.get("body") or ""
                    seen_issues[number] = {
                        "key": f"issue:{number}",
                        "kind": "issue",
                        "number": number,
                        "title": item["title"],
                        "url": item["html_url"],
                        "state": item.get("state", ""),
                        "state_reason": item.get("state_reason"),
                        "created_at": item.get("created_at", ""),
                        "updated_at": item.get("updated_at", ""),
                        "body_preview": body[:3000],
                        "source": "issue_search",
                        "matched_searches": [search_type],
                        "best_match_rank": (
                            result_rank if search_type not in ("area_label", "popular_area") else 1000
                        ),
                    }
            except requests.RequestException as e:
                log(f"  Search failed: {e}")

    similar_issues = rank_search_candidates(seen_issues.values())
    log(f"  Found {len(similar_issues)} similar issues")
    return similar_issues


def enrich_popular_candidate_comments(candidates):
    for candidate in candidates:
        if (
            candidate["kind"] != "issue"
            or "popular_area" not in candidate.get("matched_searches", [])
        ):
            continue
        try:
            comments = github_rest_api(
                "GET",
                f"repos/{REPO_OWNER}/{REPO_NAME}/issues/{candidate['number']}/comments",
                params={"per_page": 100},
            )
        except requests.RequestException as error:
            log(f"  Failed to fetch comments for {candidate['key']}: {error}")
            continue
        bodies = [comment.get("body") or "" for comment in comments]
        candidate["recent_comments_preview"] = "\n\n---\n\n".join(filter(None, bodies[-5:]))[-3000:]


def search_discussions(issue, detected_areas, search_queries, max_searches=6):
    """Search Discussions for a topic/request the new issue may duplicate.

    Discussions are not in the REST search API, so this uses GraphQL search(type: DISCUSSION).
    Zed tracks feature requests and open-ended topics as Discussions rather than Issues, so a
    new issue that re-files an existing discussion should be closed by its author in favor of
    the discussion.
    """
    log("Searching discussions")
    title_keywords = [w for w in issue["title"].split() if w.lower() not in STOPWORDS and len(w) > 2]
    keywords_query = " ".join(title_keywords) if title_keywords else None
    if not search_queries and not keywords_query:
        return []

    base = f"repo:{REPO_OWNER}/{REPO_NAME} is:open"
    queries = [("semantic_query", f"{base} {query}") for query in search_queries]
    if keywords_query and keywords_query not in search_queries:
        queries.append(("title_keywords", f"{base} {keywords_query}"))
    if keywords_query:
        for area in detected_areas:
            queries.append(("area_label", f'{base} {keywords_query} label:"area:{area}"'))

    gql = """
    query($q: String!) {
      search(query: $q, type: DISCUSSION, first: 30) {
        nodes {
          ... on Discussion {
            number
            title
            url
            bodyText
            category { name }
          }
        }
      }
    }
    """
    seen = {}
    for search_type, query in queries[:max_searches]:
        log(f"  Discussion search ({search_type}): {query}")
        try:
            data = github_graphql(gql, {"q": query})
            for result_rank, node in enumerate(data["search"]["nodes"]):
                if not node:
                    continue
                number = node["number"]
                existing = seen.get(number)
                if existing:
                    if search_type not in existing["matched_searches"]:
                        existing["matched_searches"].append(search_type)
                    existing["best_match_rank"] = min(existing["best_match_rank"], result_rank)
                    continue
                body = node.get("bodyText") or ""
                seen[number] = {
                    "key": f"discussion:{number}",
                    "kind": "discussion",
                    "number": number,
                    "title": node["title"],
                    "url": node["url"],
                    "state": "open",
                    "state_reason": None,
                    "category": (node.get("category") or {}).get("name"),
                    "body_preview": body[:3000],
                    "source": "discussion_search",
                    "matched_searches": [search_type],
                    "best_match_rank": result_rank,
                }
        except (requests.RequestException, ValueError, KeyError, TypeError) as e:
            log(f"  Discussion search failed: {e}")
    discussions = rank_search_candidates(seen.values())
    log(f"  Found {len(discussions)} candidate discussions")
    return discussions


def analyze_duplicates(anthropic_key, issue, candidates):
    """Use Claude to identify likely, possible, and related closed candidates."""
    magnets = [candidate for candidate in candidates if candidate["source"] == "known_duplicate_magnet"]
    magnet_keys = {candidate["key"] for candidate in magnets}
    open_issues = [
        candidate for candidate in candidates
        if candidate["kind"] == "issue" and candidate["state"] == "open"
        and candidate["key"] not in magnet_keys
    ]
    closed_issues = [
        candidate for candidate in candidates
        if candidate["kind"] == "issue" and candidate["state"] == "closed"
        and candidate["key"] not in magnet_keys
    ]
    open_discussions = [
        candidate for candidate in candidates
        if candidate["kind"] == "discussion" and candidate["state"] == "open"
    ]

    selected_candidates = (
        magnets[:10]
        + select_search_candidates(open_issues, 30)
        + select_search_candidates(closed_issues, 10)
        + open_discussions[:10]
    )
    if not selected_candidates:
        return {"likely_matches": [], "possible_matches": [], "related_closed_candidates": []}

    enrich_popular_candidate_comments(selected_candidates)

    log("Analyzing candidates with Claude")
    log(
        f"  Candidate pool: {len(magnets)} magnets, {len(open_issues)} open issues, "
        f"{len(closed_issues)} closed issues, {len(open_discussions)} open discussions"
    )
    log(f"  Candidates given to proposer: {[candidate['key'] for candidate in selected_candidates]}")

    system_prompt = """You analyze a new GitHub issue against candidates that may be issues or discussions.

Each candidate has a unique "key", a "kind" ("issue" or "discussion"), and a "state"
("open" or "closed"). Closed issues carry a "state_reason" ("completed", "not_planned",
or "duplicate"). Always identify a candidate using its full key.

# (a) Duplicate issues — OPEN issue candidates only

For an issue candidate, a duplicate means: caused by the SAME BUG in the code, not just similar symptoms.

CRITICAL DISTINCTION — shared symptoms vs shared root cause:
- "models missing", "can't sign in", "editor hangs", "venv not detected" are SYMPTOMS that many
  different bugs can produce. Two reports of the same symptom are NOT duplicates unless you can
  identify a specific shared root cause.
- A duplicate means: if a developer fixed the existing issue, the new issue would also be fixed.
- If the issues just happen to be in the same feature area, or describe similar-sounding problems
  with different specifics (different error messages, different triggers, different platforms,
  different configurations), they are NOT duplicates.
- Compare the causal mechanism, not only the surface observations. Differing observed effects do
  not rule out a duplicate when the provided evidence supports a specific shared trigger and
  mechanism, but do create uncertainty about the match.

Sort matches into two buckets:
- "likely_matches": Almost certainly the same bug. You can name a specific shared root cause, and
  the reproduction steps / error messages / triggers are consistent.
- "possible_matches": Plausibly the same bug because concrete evidence supports a shared trigger
  or mechanism, but incomplete evidence or differing observed effects leave meaningful uncertainty.
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

# (c) Duplicate of a discussion — OPEN discussion candidates only

Zed tracks feature requests and open-ended proposals as Discussions, not Issues. If the
new issue is essentially the SAME request or topic as a discussion candidate, its author
should close the issue and continue in the discussion.

Put a discussion in "likely_matches" when it is clearly the same request/topic, or in
"possible_matches" when it is plausibly the same request/topic but some uncertainty remains.
Do not provide a shared_root_cause for discussion matches.

The test here is "same underlying request/topic", NOT "same code bug". Do not match on
shared area alone. The same false-positives-are-worse asymmetry applies: when in doubt, omit.

# Output

Report your verdict by calling the report_duplicate_analysis tool. Fill the "reasoning"
field first with a brief scratchpad weighing the strongest candidates and whether they
share a root cause, then fill each bucket. Use empty arrays where nothing relevant is
found."""

    user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:6000]}

## Candidates to Compare
{json.dumps(selected_candidates, indent=2)}"""

    match_schema = {
        "type": "object",
        "properties": {
            "candidate_key": {"type": "string", "description": "The candidate's full key"},
            "shared_root_cause": {
                "type": "string",
                "description": "The specific shared bug/root cause. Include for issue matches only.",
            },
            "explanation": {
                "type": "string",
                "description": "Brief explanation with concrete evidence from the new issue and candidate",
            },
        },
        "required": ["candidate_key", "explanation"],
    }
    related_closed_schema = {
        "type": "object",
        "properties": {
            "candidate_key": {"type": "string", "description": "The candidate's full key"},
            "explanation": {"type": "string", "description": "Why this is useful triage context"},
        },
        "required": ["candidate_key", "explanation"],
    }
    analysis_tool = {
        "name": "report_duplicate_analysis",
        "description": "Report the duplicate analysis for the new issue.",
        "input_schema": {
            "type": "object",
            "properties": {
                "reasoning": {
                    "type": "string",
                    "description": "A brief scratchpad (at most 2-3 sentences) weighing the strongest candidates.",
                    "maxLength": 700,
                },
                "likely_matches": {"type": "array", "items": match_schema},
                "possible_matches": {"type": "array", "items": match_schema},
                "related_closed_candidates": {"type": "array", "items": related_closed_schema},
            },
            "required": ["reasoning", "likely_matches", "possible_matches", "related_closed_candidates"],
        },
    }

    data = call_claude_tool(anthropic_key, system_prompt, user_content, analysis_tool, max_tokens=3072)
    if data.get("reasoning"):
        log(f"  Reasoning: {data['reasoning']}")

    candidates_by_key = {candidate["key"]: candidate for candidate in selected_candidates}

    def resolve_matches(matches, expected_state, label):
        resolved = []
        seen = set()
        dropped = []
        for match in matches:
            key = match.get("candidate_key")
            candidate = candidates_by_key.get(key)
            if candidate is None or candidate["state"] != expected_state or key in seen:
                dropped.append(key)
                continue
            if candidate["kind"] == "issue" and expected_state == "open" and not match.get("shared_root_cause"):
                dropped.append(key)
                continue
            seen.add(key)
            resolved.append({**match, "candidate": candidate})
        if dropped:
            log(f"  Dropped {len(dropped)} invalid matches from {label}: {dropped}")
        return resolved

    likely = resolve_matches(data.get("likely_matches", []), "open", "likely_matches")
    possible = resolve_matches(data.get("possible_matches", []), "open", "possible_matches")
    related_closed = resolve_matches(
        data.get("related_closed_candidates", []), "closed", "related_closed_candidates"
    )

    likely_keys = {match["candidate_key"] for match in likely}
    overlap = [match["candidate_key"] for match in possible if match["candidate_key"] in likely_keys]
    if overlap:
        log(f"  Dropped {len(overlap)} possible matches already in likely matches: {overlap}")
    possible = [match for match in possible if match["candidate_key"] not in likely_keys]

    log(f"  Found {len(likely)} likely, {len(possible)} possible, and "
        f"{len(related_closed)} related closed matches")
    return {
        "likely_matches": likely,
        "possible_matches": possible,
        "related_closed_candidates": related_closed,
    }


PROPOSED_MATCH_CRITIQUE_SYSTEM_PROMPT = """You evaluate one proposed match for a new GitHub issue.

The proposer classified the candidate as likely or possible and supplied an explanation.
For issue candidates it also supplied a shared root cause. Verify the proposal against the
actual text of both reports. False positives are much worse than false negatives.

For an issue candidate, keep the match only when both reports plausibly describe the SAME
BUG. Shared symptoms, product area, or terminology are insufficient. The proposed shared
root cause and every concrete claim in the explanation must be supported by the provided
text. Omit matches that rely on invented mechanisms, contradictory triggers, different
errors, configurations, or platforms without evidence tying them together. Judge the proposed
causal mechanism rather than requiring every surface observation to be identical; any claimed
bridge between differing observations must itself be supported by the provided text.

Some reports framed as bugs are actually requests for behavior Zed does not support. Zed
tracks feature requests and open-ended proposals in Discussions. For a discussion candidate,
keep the match when the report's desired behavior is substantially the SAME request or topic.
A shared area or superficially similar wording is insufficient.

Verdicts:
- "keep": the proposed confidence and justification are supported as written.
- "downgrade": only for a proposed likely match whose relationship is plausible but not
  strong enough to show as likely. The proposed justification must still be supported.
- "omit": the relationship or its justification is not sufficiently supported. Use this
  for a proposed possible match that does not meet the bar; never downgrade a possible match.

Report the verdict with one concise rationale grounded in the provided text."""


PROPOSED_MATCH_CRITIQUE_VERDICT_TOOL = {
    "name": "report_proposed_match_critique_verdict",
    "description": "Report whether the proposed candidate match is supported.",
    "input_schema": {
        "type": "object",
        "properties": {
            "rationale": {
                "type": "string",
                "description": "One concise sentence explaining whether the proposed match is supported.",
                "maxLength": 400,
            },
            "verdict": {"type": "string", "enum": ["keep", "downgrade", "omit"]},
        },
        "required": ["rationale", "verdict"],
    },
}


def critique_proposed_matches(anthropic_key, issue, likely_matches, possible_matches):
    proposed = [("likely", match) for match in likely_matches]
    proposed.extend(("possible", match) for match in possible_matches)
    if not proposed:
        log("  Match critique: proposer surfaced 0 candidates; skipping")
        return {"likely_matches": [], "possible_matches": []}

    kept_likely = []
    kept_possible = []
    for confidence, match in proposed:
        candidate = match["candidate"]
        key = candidate["key"]
        shared_root_cause = match.get("shared_root_cause") or "Not applicable"
        user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:6000]}

## Candidate {key}
**Kind:** {candidate['kind']}
**Title:** {candidate['title']}

**Body preview:**
{candidate['body_preview']}

**Recent comments:**
{candidate.get('recent_comments_preview') or 'None provided'}

## Proposed Match
**Confidence:** {confidence}
**Shared root cause:** {shared_root_cause}
**Explanation:** {match['explanation']}"""

        log(f"  Match critique: evaluating {confidence} match {key}")
        try:
            verdict_data = call_claude_tool(
                anthropic_key,
                PROPOSED_MATCH_CRITIQUE_SYSTEM_PROMPT,
                user_content,
                PROPOSED_MATCH_CRITIQUE_VERDICT_TOOL,
                max_tokens=600,
            )
        except (requests.RequestException, ValueError) as e:
            log(f"  Match critique: verdict call failed for {key} ({e}); omitting candidate")
            continue

        verdict = verdict_data.get("verdict")
        rationale = verdict_data.get("rationale", "")
        if verdict == "keep":
            destination = kept_likely if confidence == "likely" else kept_possible
            destination.append(match)
            log(f"  Match critique: keeping {confidence} match {key} — {rationale}")
        elif verdict == "downgrade" and confidence == "likely":
            kept_possible.append(match)
            log(f"  Match critique: downgrading {key} to possible — {rationale}")
        else:
            log(f"  Match critique: omitting {key} — {rationale}")

    log(f"  Match critique: kept {len(kept_likely)} likely and {len(kept_possible)} possible matches")
    return {"likely_matches": kept_likely, "possible_matches": kept_possible}


RELATED_CLOSED_CANDIDATE_CRITIQUE_SYSTEM_PROMPT = """You are evaluating ONE recently closed GitHub issue to decide whether a triager looking
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

Report your decision by calling the report_critique_verdict tool. Fill "rationale" first
(one concise sentence), then "verdict". When "verdict" is "include", "rule_violated" must be
null. When "verdict" is "omit", set "rule_violated" to the most relevant rule number, or
null if the candidate is simply too unrelated for any rule to specifically apply."""


RELATED_CLOSED_CANDIDATE_CRITIQUE_VERDICT_TOOL = {
    "name": "report_critique_verdict",
    "description": "Report whether the closed candidate is worth surfacing to a triager.",
    "input_schema": {
        "type": "object",
        "properties": {
            "rationale": {
                "type": "string",
                "description": "One concise sentence justifying the verdict, grounded in the candidate's actual text.",
                "maxLength": 400,
            },
            "verdict": {"type": "string", "enum": ["include", "omit"]},
            "rule_violated": {
                "type": ["integer", "null"],
                "description": "The most relevant omit-rule number (1-7), or null when including.",
            },
        },
        "required": ["rationale", "verdict"],
    },
}


def critique_related_closed_candidates(anthropic_key, issue, proposed):
    """Run a strict per-candidate critique pass over the proposer's closed candidates."""
    if not proposed:
        log("  Related candidate critique: proposer surfaced 0 candidates; skipping")
        return []

    log(f"  Related candidate critique: proposer surfaced {len(proposed)} candidate(s): "
        f"{[m['candidate_key'] for m in proposed]}")

    kept = []
    for match in proposed:
        candidate = match["candidate"]
        key = candidate["key"]
        state_reason = candidate.get("state_reason") or "unknown"
        user_content = f"""## New Issue #{issue['number']}
**Title:** {issue['title']}

**Body:**
{issue['body'][:6000]}

## Closed Candidate {key}
**Title:** {candidate.get('title', '')}
**State reason:** {state_reason}

**Body preview:**
{candidate.get('body_preview', '')}"""

        log(f"  Related candidate critique: evaluating {key}")
        try:
            verdict_data = call_claude_tool(
                anthropic_key,
                RELATED_CLOSED_CANDIDATE_CRITIQUE_SYSTEM_PROMPT,
                user_content,
                RELATED_CLOSED_CANDIDATE_CRITIQUE_VERDICT_TOOL,
                max_tokens=600,
            )
        except (requests.RequestException, ValueError) as e:
            log(f"  Related candidate critique: verdict call failed for {key} ({e}); omitting candidate")
            continue

        verdict = verdict_data.get("verdict")
        rule = verdict_data.get("rule_violated")
        rationale = verdict_data.get("rationale", "")

        if verdict == "include":
            log(f"  Related candidate critique: keeping {key} — {rationale}")
            kept.append(match)
        else:
            rule_str = f"rule {rule}" if rule else "no specific rule"
            log(f"  Related candidate critique: omitting {key} ({rule_str}) — {rationale}")

    log(f"  Related candidate critique: kept {len(kept)} of {len(proposed)} candidates")
    return kept


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Identify potential duplicate issues")
    parser.add_argument("issue_number", type=int, help="Issue number to analyze")
    parser.add_argument("--dry-run", action="store_true", help="Skip posting comment, just log what would be posted")
    args = parser.parse_args()

    anthropic_key = os.environ.get("ANTHROPIC_API_KEY")

    if not anthropic_key:
        log("Error: ANTHROPIC_API_KEY not set")
        sys.exit(1)

    issue = fetch_issue(args.issue_number)
    if should_skip(issue):
        print(json.dumps({"skipped": True}))
        sys.exit(0)

    # detect areas
    detected_areas = detect_areas(anthropic_key, issue, fetch_area_labels())

    # search for potential duplicates and related closed issues
    search_queries = generate_search_queries(anthropic_key, issue)
    all_magnets = parse_duplicate_magnets()
    relevant_magnets = filter_magnets_by_areas(all_magnets, detected_areas)
    magnet_candidates = relevant_magnets[:10]
    enrich_magnets(magnet_candidates)
    search_results = search_for_similar_issues(issue, detected_areas, search_queries)
    discussion_results = search_discussions(issue, detected_areas, search_queries)
    candidates = magnet_candidates + search_results + discussion_results
    candidates = filter_author_referenced_candidates(issue, candidates)

    analysis = analyze_duplicates(anthropic_key, issue, candidates)
    critiqued_matches = critique_proposed_matches(
        anthropic_key, issue, analysis["likely_matches"], analysis["possible_matches"]
    )
    likely_matches = critiqued_matches["likely_matches"]
    possible_matches = critiqued_matches["possible_matches"]
    related_closed_candidates = critique_related_closed_candidates(
        anthropic_key, issue, analysis["related_closed_candidates"]
    )

    comment_body = build_comment(likely_matches, possible_matches, related_closed_candidates)
    commented = False

    if comment_body:
        if args.dry_run:
            log("Dry run - would post comment:\n" + "-" * 40 + "\n" + comment_body + "\n" + "-" * 40)
        else:
            log("Posting comment")
            try:
                post_github_comment(
                    REPO_OWNER, REPO_NAME, issue["number"], comment_body
                )
                log(f"  Posted comment on #{issue['number']}")
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
        "likely_matches": likely_matches,
        "possible_matches": possible_matches,
        "related_closed_candidates": related_closed_candidates,
        "discussion_results_count": len(discussion_results),
        "commented": commented,
    }))
