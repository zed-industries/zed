use agent_client_protocol as acp;
use anyhow::{Context as _, Result, bail};
use file_icons::FileIcons;
use prompt_store::{PromptId, UserPromptId};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::RangeInclusive,
    path::{Path, PathBuf},
};
use ui::{App, IconName, SharedString};
use url::Url;
use util::paths::PathStyle;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MentionUri {
    File {
        abs_path: PathBuf,
    },
    PastedImage,
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
    TextThread {
        path: PathBuf,
        name: String,
    },
    Rule {
        id: PromptId,
        name: String,
    },
    Selection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        abs_path: Option<PathBuf>,
        line_range: RangeInclusive<u32>,
    },
    Fetch {
        url: Url,
    },
}

impl MentionUri {
    pub fn parse(input: &str, path_style: PathStyle) -> Result<Self> {
        fn parse_line_range(fragment: &str) -> Result<RangeInclusive<u32>> {
            let range = fragment
                .strip_prefix("L")
                .context("Line range must start with \"L\"")?;
            let (start, end) = range
                .split_once(":")
                .context("Line range must use colon as separator")?;
            let range = start
                .parse::<u32>()
                .context("Parsing line range start")?
                .checked_sub(1)
                .context("Line numbers should be 1-based")?
                ..=end
                    .parse::<u32>()
                    .context("Parsing line range end")?
                    .checked_sub(1)
                    .context("Line numbers should be 1-based")?;
            Ok(range)
        }

        let url = url::Url::parse(input)?;
        let path = url.path();
        match url.scheme() {
            "file" => {
                let path = if path_style.is_windows() {
                    path.trim_start_matches("/")
                } else {
                    path
                };

                if let Some(fragment) = url.fragment() {
                    let line_range = parse_line_range(fragment)?;
                    if let Some(name) = single_query_param(&url, "symbol")? {
                        Ok(Self::Symbol {
                            name,
                            abs_path: path.into(),
                            line_range,
                        })
                    } else {
                        Ok(Self::Selection {
                            abs_path: Some(path.into()),
                            line_range,
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
                } else if path.starts_with("/agent/pasted-image") {
                    Ok(Self::PastedImage)
                } else if path.starts_with("/agent/untitled-buffer") {
                    let fragment = url
                        .fragment()
                        .context("Missing fragment for untitled buffer selection")?;
                    let line_range = parse_line_range(fragment)?;
                    Ok(Self::Selection {
                        abs_path: None,
                        line_range,
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
                    let fragment = url.fragment().context("Missing fragment for selection")?;
                    let line_range = parse_line_range(fragment)?;
                    let path =
                        single_query_param(&url, "path")?.context("Missing path for selection")?;
                    Ok(Self::Selection {
                        abs_path: Some(path.into()),
                        line_range,
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
            MentionUri::PastedImage => "Image".to_string(),
            MentionUri::Symbol { name, .. } => name.clone(),
            MentionUri::Thread { name, .. } => name.clone(),
            MentionUri::TextThread { name, .. } => name.clone(),
            MentionUri::Rule { name, .. } => name.clone(),
            MentionUri::Selection {
                abs_path: path,
                line_range,
                ..
            } => selection_name(path.as_deref(), line_range),
            MentionUri::Fetch { url } => url.to_string(),
        }
    }

    pub fn icon_path(&self, cx: &mut App) -> SharedString {
        match self {
            MentionUri::File { abs_path } => {
                FileIcons::get_icon(abs_path, cx).unwrap_or_else(|| IconName::File.path().into())
            }
            MentionUri::PastedImage => IconName::Image.path().into(),
            MentionUri::Directory { abs_path } => FileIcons::get_folder_icon(false, abs_path, cx)
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
                let mut url = Url::parse("file:///").unwrap();
                url.set_path(&abs_path.to_string_lossy());
                url
            }
            MentionUri::PastedImage => Url::parse("zed:///agent/pasted-image").unwrap(),
            MentionUri::Directory { abs_path } => {
                let mut url = Url::parse("file:///").unwrap();
                url.set_path(&abs_path.to_string_lossy());
                url
            }
            MentionUri::Symbol {
                abs_path,
                name,
                line_range,
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
            MentionUri::TextThread { path, name } => {
                let mut url = Url::parse("zed:///").unwrap();
                url.set_path(&format!(
                    "/agent/text-thread/{}",
                    path.to_string_lossy().trim_start_matches('/')
                ));
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
    fn test_to_directory_uri_without_slash() {
        let uri = MentionUri::Directory {
            abs_path: PathBuf::from(path!("/path/to/dir/")),
        };
        let expected = uri!("file:///path/to/dir/");
        assert_eq!(uri.to_uri().to_string(), expected);
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
    fn test_parse_untitled_selection_uri() {
        let selection_uri = uri!("zed:///agent/untitled-buffer#L1:10");
        let parsed = MentionUri::parse(selection_uri, PathStyle::local()).unwrap();
        match &parsed {
            MentionUri::Selection {
                abs_path: None,
                line_range,
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
    fn test_parse_rule_uri() {
        let rule_uri = "zed:///agent/rule/d8694ff2-90d5-4b6f-be33-33c1763acd52?name=Some+rule";
        let parsed = MentionUri::parse(rule_uri, PathStyle::local()).unwrap();
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
    fn test_invalid_line_range_format() {
        // Missing L prefix
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#10:20"), PathStyle::local()).is_err()
        );

        // Missing colon separator
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#L1020"), PathStyle::local()).is_err()
        );

        // Invalid numbers
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#L10:abc"), PathStyle::local()).is_err()
        );
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#Labc:20"), PathStyle::local()).is_err()
        );
    }

    #[test]
    fn test_invalid_query_parameters() {
        // Invalid query parameter name
        assert!(
            MentionUri::parse(
                uri!("file:///path/to/file.rs#L10:20?invalid=test"),
                PathStyle::local()
            )
            .is_err()
        );

        // Too many query parameters
        assert!(
            MentionUri::parse(
                uri!("file:///path/to/file.rs#L10:20?symbol=test&another=param"),
                PathStyle::local()
            )
            .is_err()
        );
    }

    #[test]
    fn test_zero_based_line_numbers() {
        // Test that 0-based line numbers are rejected (should be 1-based)
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#L0:10"), PathStyle::local()).is_err()
        );
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#L1:0"), PathStyle::local()).is_err()
        );
        assert!(
            MentionUri::parse(uri!("file:///path/to/file.rs#L0:0"), PathStyle::local()).is_err()
        );
    }
}
