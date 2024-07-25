import { danger, warn } from "danger";
const { prHygiene } = require("danger-plugin-pr-hygiene");

prHygiene({
  rules: {
    // Don't enable this rule just yet, as it can have false positives.
    useImperativeMood: "off",
  },
});

const RELEASE_NOTES_PATTERN = new RegExp("Release Notes:\\r?\\n\\s+-", "gm");

const hasReleaseNotes = RELEASE_NOTES_PATTERN.test(danger.github.pr.body);
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
      "- (Added|Fixed|Improved) ... ([#<public_issue_number_if_exists>](https://github.com/zed-industries/zed/issues/<public_issue_number_if_exists>)).",
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

const INCORRECT_ISSUE_LINK_PATTERN = new RegExp("-.*\\(#\\d+\\)", "g");

const hasIncorrectIssueLinks = INCORRECT_ISSUE_LINK_PATTERN.test(
  danger.github.pr.body,
);
if (hasIncorrectIssueLinks) {
  warn(
    [
      "This PR has incorrectly formatted GitHub issue links in the release notes.",
      "",
      "GitHub issue links must be formatted as plain Markdown links:",
      "",
      "```",
      "- Improved something ([#ISSUE](https://github.com/zed-industries/zed/issues/ISSUE)).",
      "```",
    ].join("\n"),
  );
}
