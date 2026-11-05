function releaseNotesSection(body) {
  // Inline mentions and bullet text must not override the final release notes heading.
  const headings = [...body.matchAll(/^Release Notes:(?:\r?\n)+(?=- )/gim)];
  const lastHeading = headings[headings.length - 1];
  return lastHeading ? body.slice(lastHeading.index + lastHeading[0].length) : "";
}

function releaseNotesEntries(section) {
  const entries = [];
  // Blank lines can separate bullets, but a non-bullet paragraph ends the notes
  // so commit trailers and other trailing text are not published as release notes.
  for (const paragraph of section.split(/\r?\n(?:[ \t]*\r?\n)+/)) {
    if (!paragraph.startsWith("- ")) {
      break;
    }

    entries.push(...paragraph.split(/\r?\n(?=- )/).map((entry) => entry.trimEnd().replace(/\r?\n/g, " ")));
  }
  return entries;
}

module.exports = { releaseNotesSection, releaseNotesEntries };
