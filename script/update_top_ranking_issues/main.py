import os
from datetime import datetime, timedelta
from typing import Optional

import typer
from github import Github
from github.Issue import Issue
from github.Repository import Repository
from pytz import timezone
from typer import Typer

app: Typer = typer.Typer()

DATETIME_FORMAT: str = "%m/%d/%Y %I:%M %p"
ISSUES_PER_LABEL: int = 50


class IssueData:
    def __init__(self, issue: Issue) -> None:
        self.title = issue.title
        self.url: str = issue.html_url
        self.like_count: int = issue._rawData["reactions"]["+1"]  # type: ignore [attr-defined]
        self.creation_datetime: str = issue.created_at.strftime(DATETIME_FORMAT)
        # TODO: Change script to support storing labels here, rather than directly in the script
        self.labels: set[str] = {label["name"] for label in issue._rawData["labels"]}  # type: ignore [attr-defined]
        self._issue = issue


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

    with Github(github_token, per_page=100) as github:
        remaining_requests_before: int = github.rate_limiting[0]
        print(f"Remaining requests before: {remaining_requests_before}")

        repo_name: str = "zed-industries/zed"
        repository: Repository = github.get_repo(repo_name)

        label_to_issue_data: dict[str, list[IssueData]] = get_issue_maps(
            github, repository, start_date
        )

        issue_text: str = get_issue_text(label_to_issue_data)

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
) -> dict[str, list[IssueData]]:
    label_to_issue_data: dict[str, list[IssueData]] = get_label_to_issue_data(
        github,
        repository,
        start_date,
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

    return label_to_issue_data


def get_label_to_issue_data(
    github: Github,
    repository: Repository,
    start_date: datetime | None = None,
) -> dict[str, list[IssueData]]:
    common_queries = [
        f"repo:{repository.full_name}",
        "is:open",
        "is:issue",
        '-label:"ignore top-ranking issues"',
        "sort:reactions-+1-desc",
    ]

    date_query: str | None = (
        f"created:>={start_date.strftime('%Y-%m-%d')}" if start_date else None
    )

    if date_query:
        common_queries.append(date_query)

    common_query = " ".join(common_queries)

    # Because PyGithub doesn't seem to support logical operators `AND` and `OR`
    # that GitHub issue queries can use, we use lists as values, rather than
    # using `(label:bug OR type:Bug)`. This is not as efficient, as we might
    # query the same issue multiple times. Issues that are potentially queried
    # multiple times are deduplicated in the `label_to_issues` dictionary. If
    # PyGithub ever supports logical operators, we should definitely make the
    # switch.
    section_queries: dict[str, list[str]] = {
        "bug": ["label:bug", "type:Bug"],
        "crash": ["label:crash", "type:Crash"],
        "feature": ["label:feature", "type:Feature"],
        "meta": ["type:Meta"],
        "windows": ["label:windows"],
        "unlabeled": ["no:label no:type"],
    }

    label_to_issue_data: dict[str, list[IssueData]] = {}

    for section, queries in section_queries.items():
        unique_issues = set()

        for query in queries:
            query: str = f"{common_query} {query}"
            issues = github.search_issues(query)

            for issue in issues:
                unique_issues.add(issue)

        if len(unique_issues) <= 0:
            continue

        issue_data: list[IssueData] = [IssueData(issue) for issue in unique_issues]
        issue_data.sort(
            key=lambda issue_data: (
                -issue_data.like_count,
                issue_data.creation_datetime,
            )
        )

        label_to_issue_data[section] = issue_data[0:ISSUES_PER_LABEL]

    return label_to_issue_data


def get_issue_text(
    label_to_issue_data: dict[str, list[IssueData]],
) -> str:
    tz = timezone("america/new_york")
    current_datetime: str = datetime.now(tz).strftime(f"{DATETIME_FORMAT} (%Z)")

    highest_ranking_issues_lines: list[str] = get_highest_ranking_issues_lines(
        label_to_issue_data
    )

    issue_text_lines: list[str] = [
        f"*Updated on {current_datetime}*",
        *highest_ranking_issues_lines,
        "\n---\n",
        "*For details on how this issue is generated, [see the script](https://github.com/zed-industries/zed/blob/main/script/update_top_ranking_issues/main.py)*",
    ]

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


if __name__ == "__main__":
    app()

# TODO: Sort label output into core and non core sections
