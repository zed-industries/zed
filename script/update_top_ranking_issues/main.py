import os
from datetime import date, datetime, timedelta
from typing import Any, Optional

import requests
import typer
from pytz import timezone
from typer import Typer

app: Typer = typer.Typer()

AMERICA_NEW_YORK_TIMEZONE = "America/New_York"
DATETIME_FORMAT: str = "%B %d, %Y %I:%M %p"
ISSUES_PER_SECTION: int = 50
ISSUES_TO_FETCH: int = 100

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
GITHUB_API_BASE_URL = "https://api.github.com"

EXCLUDE_LABEL = "ignore top-ranking issues"


@app.command()
def main(
    github_token: Optional[str] = None,
    issue_reference_number: Optional[int] = None,
    query_day_interval: Optional[int] = None,
) -> None:
    script_start_time: datetime = datetime.now()
    start_date: date | None = None

    if query_day_interval:
        tz = timezone(AMERICA_NEW_YORK_TIMEZONE)
        today = datetime.now(tz).date()
        start_date = today - timedelta(days=query_day_interval)

    # GitHub Workflow will pass in the token as an argument,
    # but we can place it in our env when running the script locally, for convenience
    token = github_token or os.getenv("GITHUB_ACCESS_TOKEN")
    if not token:
        raise typer.BadParameter(
            "GitHub token is required. Pass --github-token or set GITHUB_ACCESS_TOKEN env var."
        )

    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github+json",
    }

    section_to_issues = get_section_to_issues(headers, start_date)
    issue_text: str = create_issue_text(section_to_issues)

    if issue_reference_number:
        update_reference_issue(headers, issue_reference_number, issue_text)
    else:
        print(issue_text)

    run_duration: timedelta = datetime.now() - script_start_time
    print(f"Ran for {run_duration}")


def get_section_to_issues(
    headers: dict[str, str], start_date: date | None = None
) -> dict[str, list[dict[str, Any]]]:
    """Fetch top-ranked issues for each section from GitHub."""

    section_filters = {
        "Bugs": "type:Bug",
        "Crashes": "type:Crash",
        "Features": "type:Feature",
        "Tracking issues": "type:Tracking",
        "Meta issues": "type:Meta",
        "Windows": 'label:"platform:windows"',
    }

    section_to_issues: dict[str, list[dict[str, Any]]] = {}
    for section, search_qualifier in section_filters.items():
        query_parts = [
            f"repo:{REPO_OWNER}/{REPO_NAME}",
            "is:issue",
            "is:open",
            f'-label:"{EXCLUDE_LABEL}"',
            search_qualifier,
        ]

        if start_date:
            query_parts.append(f"created:>={start_date.strftime('%Y-%m-%d')}")

        query = " ".join(query_parts)
        url = f"{GITHUB_API_BASE_URL}/search/issues"
        params = {
            "q": query,
            "sort": "reactions-+1",
            "order": "desc",
            "per_page": ISSUES_TO_FETCH,  # this will work as long as it's ≤ 100
        }

        # we are only fetching one page on purpose
        response = requests.get(url, headers=headers, params=params)
        response.raise_for_status()
        items = response.json()["items"]

        issues: list[dict[str, Any]] = []
        for item in items:
            reactions = item["reactions"]
            score = reactions["+1"] - reactions["-1"]
            if score > 0:
                issues.append(
                    {
                        "url": item["html_url"],
                        "score": score,
                        "created_at": item["created_at"],
                    }
                )

        if not issues:
            continue

        issues.sort(key=lambda x: (-x["score"], x["created_at"]))
        section_to_issues[section] = issues[:ISSUES_PER_SECTION]

    # Sort sections by total score (highest total first)
    section_to_issues = dict(
        sorted(
            section_to_issues.items(),
            key=lambda item: sum(issue["score"] for issue in item[1]),
            reverse=True,
        )
    )
    return section_to_issues


def update_reference_issue(
    headers: dict[str, str], issue_number: int, body: str
) -> None:
    url = f"{GITHUB_API_BASE_URL}/repos/{REPO_OWNER}/{REPO_NAME}/issues/{issue_number}"
    response = requests.patch(url, headers=headers, json={"body": body})
    response.raise_for_status()


def create_issue_text(section_to_issues: dict[str, list[dict[str, Any]]]) -> str:
    tz = timezone(AMERICA_NEW_YORK_TIMEZONE)
    current_datetime: str = datetime.now(tz).strftime(f"{DATETIME_FORMAT} (%Z)")

    lines: list[str] = [f"*Updated on {current_datetime}*"]

    for section, issues in section_to_issues.items():
        lines.append(f"\n## {section}\n")
        for i, issue in enumerate(issues):
            lines.append(f"{i + 1}. {issue['url']} ({issue['score']} :thumbsup:)")

    lines.append("\n---\n")
    lines.append(
        "*For details on how this issue is generated, "
        "[see the script](https://github.com/zed-industries/zed/blob/main/script/update_top_ranking_issues/main.py)*"
    )

    return "\n".join(lines)


if __name__ == "__main__":
    app()
