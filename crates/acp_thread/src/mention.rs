use agent_client_protocol as acp;
use anyhow::{Result, bail};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MentionUri {
    File(PathBuf),
    Symbol(PathBuf, String),
    Thread(acp::SessionId),
    Rule(String),
}

impl MentionUri {
    pub fn parse(input: &str) -> Result<Self> {
        let url = url::Url::parse(input)?;
        let path = url.path();
        match url.scheme() {
            "file" => {
                if let Some(fragment) = url.fragment() {
                    Ok(Self::Symbol(path.into(), fragment.into()))
                } else {
                    let file_path =
                        PathBuf::from(format!("{}{}", url.host_str().unwrap_or(""), path));

                    Ok(Self::File(file_path))
                }
            }
            "zed" => {
                if let Some(thread) = path.strip_prefix("/agent/thread/") {
                    Ok(Self::Thread(acp::SessionId(thread.into())))
                } else if let Some(rule) = path.strip_prefix("/agent/rule/") {
                    Ok(Self::Rule(rule.into()))
                } else {
                    bail!("invalid zed url: {:?}", input);
                }
            }
            other => bail!("unrecognized scheme {:?}", other),
        }
    }

    pub fn name(&self) -> String {
        match self {
            MentionUri::File(path) => path.file_name().unwrap().to_string_lossy().into_owned(),
            MentionUri::Symbol(_path, name) => name.clone(),
            MentionUri::Thread(thread) => thread.to_string(),
            MentionUri::Rule(rule) => rule.clone(),
        }
    }

    pub fn to_link(&self) -> String {
        let name = self.name();
        let uri = self.to_uri();
        format!("[{name}]({uri})")
    }

    pub fn to_uri(&self) -> String {
        match self {
            MentionUri::File(path) => {
                format!("file://{}", path.display())
            }
            MentionUri::Symbol(path, name) => {
                format!("file://{}#{}", path.display(), name)
            }
            MentionUri::Thread(thread) => {
                format!("zed:///agent/thread/{}", thread.0)
            }
            MentionUri::Rule(rule) => {
                format!("zed:///agent/rule/{}", rule)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mention_uri_parse_and_display() {
        // Test file URI
        let file_uri = "file:///path/to/file.rs";
        let parsed = MentionUri::parse(file_uri).unwrap();
        match &parsed {
            MentionUri::File(path) => assert_eq!(path.to_str().unwrap(), "/path/to/file.rs"),
            _ => panic!("Expected File variant"),
        }
        assert_eq!(parsed.to_uri(), file_uri);

        // Test symbol URI
        let symbol_uri = "file:///path/to/file.rs#MySymbol";
        let parsed = MentionUri::parse(symbol_uri).unwrap();
        match &parsed {
            MentionUri::Symbol(path, symbol) => {
                assert_eq!(path.to_str().unwrap(), "/path/to/file.rs");
                assert_eq!(symbol, "MySymbol");
            }
            _ => panic!("Expected Symbol variant"),
        }
        assert_eq!(parsed.to_uri(), symbol_uri);

        // Test thread URI
        let thread_uri = "zed:///agent/thread/session123";
        let parsed = MentionUri::parse(thread_uri).unwrap();
        match &parsed {
            MentionUri::Thread(session_id) => assert_eq!(session_id.0.as_ref(), "session123"),
            _ => panic!("Expected Thread variant"),
        }
        assert_eq!(parsed.to_uri(), thread_uri);

        // Test rule URI
        let rule_uri = "zed:///agent/rule/my_rule";
        let parsed = MentionUri::parse(rule_uri).unwrap();
        match &parsed {
            MentionUri::Rule(rule) => assert_eq!(rule, "my_rule"),
            _ => panic!("Expected Rule variant"),
        }
        assert_eq!(parsed.to_uri(), rule_uri);

        // Test invalid scheme
        assert!(MentionUri::parse("http://example.com").is_err());

        // Test invalid zed path
        assert!(MentionUri::parse("zed:///invalid/path").is_err());
    }
}
