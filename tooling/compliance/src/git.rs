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
                    CommitDetails::parse(line, body).expect("Parsing from the output should suceed")
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

    pub fn find_previous_version(&self, version_tag: &VersionTag) -> Option<&VersionTag> {
        self.0
            .iter()
            .take_while(|tag| tag.version() < version_tag.version())
            .last()
            .or_else(|| {
                self.0
                    .last()
                    .filter(|tag| tag.version() < version_tag.version())
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

pub struct CommitsFromVersionToHead {
    version_tag: VersionTag,
    branch: String,
}

impl CommitsFromVersionToHead {
    pub fn new(version_tag: VersionTag, branch: String) -> Self {
        Self {
            version_tag,
            branch,
        }
    }
}

impl Subcommand for CommitsFromVersionToHead {
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
