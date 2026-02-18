import { danger, message, warn, fail } from "danger";
const { prHygiene } = require("danger-plugin-pr-hygiene");

prHygiene({
  prefixPattern: /^([a-z\d\(\)_\s]+):(.*)/g,
  rules: {
    // Don't enable this rule just yet, as it can have false positives.
    useImperativeMood: "off",
    noConventionalCommits: {
      bannedTypes: ["feat", "fix", "style", "refactor", "perf", "test", "chore", "build", "revert"],
    },
  },
});

const RELEASE_NOTES_SECTION_PATTERN = /(?:^|\n)Release Notes:\r?\n(?<section>(?:\s*-\s+.*(?:\r?\n|$))+)/m;
const RELEASE_NOTES_AT_END_PATTERN = /(?:^|\n)Release Notes:\r?\n(?:\s*-\s+.*(?:\r?\n|$))+\s*$/m;
const body = danger.github.pr.body;

const releaseNotesMatch = body.match(RELEASE_NOTES_SECTION_PATTERN);
const hasReleaseNotes = releaseNotesMatch !== null;
const hasReleaseNotesAtEnd = RELEASE_NOTES_AT_END_PATTERN.test(body);

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
      "",
      'The "Release Notes" section must be the final section in the PR body.',
    ].join("\n"),
  );
}

if (hasReleaseNotes && !hasReleaseNotesAtEnd) {
  warn('The "Release Notes" section must be the final section in the PR body.');
}

const changedFiles = [...danger.git.created_files, ...danger.git.modified_files, ...danger.git.deleted_files];
const isDocsOnlyChange = changedFiles.length > 0 && changedFiles.every((file) => file.startsWith("docs/"));

if (isDocsOnlyChange && releaseNotesMatch?.groups?.section) {
  const releaseNotesBullets = [...releaseNotesMatch.groups.section.matchAll(/^\s*-\s+(.*)$/gm)].map((match) =>
    match[1].trim(),
  );
  const hasOnlyNaBullet = releaseNotesBullets.length === 1 && releaseNotesBullets[0].toUpperCase() === "N/A";

  if (!hasOnlyNaBullet) {
    warn('Docs-only PRs should use "- N/A" under "Release Notes".');
  }
}

const ISSUE_LINK_PATTERN =
  /(?:- )?(?<!(?:Close[sd]?|Fixe[sd]|Resolve[sd]|Implement[sed]|Follow-up of|Part of):?\s+)https:\/\/github\.com\/[\w-]+\/[\w-]+\/issues\/\d+/gi;

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

const FIXTURE_CHANGE_ATTESTATION = "Changes to test fixtures are intentional and necessary.";

const FIXTURES_PATHS = ["crates/assistant_tools/src/edit_agent/evals/fixtures"];

const modifiedFixtures = danger.git.modified_files.filter((file) =>
  FIXTURES_PATHS.some((fixturePath) => file.includes(fixturePath)),
);

if (modifiedFixtures.length > 0) {
  if (!body.includes(FIXTURE_CHANGE_ATTESTATION)) {
    const modifiedFixturesStr = modifiedFixtures.map((path) => "`" + path + "`").join(", ");
    fail(
      [
        `This PR modifies eval or test fixtures (${modifiedFixturesStr}), which are typically expected to remain unchanged.`,
        "If these changes are intentional and required, please add the following attestation to your PR description: ",
        `"${FIXTURE_CHANGE_ATTESTATION}"`,
      ].join("\n\n"),
    );
  }
}
