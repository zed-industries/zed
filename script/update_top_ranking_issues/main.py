import os
from collections import defaultdict
from datetime import datetime, timedelta
from typing import Optional

from github import Github
from github.Issue import Issue
from github.Repository import Repository
from pytz import timezone

import typer
from typer import Typer

app: Typer = typer.Typer()

DATETIME_FORMAT: str = "%m/%d/%Y %I:%M %p"
CORE_LABELS: set[str] = {
    "defect",
    "design",
    "documentation",
    "enhancement",
    "panic / crash",
}
# A set of labels for adding in labels that we want present in the final
# report, but that we don't want being defined as a core label, since issues
# with without core labels are flagged as errors.
ADDITIONAL_LABELS: set[str] = {
    "ai",
    "linux",
    "vim",
    "windows",
}
IGNORED_LABEL_TEXT: str = "ignore top-ranking issues"
ISSUES_PER_LABEL: int = 20


class IssueData:
    def __init__(self, issue: Issue) -> None:
        self.url: str = issue.html_url
        self.like_count: int = issue._rawData["reactions"]["+1"]  # type: ignore [attr-defined]
        self.creation_datetime: str = issue.created_at.strftime(DATETIME_FORMAT)
        # TODO: Change script to support storing labels here, rather than directly in the script
        self.labels: set[str] = {label["name"] for label in issue._rawData["labels"]}  # type: ignore [attr-defined]


@app.command()
def main(
    github_token: Optional[str] = None,
    issue_reference_number: Optional[int] = None,
    query_day_interval: Optional[int] = None,
) -> None:
    start_time: datetime = datetime.now()

    start_date: datetime | None = None

    if query_day_interval:
        tz = timezone("america/new_york")
        current_time = datetime.now(tz).replace(
            hour=0, minute=0, second=0, microsecond=0
        )
        start_date = current_time - timedelta(days=query_day_interval)

    # GitHub Workflow will pass in the token as an environment variable,
    # but we can place it in our env when running the script locally, for convenience
    github_token = github_token or os.getenv("GITHUB_ACCESS_TOKEN")
    github = Github(github_token)

    remaining_requests_before: int = github.rate_limiting[0]
    print(f"Remaining requests before: {remaining_requests_before}")

    repo_name: str = "zed-industries/zed"
    repository: Repository = github.get_repo(repo_name)

    # There has to be a nice way of adding types to tuple unpacking
    label_to_issue_data: dict[str, list[IssueData]]
    error_message_to_erroneous_issue_data: dict[str, list[IssueData]]
    (
        label_to_issue_data,
        error_message_to_erroneous_issue_data,
    ) = get_issue_maps(github, repository, start_date)

    issue_text: str = get_issue_text(
        label_to_issue_data,
        error_message_to_erroneous_issue_data,
    )

    if issue_reference_number:
        top_ranking_issues_issue: Issue = repository.get_issue(issue_reference_number)
        top_ranking_issues_issue.edit(body=issue_text)
    else:
        print(issue_text)

    remaining_requests_after: int = github.rate_limiting[0]
    print(f"Remaining requests after: {remaining_requests_after}")
    print(f"Requests used: {remaining_requests_before - remaining_requests_after}")

    run_duration: timedelta = datetime.now() - start_time
    print(run_duration)


def get_issue_maps(
    github: Github,
    repository: Repository,
    start_date: datetime | None = None,
) -> tuple[dict[str, list[IssueData]], dict[str, list[IssueData]]]:
    label_to_issues: defaultdict[str, list[Issue]] = get_label_to_issues(
        github,
        repository,
        start_date,
    )
    label_to_issue_data: dict[str, list[IssueData]] = get_label_to_issue_data(
        label_to_issues
    )

    error_message_to_erroneous_issues: defaultdict[str, list[Issue]] = (
        get_error_message_to_erroneous_issues(github, repository)
    )
    error_message_to_erroneous_issue_data: dict[str, list[IssueData]] = (
        get_error_message_to_erroneous_issue_data(error_message_to_erroneous_issues)
    )

    # Create a new dictionary with labels ordered by the summation the of likes on the associated issues
    labels = list(label_to_issue_data.keys())

    labels.sort(
        key=lambda label: sum(
            issue_data.like_count for issue_data in label_to_issue_data[label]
        ),
        reverse=True,
    )

    label_to_issue_data = {label: label_to_issue_data[label] for label in labels}

    return (
        label_to_issue_data,
        error_message_to_erroneous_issue_data,
    )


def get_label_to_issues(
    github: Github,
    repository: Repository,
    start_date: datetime | None = None,
) -> defaultdict[str, list[Issue]]:
    label_to_issues: defaultdict[str, list[Issue]] = defaultdict(list)

    labels: set[str] = CORE_LABELS | ADDITIONAL_LABELS

    date_query: str = (
        f"created:>={start_date.strftime('%Y-%m-%d')}" if start_date else ""
    )

    for label in labels:
        query: str = f'repo:{repository.full_name} is:open is:issue {date_query} label:"{label}" -label:"{IGNORED_LABEL_TEXT}" sort:reactions-+1-desc'

        issues = github.search_issues(query)

        if issues.totalCount > 0:
            for issue in issues[0:ISSUES_PER_LABEL]:
                label_to_issues[label].append(issue)

    return label_to_issues


def get_label_to_issue_data(
    label_to_issues: defaultdict[str, list[Issue]],
) -> dict[str, list[IssueData]]:
    label_to_issue_data: dict[str, list[IssueData]] = {}

    for label in label_to_issues:
        issues: list[Issue] = label_to_issues[label]
        issue_data: list[IssueData] = [IssueData(issue) for issue in issues]
        issue_data.sort(
            key=lambda issue_data: (
                -issue_data.like_count,
                issue_data.creation_datetime,
            )
        )

        if issue_data:
            label_to_issue_data[label] = issue_data

    return label_to_issue_data


def get_error_message_to_erroneous_issues(
    github: Github, repository: Repository
) -> defaultdict[str, list[Issue]]:
    error_message_to_erroneous_issues: defaultdict[str, list[Issue]] = defaultdict(list)

    # Query for all open issues that don't have either a core or the ignored label and mark those as erroneous
    filter_labels: set[str] = CORE_LABELS | {IGNORED_LABEL_TEXT}
    filter_labels_text: str = " ".join([f'-label:"{label}"' for label in filter_labels])
    query: str = f"repo:{repository.full_name} is:open is:issue {filter_labels_text}"

    for issue in github.search_issues(query):
        error_message_to_erroneous_issues["missing core label"].append(issue)

    return error_message_to_erroneous_issues


def get_error_message_to_erroneous_issue_data(
    error_message_to_erroneous_issues: defaultdict[str, list[Issue]],
) -> dict[str, list[IssueData]]:
    error_message_to_erroneous_issue_data: dict[str, list[IssueData]] = {}

    for label in error_message_to_erroneous_issues:
        issues: list[Issue] = error_message_to_erroneous_issues[label]
        issue_data: list[IssueData] = [IssueData(issue) for issue in issues]
        error_message_to_erroneous_issue_data[label] = issue_data

    return error_message_to_erroneous_issue_data


def get_issue_text(
    label_to_issue_data: dict[str, list[IssueData]],
    error_message_to_erroneous_issue_data: dict[str, list[IssueData]],
) -> str:
    tz = timezone("america/new_york")
    current_datetime: str = datetime.now(tz).strftime(f"{DATETIME_FORMAT} (%Z)")

    highest_ranking_issues_lines: list[str] = get_highest_ranking_issues_lines(
        label_to_issue_data
    )

    issue_text_lines: list[str] = [
        f"*Updated on {current_datetime}*",
        *highest_ranking_issues_lines,
        "",
        "---\n",
    ]

    erroneous_issues_lines: list[str] = get_erroneous_issues_lines(
        error_message_to_erroneous_issue_data
    )

    if erroneous_issues_lines:
        core_labels_text: str = ", ".join(
            f'"{core_label}"' for core_label in CORE_LABELS
        )

        issue_text_lines.extend(
            [
                "## errors with issues (this section only shows when there are errors with issues)\n",
                f"This script expects every issue to have at least one of the following core labels: {core_labels_text}",
                f"This script currently ignores issues that have the following label: {IGNORED_LABEL_TEXT}\n",
                "### what to do?\n",
                "- Adjust the core labels on an issue to put it into a correct state or add a currently-ignored label to the issue",
                "- Adjust the core and ignored labels registered in this script",
                *erroneous_issues_lines,
                "",
                "---\n",
            ]
        )

    issue_text_lines.extend(
        [
            "*For details on how this issue is generated, [see the script](https://github.com/zed-industries/zed/blob/main/script/update_top_ranking_issues/main.py)*",
        ]
    )

    return "\n".join(issue_text_lines)


def get_highest_ranking_issues_lines(
    label_to_issue_data: dict[str, list[IssueData]],
) -> list[str]:
    highest_ranking_issues_lines: list[str] = []

    if label_to_issue_data:
        for label, issue_data in label_to_issue_data.items():
            highest_ranking_issues_lines.append(f"\n## {label}\n")

            for i, issue_data in enumerate(issue_data):
                markdown_bullet_point: str = (
                    f"{issue_data.url} ({issue_data.like_count} :thumbsup:)"
                )

                markdown_bullet_point = f"{i + 1}. {markdown_bullet_point}"
                highest_ranking_issues_lines.append(markdown_bullet_point)

    return highest_ranking_issues_lines


def get_erroneous_issues_lines(
    error_message_to_erroneous_issue_data,
) -> list[str]:
    erroneous_issues_lines: list[str] = []

    if error_message_to_erroneous_issue_data:
        for (
            error_message,
            erroneous_issue_data,
        ) in error_message_to_erroneous_issue_data.items():
            erroneous_issues_lines.append(f"\n#### {error_message}\n")

            for erroneous_issue_data in erroneous_issue_data:
                erroneous_issues_lines.append(f"- {erroneous_issue_data.url}")

    return erroneous_issues_lines


if __name__ == "__main__":
    app()

# TODO: Sort label output into core and non core sections
