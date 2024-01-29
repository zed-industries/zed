import { danger, warn } from "danger";

const RELEASE_NOTES_PATTERN = new RegExp("Release Notes:\ns+-", "gm");

const hasReleaseNotes = RELEASE_NOTES_PATTERN.test(danger.github.pr.body);
if (!hasReleaseNotes) {
  warn(
    "This PR is missing release notes. " +
      'Please add a "Release Notes" section that describes the change. ' +
      'If your change is not user-facing, you can use "N/A" for the release notes.',
  );
}
