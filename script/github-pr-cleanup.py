#!/usr/bin/env python3
"""
Clean up the community pull request queue:
    - retry CLA check and close PRs if that didn't help
    - close stale draft PRs (including staff's)
    - close stale PRs with changes requested
    - close PRs labeled "no self-review" that keep the README notice

Requires: requests
"""

import argparse
from collections import Counter
from datetime import datetime, timedelta, timezone
from functools import cache
from itertools import chain

from github_helpers import (
    github_graphql,
    github_rest_api,
    is_community_pull_request,
    is_staff_member,
    post_github_comment,
)

REPO_OWNER = "zed-industries"
REPO_NAME = "zed"
COMMUNITY_BOT_LOGIN = "zed-community-bot"
CLA_SIGNED_LABEL = "cla-signed"
NEVER_STALE_LABEL = "never stale"
NO_SELF_REVIEW_LABEL = "no self-review"
# Must match SELF_REVIEW_MARKER in script/danger/dangerfile.ts.
SELF_REVIEW_MARKER = "Remove this line to confirm you've reviewed this PR before submitting."

CLA_RETRY_AFTER = timedelta(days=7)
CLA_CLOSE_AFTER_RETRY = timedelta(days=1)
DRAFT_PING_AFTER = timedelta(days=21)
DRAFT_CLOSE_AFTER_PING = timedelta(days=7)
CHANGES_REQUESTED_PING_AFTER = timedelta(days=14)
CHANGES_REQUESTED_CLOSE_AFTER_PING = timedelta(days=7)
CHANGES_REQUESTED_CLOSE_AFTER_ACK = timedelta(days=14)
NO_SELF_REVIEW_PING_AFTER = timedelta(days=1)
NO_SELF_REVIEW_CLOSE_AFTER_PING = timedelta(days=3)

CLA_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=CLA-not-signed -->"
# Pings posted before the rename carry the old name, so the marker keeps it.
DRAFT_PING_MARKER = "<!-- zed-community-automation:stale-draft-warning -->"
DRAFT_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=stale_draft -->"
CHANGES_REQUESTED_PING_MARKER = "<!-- zed-community-automation:changes-requested-ping -->"
CHANGES_REQUESTED_ACK_MARKER = "<!-- zed-community-automation:changes-requested-ack -->"
CHANGES_REQUESTED_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=changes-requested-inactive -->"
NO_SELF_REVIEW_PING_MARKER = "<!-- zed-community-automation:no-self-review-ping -->"
NO_SELF_REVIEW_CLOSE_MARKER = "<!-- zed-community-automation:close-reason=no-self-review -->"

CLA_RETRY_COMMENT = "@cla-bot check"

CLA_CLOSE_COMMENT = f"""{CLA_CLOSE_MARKER}

Closing this pull request because Zed's contribution requirements have not been
met: the Contributor License Agreement is unsigned after seven days and a
final automated re-check.

We need a signed CLA before we can review or merge a contribution. If you would like
to continue, please sign the CLA at https://zed.dev/cla and open a new pull request.
If the CLA bot does not recognize your completed signature, mention that in the
new pull request so a maintainer can help."""

DRAFT_PING_COMMENT = f"""{DRAFT_PING_MARKER}

This pull request has remained in draft without new commits for three weeks.
If it remains a draft without new commits for another week, it will be closed
automatically."""

DRAFT_CLOSE_COMMENT = f"""{DRAFT_CLOSE_MARKER}

Closing this pull request because it remained in draft without new commits for
another week after the ping above.

We close long-running drafts to keep the pull request queue focused on actionable work
and to avoid accumulating conflicts with `main`. If you want to continue this work,
please open a new pull request when it is ready for review."""

CHANGES_REQUESTED_PING_COMMENT = f"""{CHANGES_REQUESTED_PING_MARKER}

Changes were requested on this pull request, and there have been no new commits or comments from you in the last two weeks.

If there's no response by the {{closing_date}}, this pull request will be closed automatically. That's to keep the review queue focused, and you're welcome to come back to this work later by opening a new pull request.

To keep this pull request open, do one of the following:
- If you've addressed the feedback, push your commits and **re-request review** using the 🔄 button next to the reviewer's name.
- If you think no changes are needed, explain why in a comment if you haven't already, and **re-request review**.
- If you need more time, leave a comment. That postpones the closure by two weeks.

Replying alone doesn't notify the reviewer that the pull request is ready for another look."""

CHANGES_REQUESTED_ACK_COMMENT = f"""{CHANGES_REQUESTED_ACK_MARKER}

The automatic closure has been postponed because of your comment.

If there are no new commits by the {{closing_date}}, this pull request will be closed automatically.

To keep this pull request open, do one of the following:
- Push commits that address the feedback, then **re-request review** using the 🔄 button next to the reviewer's name.
- If you think no changes are needed, explain why in a comment and **re-request review** so the reviewer can take another look."""

CHANGES_REQUESTED_CLOSE_COMMENT = f"""{CHANGES_REQUESTED_CLOSE_MARKER}

Closing this pull request because no new commits were pushed by the deadline in this bot's comment above.

Inactive pull requests are closed to keep the review queue focused. This pull request can't be reopened. To continue this work, open a new pull request with the requested changes and link to this one so reviewers can see the earlier discussion."""

NO_SELF_REVIEW_PING_COMMENT = f"""{NO_SELF_REVIEW_PING_MARKER}

This pull request will be closed automatically on the {{closing_date}} because it has a notice at the top of `README.md` that asks to be removed before submitting to confirm that you've reviewed the changes yourself.

To keep this pull request open, do both of the following:
- Remove the two-line `> [!IMPORTANT]` notice from the top of `README.md` and push the change.
- Leave a comment describing how you, the human, tested your changes manually: what you did, what happened, which platforms you tested on. Screenshots/videos are especially helpful."""

NO_SELF_REVIEW_CLOSE_COMMENT = f"""{NO_SELF_REVIEW_CLOSE_MARKER}

Closing this pull request because the notice in `README.md` wasn't removed, or no comment was posted by the author, by the deadline in this bot's comment above.

This pull request can't be reopened. To continue this work, review and test your changes, then open a new pull request without the notice in `README.md` and with a description of how you tested them."""

SUMMARY_FRAGMENT = """
fragment Summary on PullRequest {
  number
  isDraft
  createdAt
  author { login }
  labels(first: 100) { nodes { name } }
  reviews(last: 50, states: [APPROVED, CHANGES_REQUESTED, DISMISSED]) {
    nodes { state submittedAt author { login } }
  }
  latestCommit: commits(last: 1) { nodes { commit { committedDate } } }
  convertedToDraft: timelineItems(
    last: 1, itemTypes: [CONVERT_TO_DRAFT_EVENT]
  ) {
    nodes { ... on ConvertToDraftEvent { createdAt } }
  }
}
"""


def fetch_open_pull_requests():
    summaries = []
    cursor = None
    while True:
        data = github_graphql(
            """
            query($owner: String!, $repo: String!, $cursor: String) {
              repository(owner: $owner, name: $repo) {
                pullRequests(states: OPEN, first: 100, after: $cursor) {
                  pageInfo { hasNextPage endCursor }
                  nodes { ...Summary }
                }
              }
            }
            """
            + SUMMARY_FRAGMENT,
            {"owner": REPO_OWNER, "repo": REPO_NAME, "cursor": cursor},
        )
        page = data["repository"]["pullRequests"]
        summaries += page["nodes"]
        if not page["pageInfo"]["hasNextPage"]:
            return summaries
        cursor = page["pageInfo"]["endCursor"]


@cache
def fetch_pull_request(number):
    """Fetch the summary plus recent comments, commits, timeline events and
    the README at the head commit.

    Each cleanup rules out what it can from the summary first, so most pull
    requests never need this call.
    """
    data = github_graphql(
        """
        query($owner: String!, $repo: String!, $number: Int!) {
          repository(owner: $owner, name: $repo) {
            pullRequest(number: $number) {
              ...Summary
              comments(last: 100) {
                nodes { createdAt body author { login } }
              }
              commits(last: 100) {
                nodes {
                  commit {
                    committedDate
                    author { user { login } }
                    parents { totalCount }
                  }
                }
              }
              timelineItems(
                last: 100,
                itemTypes: [
                  LABELED_EVENT,
                  REOPENED_EVENT,
                  READY_FOR_REVIEW_EVENT,
                  REVIEW_REQUESTED_EVENT,
                  PULL_REQUEST_REVIEW
                ]
              ) {
                nodes {
                  __typename
                  ... on LabeledEvent { createdAt label { name } }
                  ... on ReopenedEvent { createdAt }
                  ... on ReadyForReviewEvent { createdAt }
                  ... on ReviewRequestedEvent { createdAt }
                  ... on PullRequestReview { submittedAt author { login } }
                }
              }
              headCommit: commits(last: 1) {
                nodes {
                  commit {
                    file(path: "README.md") {
                      object { ... on Blob { text } }
                    }
                  }
                }
              }
            }
          }
        }
        """
        + SUMMARY_FRAGMENT,
        {"owner": REPO_OWNER, "repo": REPO_NAME, "number": number},
    )
    return data["repository"]["pullRequest"]


def label_names(pull_request):
    return {
        label["name"].lower() for label in pull_request["labels"]["nodes"]
    }


def author_login(node):
    # Deleted accounts show up as a missing author.
    return (node["author"] or {}).get("login")


def event_times(pull_request, typename):
    return [
        datetime.fromisoformat(item["createdAt"])
        for item in pull_request["timelineItems"]["nodes"]
        if item["__typename"] == typename
    ]


def clock_start(pull_request, started_at):
    """Return when a cleanup's clock started: at `started_at`, or at the
    latest reopen if that's later.

    Restarting the clock on reopen also makes the bot ignore its comments from
    before the reopen, so a reopened pull request isn't closed again at once.
    """
    return max([started_at, *event_times(pull_request, "ReopenedEvent")])


def bot_commented_at(pull_request, text, since):
    """Return when this bot last posted a comment containing `text` after
    `since`."""
    times = (
        datetime.fromisoformat(comment["createdAt"])
        for comment in pull_request["comments"]["nodes"]
        if author_login(comment) == COMMUNITY_BOT_LOGIN
        and text in comment["body"]
    )
    return max((time for time in times if time > since), default=None)


def author_commented_at(pull_request, since):
    """Return when the author last commented or replied to a review after
    `since`. A bare `@cla-bot check` doesn't count as a response."""
    author = author_login(pull_request)
    comments = (
        datetime.fromisoformat(comment["createdAt"])
        for comment in pull_request["comments"]["nodes"]
        if author_login(comment) == author
        and " ".join(comment["body"].lower().split()) != "@cla-bot check"
    )
    reviews = (
        datetime.fromisoformat(item["submittedAt"])
        for item in pull_request["timelineItems"]["nodes"]
        if item["__typename"] == "PullRequestReview"
        and author_login(item) == author
    )
    return max(
        (time for time in chain(comments, reviews) if time > since),
        default=None,
    )


def author_pushed_since(pull_request, timestamp):
    """Return whether the author committed after `timestamp`. Merge commits,
    such as merging `main`, don't address feedback, so they don't count;
    neither do commits by others, such as staff or the autofix bot."""
    author = author_login(pull_request)
    for node in pull_request["commits"]["nodes"]:
        commit = node["commit"]
        if commit["parents"]["totalCount"] > 1:
            continue
        user = commit["author"]["user"]
        # A commit email that isn't linked to a GitHub account leaves the
        # commit without a user, and in practice that's the author's own work.
        if (user is None or user["login"] == author) and (
            datetime.fromisoformat(commit["committedDate"]) > timestamp
        ):
            return True
    return False


def with_closing_date(comment, closes_after, now):
    date = (now + closes_after).date()
    if 11 <= date.day <= 13:
        suffix = "th"
    else:
        suffix = {1: "st", 2: "nd", 3: "rd"}.get(date.day % 10, "th")
    return comment.format(closing_date=f"{date.day}{suffix} of {date:%B %Y}")


def closing_date_reached(notice_at, closes_after, now):
    # Compare dates rather than durations so the pull request closes on the
    # date the comment promised, even if today's run starts a little earlier.
    return now.date() >= (notice_at + closes_after).date()


def cla_action(summary, now):
    created_at = datetime.fromisoformat(summary["createdAt"])
    labels = label_names(summary)
    if (
        not is_community_pull_request(labels)
        or CLA_SIGNED_LABEL in labels
        or now - created_at < CLA_RETRY_AFTER
    ):
        return None

    pull_request = fetch_pull_request(summary["number"])
    since = clock_start(pull_request, created_at)
    if bot_commented_at(pull_request, CLA_CLOSE_MARKER, since):
        return "close", None
    if now - since < CLA_RETRY_AFTER:
        return None
    if author_commented_at(pull_request, since):
        return None
    retried_at = bot_commented_at(pull_request, CLA_RETRY_COMMENT, since)
    if retried_at is None:
        return "retry CLA check", CLA_RETRY_COMMENT
    if now - retried_at >= CLA_CLOSE_AFTER_RETRY:
        return "close", CLA_CLOSE_COMMENT
    return None


def draft_action(summary, now):
    if not summary["isDraft"]:
        return None
    conversions = summary["convertedToDraft"]["nodes"]
    # A pull request opened as a draft has no conversion event.
    drafted_at = (conversions[-1] if conversions else summary)["createdAt"]
    commit = summary["latestCommit"]["nodes"][-1]["commit"]
    active_at = max(
        datetime.fromisoformat(drafted_at),
        datetime.fromisoformat(commit["committedDate"]),
    )
    if now - active_at < DRAFT_PING_AFTER:
        return None

    pull_request = fetch_pull_request(summary["number"])
    since = clock_start(pull_request, active_at)
    if bot_commented_at(pull_request, DRAFT_CLOSE_MARKER, since):
        return "close", None
    if now - since < DRAFT_PING_AFTER:
        return None
    pinged_at = bot_commented_at(pull_request, DRAFT_PING_MARKER, since)
    if pinged_at is None:
        return "ping", DRAFT_PING_COMMENT
    if now - pinged_at >= DRAFT_CLOSE_AFTER_PING:
        return "close", DRAFT_CLOSE_COMMENT
    return None


def staff_change_request_at(summary):
    """Return when staff requested changes, if that's still the latest staff
    verdict on the pull request."""
    reviews = summary["reviews"]["nodes"]
    # Most pull requests have no change request at all, and ruling them out
    # first saves the staff membership lookups below.
    if not any(review["state"] == "CHANGES_REQUESTED" for review in reviews):
        return None
    for review in reversed(reviews):
        reviewer = author_login(review)
        if reviewer and is_staff_member(reviewer):
            if review["state"] != "CHANGES_REQUESTED":
                return None
            return datetime.fromisoformat(review["submittedAt"])
    return None


def changes_requested_action(summary, now):
    community = is_community_pull_request(label_names(summary))
    if summary["isDraft"] or not community:
        return None
    requested_at = staff_change_request_at(summary)
    if (
        requested_at is None
        or now - requested_at < CHANGES_REQUESTED_PING_AFTER
    ):
        return None

    pull_request = fetch_pull_request(summary["number"])
    review_requested = any(
        time > requested_at
        for time in event_times(pull_request, "ReviewRequestedEvent")
    )
    if review_requested or author_pushed_since(pull_request, requested_at):
        return None
    since = clock_start(pull_request, requested_at)
    if bot_commented_at(pull_request, CHANGES_REQUESTED_CLOSE_MARKER, since):
        return "close", None

    commented_at = author_commented_at(pull_request, since)
    pinged_at = bot_commented_at(
        pull_request, CHANGES_REQUESTED_PING_MARKER, since
    )
    if pinged_at is None:
        # Before the ping, an author comment restarts the clock silently.
        if now - (commented_at or since) < CHANGES_REQUESTED_PING_AFTER:
            return None
        return "ping", with_closing_date(
            CHANGES_REQUESTED_PING_COMMENT,
            CHANGES_REQUESTED_CLOSE_AFTER_PING,
            now,
        )

    acknowledged_at = bot_commented_at(
        pull_request, CHANGES_REQUESTED_ACK_MARKER, since
    )
    if acknowledged_at:
        notice_at = acknowledged_at
        closes_after = CHANGES_REQUESTED_CLOSE_AFTER_ACK
    else:
        notice_at = pinged_at
        closes_after = CHANGES_REQUESTED_CLOSE_AFTER_PING
    if commented_at and commented_at > notice_at:
        return "acknowledge", with_closing_date(
            CHANGES_REQUESTED_ACK_COMMENT,
            CHANGES_REQUESTED_CLOSE_AFTER_ACK,
            now,
        )
    if closing_date_reached(notice_at, closes_after, now):
        return "close", CHANGES_REQUESTED_CLOSE_COMMENT
    return None


def no_self_review_action(summary, now):
    labels = label_names(summary)
    # Drafts are left alone, since authors may still be cleaning them up.
    if summary["isDraft"] or not (
        NO_SELF_REVIEW_LABEL in labels and is_community_pull_request(labels)
    ):
        return None

    pull_request = fetch_pull_request(summary["number"])
    labeled_at = max(
        (
            datetime.fromisoformat(item["createdAt"])
            for item in pull_request["timelineItems"]["nodes"]
            if item["__typename"] == "LabeledEvent"
            and item["label"]["name"].lower() == NO_SELF_REVIEW_LABEL
        ),
        # On a busy pull request the label event can be older than the
        # fetched timeline, and zed-zippy adds the label right at opening.
        default=datetime.fromisoformat(pull_request["createdAt"]),
    )
    # A draft labeled long ago gets the full 24 hours once it's ready.
    started_at = max(
        [labeled_at, *event_times(pull_request, "ReadyForReviewEvent")]
    )
    since = clock_start(pull_request, started_at)
    if bot_commented_at(pull_request, NO_SELF_REVIEW_CLOSE_MARKER, since):
        return "close", None

    readme = pull_request["headCommit"]["nodes"][0]["commit"]["file"]
    notice_present = (
        readme is not None and SELF_REVIEW_MARKER in readme["object"]["text"]
    )
    pinged_at = bot_commented_at(
        pull_request, NO_SELF_REVIEW_PING_MARKER, since
    )
    if pinged_at is None:
        if notice_present and now - since >= NO_SELF_REVIEW_PING_AFTER:
            return "ping", with_closing_date(
                NO_SELF_REVIEW_PING_COMMENT,
                NO_SELF_REVIEW_CLOSE_AFTER_PING,
                now,
            )
        return None
    if not closing_date_reached(
        pinged_at, NO_SELF_REVIEW_CLOSE_AFTER_PING, now
    ):
        return None
    if notice_present or author_commented_at(pull_request, pinged_at) is None:
        return "close", NO_SELF_REVIEW_CLOSE_COMMENT
    return None


CLEANUPS = (
    ("CLA", cla_action),
    ("stale draft", draft_action),
    ("changes requested", changes_requested_action),
    ("no self-review", no_self_review_action),
)


def take_step(number, kind, comment, dry_run):
    if dry_run:
        if comment:
            print(f"  Would comment:\n{comment}\n")
        if kind == "close":
            print("  Would close")
        return
    if comment:
        post_github_comment(REPO_OWNER, REPO_NAME, number, comment)
    if kind == "close":
        github_rest_api(
            "PATCH",
            f"repos/{REPO_OWNER}/{REPO_NAME}/pulls/{number}",
            body={"state": "closed"},
        )


def run(now, dry_run):
    summaries = [
        summary
        for summary in fetch_open_pull_requests()
        if NEVER_STALE_LABEL not in label_names(summary)
    ]
    print(f"Checking {len(summaries)} open pull requests")
    if dry_run:
        print("Dry-run mode: no comments or closures will be made")

    steps_taken = Counter()
    for summary in summaries:
        for cleanup, action in CLEANUPS:
            step = action(summary, now)
            if step is None:
                continue
            kind, comment = step
            print(f"PR #{summary['number']}: {cleanup}: {kind}")
            take_step(summary["number"], kind, comment, dry_run)
            steps_taken[f"{cleanup}: {kind}"] += 1
            if kind == "close":
                break
    print(
        "Cleanup complete:",
        ", ".join(f"{step} ×{count}" for step, count in steps_taken.items())
        or "nothing to do",
    )


if __name__ == "__main__":
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Report actions without commenting on or closing pull requests",
    )
    arguments = argument_parser.parse_args()
    run(datetime.now(timezone.utc), arguments.dry_run)
