import os
import sys
import time

import requests

GITHUB_API_URL = "https://api.github.com"
RETRYABLE_STATUS_CODES = {429, 500, 502, 503, 504}
MAX_ATTEMPTS = 4
REQUEST_TIMEOUT_SECONDS = 30


def _github_headers():
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        raise RuntimeError("GITHUB_TOKEN environment variable is required")
    return {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }


def _retry_delay(response, attempt):
    retry_after = response.headers.get("Retry-After")
    if retry_after:
        try:
            return max(float(retry_after), 0)
        except ValueError:
            pass
    return 2**attempt


def _github_request(method, path, *, params=None, body=None, retry=True):
    url = f"{GITHUB_API_URL}/{path}"
    max_attempts = MAX_ATTEMPTS if retry else 1
    for attempt in range(max_attempts):
        try:
            response = requests.request(
                method,
                url,
                headers=_github_headers(),
                params=params,
                json=body,
                timeout=REQUEST_TIMEOUT_SECONDS,
            )
        except (requests.ConnectionError, requests.Timeout) as error:
            if attempt == max_attempts - 1:
                raise
            delay = 2**attempt
            print(
                f"Transient GitHub API error ({error}); retrying in {delay}s",
                file=sys.stderr,
            )
            time.sleep(delay)
            continue

        if (
            response.status_code in RETRYABLE_STATUS_CODES
            and attempt < max_attempts - 1
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


def github_graphql(query, variables=None, *, partial_errors_ok=False):
    # GraphQL uses POST for reads and writes, so classify the operation before retrying.
    retry = not query.lstrip().startswith("mutation")
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
    )
    return data["addProjectV2ItemById"]["item"]["id"]


def set_github_project_field(project, item_id, field_name, option_name):
    field = next(
        (field for field in project["fields"]["nodes"] if field.get("name") == field_name),
        None,
    )
    if not field:
        available = [
            field["name"] for field in project["fields"]["nodes"] if "name" in field
        ]
        raise RuntimeError(
            f"Field '{field_name}' not found on project. Available: {available}"
        )

    option_id = next(
        (
            option["id"]
            for option in field.get("options", [])
            if option["name"] == option_name
        ),
        None,
    )
    if not option_id:
        available = [option["name"] for option in field.get("options", [])]
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
            "fieldId": field["id"],
            "optionId": option_id,
        },
    )
    print(f"Set '{field_name}' to '{option_name}'")
