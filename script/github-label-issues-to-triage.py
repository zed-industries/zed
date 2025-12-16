#!/usr/bin/env python3
"""Add `state:needs triage` label to open GitHub issues of types Bug and Crash
if they're missing area, priority, or frequency labels. Don't touch issues
with an assignee or another `state:` label.

Requires `requests` library and a GitHub access token with "Issues (write)"
permission passed as an environment variable. Was used as a quick-and-dirty
one-off-bulk-operation script to surface older untriaged issues in the `zed`
repository. Leaving it here for reference only; there's no error handling or
guardrails, you've been warned.
"""

import itertools
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
REQUIRED_LABELS_PREFIXES = ["area:", "priority:", "frequency:"]
NEEDS_TRIAGE_LABEL = "state:needs triage"


def get_open_issues(repo, issue_type):
    """Get open issues of certain type(s) via GitHub's REST API."""
    issues = []
    issues_url = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{repo}/issues"

    log.info("Start fetching open issues from the GitHub API.")
    params = {
        "state": "open",
        "type": issue_type,
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


def is_untriaged(issue):
    issue_labels = [label['name'] for label in issue['labels']]
    # don't want to overwrite existing state labels
    no_state_label = not any(label.startswith('state:') for label in issue_labels)
    # we want at least one label for each of the required prefixes
    has_all_required_labels = all(
        any(label.startswith(prefix) for label in issue_labels)
        for prefix in REQUIRED_LABELS_PREFIXES
    )
    # let's also assume if we managed to assign an issue it's triaged enough
    no_assignee = not issue['assignee']
    return no_state_label and no_assignee and not has_all_required_labels


def label_issues(issues, label):
    for issue in issues:
        log.debug(f"Processing issue {issue['number']}.")
        api_url_add_label = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue['number']}/labels"
        add_response = requests.post(
            api_url_add_label, headers=HEADERS, json={"labels": [label]}
        )
        add_response.raise_for_status()
        log.info(f"Added label '{label}' to issue {issue['title']}.")


if __name__ == "__main__":
    open_bugs = get_open_issues(REPO_NAME, "Bug")
    open_crashes = get_open_issues(REPO_NAME, "Crash")
    untriaged_issues = filter(
        is_untriaged, itertools.chain(open_bugs, open_crashes))
    label_issues(untriaged_issues, label=NEEDS_TRIAGE_LABEL)
