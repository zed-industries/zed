import { danger, message, warn, fail } from "danger";
const { prHygiene } = require("danger-plugin-pr-hygiene");

prHygiene({
  prefixPattern: /^([a-z\d\(\)_\s]+):(.*)/g,
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

const PROMPT_PATHS = [
  "assets/prompts/content_prompt.hbs",
  "assets/prompts/terminal_assistant_prompt.hbs",
  "crates/agent_settings/src/prompts/summarize_thread_detailed_prompt.txt",
  "crates/agent_settings/src/prompts/summarize_thread_prompt.txt",
  "crates/agent/src/templates/create_file_prompt.hbs",
  "crates/agent/src/templates/edit_file_prompt_xml.hbs",
  "crates/agent/src/templates/edit_file_prompt_diff_fenced.hbs",
  "crates/git_ui/src/commit_message_prompt.txt",
];

const PROMPT_CHANGE_ATTESTATION = "I have ensured the LLM Worker works with these prompt changes.";

const modifiedPrompts = danger.git.modified_files.filter((file) =>
  PROMPT_PATHS.some((promptPath) => file.includes(promptPath)),
);

for (const promptPath of modifiedPrompts) {
  if (body.includes(PROMPT_CHANGE_ATTESTATION)) {
    message(
      [
        `This PR contains changes to "${promptPath}".`,
        "The author has attested the LLM Worker works with the changes to this prompt.",
      ].join("\n"),
    );
  } else {
    fail(
      [
        `Modifying the "${promptPath}" prompt may require corresponding changes in the LLM Worker.`,
        "If you are ensure what this entails, talk to @maxdeviant or another AI team member.",
        `Once you have made the changes—or determined that none are necessary—add "${PROMPT_CHANGE_ATTESTATION}" to the PR description.`,
      ].join("\n"),
    );
  }
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
