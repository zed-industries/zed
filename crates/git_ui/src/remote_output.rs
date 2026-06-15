use anyhow::Context as _;

use git::repository::{Remote, RemoteCommandOutput};
use linkify::{LinkFinder, LinkKind};
use ui::SharedString;
use util::ResultExt as _;

#[derive(Clone)]
pub enum RemoteAction {
    Fetch(Option<Remote>),
    Pull(Remote),
    Push(SharedString, Remote),
}

impl RemoteAction {
    pub fn name(&self) -> &'static str {
        match self {
            RemoteAction::Fetch(_) => "fetch",
            RemoteAction::Pull(_) => "pull",
            RemoteAction::Push(_, _) => "push",
        }
    }
}

/// Returns the first URL printed on a `remote:` line of git's output.
fn find_remote_link(stderr: &str) -> Option<String> {
    let finder = LinkFinder::new();
    stderr
        .lines()
        .filter(|line| line.trim_start().starts_with("remote:"))
        .find_map(|line| {
            finder
                .links(line)
                .find(|link| *link.kind() == LinkKind::Url)
                .map(|link| link.as_str().to_string())
        })
}

pub enum SuccessStyle {
    Toast,
    ToastWithLog { output: RemoteCommandOutput },
    /// The push created a branch for which a new pull/merge request can be
    /// opened. The button triggers the same `CreatePullRequest` action as the
    /// command palette, deriving the URL from the git hosting provider, and
    /// falls back to `fallback_url` (the link git printed) if that fails.
    CreatePullRequest { label: String, fallback_url: String },
    /// The push references an existing merge request; follow the link git
    /// printed since there is nothing to create.
    OpenLink { label: String, link: String },
}

pub struct SuccessMessage {
    pub message: String,
    pub style: SuccessStyle,
}

pub fn format_output(action: &RemoteAction, output: RemoteCommandOutput) -> SuccessMessage {
    match action {
        RemoteAction::Fetch(remote) => {
            if output.stderr.is_empty() {
                SuccessMessage {
                    message: "Fetch: Already up to date".into(),
                    style: SuccessStyle::Toast,
                }
            } else {
                let message = match remote {
                    Some(remote) => format!("Synchronized with {}", remote.name),
                    None => "Synchronized with remotes".into(),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Pull(remote_ref) => {
            let get_changes = |output: &RemoteCommandOutput| -> anyhow::Result<u32> {
                let last_line = output
                    .stdout
                    .lines()
                    .last()
                    .context("Failed to get last line of output")?
                    .trim();

                let files_changed = last_line
                    .split_whitespace()
                    .next()
                    .context("Failed to get first word of last line")?
                    .parse()?;

                Ok(files_changed)
            };
            if output.stdout.ends_with("Already up to date.\n") {
                SuccessMessage {
                    message: "Pull: Already up to date".into(),
                    style: SuccessStyle::Toast,
                }
            } else if output.stdout.starts_with("Updating") {
                let files_changed = get_changes(&output).log_err();
                let message = if let Some(files_changed) = files_changed {
                    format!(
                        "Received {} file change{} from {}",
                        files_changed,
                        if files_changed == 1 { "" } else { "s" },
                        remote_ref.name
                    )
                } else {
                    format!("Fast forwarded from {}", remote_ref.name)
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.starts_with("Merge") {
                let files_changed = get_changes(&output).log_err();
                let message = if let Some(files_changed) = files_changed {
                    format!(
                        "Merged {} file change{} from {}",
                        files_changed,
                        if files_changed == 1 { "" } else { "s" },
                        remote_ref.name
                    )
                } else {
                    format!("Merged from {}", remote_ref.name)
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.contains("Successfully rebased") {
                SuccessMessage {
                    message: format!("Successfully rebased from {}", remote_ref.name),
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else {
                SuccessMessage {
                    message: format!("Successfully pulled from {}", remote_ref.name),
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Push(branch_name, remote_ref) => {
            let message = if output.stderr.ends_with("Everything up-to-date\n") {
                "Push: Everything is up-to-date".to_string()
            } else {
                format!("Pushed {} to {}", branch_name, remote_ref.name)
            };

            let style = if output.stderr.ends_with("Everything up-to-date\n") {
                Some(SuccessStyle::Toast)
            } else if output.stderr.contains("\nremote: ") {
                // Hints git prints after a push. The "create" hints are handled
                // by the canonical `CreatePullRequest` action (which builds the
                // URL from the hosting provider), keeping git's link only as a
                // fallback. The "view" hint points at an already-existing merge
                // request, which we can only reach via the link git gave us.
                const CREATE_HINTS: &[(&str, &str)] = &[
                    ("Create a pull request", "Create Pull Request"), // GitHub
                    ("Create pull request", "Create Pull Request"),   // Bitbucket
                    ("create a merge request", "Create Merge Request"), // GitLab
                ];
                const VIEW_HINTS: &[(&str, &str)] = &[
                    ("View merge request", "View Merge Request"), // GitLab
                ];

                if let Some((_, label)) = CREATE_HINTS
                    .iter()
                    .find(|(indicator, _)| output.stderr.contains(indicator))
                {
                    find_remote_link(&output.stderr).map(|link| {
                        SuccessStyle::CreatePullRequest {
                            label: label.to_string(),
                            fallback_url: link,
                        }
                    })
                } else if let Some((_, label)) = VIEW_HINTS
                    .iter()
                    .find(|(indicator, _)| output.stderr.contains(indicator))
                {
                    find_remote_link(&output.stderr).map(|link| SuccessStyle::OpenLink {
                        label: label.to_string(),
                        link,
                    })
                } else {
                    None
                }
            } else {
                None
            };
            SuccessMessage {
                message,
                style: style.unwrap_or(SuccessStyle::ToastWithLog { output }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_push_new_branch_pull_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: Create a pull request for 'test' on GitHub by visiting:
                remote:      https://example.com/test/test/pull/new/test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::CreatePullRequest {
            label,
            fallback_url,
        } = &msg.style
        {
            assert_eq!(label, "Create Pull Request");
            assert_eq!(fallback_url, "https://example.com/test/test/pull/new/test");
        } else {
            panic!("Expected CreatePullRequest variant");
        }
    }

    #[test]
    fn test_push_new_branch_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: To create a merge request for test, visit:
                remote:   https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string()
            };

        let msg = format_output(&action, output);

        if let SuccessStyle::CreatePullRequest {
            label,
            fallback_url,
        } = &msg.style
        {
            assert_eq!(label, "Create Merge Request");
            assert_eq!(
                fallback_url,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            );
        } else {
            panic!("Expected CreatePullRequest variant");
        }
    }

    #[test]
    fn test_push_branch_existing_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            // Simulate an extraneous link that should not be found in top 3 lines
            stderr: indoc! {"
                ** WARNING: connection is not using a post-quantum key exchange algorithm.
                ** This session may be vulnerable to \"store now, decrypt later\" attacks.
                ** The server may need to be upgraded. See https://openssh.com/pq.html
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: View merge request for test:
                remote:    https://example.com/test/test/-/merge_requests/99999
                remote:
                To example.com:test/test.git
                    + 80bd3c83be...e03d499d2e test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::OpenLink { label, link } = &msg.style {
            assert_eq!(label, "View Merge Request");
            assert_eq!(link, "https://example.com/test/test/-/merge_requests/99999");
        } else {
            panic!("Expected OpenLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_no_link() {
        let action = RemoteAction::Push(
            SharedString::new_static("test_branch"),
            Remote {
                name: SharedString::new_static("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                To http://example.com/test/test.git
                 * [new branch]      test -> test
                ",
            }
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::ToastWithLog { output } = &msg.style {
            assert_eq!(
                output.stderr,
                "To http://example.com/test/test.git\n * [new branch]      test -> test\n"
            );
        } else {
            panic!("Expected ToastWithLog variant");
        }
    }
}
