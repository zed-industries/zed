#![allow(clippy::disallowed_methods, reason = "This is only used in xtasks")]
use std::{
    fmt::{self, Debug},
    ops::Not,
    process::Command,
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{Context, Result, anyhow};
use derive_more::{Deref, DerefMut, FromStr};

use itertools::Itertools;
use regex::Regex;
use semver::Version;
use serde::Deserialize;

pub trait Subcommand {
    type ParsedOutput: FromStr<Err = anyhow::Error>;

    fn args(&self) -> impl IntoIterator<Item = String>;
}

#[derive(Deref, DerefMut)]
pub struct GitCommand<G: Subcommand> {
    #[deref]
    #[deref_mut]
    subcommand: G,
}

impl<G: Subcommand> GitCommand<G> {
    #[must_use]
    pub fn run(subcommand: G) -> Result<G::ParsedOutput> {
        Self { subcommand }.run_impl()
    }

    fn run_impl(self) -> Result<G::ParsedOutput> {
        let command_output = Command::new("git")
            .args(self.subcommand.args())
            .output()
            .context("Failed to spawn command")?;

        if command_output.status.success() {
            String::from_utf8(command_output.stdout)
                .map_err(|_| anyhow!("Invalid UTF8"))
                .and_then(|s| {
                    G::ParsedOutput::from_str(s.trim())
                        .map_err(|e| anyhow!("Failed to parse from string: {e:?}"))
                })
        } else {
            anyhow::bail!(
                "Command failed with exit code {}, stderr: {}",
                command_output.status.code().unwrap_or_default(),
                String::from_utf8(command_output.stderr).unwrap_or_default()
            )
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Stable,
    Preview,
}

impl ReleaseChannel {
    pub(crate) fn tag_suffix(&self) -> &'static str {
        match self {
            ReleaseChannel::Stable => "",
            ReleaseChannel::Preview => "-pre",
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionTag(Version, ReleaseChannel);

impl VersionTag {
    pub fn parse(input: &str) -> Result<Self, anyhow::Error> {
        // Being a bit more lenient for human inputs
        let version = input.strip_prefix('v').unwrap_or(input);

        let (version_str, channel) = version
            .strip_suffix("-pre")
            .map_or((version, ReleaseChannel::Stable), |version_str| {
                (version_str, ReleaseChannel::Preview)
            });

        Version::parse(version_str)
            .map(|version| Self(version, channel))
            .map_err(|_| anyhow::anyhow!("Failed to parse version from tag!"))
    }

    pub fn version(&self) -> &Version {
        &self.0
    }
}

impl ToString for VersionTag {
    fn to_string(&self) -> String {
        format!(
            "v{version}{channel_suffix}",
            version = self.0,
            channel_suffix = self.1.tag_suffix()
        )
    }
}

#[derive(Debug, Deref, FromStr, PartialEq, Eq, Hash, Deserialize)]
pub struct CommitSha(pub(crate) String);

impl CommitSha {
    pub fn short(&self) -> &str {
        self.0.as_str().split_at(8).0
    }
}

#[derive(Debug)]
pub struct CommitDetails {
    sha: CommitSha,
    author: Committer,
    title: String,
    body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Committer {
    name: String,
    email: String,
}

impl Committer {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_owned(),
            email: email.to_owned(),
        }
    }
}

impl fmt::Display for Committer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.email)
    }
}

impl CommitDetails {
    const BODY_DELIMITER: &str = "|body-delimiter|";
    const COMMIT_DELIMITER: &str = "|commit-delimiter|";
    const FIELD_DELIMITER: &str = "|field-delimiter|";
    const FORMAT_STRING: &str = "%H|field-delimiter|%an|field-delimiter|%ae|field-delimiter|%s|body-delimiter|%b|commit-delimiter|";

    fn parse(line: &str, body: &str) -> Result<Self, anyhow::Error> {
        let Some([sha, author_name, author_email, title]) =
            line.splitn(4, Self::FIELD_DELIMITER).collect_array()
        else {
            return Err(anyhow!("Failed to parse commit fields from input {line}"));
        };

        Ok(CommitDetails {
            sha: CommitSha(sha.to_owned()),
            author: Committer::new(author_name, author_email),
            title: title.to_owned(),
            body: body.to_owned(),
        })
    }

    pub fn pr_number(&self) -> Option<u64> {
        // Since we use squash merge, all commit titles end with the '(#12345)' pattern.
        // While we could strictly speaking index into this directly, go for a slightly
        // less prone approach to errors
        const PATTERN: &str = " (#";
        self.title
            .rfind(PATTERN)
            .and_then(|location| {
                self.title[location..]
                    .find(')')
                    .map(|relative_end| location + PATTERN.len()..location + relative_end)
            })
            .and_then(|range| self.title[range].parse().ok())
    }

    pub(crate) fn co_authors(&self) -> Option<Vec<Committer>> {
        static CO_AUTHOR_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"Co-authored-by: (.+) <(.+)>").unwrap());

        let mut co_authors = Vec::new();

        for cap in CO_AUTHOR_REGEX.captures_iter(&self.body.as_ref()) {
            let Some((name, email)) = cap
                .get(1)
                .map(|m| m.as_str())
                .zip(cap.get(2).map(|m| m.as_str()))
            else {
                continue;
            };
            co_authors.push(Committer::new(name, email));
        }

        co_authors.is_empty().not().then_some(co_authors)
    }

    pub(crate) fn author(&self) -> &Committer {
        &self.author
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn sha(&self) -> &CommitSha {
        &self.sha
    }
}

#[derive(Debug, Deref, Default, DerefMut)]
pub struct CommitList(Vec<CommitDetails>);

impl CommitList {
    pub fn range(&self) -> Option<String> {
        self.0
            .first()
            .zip(self.0.last())
            .map(|(first, last)| format!("{}..{}", last.sha().0, first.sha().0))
    }
}

impl IntoIterator for CommitList {
    type IntoIter = std::vec::IntoIter<CommitDetails>;
    type Item = CommitDetails;

    fn into_iter(self) -> std::vec::IntoIter<Self::Item> {
        self.0.into_iter()
    }
}

impl FromStr for CommitList {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(CommitList(
            input
                .split(CommitDetails::COMMIT_DELIMITER)
                .filter(|commit_details| !commit_details.is_empty())
                .map(|commit_details| {
                    let (line, body) = commit_details
                        .trim()
                        .split_once(CommitDetails::BODY_DELIMITER)
                        .expect("Missing body delimiter");
                    CommitDetails::parse(line, body)
                        .expect("Parsing from the output should succeed")
                })
                .collect(),
        ))
    }
}

pub struct GetVersionTags;

impl Subcommand for GetVersionTags {
    type ParsedOutput = VersionTagList;

    fn args(&self) -> impl IntoIterator<Item = String> {
        ["tag", "-l", "v*"].map(ToOwned::to_owned)
    }
}

pub struct VersionTagList(Vec<VersionTag>);

impl VersionTagList {
    pub fn sorted(mut self) -> Self {
        self.0.sort_by(|a, b| a.version().cmp(b.version()));
        self
    }

    pub fn find_previous_minor_version(&self, version_tag: &VersionTag) -> Option<&VersionTag> {
        self.0
            .iter()
            .take_while(|tag| tag.version() < version_tag.version())
            .collect_vec()
            .into_iter()
            .rev()
            .find(|tag| {
                (tag.version().major < version_tag.version().major
                    || (tag.version().major == version_tag.version().major
                        && tag.version().minor < version_tag.version().minor))
                    && tag.version().patch == 0
            })
    }
}

impl FromStr for VersionTagList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version_tags = s.lines().flat_map(VersionTag::parse).collect_vec();

        version_tags
            .is_empty()
            .not()
            .then_some(Self(version_tags))
            .ok_or_else(|| anyhow::anyhow!("No version tags found"))
    }
}

pub struct CommitsFromVersionToVersion {
    version_tag: VersionTag,
    branch: String,
}

impl CommitsFromVersionToVersion {
    pub fn new(version_tag: VersionTag, branch: String) -> Self {
        Self {
            version_tag,
            branch,
        }
    }
}

impl Subcommand for CommitsFromVersionToVersion {
    type ParsedOutput = CommitList;

    fn args(&self) -> impl IntoIterator<Item = String> {
        [
            "log".to_string(),
            format!("--pretty=format:{}", CommitDetails::FORMAT_STRING),
            format!(
                "{version}..{branch}",
                version = self.version_tag.to_string(),
                branch = self.branch
            ),
        ]
    }
}

pub struct NoOutput;

impl FromStr for NoOutput {
    type Err = anyhow::Error;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(NoOutput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_stable_version_tag() {
        let tag = VersionTag::parse("v0.172.8").unwrap();
        assert_eq!(tag.version().major, 0);
        assert_eq!(tag.version().minor, 172);
        assert_eq!(tag.version().patch, 8);
        assert_eq!(tag.1, ReleaseChannel::Stable);
    }

    #[test]
    fn parse_preview_version_tag() {
        let tag = VersionTag::parse("v0.172.1-pre").unwrap();
        assert_eq!(tag.version().major, 0);
        assert_eq!(tag.version().minor, 172);
        assert_eq!(tag.version().patch, 1);
        assert_eq!(tag.1, ReleaseChannel::Preview);
    }

    #[test]
    fn parse_version_tag_without_v_prefix() {
        let tag = VersionTag::parse("0.172.8").unwrap();
        assert_eq!(tag.version().major, 0);
        assert_eq!(tag.version().minor, 172);
        assert_eq!(tag.version().patch, 8);
    }

    #[test]
    fn parse_invalid_version_tag() {
        let result = VersionTag::parse("vConradTest");
        assert!(result.is_err());
    }

    #[test]
    fn version_tag_stable_roundtrip() {
        let tag = VersionTag::parse("v0.172.8").unwrap();
        assert_eq!(tag.to_string(), "v0.172.8");
    }

    #[test]
    fn version_tag_preview_roundtrip() {
        let tag = VersionTag::parse("v0.172.1-pre").unwrap();
        assert_eq!(tag.to_string(), "v0.172.1-pre");
    }

    #[test]
    fn sorted_orders_by_semver() {
        let input = indoc! {"
            v0.172.8
            v0.170.1
            v0.171.4
            v0.170.2
            v0.172.11
            v0.171.3
            v0.172.9
        "};
        let list = VersionTagList::from_str(input).unwrap().sorted();
        for window in list.0.windows(2) {
            assert!(
                window[0].version() <= window[1].version(),
                "{} should come before {}",
                window[0].to_string(),
                window[1].to_string()
            );
        }
        assert_eq!(list.0[0].to_string(), "v0.170.1");
        assert_eq!(list.0[list.0.len() - 1].to_string(), "v0.172.11");
    }

    #[test]
    fn find_previous_minor_for_173_returns_172() {
        let input = indoc! {"
            v0.170.1
            v0.170.2
            v0.171.3
            v0.171.4
            v0.172.0
            v0.172.8
            v0.172.9
            v0.172.11
        "};
        let list = VersionTagList::from_str(input).unwrap().sorted();
        let target = VersionTag::parse("v0.173.0").unwrap();
        let previous = list.find_previous_minor_version(&target).unwrap();
        assert_eq!(previous.version().major, 0);
        assert_eq!(previous.version().minor, 172);
        assert_eq!(previous.version().patch, 0);
    }

    #[test]
    fn find_previous_minor_skips_same_minor() {
        let input = indoc! {"
            v0.172.8
            v0.172.9
            v0.172.11
        "};
        let list = VersionTagList::from_str(input).unwrap().sorted();
        let target = VersionTag::parse("v0.172.8").unwrap();
        assert!(list.find_previous_minor_version(&target).is_none());
    }

    #[test]
    fn find_previous_minor_with_major_version_gap() {
        let input = indoc! {"
            v0.172.0
            v0.172.9
            v0.172.11
        "};
        let list = VersionTagList::from_str(input).unwrap().sorted();
        let target = VersionTag::parse("v1.0.0").unwrap();
        let previous = list.find_previous_minor_version(&target).unwrap();
        assert_eq!(previous.to_string(), "v0.172.0");
    }

    #[test]
    fn find_previous_minor_requires_zero_patch_version() {
        let input = indoc! {"
            v0.172.1
            v0.172.9
            v0.172.11
        "};
        let list = VersionTagList::from_str(input).unwrap().sorted();
        let target = VersionTag::parse("v1.0.0").unwrap();
        assert!(list.find_previous_minor_version(&target).is_none());
    }

    #[test]
    fn parse_tag_list_from_real_tags() {
        let input = indoc! {"
            v0.9999-temporary
            vConradTest
            v0.172.8
        "};
        let list = VersionTagList::from_str(input).unwrap();
        assert_eq!(list.0.len(), 1);
        assert_eq!(list.0[0].to_string(), "v0.172.8");
    }

    #[test]
    fn parse_empty_tag_list_fails() {
        let result = VersionTagList::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn pr_number_from_squash_merge_title() {
        let line = format!(
            "abc123{d}Author Name{d}author@email.com{d}Add cool feature (#12345)",
            d = CommitDetails::FIELD_DELIMITER
        );
        let commit = CommitDetails::parse(&line, "").unwrap();
        assert_eq!(commit.pr_number(), Some(12345));
    }

    #[test]
    fn pr_number_missing() {
        let line = format!(
            "abc123{d}Author Name{d}author@email.com{d}Some commit without PR ref",
            d = CommitDetails::FIELD_DELIMITER
        );
        let commit = CommitDetails::parse(&line, "").unwrap();
        assert_eq!(commit.pr_number(), None);
    }

    #[test]
    fn pr_number_takes_last_match() {
        let line = format!(
            "abc123{d}Author Name{d}author@email.com{d}Fix (#123) and refactor (#456)",
            d = CommitDetails::FIELD_DELIMITER
        );
        let commit = CommitDetails::parse(&line, "").unwrap();
        assert_eq!(commit.pr_number(), Some(456));
    }

    #[test]
    fn co_authors_parsed_from_body() {
        let line = format!(
            "abc123{d}Author Name{d}author@email.com{d}Some title",
            d = CommitDetails::FIELD_DELIMITER
        );
        let body = indoc! {"
            Co-authored-by: Alice Smith <alice@example.com>
            Co-authored-by: Bob Jones <bob@example.com>
        "};
        let commit = CommitDetails::parse(&line, body).unwrap();
        let co_authors = commit.co_authors().unwrap();
        assert_eq!(co_authors.len(), 2);
        assert_eq!(
            co_authors[0],
            Committer::new("Alice Smith", "alice@example.com")
        );
        assert_eq!(
            co_authors[1],
            Committer::new("Bob Jones", "bob@example.com")
        );
    }

    #[test]
    fn no_co_authors_returns_none() {
        let line = format!(
            "abc123{d}Author Name{d}author@email.com{d}Some title",
            d = CommitDetails::FIELD_DELIMITER
        );
        let commit = CommitDetails::parse(&line, "").unwrap();
        assert!(commit.co_authors().is_none());
    }

    #[test]
    fn commit_sha_short_returns_first_8_chars() {
        let sha = CommitSha("abcdef1234567890abcdef1234567890abcdef12".into());
        assert_eq!(sha.short(), "abcdef12");
    }

    #[test]
    fn parse_commit_list_from_git_log_format() {
        let fd = CommitDetails::FIELD_DELIMITER;
        let bd = CommitDetails::BODY_DELIMITER;
        let cd = CommitDetails::COMMIT_DELIMITER;

        let input = format!(
            "sha111{fd}Alice{fd}alice@test.com{fd}First commit (#100){bd}First body{cd}sha222{fd}Bob{fd}bob@test.com{fd}Second commit (#200){bd}Second body{cd}"
        );

        let list = CommitList::from_str(&input).unwrap();
        assert_eq!(list.0.len(), 2);

        assert_eq!(list.0[0].sha().0, "sha111");
        assert_eq!(
            list.0[0].author(),
            &Committer::new("Alice", "alice@test.com")
        );
        assert_eq!(list.0[0].title(), "First commit (#100)");
        assert_eq!(list.0[0].pr_number(), Some(100));
        assert_eq!(list.0[0].body, "First body");

        assert_eq!(list.0[1].sha().0, "sha222");
        assert_eq!(list.0[1].author(), &Committer::new("Bob", "bob@test.com"));
        assert_eq!(list.0[1].title(), "Second commit (#200)");
        assert_eq!(list.0[1].pr_number(), Some(200));
        assert_eq!(list.0[1].body, "Second body");
    }
}
