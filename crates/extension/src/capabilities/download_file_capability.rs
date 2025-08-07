use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadFileCapability {
    pub host: String,
    pub path: Vec<String>,
}

impl DownloadFileCapability {
    /// Returns whether the capability allows downloading a file from the given URL.
    pub fn allows(&self, url: &Url) -> bool {
        let Some(desired_host) = url.host_str() else {
            return false;
        };

        let Some(desired_path) = url.path_segments() else {
            return false;
        };
        let desired_path = desired_path.collect::<Vec<_>>();

        if self.host != desired_host && self.host != "*" {
            return false;
        }

        for (ix, path_segment) in self.path.iter().enumerate() {
            if path_segment == "**" {
                return true;
            }

            if ix >= desired_path.len() {
                return false;
            }

            if path_segment != "*" && path_segment != desired_path[ix] {
                return false;
            }
        }

        if self.path.len() < desired_path.len() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_allows() {
        let capability = DownloadFileCapability {
            host: "*".to_string(),
            path: vec!["**".to_string()],
        };
        assert_eq!(
            capability.allows(&"https://example.com/some/path".parse().unwrap()),
            true
        );

        let capability = DownloadFileCapability {
            host: "github.com".to_string(),
            path: vec!["**".to_string()],
        };
        assert_eq!(
            capability.allows(&"https://github.com/some-owner/some-repo".parse().unwrap()),
            true
        );
        assert_eq!(
            capability.allows(
                &"https://fake-github.com/some-owner/some-repo"
                    .parse()
                    .unwrap()
            ),
            false
        );

        let capability = DownloadFileCapability {
            host: "github.com".to_string(),
            path: vec!["specific-owner".to_string(), "*".to_string()],
        };
        assert_eq!(
            capability.allows(&"https://github.com/some-owner/some-repo".parse().unwrap()),
            false
        );
        assert_eq!(
            capability.allows(
                &"https://github.com/specific-owner/some-repo"
                    .parse()
                    .unwrap()
            ),
            true
        );

        let capability = DownloadFileCapability {
            host: "github.com".to_string(),
            path: vec!["specific-owner".to_string(), "*".to_string()],
        };
        assert_eq!(
            capability.allows(
                &"https://github.com/some-owner/some-repo/extra"
                    .parse()
                    .unwrap()
            ),
            false
        );
        assert_eq!(
            capability.allows(
                &"https://github.com/specific-owner/some-repo/extra"
                    .parse()
                    .unwrap()
            ),
            false
        );
    }
}
