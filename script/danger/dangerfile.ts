import { danger, warn } from "danger";

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
