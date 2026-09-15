function releaseNotesSection(body) {
  // Inline mentions and bullet text must not override the final release notes heading.
  const headings = [...body.matchAll(/^Release Notes:(?:\r?\n)+(?=- )/gim)];
  const lastHeading = headings[headings.length - 1];
  return lastHeading ? body.slice(lastHeading.index + lastHeading[0].length) : "";
}

module.exports = { releaseNotesSection };
