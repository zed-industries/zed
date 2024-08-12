import { danger, warn } from "danger";
const { prHygiene } = require("danger-plugin-pr-hygiene");

prHygiene({
  rules: {
    // Don't enable this rule just yet, as it can have false positives.
    useImperativeMood: "off",
  },
});

const RELEASE_NOTES_PATTERN = new RegExp("Release Notes:\\r?\\n\\s+-", "gm");
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

const ISSUE_LINK_PATTERN = new RegExp(
  "(?:https://github\\.com/[\\w-]+/[\\w-]+/issues/\\d+|#\\d+)",
  "g",
);

const includesIssueUrl = ISSUE_LINK_PATTERN.test(body);

if (includesIssueUrl) {
  const matches = body.match(ISSUE_LINK_PATTERN);
  const issues = matches
    .map((match) =>
      match
        .replace(/^#/, "")
        .replace(/https:\/\/github\.com\/zed-industries\/zed\/issues\//, ""),
    )
    .filter((issue, index, self) => self.indexOf(issue) === index);

  warn(
    [
      "This PR includes links to the following GitHub Issues: " +
        issues.map((issue) => `#${issue}`).join(", "),
      "If this PR aims to close an issue, please include a `Closes #ISSUE` line at the top of the PR body.",
    ].join("\n"),
  );
}
