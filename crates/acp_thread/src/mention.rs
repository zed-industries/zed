use agent_client_protocol as acp;
use anyhow::{Context as _, Result, bail};
use file_icons::FileIcons;
use prompt_store::{PromptId, UserPromptId};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
};
use ui::{App, IconName, SharedString};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MentionUri {
    File {
        abs_path: PathBuf,
    },
    Directory {
        abs_path: PathBuf,
    },
    Symbol {
        path: PathBuf,
        name: String,
        line_range: Range<u32>,
    },
    Thread {
        id: acp::SessionId,
        name: String,
    },
    TextThread {
        path: PathBuf,
        name: String,
    },
    Rule {
        id: PromptId,
        name: String,
    },
    Selection {
        path: PathBuf,
        line_range: Range<u32>,
    },
    Fetch {
        url: Url,
    },
}

impl MentionUri {
    pub fn parse(input: &str) -> Result<Self> {
        let url = url::Url::parse(input)?;
        let path = url.path();
        match url.scheme() {
            "file" => {
                let path = url.to_file_path().ok().context("Extracting file path")?;
                if let Some(fragment) = url.fragment() {
                    let range = fragment
                        .strip_prefix("L")
                        .context("Line range must start with \"L\"")?;
                    let (start, end) = range
                        .split_once(":")
                        .context("Line range must use colon as separator")?;
                    let line_range = start
                        .parse::<u32>()
                        .context("Parsing line range start")?
                        .checked_sub(1)
                        .context("Line numbers should be 1-based")?
                        ..end
                            .parse::<u32>()
                            .context("Parsing line range end")?
                            .checked_sub(1)
                            .context("Line numbers should be 1-based")?;
                    if let Some(name) = single_query_param(&url, "symbol")? {
                        Ok(Self::Symbol {
                            name,
                            path,
                            line_range,
                        })
                    } else {
                        Ok(Self::Selection { path, line_range })
                    }
                } else if input.ends_with("/") {
                    Ok(Self::Directory { abs_path: path })
                } else {
                    Ok(Self::File { abs_path: path })
                }
            }
            "zed" => {
                if let Some(thread_id) = path.strip_prefix("/agent/thread/") {
                    let name = single_query_param(&url, "name")?.context("Missing thread name")?;
                    Ok(Self::Thread {
                        id: acp::SessionId(thread_id.into()),
                        name,
                    })
                } else if let Some(path) = path.strip_prefix("/agent/text-thread/") {
                    let name = single_query_param(&url, "name")?.context("Missing thread name")?;
                    Ok(Self::TextThread {
                        path: path.into(),
                        name,
                    })
                } else if let Some(rule_id) = path.strip_prefix("/agent/rule/") {
                    let name = single_query_param(&url, "name")?.context("Missing rule name")?;
                    let rule_id = UserPromptId(rule_id.parse()?);
                    Ok(Self::Rule {
                        id: rule_id.into(),
                        name,
                    })
                } else {
                    bail!("invalid zed url: {:?}", input);
                }
            }
            "http" | "https" => Ok(MentionUri::Fetch { url }),
            other => bail!("unrecognized scheme {:?}", other),
        }
    }

    pub fn name(&self) -> String {
        match self {
            MentionUri::File { abs_path, .. } | MentionUri::Directory { abs_path, .. } => abs_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            MentionUri::Symbol { name, .. } => name.clone(),
            MentionUri::Thread { name, .. } => name.clone(),
            MentionUri::TextThread { name, .. } => name.clone(),
            MentionUri::Rule { name, .. } => name.clone(),
            MentionUri::Selection {
                path, line_range, ..
            } => selection_name(path, line_range),
            MentionUri::Fetch { url } => url.to_string(),
        }
    }

    pub fn icon_path(&self, cx: &mut App) -> SharedString {
        match self {
            MentionUri::File { abs_path } => {
                FileIcons::get_icon(abs_path, cx).unwrap_or_else(|| IconName::File.path().into())
            }
            MentionUri::Directory { .. } => FileIcons::get_folder_icon(false, cx)
                .unwrap_or_else(|| IconName::Folder.path().into()),
            MentionUri::Symbol { .. } => IconName::Code.path().into(),
            MentionUri::Thread { .. } => IconName::Thread.path().into(),
            MentionUri::TextThread { .. } => IconName::Thread.path().into(),
            MentionUri::Rule { .. } => IconName::Reader.path().into(),
            MentionUri::Selection { .. } => IconName::Reader.path().into(),
            MentionUri::Fetch { .. } => IconName::ToolWeb.path().into(),
        }
    }

    pub fn as_link<'a>(&'a self) -> MentionLink<'a> {
        MentionLink(self)
    }

    pub fn to_uri(&self) -> Url {
        match self {
            MentionUri::File { abs_path } => {
                Url::from_file_path(abs_path).expect("mention path should be absolute")
            }
            MentionUri::Directory { abs_path } => {
                Url::from_directory_path(abs_path).expect("mention path should be absolute")
            }
            MentionUri::Symbol {
                path,
                name,
                line_range,
            } => {
                let mut url = Url::from_file_path(path).expect("mention path should be absolute");
                url.query_pairs_mut().append_pair("symbol", name);
                url.set_fragment(Some(&format!(
                    "L{}:{}",
                    line_range.start + 1,
                    line_range.end + 1
                )));
                url
            }
            MentionUri::Selection { path, line_range } => {
                let mut url = Url::from_file_path(path).expect("mention path should be absolute");
                url.set_fragment(Some(&format!(
                    "L{}:{}",
                    line_range.start + 1,
                    line_range.end + 1
                )));
                url
            }
            MentionUri::Thread { name, id } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path(&format!("/agent/thread/{id}"));
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::TextThread { path, name } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path(&format!("/agent/text-thread/{}", path.to_string_lossy()));
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::Rule { name, id } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path(&format!("/agent/rule/{id}"));
                url.query_pairs_mut().append_pair("name", name);
                url
            }
            MentionUri::Fetch { url } => url.clone(),
        }
    }
}

impl FromStr for MentionUri {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Self::parse(s)
    }
}

pub struct MentionLink<'a>(&'a MentionUri);

impl fmt::Display for MentionLink<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[@{}]({})", self.0.name(), self.0.to_uri())
    }
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

pub fn selection_name(path: &Path, line_range: &Range<u32>) -> String {
    format!(
        "{} ({}:{})",
        path.file_name().unwrap_or_default().display(),
        line_range.start + 1,
        line_range.end + 1
    )
}

#[cfg(test)]
mod tests {
    use util::{path, uri};

    use super::*;

    #[test]
    fn test_parse_file_uri() {
        let file_uri = uri!("file:///path/to/file.rs");
        let parsed = MentionUri::parse(file_uri).unwrap();
        match &parsed {
            MentionUri::File { abs_path } => {
                assert_eq!(abs_path.to_str().unwrap(), path!("/path/to/file.rs"));
            }
            _ => panic!("Expected File variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), file_uri);
    }

    #[test]
    fn test_parse_directory_uri() {
        let file_uri = uri!("file:///path/to/dir/");
        let parsed = MentionUri::parse(file_uri).unwrap();
        match &parsed {
            MentionUri::Directory { abs_path } => {
                assert_eq!(abs_path.to_str().unwrap(), path!("/path/to/dir/"));
            }
            _ => panic!("Expected Directory variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), file_uri);
    }

    #[test]
    fn test_to_directory_uri_with_slash() {
        let uri = MentionUri::Directory {
            abs_path: PathBuf::from(path!("/path/to/dir/")),
        };
        let expected = uri!("file:///path/to/dir/");
        assert_eq!(uri.to_uri().to_string(), expected);
    }

    #[test]
    fn test_to_directory_uri_without_slash() {
        let uri = MentionUri::Directory {
            abs_path: PathBuf::from(path!("/path/to/dir")),
        };
        let expected = uri!("file:///path/to/dir/");
        assert_eq!(uri.to_uri().to_string(), expected);
    }

    #[test]
    fn test_parse_symbol_uri() {
        let symbol_uri = uri!("file:///path/to/file.rs?symbol=MySymbol#L10:20");
        let parsed = MentionUri::parse(symbol_uri).unwrap();
        match &parsed {
            MentionUri::Symbol {
                path,
                name,
                line_range,
            } => {
                assert_eq!(path.to_str().unwrap(), path!("/path/to/file.rs"));
                assert_eq!(name, "MySymbol");
                assert_eq!(line_range.start, 9);
                assert_eq!(line_range.end, 19);
            }
            _ => panic!("Expected Symbol variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), symbol_uri);
    }

    #[test]
    fn test_parse_selection_uri() {
        let selection_uri = uri!("file:///path/to/file.rs#L5:15");
        let parsed = MentionUri::parse(selection_uri).unwrap();
        match &parsed {
            MentionUri::Selection { path, line_range } => {
                assert_eq!(path.to_str().unwrap(), path!("/path/to/file.rs"));
                assert_eq!(line_range.start, 4);
                assert_eq!(line_range.end, 14);
            }
            _ => panic!("Expected Selection variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), selection_uri);
    }

    #[test]
    fn test_parse_thread_uri() {
        let thread_uri = "zed:///agent/thread/session123?name=Thread+name";
        let parsed = MentionUri::parse(thread_uri).unwrap();
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
    fn test_parse_rule_uri() {
        let rule_uri = "zed:///agent/rule/d8694ff2-90d5-4b6f-be33-33c1763acd52?name=Some+rule";
        let parsed = MentionUri::parse(rule_uri).unwrap();
        match &parsed {
            MentionUri::Rule { id, name } => {
                assert_eq!(id.to_string(), "d8694ff2-90d5-4b6f-be33-33c1763acd52");
                assert_eq!(name, "Some rule");
            }
            _ => panic!("Expected Rule variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), rule_uri);
    }

    #[test]
    fn test_parse_fetch_http_uri() {
        let http_uri = "http://example.com/path?query=value#fragment";
        let parsed = MentionUri::parse(http_uri).unwrap();
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
        let parsed = MentionUri::parse(https_uri).unwrap();
        match &parsed {
            MentionUri::Fetch { url } => {
                assert_eq!(url.to_string(), https_uri);
            }
            _ => panic!("Expected Fetch variant"),
        }
        assert_eq!(parsed.to_uri().to_string(), https_uri);
    }

    #[test]
    fn test_invalid_scheme() {
        assert!(MentionUri::parse("ftp://example.com").is_err());
        assert!(MentionUri::parse("ssh://example.com").is_err());
        assert!(MentionUri::parse("unknown://example.com").is_err());
    }

    #[test]
    fn test_invalid_zed_path() {
        assert!(MentionUri::parse("zed:///invalid/path").is_err());
        assert!(MentionUri::parse("zed:///agent/unknown/test").is_err());
    }

    #[test]
    fn test_invalid_line_range_format() {
        // Missing L prefix
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#10:20")).is_err());

        // Missing colon separator
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L1020")).is_err());

        // Invalid numbers
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L10:abc")).is_err());
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#Labc:20")).is_err());
    }

    #[test]
    fn test_invalid_query_parameters() {
        // Invalid query parameter name
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L10:20?invalid=test")).is_err());

        // Too many query parameters
        assert!(
            MentionUri::parse(uri!(
                "file:///path/to/file.rs#L10:20?symbol=test&another=param"
            ))
            .is_err()
        );
    }

    #[test]
    fn test_zero_based_line_numbers() {
        // Test that 0-based line numbers are rejected (should be 1-based)
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L0:10")).is_err());
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L1:0")).is_err());
        assert!(MentionUri::parse(uri!("file:///path/to/file.rs#L0:0")).is_err());
    }
}
