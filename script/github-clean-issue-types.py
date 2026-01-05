#!/usr/bin/env python3
"""Replace 'bug/feature/crash' labels with 'Bug/Feature/Crash' types on open
GitHub issues.

Requires `requests` library and a GitHub access token with "Issues (write)"
permission passed as an environment variable.
Was used as a quick-and-dirty one-off-bulk-operation script to clean up issue
types in the `zed` repository. Leaving it here for reference only; there's no
error handling, you've been warned.
"""


import logging
import os

import requests

logging.basicConfig(level=logging.INFO)
log = logging.getLogger(__name__)

GITHUB_API_BASE_URL = "https://api.github.com"
REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")
HEADERS = {
    "Authorization": f"token {GITHUB_TOKEN}",
    "Accept": "application/vnd.github+json"
}
LABELS_TO_TYPES = {
    'bug': 'Bug',
    'feature': 'Feature',
    'crash': 'Crash',
 }


def get_open_issues_without_type(repo):
    """Get open issues without type via GitHub's REST API."""
    issues = []
    issues_url = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{repo}/issues"

    log.info("Start fetching issues from the GitHub API.")
    params = {
        "state": "open",
        "type": "none",
        "page": 1,
        "per_page": 100, # worked fine despite the docs saying 30
    }
    while True:
        response = requests.get(issues_url, headers=HEADERS, params=params)
        response.raise_for_status()
        issues.extend(response.json())
        log.info(f"Fetched the next page, total issues so far: {len(issues)}.")

        # is there a next page?
        link_header = response.headers.get('Link', '')
        if 'rel="next"' not in link_header:
            break
        params['page'] += 1

    log.info("Done fetching issues.")
    return issues


def replace_labels_with_types(issues, labels_to_types):
    """Replace labels with types, a new attribute of issues.

    Only changes the issues with one type-sounding label, leaving those with
    two labels (e.g. `bug` *and* `crash`) alone, logging a warning.
    """
    for issue in issues:
        log.debug(f"Processing issue {issue['number']}.")
        # for GitHub, all PRs are issues but not all issues are PRs; skip PRs
        if 'pull_request' in issue:
            continue
        issue_labels = (label['name'] for label in issue['labels'])
        matching_labels = labels_to_types.keys() & set(issue_labels)
        if len(matching_labels) != 1:
            log.warning(
                f"Issue {issue['url']} has either no or multiple type-sounding "
                "labels, won't be processed.")
            continue
        label_to_replace = matching_labels.pop()
        issue_type = labels_to_types[label_to_replace]
        log.debug(
            f"Replacing label {label_to_replace} with type {issue_type} "
            f"for issue {issue['title']}.")

        # add the type
        api_url_issue = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue['number']}"
        add_type_response = requests.patch(
            api_url_issue, headers=HEADERS, json={"type": issue_type})
        add_type_response.raise_for_status()
        log.debug(f"Added type {issue_type} to issue {issue['title']}.")

        # delete the label
        api_url_delete_label = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue['number']}/labels/{label_to_replace}"
        delete_response = requests.delete(api_url_delete_label, headers=HEADERS)
        delete_response.raise_for_status()
        log.info(
            f"Deleted label {label_to_replace} from issue {issue['title']}.")


if __name__ == "__main__":
    open_issues_without_type = get_open_issues_without_type(REPO_NAME)
    replace_labels_with_types(open_issues_without_type, LABELS_TO_TYPES)
