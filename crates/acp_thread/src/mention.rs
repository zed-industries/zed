use agent_client_protocol::schema::v1 as acp;
use anyhow::{Context as _, Result, bail};
use file_icons::FileIcons;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};
use ui::{App, IconName, SharedString};
use url::Url;
use urlencoding::decode;
use util::{
    ResultExt,
    paths::{PathStyle, PathWithPosition, is_absolute},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MentionUri {
    File {
        abs_path: PathBuf,
    },
    PastedImage {
        name: String,
    },
    Directory {
        abs_path: PathBuf,
    },
    Symbol {
        abs_path: PathBuf,
        name: String,
        line_range: RangeInclusive<u32>,
    },
    Thread {
        id: acp::SessionId,
        name: String,
    },
    /// Deprecated: kept so threads from before rules became skills still
    /// deserialize. `id` (an opaque `prompt_store::PromptId`) is preserved
    /// verbatim so re-saved threads stay loadable by older Zed versions.
    Rule {
        #[serde(default = "default_deprecated_rule_id")]
        id: serde_json::Value,
        name: String,
    },
    Diagnostics {
        #[serde(default = "default_include_errors")]
        include_errors: bool,
        #[serde(default)]
        include_warnings: bool,
    },
    Selection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        abs_path: Option<PathBuf>,
        line_range: RangeInclusive<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        column: Option<u32>,
    },
    Fetch {
        url: Url,
    },
    TerminalSelection {
        line_count: u32,
    },
    GitDiff {
        base_ref: String,
    },
    MergeConflict {
        file_path: String,
    },
    Skill {
        name: String,
        source: String,
        skill_file_path: PathBuf,
    },
}

impl MentionUri {
    pub fn parse(input: &str, path_style: PathStyle) -> Result<Self> {
        let input = input
            .strip_prefix('`')
            .and_then(|input| input.strip_suffix('`'))
            .unwrap_or(input);

        let parse_column =
            |input: Option<String>| -> Option<u32> { input?.parse::<u32>().ok()?.checked_sub(1) };
        let validate_query_params = |url: &Url, allowed: &[&str]| -> Result<()> {
            for (key, _) in url.query_pairs() {
                if !allowed.contains(&key.as_ref()) {
                    bail!("invalid query parameter")
                }
            }
            Ok(())
        };

        if is_absolute(input, path_style) && !input.contains("://") {
            return parse_absolute_path(input)
                .with_context(|| format!("Invalid absolute path mention URI: {input}"));
        }

        let url = url::Url::parse(input)?;
        let path = url.path();
        match url.scheme() {
            "file" => {
                let trimmed = if path_style.is_windows() {
                    path.trim_start_matches("/")
                } else {
                    path
                };
                let decoded = decode(trimmed).unwrap_or(Cow::Borrowed(trimmed));
                let normalized: Cow<str> = if path_style.is_windows() {
                    match to_native_windows_path(&decoded) {
                        Some(native) => Cow::Owned(native),
                        None => decoded,
                    }
                } else {
                    decoded
                };
                let path = normalized.as_ref();

                if let Some(fragment) = url.fragment() {
                    validate_query_params(&url, &["symbol", "column"])?;
                    let line_range = parse_line_range(fragment).log_err().unwrap_or(1..=1);
                    let column = parse_column(query_param(&url, "column"));
                    if let Some(name) = query_param(&url, "symbol") {
                        Ok(Self::Symbol {
                            name,
                            abs_path: path.into(),
                            line_range,
                        })
                    } else {
                        Ok(Self::Selection {
                            abs_path: Some(path.into()),
                            line_range,
                            column,
                        })
                    }
                } else if input.ends_with("/") {
                    Ok(Self::Directory {
                        abs_path: path.into(),
                    })
                } else {
                    Ok(Self::File {
                        abs_path: path.into(),
                    })
                }
            }
            "zed" => {
                if let Some(thread_id) = path.strip_prefix("/agent/thread/") {
                    let name = single_query_param(&url, "name")?.context("Missing thread name")?;
                    Ok(Self::Thread {
                        id: acp::SessionId::new(thread_id),
                        name,
                    })
                } else if let Some(rule_id) = path.strip_prefix("/agent/rule/") {
                    // Deprecated: parses legacy rule mentions.
                    let name = single_query_param(&url, "name")?.context("Missing rule name")?;
                    let id = if rule_id.is_empty() {
                        default_deprecated_rule_id()
                    } else {
                        serde_json::json!({ "User": { "uuid": rule_id } })
                    };
                    Ok(Self::Rule { id, name })
                } else if path == "/agent/diagnostics" {
                    let mut include_errors = default_include_errors();
                    let mut include_warnings = false;
                    for (key, value) in url.query_pairs() {
                        match key.as_ref() {
                            "include_warnings" => include_warnings = value == "true",
                            "include_errors" => include_errors = value == "true",
                            _ => bail!("invalid query parameter"),
                        }
                    }
                    Ok(Self::Diagnostics {
                        include_errors,
                        include_warnings,
                    })
                } else if path.starts_with("/agent/pasted-image") {
                    let name =
                        single_query_param(&url, "name")?.unwrap_or_else(|| "Image".to_string());
                    Ok(Self::PastedImage { name })
                } else if path.starts_with("/agent/untitled-buffer") {
                    let fragment = url
                        .fragment()
                        .context("Missing fragment for untitled buffer selection")?;
                    let line_range = parse_line_range(fragment)?;
                    validate_query_params(&url, &["column"])?;
                    Ok(Self::Selection {
                        abs_path: None,
                        line_range,
                        column: parse_column(query_param(&url, "column")),
                    })
                } else if let Some(name) = path.strip_prefix("/agent/symbol/") {
                    let fragment = url
                        .fragment()
                        .context("Missing fragment for untitled buffer selection")?;
                    let line_range = parse_line_range(fragment)?;
                    let path =
                        single_query_param(&url, "path")?.context("Missing path for symbol")?;
                    Ok(Self::Symbol {
                        name: name.to_string(),
                        abs_path: path.into(),
                        line_range,
                    })
                } else if path.starts_with("/agent/file") {
                    let path =
                        single_query_param(&url, "path")?.context("Missing path for file")?;
                    Ok(Self::File {
                        abs_path: path.into(),
                    })
                } else if path.starts_with("/agent/directory") {
                    let path =
                        single_query_param(&url, "path")?.context("Missing path for directory")?;
                    Ok(Self::Directory {
                        abs_path: path.into(),
                    })
                } else if path.starts_with("/agent/selection") {
                    validate_query_params(&url, &["path", "column"])?;
                    let fragment = url.fragment().context("Missing fragment for selection")?;
                    let line_range = parse_line_range(fragment)?;
                    let column = parse_column(query_param(&url, "column"));
                    let path = query_param(&url, "path").context("Missing path for selection")?;
                    Ok(Self::Selection {
                        abs_path: Some(path.into()),
                        line_range,
                        column,
                    })
                } else if path.starts_with("/agent/terminal-selection") {
                    let line_count = single_query_param(&url, "lines")?
                        .unwrap_or_else(|| "0".to_string())
                        .parse::<u32>()
                        .unwrap_or(0);
                    Ok(Self::TerminalSelection { line_count })
                } else if path.starts_with("/agent/git-diff") {
                    let base_ref =
                        single_query_param(&url, "base")?.unwrap_or_else(|| "main".to_string());
                    Ok(Self::GitDiff { base_ref })
                } else if path.starts_with("/agent/merge-conflict") {
                    let file_path = single_query_param(&url, "path")?.unwrap_or_default();
                    Ok(Self::MergeConflict { file_path })
                } else if path.starts_with("/agent/skill") {
                    let mut name = None;
                    let mut source = None;
                    let mut skill_file_path = None;

                    for (key, value) in url.query_pairs() {
                        match key.as_ref() {
                            "name" => {
                                if name.replace(value.to_string()).is_some() {
                                    bail!("duplicate skill name query parameter");
                                }
                            }
                            "source" => {
                                if source.replace(value.to_string()).is_some() {
                                    bail!("duplicate skill source query parameter");
                                }
                            }
                            "path" => {
                                if skill_file_path
                                    .replace(PathBuf::from(value.to_string()))
                                    .is_some()
                                {
                                    bail!("duplicate skill file path query parameter");
                                }
                            }
                            _ => bail!("invalid query parameter"),
                        }
                    }

                    Ok(Self::Skill {
                        name: name.context("missing skill name")?,
                        source: source.context("missing skill source")?,
                        skill_file_path: skill_file_path.context("missing skill file path")?,
                    })
                } else {
                    bail!("invalid zed url: {:?}", input);
                }
            }
            "http" | "https" => Ok(MentionUri::Fetch { url }),
            other => bail!("unrecognized scheme {:?}", other),
        }
    }

    /// Parses a hyperlink target from agent-authored Markdown.
    ///
    /// Unlike [`MentionUri::parse`] — which stays strict so canonical mention
    /// URIs round-trip verbatim — bare path targets are normalized first:
    /// percent escapes are decoded (see [`decode_path_escapes`]) and
    /// Windows-compatible spellings like `/C:/foo` or `/c/foo` become native
    /// paths (see [`to_native_windows_path`]).
    pub fn parse_hyperlink(input: &str, path_style: PathStyle) -> Result<Self> {
        if let Some(target) = bare_path_target(input, path_style) {
            return parse_hyperlink_path(target, path_style, DecodePercentEscapes::Yes)
                .with_context(|| format!("Invalid hyperlink path target: {input}"));
        }
        Self::parse(input, path_style)
    }

    /// Returns the literal (un-decoded) interpretation of a bare-path
    /// hyperlink target, for files whose names literally contain an escape
    /// sequence (e.g. `a%20b.rs`). Returns `None` when this wouldn't differ
    /// from [`MentionUri::parse_hyperlink`], including for URLs, whose
    /// escapes are unambiguous.
    pub fn parse_hyperlink_literal(input: &str, path_style: PathStyle) -> Option<Self> {
        let target = bare_path_target(input, path_style)?;
        let (path_input, _) = split_path_fragment(target);
        if !matches!(decode_path_escapes(path_input), Cow::Owned(_)) {
            return None;
        }
        parse_hyperlink_path(target, path_style, DecodePercentEscapes::No).ok()
    }

    /// The absolute path this mention refers to, if it refers to one.
    pub fn abs_path(&self) -> Option<&Path> {
        match self {
            MentionUri::File { abs_path }
            | MentionUri::Directory { abs_path }
            | MentionUri::Symbol { abs_path, .. } => Some(abs_path),
            MentionUri::Selection { abs_path, .. } => abs_path.as_deref(),
            MentionUri::Skill {
                skill_file_path, ..
            } => Some(skill_file_path),
            MentionUri::PastedImage { .. }
            | MentionUri::Thread { .. }
            | MentionUri::Rule { .. }
            | MentionUri::Diagnostics { .. }
            | MentionUri::Fetch { .. }
            | MentionUri::TerminalSelection { .. }
            | MentionUri::GitDiff { .. }
            | MentionUri::MergeConflict { .. } => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            MentionUri::File { abs_path, .. } | MentionUri::Directory { abs_path, .. } => abs_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            MentionUri::PastedImage { name } => name.clone(),
            MentionUri::Symbol { name, .. } => name.clone(),
            MentionUri::Thread { name, .. } => name.clone(),
            MentionUri::Rule { name, .. } => name.clone(),
            MentionUri::Diagnostics { .. } => "Diagnostics".to_string(),
            MentionUri::TerminalSelection { line_count } => {
                if *line_count == 1 {
                    "Terminal (1 line)".to_string()
                } else {
                    format!("Terminal ({} lines)", line_count)
                }
            }
            MentionUri::GitDiff { base_ref } => format!("Branch Diff ({})", base_ref),
            MentionUri::MergeConflict { file_path } => {
                let name = Path::new(file_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                format!("Merge Conflict ({name})")
            }
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => selection_name(path.as_deref(), line_range),
            MentionUri::Fetch { url } => url.to_string(),
            MentionUri::Skill { name, .. } => name.clone(),
        }
    }

    /// Returns a label for this mention at the given disambiguation `detail`
    /// level. `detail == 0` is the base name returned by [`Self::name`]; higher
    /// levels include progressively more context (e.g. additional parent path
    /// components for files, or the source for skills) until a fixed point is
    /// reached. Intended to be driven by [`util::disambiguate::compute_disambiguation_details`].
    pub fn disambiguated_name(&self, detail: usize) -> String {
        if detail == 0 {
            return self.name();
        }

        match self {
            MentionUri::Skill { name, source, .. } => {
                if source.is_empty() {
                    // Must match `SkillSource::display_label()` in agent_skills.
                    format!("{} (global)", name)
                } else {
                    format!("{} ({})", name, source)
                }
            }
            MentionUri::File { abs_path, .. } | MentionUri::Directory { abs_path, .. } => {
                project::path_suffix(abs_path, detail)
            }
            _ => self.name(),
        }
    }

    pub fn tooltip_text(&self) -> Option<SharedString> {
        match self {
            MentionUri::File { abs_path } | MentionUri::Directory { abs_path } => {
                Some(abs_path.to_string_lossy().into_owned().into())
            }
            MentionUri::Symbol {
                abs_path,
                line_range,
                ..
            } => Some(
                format!(
                    "{}:{}-{}",
                    abs_path.display(),
                    line_range.start(),
                    line_range.end()
                )
                .into(),
            ),
            MentionUri::Selection {
                abs_path: Some(path),
                line_range,
                ..
            } => Some(
                format!(
                    "{}:{}-{}",
                    path.display(),
                    line_range.start(),
                    line_range.end()
                )
                .into(),
            ),
            MentionUri::Skill {
                skill_file_path, ..
            } => Some(skill_file_path.to_string_lossy().into_owned().into()),
            _ => None,
        }
    }

    pub fn icon_path(&self, cx: &mut App) -> SharedString {
        match self {
            MentionUri::File { abs_path } => {
                FileIcons::get_icon(abs_path, cx).unwrap_or_else(|| IconName::File.path().into())
            }
            MentionUri::PastedImage { .. } => IconName::Image.path().into(),
            MentionUri::Directory { abs_path } => FileIcons::get_folder_icon(false, abs_path, cx)
                .unwrap_or_else(|| IconName::Folder.path().into()),
            MentionUri::Symbol { .. } => IconName::Code.path().into(),
            MentionUri::Thread { .. } => IconName::Thread.path().into(),
            MentionUri::Rule { .. } => IconName::Reader.path().into(),
            MentionUri::Diagnostics { .. } => IconName::Warning.path().into(),
            MentionUri::TerminalSelection { .. } => IconName::Terminal.path().into(),
            MentionUri::Selection { .. } => IconName::Reader.path().into(),
            MentionUri::Fetch { .. } => IconName::ToolWeb.path().into(),
            MentionUri::GitDiff { .. } => IconName::GitBranch.path().into(),
            MentionUri::MergeConflict { .. } => IconName::GitMergeConflict.path().into(),
            MentionUri::Skill { .. } => IconName::Sparkle.path().into(),
        }
    }

    pub fn as_link<'a>(&'a self) -> MentionLink<'a> {
        MentionLink(self)
    }

    pub fn to_uri(&self) -> Url {
        match self {
            MentionUri::File { abs_path } => {
                let mut url = Url::parse("file:///").unwrap();
                url.set_path(&abs_path.to_string_lossy());
                url
            }
            MentionUri::PastedImage { name } => {
                let mut url = Url::parse("zed:///agent/pasted-image").unwrap();
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::Directory { abs_path } => {
                let mut url = Url::parse("file:///").unwrap();
                let mut path = abs_path.to_string_lossy().into_owned();
                if !path.ends_with('/') && !path.ends_with('\\') {
                    path.push('/');
                }
                url.set_path(&path);
                url
            }
            MentionUri::Symbol {
                abs_path,
                name,
                line_range,
                ..
            } => {
                let mut url = Url::parse("file:///").unwrap();
                url.set_path(&abs_path.to_string_lossy());
                url.query_pairs_mut().append_pair("symbol", name);
                url.set_fragment(Some(&format!(
                    "L{}:{}",
                    line_range.start() + 1,
                    line_range.end() + 1
                )));
                url
            }
            MentionUri::Selection {
                abs_path,
                line_range,
                column,
            } => {
                let mut url = if let Some(path) = abs_path {
                    let mut url = Url::parse("file:///").unwrap();
                    url.set_path(&path.to_string_lossy());
                    url
                } else {
                    let mut url = Url::parse("zed:///").unwrap();
                    url.set_path("/agent/untitled-buffer");
                    url
                };
                if let Some(column) = column {
                    url.query_pairs_mut()
                        .append_pair("column", &(column + 1).to_string());
                }
                url.set_fragment(Some(&format!(
                    "L{}:{}",
                    line_range.start() + 1,
                    line_range.end() + 1
                )));
                url
            }
            MentionUri::Thread { name, id } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path(&format!("/agent/thread/{id}"));
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::Rule { id, name } => {
                let mut url = Url::parse("zed:///").unwrap();
                let rule_id = id
                    .get("User")
                    .and_then(|user| user.get("uuid"))
                    .and_then(|uuid| uuid.as_str())
                    .unwrap_or_default();
                url.set_path(&format!("/agent/rule/{rule_id}"));
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::Diagnostics {
                include_errors,
                include_warnings,
            } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path("/agent/diagnostics");
                if *include_warnings {
                    url.query_pairs_mut()
                        .append_pair("include_warnings", "true");
                }
                if !include_errors {
                    url.query_pairs_mut().append_pair("include_errors", "false");
                }
                url
            }
            MentionUri::Fetch { url } => url.clone(),
            MentionUri::TerminalSelection { line_count } => {
                let mut url = Url::parse("zed:///agent/terminal-selection").unwrap();
                url.query_pairs_mut()
                    .append_pair("lines", &line_count.to_string());
                url
            }
            MentionUri::GitDiff { base_ref } => {
                let mut url = Url::parse("zed:///agent/git-diff").unwrap();
                url.query_pairs_mut().append_pair("base", base_ref);
                url
            }
            MentionUri::MergeConflict { file_path } => {
                let mut url = Url::parse("zed:///agent/merge-conflict").unwrap();
                url.query_pairs_mut().append_pair("path", file_path);
                url
            }
            MentionUri::Skill {
                name,
                source,
                skill_file_path,
            } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path("/agent/skill");
                url.query_pairs_mut()
                    .append_pair("name", name)
                    .append_pair("source", source)
                    .append_pair("path", &skill_file_path.to_string_lossy());
                url
            }
        }
    }
}

pub struct MentionLink<'a>(&'a MentionUri);

impl fmt::Display for MentionLink<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[@{}]({})", self.0.name(), self.0.to_uri())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DecodePercentEscapes {
    Yes,
    No,
}

fn parse_line_range(fragment: &str) -> Result<RangeInclusive<u32>> {
    let range = fragment.strip_prefix("L").unwrap_or(fragment);

    let (start, end) = if let Some((start, end)) = range.split_once(":") {
        (start, end)
    } else if let Some((start, end)) = range.split_once("-") {
        // Also handle L10-20 or L10-L20 format
        (start, end.strip_prefix("L").unwrap_or(end))
    } else {
        // Single line number like L1872 - treat as a range of one line
        (range, range)
    };

    let start_line = start
        .parse::<u32>()
        .context("Parsing line range start")?
        .checked_sub(1)
        .context("Line numbers should be 1-based")?;
    let end_line = end
        .parse::<u32>()
        .context("Parsing line range end")?
        .checked_sub(1)
        .context("Line numbers should be 1-based")?;

    Ok(start_line..=end_line)
}

/// Returns the mention target as a bare absolute path (not a URL), with the
/// backticks agents sometimes add stripped.
fn bare_path_target(input: &str, path_style: PathStyle) -> Option<&str> {
    let input = input
        .strip_prefix('`')
        .and_then(|input| input.strip_suffix('`'))
        .unwrap_or(input);
    (is_absolute(input, path_style) && !input.contains("://")).then_some(input)
}

fn split_path_fragment(input: &str) -> (&str, Option<&str>) {
    input
        .split_once('#')
        .map_or((input, None), |(path, fragment)| (path, Some(fragment)))
}

fn parse_absolute_path(input: &str) -> Result<MentionUri> {
    let (path_input, fragment) = split_path_fragment(input);
    absolute_path_mention(path_input, fragment)
}

/// Like [`parse_absolute_path`], but normalizes hyperlink spellings first.
fn parse_hyperlink_path(
    input: &str,
    path_style: PathStyle,
    decode_escapes: DecodePercentEscapes,
) -> Result<MentionUri> {
    let (path_input, fragment) = split_path_fragment(input);
    let path_input = normalize_path_mention(path_input, path_style, decode_escapes);
    absolute_path_mention(&path_input, fragment)
}

fn absolute_path_mention(path_input: &str, fragment: Option<&str>) -> Result<MentionUri> {
    if let Some(fragment) = fragment.and_then(|fragment| parse_line_range(fragment).ok()) {
        return Ok(MentionUri::Selection {
            abs_path: Some(path_input.into()),
            line_range: fragment,
            column: None,
        });
    }

    let path_with_position = PathWithPosition::parse_str(path_input);
    let abs_path = path_with_position.path;
    if let Some(row) = path_with_position.row {
        let line = row
            .checked_sub(1)
            .context("Line numbers should be 1-based")?;
        Ok(MentionUri::Selection {
            abs_path: Some(abs_path),
            line_range: line..=line,
            column: path_with_position
                .column
                .map(|column| column.saturating_sub(1)),
        })
    } else {
        Ok(MentionUri::File { abs_path })
    }
}

fn normalize_path_mention(
    input: &str,
    path_style: PathStyle,
    decode_escapes: DecodePercentEscapes,
) -> Cow<'_, str> {
    let decoded = match decode_escapes {
        DecodePercentEscapes::Yes => decode_path_escapes(input),
        DecodePercentEscapes::No => Cow::Borrowed(input),
    };
    if !path_style.is_windows() {
        return decoded;
    }
    match to_native_windows_path(&decoded) {
        Some(native) => Cow::Owned(native),
        None => decoded,
    }
}

/// Decodes percent escapes in a path, leaving separator escapes (`%2F`,
/// `%5C`) encoded so decoding can't change which directories the path
/// traverses. Invalid sequences and non-UTF-8 results leave the input
/// unchanged. Returns `Cow::Owned` iff decoding changed the input
/// (`parse_hyperlink_literal` relies on this).
fn decode_path_escapes(input: &str) -> Cow<'_, str> {
    fn hex_digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    if !input.contains('%') {
        return Cow::Borrowed(input);
    }
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && let Some(high) = bytes.get(index + 1).copied().and_then(hex_digit)
            && let Some(low) = bytes.get(index + 2).copied().and_then(hex_digit)
        {
            let byte = (high << 4) | low;
            if byte != b'/' && byte != b'\\' {
                decoded.push(byte);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    if decoded == bytes {
        return Cow::Borrowed(input);
    }
    match String::from_utf8(decoded) {
        Ok(decoded) => Cow::Owned(decoded),
        Err(_) => Cow::Borrowed(input),
    }
}

/// Converts Windows-compatible path spellings into a native Windows path,
/// normalizing separators to backslashes and drive letters to uppercase so
/// parsed paths compare equal to worktree paths. Returns `None` when the
/// input needs no changes.
fn to_native_windows_path(path: &str) -> Option<String> {
    fn join_drive(drive: char, rest: &str) -> String {
        format!(
            "{}:\\{}",
            drive.to_ascii_uppercase(),
            rest.replace('/', "\\")
        )
    }

    if let Some(rest) = path.strip_prefix('/') {
        // URL-style path with a leading slash before the drive: `/C:/foo`.
        let mut chars = rest.chars();
        if let (Some(drive), Some(':'), Some('/' | '\\')) =
            (chars.next(), chars.next(), chars.next())
            && drive.is_ascii_alphabetic()
        {
            return Some(join_drive(drive, chars.as_str()));
        }

        // MSYS/Git Bash style: `/c/foo`. Lowercase-only, since that's what
        // those shells emit and uppercase risks misreading real directories.
        let mut chars = rest.chars();
        if let (Some(drive), Some('/' | '\\')) = (chars.next(), chars.next())
            && drive.is_ascii_lowercase()
        {
            return Some(join_drive(drive, chars.as_str()));
        }
    }

    // A native path with a drive prefix: uppercase the drive and normalize
    // separators, e.g. `c:/foo` or `c:\foo`.
    let mut chars = path.chars();
    if let (Some(drive), Some(':')) = (chars.next(), chars.next())
        && drive.is_ascii_alphabetic()
    {
        if drive.is_ascii_uppercase() && !path.contains('/') {
            return None;
        }
        return Some(format!(
            "{}:{}",
            drive.to_ascii_uppercase(),
            chars.as_str().replace('/', "\\")
        ));
    }

    if path.contains('/') {
        return Some(path.replace('/', "\\"));
    }

    None
}

fn default_include_errors() -> bool {
    true
}

/// Placeholder rule `id` for legacy mentions missing one, shaped so older Zed
/// versions can still deserialize it as a `prompt_store::PromptId`.
fn default_deprecated_rule_id() -> serde_json::Value {
    serde_json::json!({ "User": { "uuid": "00000000-0000-0000-0000-000000000000" } })
}

fn query_param(url: &Url, name: &'static str) -> Option<String> {
    url.query_pairs()
        .find_map(|(key, value)| (key == name).then(|| value.to_string()))
}

fn single_query_param(url: &Url, name: &'static str) -> Result<Option<String>> {
    let pairs = url.query_pairs().collect::<Vec<_>>();
    match pairs.as_slice() {
        [] => Ok(None),
        [(k, v)] => {
            if k != name {
                bail!("invalid query parameter")
            }

            Ok(Some(v.to_string()))
        }
        _ => bail!("too many query pairs"),
    }
}

pub fn selection_name(path: Option<&Path>, line_range: &RangeInclusive<u32>) -> String {
    format!(
        "{} ({}:{})",
        path.and_then(|path| path.file_name())
            .unwrap_or("Untitled".as_ref())
            .display(),
        *line_range.start() + 1,
        *line_range.end() + 1
    )
}

/// Formats a 0-based, inclusive line range as a 1-based path suffix: `:5` for a
/// single line or `:5-9` for a span. Used for `path:line` mentions in text.
pub fn line_range_suffix(line_range: &RangeInclusive<u32>) -> String {
    let start = *line_range.start() + 1;
    let end = *line_range.end() + 1;
    if start == end {
        format!(":{start}")
    } else {
        format!(":{start}-{end}")
    }
}

#[cfg(test)]
mod tests {
    use util::{path, uri};

    use super::*;

    #[test]
    fn test_parse_file_uri() {
        let file_uri = uri!("file:///path/to/file.rs");
        let parsed = MentionUri::parse(file_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, Path::new(path!("/path/to/file.rs")));
            }
            _ => panic!("Expected File variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), file_uri);
    }

    #[test]
    fn test_parse_directory_uri() {
        let file_uri = uri!("file:///path/to/dir/");
        let parsed = MentionUri::parse(file_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Directory { abs_path } => {
                assert_eq!(abs_path, Path::new(path!("/path/to/dir/")));
            }
            _ => panic!("Expected Directory variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), file_uri);
    }

    #[test]
    fn test_parse_file_uris_use_native_separators_on_windows() {
        let parsed = MentionUri::parse("file:///C:/path/to/file.rs", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("C:\\path\\to\\file.rs"));
            }
            other => panic!("Expected File variant, got {other:?}"),
        }

        let parsed = MentionUri::parse("file:///C:/path/to/dir/", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::Directory { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("C:\\path\\to\\dir\\"));
            }
            other => panic!("Expected Directory variant, got {other:?}"),
        }

        let parsed = MentionUri::parse(
            "file:///C:/path/to/file.rs?symbol=MySymbol#L10:20",
            PathStyle::Windows,
        )
        .unwrap();
        match parsed {
            MentionUri::Symbol { abs_path, .. } => {
                assert_eq!(abs_path, PathBuf::from("C:\\path\\to\\file.rs"));
            }
            other => panic!("Expected Symbol variant, got {other:?}"),
        }

        let parsed =
            MentionUri::parse("file:///C:/path/to/file.rs#L5:15", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::Selection {
                abs_path: Some(abs_path),
                ..
            } => {
                assert_eq!(abs_path, PathBuf::from("C:\\path\\to\\file.rs"));
            }
            other => panic!("Expected Selection variant, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_file_uri_with_spaces() {
        let parsed =
            MentionUri::parse("file:///C:/path%20with%20space/file.rs", PathStyle::Windows)
                .unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("C:\\path with space\\file.rs"));
            }
            other => panic!("Expected File variant, got {other:?}"),
        }
        assert_eq!(
            MentionUri::File {
                abs_path: PathBuf::from("C:\\path with space\\file.rs")
            }
            .to_uri()
            .to_string(),
            "file:///C:/path%20with%20space/file.rs"
        );
    }

    #[test]
    fn test_parse_windows_drive_path_with_leading_slash_and_line() {
        let parsed = MentionUri::parse_hyperlink(
            "/C:/Projects/Example Workspace/Cargo.toml:2",
            PathStyle::Windows,
        )
        .unwrap();
        match parsed {
            MentionUri::Selection {
                abs_path: Some(abs_path),
                line_range,
                ..
            } => {
                assert_eq!(
                    abs_path,
                    PathBuf::from("C:\\Projects\\Example Workspace\\Cargo.toml")
                );
                assert_eq!(line_range, 1..=1);
            }
            other => panic!("Expected Selection variant, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_windows_path_with_percent_escaped_spaces_and_line() {
        let parsed = MentionUri::parse_hyperlink(
            "C:\\Projects\\Example%20Workspace\\path\\to\\filename.ext:42",
            PathStyle::Windows,
        )
        .unwrap();
        match parsed {
            MentionUri::Selection {
                abs_path: Some(abs_path),
                line_range,
                ..
            } => {
                assert_eq!(
                    abs_path,
                    PathBuf::from("C:\\Projects\\Example Workspace\\path\\to\\filename.ext")
                );
                assert_eq!(line_range, 41..=41);
            }
            other => panic!("Expected Selection variant, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_windows_compat_path_with_spaces() {
        let parsed = MentionUri::parse_hyperlink(
            "/c/Projects/Example Workspace/AGENTS.md",
            PathStyle::Windows,
        )
        .unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(
                    abs_path,
                    PathBuf::from("C:\\Projects\\Example Workspace\\AGENTS.md")
                );
            }
            other => panic!("Expected File variant, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_windows_drive_path_with_leading_slash_and_fragment_line() {
        let parsed =
            MentionUri::parse_hyperlink("/C:/Projects/Cargo.toml#L4", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::Selection {
                abs_path: Some(abs_path),
                line_range,
                ..
            } => {
                assert_eq!(abs_path, PathBuf::from("C:\\Projects\\Cargo.toml"));
                assert_eq!(line_range, 3..=3);
            }
            other => panic!("Expected Selection variant, got {other:?}"),
        }
    }

    #[test]
    fn test_windows_drive_path_with_leading_slash_round_trips() {
        let parsed = MentionUri::parse_hyperlink("/C:/dir/file.rs", PathStyle::Windows).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("C:\\dir\\file.rs")
            }
        );
        let uri = parsed.to_uri().to_string();
        assert_eq!(uri, "file:///C:/dir/file.rs");
        assert_eq!(MentionUri::parse(&uri, PathStyle::Windows).unwrap(), parsed);
    }

    #[test]
    fn test_parse_windows_unc_path() {
        let parsed =
            MentionUri::parse_hyperlink("//server/share/dir/file.rs", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("\\\\server\\share\\dir\\file.rs"));
            }
            other => panic!("Expected File variant, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_windows_drive_letters_are_uppercased() {
        for input in [
            "file:///c:/foo/bar.rs",
            "/c:/foo/bar.rs",
            "/c/foo/bar.rs",
            "c:\\foo\\bar.rs",
            "c:/foo/bar.rs",
        ] {
            let parsed = MentionUri::parse_hyperlink(input, PathStyle::Windows).unwrap();
            assert_eq!(
                parsed,
                MentionUri::File {
                    abs_path: PathBuf::from("C:\\foo\\bar.rs")
                },
                "input: {input}"
            );
        }
    }

    #[test]
    fn test_msys_style_paths_require_lowercase_drive() {
        // Uppercase `/C/foo` is more likely a real directory than a drive.
        let parsed = MentionUri::parse_hyperlink("/C/Users/readme.md", PathStyle::Windows).unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("\\C\\Users\\readme.md"));
            }
            other => panic!("Expected File variant, got {other:?}"),
        }
    }

    #[test]
    fn test_posix_paths_are_not_rewritten_as_windows_drives() {
        let parsed =
            MentionUri::parse_hyperlink("/c/Projects/AGENTS.md", PathStyle::Posix).unwrap();
        match parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, PathBuf::from("/c/Projects/AGENTS.md"));
            }
            other => panic!("Expected File variant, got {other:?}"),
        }
    }

    #[test]
    fn test_hyperlink_percent_escapes_are_decoded() {
        let parsed = MentionUri::parse_hyperlink("/tmp/a%20b.rs", PathStyle::Posix).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("/tmp/a b.rs")
            }
        );

        // Invalid escape sequences pass through unchanged.
        let parsed =
            MentionUri::parse_hyperlink("C:\\dir\\100%_done.txt", PathStyle::Windows).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("C:\\dir\\100%_done.txt")
            }
        );

        // Separator escapes stay encoded (no introduced path traversal).
        let parsed = MentionUri::parse_hyperlink("/tmp/a%2Fb.rs", PathStyle::Posix).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("/tmp/a%2Fb.rs")
            }
        );
        let parsed =
            MentionUri::parse_hyperlink("/tmp/..%2F..%2Fsecret", PathStyle::Posix).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("/tmp/..%2F..%2Fsecret")
            }
        );
    }

    #[test]
    fn test_parse_keeps_bare_path_targets_verbatim() {
        let parsed = MentionUri::parse("/tmp/a%20b.rs", PathStyle::Posix).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("/tmp/a%20b.rs")
            }
        );

        let parsed = MentionUri::parse("/c/Projects/AGENTS.md", PathStyle::Windows).unwrap();
        assert_eq!(
            parsed,
            MentionUri::File {
                abs_path: PathBuf::from("/c/Projects/AGENTS.md")
            }
        );
    }

    #[test]
    fn test_parse_hyperlink_literal_keeps_percent_escapes() {
        let literal =
            MentionUri::parse_hyperlink_literal("/tmp/a%20b.rs", PathStyle::Posix).unwrap();
        assert_eq!(
            literal,
            MentionUri::File {
                abs_path: PathBuf::from("/tmp/a%20b.rs")
            }
        );

        // Line suffixes still parse.
        let literal =
            MentionUri::parse_hyperlink_literal("/tmp/a%20b.rs:42", PathStyle::Posix).unwrap();
        assert_eq!(
            literal,
            MentionUri::Selection {
                abs_path: Some(PathBuf::from("/tmp/a%20b.rs")),
                line_range: 41..=41,
                column: None,
            }
        );

        // Windows normalization still applies.
        let literal =
            MentionUri::parse_hyperlink_literal("/C:/dir/a%20b.rs", PathStyle::Windows).unwrap();
        assert_eq!(
            literal,
            MentionUri::File {
                abs_path: PathBuf::from("C:\\dir\\a%20b.rs")
            }
        );
    }

    #[test]
    fn test_parse_hyperlink_literal_returns_none_when_unambiguous() {
        // No percent escapes: identical to `parse_hyperlink`.
        assert_eq!(
            MentionUri::parse_hyperlink_literal("/tmp/a b.rs", PathStyle::Posix),
            None
        );
        // Invalid escape sequences are also left alone by `parse_hyperlink`.
        assert_eq!(
            MentionUri::parse_hyperlink_literal("/tmp/100%_done.txt", PathStyle::Posix),
            None
        );
        // Separator escapes are never decoded, so they're not ambiguous.
        assert_eq!(
            MentionUri::parse_hyperlink_literal("/tmp/a%2Fb.rs", PathStyle::Posix),
            None
        );
        // URLs are spec-encoded, not ambiguous.
        assert_eq!(
            MentionUri::parse_hyperlink_literal("file:///tmp/a%20b.rs", PathStyle::Posix),
            None
        );
        // Relative paths are not bare-path mentions.
        assert_eq!(
            MentionUri::parse_hyperlink_literal("tmp/a%20b.rs", PathStyle::Posix),
            None
        );
    }

    #[test]
    fn test_to_directory_uri_without_slash() {
        let uri = MentionUri::Directory {
            abs_path: PathBuf::from(path!("/path/to/dir/")),
        };
        let expected = uri!("file:///path/to/dir/");
        assert_eq!(uri.to_uri().to_string(), expected);
    }

    #[test]
    fn test_directory_uri_round_trip_without_trailing_slash() {
        let uri = MentionUri::Directory {
            abs_path: PathBuf::from(path!("/path/to/dir")),
        };
        let serialized = uri.to_uri().to_string();
        assert!(serialized.ends_with('/'), "directory URI must end with /");
        let parsed = MentionUri::parse(&serialized, PathStyle::local()).unwrap();
        assert!(
            matches!(parsed, MentionUri::Directory { .. }),
            "expected Directory variant, got {:?}",
            parsed
        );
    }

    #[test]
    fn test_parse_symbol_uri() {
        let symbol_uri = uri!("file:///path/to/file.rs?symbol=MySymbol#L10:20");
        let parsed = MentionUri::parse(symbol_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Symbol {
                abs_path: path,
                name,
                line_range,
                ..
            } => {
                assert_eq!(path, Path::new(path!("/path/to/file.rs")));
                assert_eq!(name, "MySymbol");
                assert_eq!(line_range.start(), &9);
                assert_eq!(line_range.end(), &19);
            }
            _ => panic!("Expected Symbol variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), symbol_uri);
    }

    #[test]
    fn test_parse_selection_uri() {
        let selection_uri = uri!("file:///path/to/file.rs#L5:15");
        let parsed = MentionUri::parse(selection_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new(path!("/path/to/file.rs")));
                assert_eq!(line_range.start(), &4);
                assert_eq!(line_range.end(), &14);
            }
            _ => panic!("Expected Selection variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), selection_uri);
    }

    #[test]
    fn test_parse_file_uri_with_non_ascii() {
        let file_uri = uri!("file:///path/to/%E6%97%A5%E6%9C%AC%E8%AA%9E.txt");
        let parsed = MentionUri::parse(file_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, Path::new(path!("/path/to/日本語.txt")));
            }
            _ => panic!("Expected File variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), file_uri);
    }

    #[test]
    fn test_parse_untitled_selection_uri() {
        let selection_uri = uri!("zed:///agent/untitled-buffer#L1:10");
        let parsed = MentionUri::parse(selection_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: None,
                line_range,
                ..
            } => {
                assert_eq!(line_range.start(), &0);
                assert_eq!(line_range.end(), &9);
            }
            _ => panic!("Expected Selection variant without path"),
        }
        assert_eq!(parsed.to_uri().to_string(), selection_uri);
    }

    #[test]
    fn test_parse_thread_uri() {
        let thread_uri = "zed:///agent/thread/session123?name=Thread+name";
        let parsed = MentionUri::parse(thread_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Thread {
                id: thread_id,
                name,
            } => {
                assert_eq!(thread_id.to_string(), "session123");
                assert_eq!(name, "Thread name");
            }
            _ => panic!("Expected Thread variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), thread_uri);
    }

    #[test]
    fn test_parse_legacy_rule_uri() {
        let rule_uri = "zed:///agent/rule/d8694ff2-90d5-4b6f-be33-33c1763acd52?name=Some+rule";
        let parsed = MentionUri::parse(rule_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Rule { name, .. } => assert_eq!(name, "Some rule"),
            _ => panic!("Expected Rule variant"),
        }
        // The id round-trips through the URI.
        assert_eq!(parsed.to_uri().to_string(), rule_uri);
    }

    #[test]
    fn test_legacy_rule_mention_preserves_id() {
        // The `id` older Zed versions require must survive a load + save.
        let json = r#"{"Rule":{"id":{"User":{"uuid":"d8694ff2-90d5-4b6f-be33-33c1763acd52"}},"name":"Some rule"}}"#;
        let parsed: MentionUri = serde_json::from_str(json).unwrap();
        match &parsed {
            MentionUri::Rule { name, .. } => assert_eq!(name, "Some rule"),
            _ => panic!("Expected Rule variant"),
        }
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert_eq!(
            reserialized["Rule"]["id"]["User"]["uuid"],
            "d8694ff2-90d5-4b6f-be33-33c1763acd52"
        );
    }

    #[test]
    fn test_legacy_rule_mention_without_id_gets_placeholder() {
        // A mention missing its id still serializes a valid id for older versions.
        let json = r#"{"Rule":{"name":"Some rule"}}"#;
        let parsed: MentionUri = serde_json::from_str(json).unwrap();
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert!(reserialized["Rule"]["id"]["User"]["uuid"].is_string());
    }

    #[test]
    fn test_parse_skill_uri_round_trip() {
        let skill_uri = MentionUri::Skill {
            name: "rust-best-practices".to_string(),
            source: "my-personal-project".to_string(),
            skill_file_path: PathBuf::from(path!("/path/to/skills/rust-best-practices/SKILL.md")),
        };

        let serialized = skill_uri.to_uri().to_string();
        let parsed = MentionUri::parse(&serialized, PathStyle::local()).unwrap();

        assert_eq!(parsed, skill_uri);
    }

    #[test]
    fn test_parse_fetch_http_uri() {
        let http_uri = "http://example.com/path?query=value#fragment";
        let parsed = MentionUri::parse(http_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Fetch { url } => {
                assert_eq!(url.to_string(), http_uri);
            }
            _ => panic!("Expected Fetch variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), http_uri);
    }

    #[test]
    fn test_parse_fetch_https_uri() {
        let https_uri = "https://example.com/api/endpoint";
        let parsed = MentionUri::parse(https_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Fetch { url } => {
                assert_eq!(url.to_string(), https_uri);
            }
            _ => panic!("Expected Fetch variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), https_uri);
    }

    #[test]
    fn test_parse_diagnostics_uri() {
        let uri = "zed:///agent/diagnostics?include_warnings=true";
        let parsed = MentionUri::parse(uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Diagnostics {
                include_errors,
                include_warnings,
            } => {
                assert!(include_errors);
                assert!(include_warnings);
            }
            _ => panic!("Expected Diagnostics variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), uri);
    }

    #[test]
    fn test_parse_diagnostics_uri_warnings_only() {
        let uri = "zed:///agent/diagnostics?include_warnings=true&include_errors=false";
        let parsed = MentionUri::parse(uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Diagnostics {
                include_errors,
                include_warnings,
            } => {
                assert!(!include_errors);
                assert!(include_warnings);
            }
            _ => panic!("Expected Diagnostics variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), uri);
    }

    #[test]
    fn test_invalid_scheme() {
        assert!(MentionUri::parse("ftp://example.com", PathStyle::local()).is_err());
        assert!(MentionUri::parse("ssh://example.com", PathStyle::local()).is_err());
        assert!(MentionUri::parse("unknown://example.com", PathStyle::local()).is_err());
    }

    #[test]
    fn test_invalid_zed_path() {
        assert!(MentionUri::parse("zed:///invalid/path", PathStyle::local()).is_err());
        assert!(MentionUri::parse("zed:///agent/unknown/test", PathStyle::local()).is_err());
    }

    #[test]
    fn test_parse_absolute_file_path() {
        let file_path = path!("/path/to/file.rs");
        let parsed = MentionUri::parse(file_path, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, Path::new(file_path));
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_parse_absolute_file_path_with_row() {
        let file_path = "/path/to/file.rs:42";
        let parsed = MentionUri::parse(file_path, PathStyle::Posix).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new("/path/to/file.rs"));
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_absolute_file_path_with_row_and_column() {
        let file_path = "/path/to/file.rs:42:5";
        let parsed = MentionUri::parse(file_path, PathStyle::Posix).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                column,
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new("/path/to/file.rs"));
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
                assert_eq!(column, &Some(4));

                let parsed_again = MentionUri::parse(parsed.to_uri().as_ref(), PathStyle::Posix)
                    .expect("selection URI with column should parse");
                assert_eq!(parsed_again, parsed.clone());
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_absolute_file_path_with_fragment_line() {
        let file_path = "/path/to/file.rs#L42";
        let parsed = MentionUri::parse(file_path, PathStyle::Posix).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new("/path/to/file.rs"));
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_absolute_windows_path() {
        let file_path = "C:\\Users\\zed\\project\\main.rs";
        let parsed = MentionUri::parse(file_path, PathStyle::Windows).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, Path::new("C:\\Users\\zed\\project\\main.rs"));
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_parse_absolute_windows_file_path_with_row() {
        let file_path = "C:\\Users\\zed\\project\\main.rs:42";
        let parsed = MentionUri::parse(file_path, PathStyle::Windows).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(
                    path.as_ref().unwrap(),
                    Path::new("C:\\Users\\zed\\project\\main.rs")
                );
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_absolute_windows_file_path_with_fragment_line() {
        let file_path = "C:\\Users\\zed\\project\\main.rs#L42";
        let parsed = MentionUri::parse(file_path, PathStyle::Windows).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(
                    path.as_ref().unwrap(),
                    Path::new("C:\\Users\\zed\\project\\main.rs")
                );
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_backticked_absolute_file_path() {
        let file_path = "`/path/to/file.rs`";
        let parsed = MentionUri::parse(file_path, PathStyle::Posix).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path, Path::new("/path/to/file.rs"));
            }
            _ => panic!("Expected File variant"),
        }
    }

    #[test]
    fn test_parse_backticked_absolute_file_path_with_fragment_line() {
        let file_path = "`/path/to/file.rs#L42`";
        let parsed = MentionUri::parse(file_path, PathStyle::Posix).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new("/path/to/file.rs"));
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_backticked_absolute_windows_file_path_with_fragment_line() {
        let file_path = "`C:\\Users\\zed\\project\\main.rs#L42`";
        let parsed = MentionUri::parse(file_path, PathStyle::Windows).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(
                    path.as_ref().unwrap(),
                    Path::new("C:\\Users\\zed\\project\\main.rs")
                );
                assert_eq!(line_range.start(), &41);
                assert_eq!(line_range.end(), &41);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_single_line_number() {
        // https://github.com/zed-industries/zed/issues/46114
        let uri = uri!("file:///path/to/file.rs#L1872");
        let parsed = MentionUri::parse(uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new(path!("/path/to/file.rs")));
                assert_eq!(line_range.start(), &1871);
                assert_eq!(line_range.end(), &1871);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_dash_separated_line_range() {
        let uri = uri!("file:///path/to/file.rs#L10-20");
        let parsed = MentionUri::parse(uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new(path!("/path/to/file.rs")));
                assert_eq!(line_range.start(), &9);
                assert_eq!(line_range.end(), &19);
            }
            _ => panic!("Expected Selection variant"),
        }

        // Also test L10-L20 format
        let uri = uri!("file:///path/to/file.rs#L10-L20");
        let parsed = MentionUri::parse(uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => {
                assert_eq!(path.as_ref().unwrap(), Path::new(path!("/path/to/file.rs")));
                assert_eq!(line_range.start(), &9);
                assert_eq!(line_range.end(), &19);
            }
            _ => panic!("Expected Selection variant"),
        }
    }

    #[test]
    fn test_parse_terminal_selection_uri() {
        let terminal_uri = "zed:///agent/terminal-selection?lines=42";
        let parsed = MentionUri::parse(terminal_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::TerminalSelection { line_count } => {
                assert_eq!(*line_count, 42);
            }
            _ => panic!("Expected TerminalSelection variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), terminal_uri);
        assert_eq!(parsed.name(), "Terminal (42 lines)");

        // Test single line
        let single_line_uri = "zed:///agent/terminal-selection?lines=1";
        let parsed_single = MentionUri::parse(single_line_uri, PathStyle::local()).unwrap();
        assert_eq!(parsed_single.name(), "Terminal (1 line)");
    }

    #[test]
    fn test_disambiguated_name() {
        // Two files with the same name — should disambiguate with parent dir
        let file_a = MentionUri::File {
            abs_path: PathBuf::from(path!("/project/src/README.md")),
        };
        let file_b = MentionUri::File {
            abs_path: PathBuf::from(path!("/project/docs/README.md")),
        };
        assert_eq!(file_a.name(), "README.md");
        assert_eq!(file_b.name(), "README.md");
        assert_eq!(file_a.disambiguated_name(0), "README.md");
        assert_eq!(file_a.disambiguated_name(1), "src/README.md");
        assert_eq!(file_b.disambiguated_name(1), "docs/README.md");

        // Files that still collide at one parent should grow further.
        let deep_a = MentionUri::File {
            abs_path: PathBuf::from(path!("/a/src/foo.rs")),
        };
        let deep_b = MentionUri::File {
            abs_path: PathBuf::from(path!("/b/src/foo.rs")),
        };
        assert_eq!(deep_a.disambiguated_name(1), "src/foo.rs");
        assert_eq!(deep_b.disambiguated_name(1), "src/foo.rs");
        assert_eq!(deep_a.disambiguated_name(2), "a/src/foo.rs");
        assert_eq!(deep_b.disambiguated_name(2), "b/src/foo.rs");

        // Two skills with the same name — should disambiguate with source
        let global_skill = MentionUri::Skill {
            name: "create-skill".into(),
            source: "".into(),
            skill_file_path: PathBuf::from("/global/create-skill/SKILL.md"),
        };
        let project_skill = MentionUri::Skill {
            name: "create-skill".into(),
            source: "my-project".into(),
            skill_file_path: PathBuf::from("/project/create-skill/SKILL.md"),
        };
        assert_eq!(global_skill.name(), "create-skill");
        assert_eq!(global_skill.disambiguated_name(0), "create-skill");
        assert_eq!(global_skill.disambiguated_name(1), "create-skill (global)");
        assert_eq!(
            project_skill.disambiguated_name(1),
            "create-skill (my-project)"
        );

        // A type without special disambiguation (Thread) — detail has no effect
        // (the value is a fixed point so the disambiguation loop terminates).
        let thread = MentionUri::Thread {
            id: acp::SessionId::new("123"),
            name: "My Thread".into(),
        };
        assert_eq!(thread.disambiguated_name(0), "My Thread");
        assert_eq!(thread.disambiguated_name(1), "My Thread");
        assert_eq!(thread.disambiguated_name(5), "My Thread");

        // Edge case: file at filesystem root has no parent to show
        let root_file = MentionUri::File {
            abs_path: PathBuf::from(path!("/README.md")),
        };
        assert_eq!(root_file.disambiguated_name(1), "README.md");
        assert_eq!(root_file.disambiguated_name(5), "README.md");
    }
}
