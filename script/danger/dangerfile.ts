import { danger, message, warn } from "danger";
const { prHygiene } = require("danger-plugin-pr-hygiene");

prHygiene({
  prefixPattern: new RegExp("^([a-z\\d\\(\\)_\\s]+):(.*)"),
  rules: {
    // Don't enable this rule just yet, as it can have false positives.
    useImperativeMood: "off",
  },
});

const RELEASE_NOTES_PATTERN = /Release Notes:\r?\n\s+-/gm;
const body = danger.github.pr.body;

const hasReleaseNotes = RELEASE_NOTES_PATTERN.test(body);

if (!hasReleaseNotes) {
  warn(
    [
      "This PR is missing release notes.",
      "",
      'Please add a "Release Notes" section that describes the change:',
      "",
      "```",
      "Release Notes:",
      "",
      "- Added/Fixed/Improved ...",
      "```",
      "",
      'If your change is not user-facing, you can use "N/A" for the entry:',
      "```",
      "Release Notes:",
      "",
      "- N/A",
      "```",
    ].join("\n"),
  );
}

const ISSUE_LINK_PATTERN =
  /(?<!(?:Close[sd]?|Fixe[sd]|Resolve[sd]|Implement[sed]|Follow-up of|Part of)\s+)https:\/\/github\.com\/[\w-]+\/[\w-]+\/issues\/\d+/gi;

const bodyWithoutReleaseNotes = hasReleaseNotes ? body.split(/Release Notes:/)[0] : body;
const includesIssueUrl = ISSUE_LINK_PATTERN.test(bodyWithoutReleaseNotes);

if (includesIssueUrl) {
  const matches = bodyWithoutReleaseNotes.match(ISSUE_LINK_PATTERN) ?? [];
  const issues = matches
    .map((match) => match.replace(/^#/, "").replace(/https:\/\/github\.com\/zed-industries\/zed\/issues\//, ""))
    .filter((issue, index, self) => self.indexOf(issue) === index);

  const issuesToReport = issues.map((issue) => `#${issue}`).join(", ");
  message(
    [
      `This PR includes links to the following GitHub Issues: ${issuesToReport}`,
      "If this PR aims to close an issue, please include a `Closes #ISSUE` line at the top of the PR body.",
    ].join("\n"),
  );
}
