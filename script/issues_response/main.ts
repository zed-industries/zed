import { Octokit } from "@octokit/rest";
import { IncomingWebhook } from "@slack/webhook";

async function main() {
  const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });
  const webhook = new IncomingWebhook(
    process.env.SLACK_ISSUE_RESPONSE_WEBHOOK_URL!,
  );

  const owner = "zed-industries";
  const repo = "zed";
  const staff = await octokit.paginate(octokit.rest.orgs.listMembers, {
    org: owner,
    per_page: 100,
  });
  let staffHandles = staff.map((member) => member.login);
  let commenterFilters = staffHandles.map((name) => `-commenter:${name}`);
  let authorFilters = staffHandles.map((name) => `-author:${name}`);

  const q = [
    `repo:${owner}/${repo}`,
    "is:issue",
    "state:open",
    "created:>=2025-02-01",
    "sort:created-asc",
    ...commenterFilters,
    ...authorFilters,
  ];

  const response = await octokit.rest.search.issuesAndPullRequests({
    q: q.join("+"),
    per_page: 100,
  });

  let issues = response.data.items;
  let issueLines = issues.map((issue, index) => {
    const formattedDate = new Date(issue.created_at).toLocaleDateString(
      "en-US",
      {
        year: "numeric",
        month: "short",
        day: "numeric",
      },
    );
    return `${index + 1}. ${formattedDate}: <${issue.html_url}|${issue.title}>`;
  });

  const blocks = [
    {
      type: "section",
      text: {
        type: "mrkdwn",
        text: issueLines.join("\n"),
      },
    },
  ];

  await webhook.send({ blocks: blocks });
}

main().catch((error) => console.error("An error occurred:", error));
