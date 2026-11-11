import os
import sys
import time
from functools import cache

import requests

GITHUB_API_URL = "https://api.github.com"
RETRYABLE_STATUS_CODES = {500, 502, 503, 504}
# GitHub asks to wait at least a minute after a secondary rate limit that
# doesn't say how long.
RATE_LIMIT_DEFAULT_DELAY_SECONDS = 60
MAX_ATTEMPTS = 4
REQUEST_TIMEOUT_SECONDS = 30
NON_COMMUNITY_LABELS = {"staff", "bot"}
PROJECT_FIELD_VALUE_KEYS = {"NUMBER": "number", "DATE": "date", "TEXT": "text"}

_session = requests.Session()


def _github_headers():
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        raise RuntimeError("GITHUB_TOKEN environment variable is required")
    return {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }


def _is_rate_limited(response):
    return response.status_code == 429 or (
        response.status_code == 403
        and (
            "Retry-After" in response.headers
            or response.headers.get("X-RateLimit-Remaining") == "0"
            or "rate limit" in response.text.lower()
        )
    )


def _retry_delay(response, attempt):
    retry_after = response.headers.get("Retry-After")
    if retry_after:
        try:
            return max(float(retry_after), 0)
        except ValueError:
            pass
    reset_at = response.headers.get("X-RateLimit-Reset")
    if response.headers.get("X-RateLimit-Remaining") == "0" and reset_at:
        return max(float(reset_at) - time.time(), 0)
    if _is_rate_limited(response):
        return RATE_LIMIT_DEFAULT_DELAY_SECONDS
    return 2**attempt


def _github_request(method, path, *, params=None, body=None, retry=True):
    url = f"{GITHUB_API_URL}/{path}"
    for attempt in range(MAX_ATTEMPTS):
        is_last_attempt = attempt == MAX_ATTEMPTS - 1
        try:
            response = _session.request(
                method,
                url,
                headers=_github_headers(),
                params=params,
                json=body,
                timeout=REQUEST_TIMEOUT_SECONDS,
            )
        except (requests.ConnectionError, requests.Timeout) as error:
            if not retry or is_last_attempt:
                raise
            delay = 2**attempt
            print(
                f"Transient GitHub API error ({error}); retrying in {delay}s",
                file=sys.stderr,
            )
            time.sleep(delay)
            continue

        # GitHub doesn't carry out a rate-limited request, so even a write is
        # safe to send again.
        if not is_last_attempt and (
            _is_rate_limited(response)
            or (retry and response.status_code in RETRYABLE_STATUS_CODES)
        ):
            delay = _retry_delay(response, attempt)
            print(
                f"GitHub API returned {response.status_code}; retrying in {delay}s",
                file=sys.stderr,
            )
            time.sleep(delay)
            continue

        response.raise_for_status()
        return response

    raise RuntimeError("GitHub API retry loop exited without returning")


def github_rest_api(method, path, params=None, body=None):
    # Retrying writes after an ambiguous timeout can duplicate comments or mutations.
    retry = method.upper() in {"GET", "HEAD", "OPTIONS"}
    response = _github_request(
        method, path, params=params, body=body, retry=retry
    )
    if response.status_code == 204 or not response.content:
        return {}
    return response.json()


def github_rest_get_paginated(path):
    results = []
    page = 1
    while True:
        batch = github_rest_api(
            "GET", path, params={"per_page": 100, "page": page}
        )
        if not batch:
            return results
        results.extend(batch)
        if len(batch) < 100:
            return results
        page += 1


def post_github_comment(owner, repository, issue_number, body):
    github_rest_api(
        "POST",
        f"repos/{owner}/{repository}/issues/{issue_number}/comments",
        body={"body": body},
    )


@cache
def _fetch_staff_logins():
    members = github_rest_get_paginated("orgs/zed-industries/teams/staff/members")
    return frozenset(member["login"].lower() for member in members)


def is_staff_member(login):
    return login.lower() in _fetch_staff_logins()


def is_community_pull_request(label_names):
    # The PR Issue Labeler workflow labels pull requests by staff and bots.
    return not NON_COMMUNITY_LABELS & {name.lower() for name in label_names}


def github_graphql(query, variables=None, *, partial_errors_ok=False, idempotent=False):
    # GraphQL uses POST for reads and writes, so classify the operation before
    # retrying. `idempotent` marks mutations that are safe to send twice.
    retry = idempotent or not query.lstrip().startswith("mutation")
    response = _github_request(
        "POST",
        "graphql",
        body={"query": query, "variables": variables or {}},
        retry=retry,
    )
    result = response.json()
    if "errors" in result:
        if not partial_errors_ok or "data" not in result:
            raise RuntimeError(f"GraphQL errors: {result['errors']}")
        print(f"GraphQL partial errors (ignored): {result['errors']}", file=sys.stderr)
    return result["data"]


def fetch_github_project(project_number, owner="zed-industries"):
    data = github_graphql(
        """
        query($owner: String!, $number: Int!) {
          organization(login: $owner) {
            projectV2(number: $number) {
              id
              fields(first: 50) {
                nodes {
                  ... on ProjectV2Field { id name dataType }
                  ... on ProjectV2SingleSelectField { id name options { id name } }
                }
              }
            }
          }
        }
        """,
        {"owner": owner, "number": project_number},
    )
    project = data["organization"]["projectV2"]
    if not project:
        raise RuntimeError(f"Project #{project_number} not found in {owner}")
    return project


def add_github_project_item(project_id, content_id):
    data = github_graphql(
        """
        mutation($projectId: ID!, $contentId: ID!) {
          addProjectV2ItemById(input: {projectId: $projectId, contentId: $contentId}) {
            item { id }
          }
        }
        """,
        {"projectId": project_id, "contentId": content_id},
        idempotent=True,
    )
    return data["addProjectV2ItemById"]["item"]["id"]


def _raise_unless_missing_ok(message, missing_ok):
    if not missing_ok:
        raise RuntimeError(message)
    print(f"{message}, skipping")


def set_github_project_field(project, item_id, field_name, value, *, missing_ok=False):
    """Set a single-select, number, date or text field on a project item, or
    clear it when `value` is `None`.

    With `missing_ok`, a missing field or option is logged and skipped instead
    of raising.
    """
    fields = [field for field in project["fields"]["nodes"] if "name" in field]
    field = next((field for field in fields if field["name"] == field_name), None)
    if not field:
        available = [field["name"] for field in fields]
        _raise_unless_missing_ok(
            f"Field '{field_name}' not found on project. Available: {available}",
            missing_ok,
        )
        return

    ids = {"projectId": project["id"], "itemId": item_id, "fieldId": field["id"]}
    if value is None:
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
            ids,
            idempotent=True,
        )
        print(f"Cleared '{field_name}'")
        return

    if "options" in field:
        option_id = next(
            (
                option["id"]
                for option in field["options"]
                # Case-insensitive mainly for Track, which has a mapping maintained manually
                if option["name"].casefold() == value.casefold()
            ),
            None,
        )
        if not option_id:
            available = [option["name"] for option in field["options"]]
            _raise_unless_missing_ok(
                f"Option '{value}' not found in field '{field_name}'. "
                f"Available: {available}",
                missing_ok,
            )
            return
        field_value = {"singleSelectOptionId": option_id}
    else:
        field_value = {PROJECT_FIELD_VALUE_KEYS[field["dataType"]]: value}

    github_graphql(
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
        {**ids, "value": field_value},
        idempotent=True,
    )
    print(f"Set '{field_name}'")
