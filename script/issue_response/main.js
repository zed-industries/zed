import { Octokit } from "@octokit/rest";
import { IncomingWebhook } from "@slack/webhook";

/**
 * The maximum length of the `text` in a section block.
 *
 * [Slack Docs](https://api.slack.com/reference/block-kit/blocks#section)
 */
const SECTION_BLOCK_TEXT_LIMIT = 3000;
const GITHUB_ISSUES_URL = "https://github.com/zed-industries/zed/issues";

async function main() {
  const octokit = new Octokit({
    auth: process.env["ISSUE_RESPONSE_GITHUB_TOKEN"],
  });

  if (!process.env["SLACK_ISSUE_RESPONSE_WEBHOOK_URL"]) {
    throw new Error("SLACK_ISSUE_RESPONSE_WEBHOOK_URL is not set");
  }

  const webhook = new IncomingWebhook(
    process.env["SLACK_ISSUE_RESPONSE_WEBHOOK_URL"],
  );

  const owner = "zed-industries";
  const repo = "zed";
  const teams = ["staff"];
  const githubHandleSet = new Set();

  for (const team of teams) {
    const teamMembers = await octokit.paginate(
      octokit.rest.teams.listMembersInOrg,
      {
        org: owner,
        team_slug: team,
        per_page: 100,
      },
    );

    for (const teamMember of teamMembers) {
      githubHandleSet.add(teamMember.login);
    }
  }

  const githubHandles = Array.from(githubHandleSet);
  githubHandles.sort((a, b) => a.toLowerCase().localeCompare(b.toLowerCase()));
  const commenterFilters = githubHandles.map((name) => `-commenter:${name}`);
  const authorFilters = githubHandles.map((name) => `-author:${name}`);
  const twoDaysAgo = new Date();
  twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
  const twoDaysAgoString = twoDaysAgo.toISOString().split("T")[0];
  const dateRangeFilter = `2025-02-01..${twoDaysAgoString}`;

  const q = [
    `repo:${owner}/${repo}`,
    "is:issue",
    "-type:feature",
    "-type:meta",
    "state:open",
    `created:${dateRangeFilter}`,
    "sort:created-asc",
    ...commenterFilters,
    ...authorFilters,
  ];

  const issues = await octokit.paginate(
    octokit.rest.search.issuesAndPullRequests,
    {
      q: q.join("+"),
      per_page: 100,
    },
  );
  const issueLines = issues.map((issue, index) => {
    const formattedDate = new Date(issue.created_at).toLocaleDateString(
      "en-US",
      {
        year: "numeric",
        month: "short",
        day: "numeric",
      },
    );
    const sanitizedTitle = issue.title
      .replaceAll("&", "&amp;")
      .replaceAll("<", "&lt;")
      .replaceAll(">", "&gt;");

    return `${index + 1}. ${formattedDate}: <${issue.html_url}|${sanitizedTitle}>\n`;
  });

  const sections = [];
  /** @type {string[]} */
  let currentSection = [];
  let currentSectionLength = 0;

  for (const issueLine of issueLines) {
    if (currentSectionLength + issueLine.length <= SECTION_BLOCK_TEXT_LIMIT) {
      currentSection.push(issueLine);
      currentSectionLength += issueLine.length;
    } else {
      sections.push(currentSection);
      currentSection = [];
      currentSectionLength = 0;
    }
  }

  if (currentSection.length > 0) {
    sections.push(currentSection);
  }

  const blocks = sections.map((section) => ({
    type: "section",
    text: {
      type: "mrkdwn",
      text: section.join("").trimEnd(),
    },
  }));

  const issuesUrl = `${GITHUB_ISSUES_URL}?q=${encodeURIComponent(q.join(" "))}`;

  blocks.push({
    type: "section",
    text: {
      type: "mrkdwn",
      text: `<${issuesUrl}|View on GitHub>`,
    },
  });

  await webhook.send({ blocks });
}

main().catch((error) => {
  console.error("An error occurred:", error);
  process.exit(1);
});
