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

#[derive(Debug)]
pub enum SuccessStyle {
    Toast,
    ToastWithLog {
        output: RemoteCommandOutput,
    },
    /// A push whose stderr contained a link to create or view a pull/merge
    /// request. Opening this URL directly avoids relying on a hosting provider
    /// being registered for the remote, which is not the case for every host
    /// (e.g. self-hosted GitLab instances behind a private domain).
    PushPrLink {
        link: String,
    },
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
            if output.stderr.ends_with("Everything up-to-date\n") {
                SuccessMessage {
                    message: "Push: Everything is up-to-date".to_string(),
                    style: SuccessStyle::Toast,
                }
            } else {
                // Many hosting providers print a link to create or view a pull/merge
                // request in the push output (prefixed with `remote:`). Prefer that
                // link when present: it is produced by the server itself, so it works
                // for any host regardless of whether Zed has a matching provider.
                let link = extract_pull_request_link(&output.stderr);
                let message = format!("Pushed {} to {}", branch_name, remote_ref.name);
                let style = match link {
                    Some(link) => SuccessStyle::PushPrLink { link },
                    None => SuccessStyle::ToastWithLog { output },
                };
                SuccessMessage { message, style }
            }
        }
    }
}

/// Extracts a pull/merge request link from a push command's stderr, if any.
///
/// Hosting providers surface these links on lines prefixed with `remote:`
/// (e.g. GitHub's "Create a pull request for ... on GitHub by visiting:"
/// followed by the URL, or GitLab's "To create a merge request for ..., visit:").
/// We only inspect `remote:` lines so that unrelated URLs printed earlier in
/// the output (such as OpenSSH's post-quantum warning linking to openssh.com)
/// are not picked up.
fn extract_pull_request_link(stderr: &str) -> Option<String> {
    let finder = LinkFinder::new();
    stderr.lines().find_map(|line| {
        let trimmed = line.trim_start();
        trimmed
            .strip_prefix("remote:")
            .and_then(|rest| {
                finder
                    .links(rest)
                    .find(|link| *link.kind() == LinkKind::Url)
                    .map(|link| link.as_str().trim().to_string())
            })
            .filter(|link| !link.is_empty())
    })
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

        let SuccessStyle::PushPrLink { link } = &msg.style else {
            panic!("Expected PushPrLink variant, got {:?}", msg.style);
        };
        assert_eq!(link, "https://example.com/test/test/pull/new/test");
        assert_eq!(msg.message, "Pushed test_branch to test_remote");
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

        let SuccessStyle::PushPrLink { link } = &msg.style else {
            panic!("Expected PushPrLink variant, got {:?}", msg.style);
        };
        assert_eq!(
            link,
            "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
        );
        assert_eq!(msg.message, "Pushed test_branch to test_remote");
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

        let SuccessStyle::PushPrLink { link } = &msg.style else {
            panic!("Expected PushPrLink variant, got {:?}", msg.style);
        };
        // The openssh.com URL on a non-`remote:` line must be ignored.
        assert_eq!(link, "https://example.com/test/test/-/merge_requests/99999");
        assert_eq!(msg.message, "Pushed test_branch to test_remote");
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

    /// Regression test for an internal GitLab host with no registered provider:
    /// the create-merge-request URL printed by the server must still be picked
    /// up from stderr so the toast can open it directly, instead of failing
    /// with "Unsupported remote URL".
    #[test]
    fn test_push_internal_host_merge_request_link() {
        let action = RemoteAction::Push(
            SharedString::new_static("dtm-harness"),
            Remote {
                name: SharedString::new_static("origin"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                remote:
                remote: To create a merge request for dtm-harness, visit:
                remote:   https://git.woa.com/ybtm-client/dtm-harness/-/merge_requests/new?merge_request%5Bsource_branch%5D=dtm-harness
                remote:
                To git.woa.com:ybtm-client/dtm-harness.git
                 * [new branch]      dtm-harness -> dtm-harness
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        let SuccessStyle::PushPrLink { link } = &msg.style else {
            panic!("Expected PushPrLink variant, got {:?}", msg.style);
        };
        assert_eq!(
            link,
            "https://git.woa.com/ybtm-client/dtm-harness/-/merge_requests/new?merge_request%5Bsource_branch%5D=dtm-harness"
        );
        assert_eq!(msg.message, "Pushed dtm-harness to origin");
    }
}
