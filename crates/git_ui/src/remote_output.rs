use anyhow::Context as _;

use git::repository::{Remote, RemoteCommandOutput};
use ui::SharedString;
use util::ResultExt as _;

/// Whether a pull request link printed by the remote points at a page for
/// creating a new pull request or at an already existing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PullRequestLinkKind {
    Create,
    View,
}

struct PullRequestHint {
    needle: &'static str,
    label: &'static str,
    kind: PullRequestLinkKind,
}

const PULL_REQUEST_HINTS: &[PullRequestHint] = &[
    // GitHub: "Create a pull request for 'branch' on GitHub by visiting:"
    PullRequestHint {
        needle: "Create a pull request",
        label: "Create Pull Request",
        kind: PullRequestLinkKind::Create,
    },
    // Bitbucket: "Create pull request for branch:"
    PullRequestHint {
        needle: "Create pull request",
        label: "Create Pull Request",
        kind: PullRequestLinkKind::Create,
    },
    // GitLab: "To create a merge request for branch, visit:"
    PullRequestHint {
        needle: "create a merge request",
        label: "Create Merge Request",
        kind: PullRequestLinkKind::Create,
    },
    // GitLab: "View merge request for branch:"
    PullRequestHint {
        needle: "View merge request",
        label: "View Merge Request",
        kind: PullRequestLinkKind::View,
    },
];

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

pub enum SuccessStyle {
    Toast,
    ToastWithLog {
        output: RemoteCommandOutput,
    },
    /// The push created a branch for which a new pull request can be opened.
    /// The URL is built from the git hosting provider, the same way the
    /// `git: create pull request` command does, and `fallback_url` (the link
    /// printed by the remote) is only used if that fails.
    CreatePullRequest {
        label: &'static str,
        fallback_url: String,
    },
    /// The push references an existing pull request, which can only be reached
    /// via the link printed by the remote.
    OpenPullRequest {
        label: &'static str,
        url: String,
    },
}

pub struct SuccessMessage {
    pub message: String,
    pub style: SuccessStyle,
}

#[derive(Debug, PartialEq, Eq)]
struct PullRequestLink {
    label: &'static str,
    url: String,
    kind: PullRequestLinkKind,
}

fn extract_pull_request_link(output: &RemoteCommandOutput) -> Option<PullRequestLink> {
    let mut pending_hint: Option<&'static PullRequestHint> = None;

    for line in output.stderr.lines() {
        let Some(remote_line) = line.trim_start().strip_prefix("remote:") else {
            pending_hint = None;
            continue;
        };

        if let Some(hint) = PULL_REQUEST_HINTS
            .iter()
            .find(|hint| remote_line.contains(hint.needle))
        {
            pending_hint = Some(hint);
        }

        if let Some(url) = extract_url(remote_line)
            && let Some(hint) = pending_hint
        {
            return Some(PullRequestLink {
                label: hint.label,
                url,
                kind: hint.kind,
            });
        }
    }

    None
}

fn extract_url(line: &str) -> Option<String> {
    let http_index = line.find("https://").or_else(|| line.find("http://"))?;
    let url = line[http_index..]
        .split_whitespace()
        .next()?
        .trim_end_matches(|character| matches!(character, ',' | '.' | ')' | ']' | '>'));

    Some(url.to_string())
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
            } else if let Some(PullRequestLink { label, url, kind }) =
                extract_pull_request_link(&output)
            {
                let style = match kind {
                    PullRequestLinkKind::Create => SuccessStyle::CreatePullRequest {
                        label,
                        fallback_url: url,
                    },
                    PullRequestLinkKind::View => SuccessStyle::OpenPullRequest { label, url },
                };
                SuccessMessage {
                    message: format!("Pushed {} to {}", branch_name, remote_ref.name),
                    style,
                }
            } else {
                SuccessMessage {
                    message: format!("Pushed {} to {}", branch_name, remote_ref.name),
                    style: SuccessStyle::ToastWithLog { output },
                }
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
        } = msg.style
        {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
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
        } = msg.style
        {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
            assert_eq!(label, "Create Merge Request");
            assert_eq!(
                fallback_url,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            )
        } else {
            panic!("Expected CreatePullRequest variant")
        }
    }

    #[test]
    fn test_push_new_branch_bitbucket_pull_request() {
        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                remote:
                remote: Create pull request for test:
                remote:   https://bitbucket.example.com/projects/TEST/repos/test/pull-requests?create&sourceBranch=refs/heads/test
                "}
            .to_string(),
        };

        assert_eq!(
            extract_pull_request_link(&output),
            Some(PullRequestLink {
                label: "Create Pull Request",
                url: "https://bitbucket.example.com/projects/TEST/repos/test/pull-requests?create&sourceBranch=refs/heads/test".to_string(),
                kind: PullRequestLinkKind::Create,
            })
        );
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
            // Include an unrelated URL outside of the `remote:` lines, in this
            // case, an OpenSSH warning, to ensure that it is not mistaken for
            // the merge request link.
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

        if let SuccessStyle::OpenPullRequest { label, url } = msg.style {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
            assert_eq!(label, "View Merge Request");
            assert_eq!(url, "https://example.com/test/test/-/merge_requests/99999");
        } else {
            panic!("Expected OpenPullRequest variant")
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
            assert_eq!(extract_pull_request_link(output), None);
        } else {
            panic!("Expected ToastWithLog variant");
        }
    }
}
