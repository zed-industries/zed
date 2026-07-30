use anyhow::Context as _;

use git::repository::{Remote, RemoteCommandOutput};
use i18n::{LocalizedString, t};
use ui::SharedString;
use util::ResultExt as _;

/// Matches a `remote:` line from a push against the link it announces.
///
/// The hints are substrings of the remote's own output, so they stay in English
/// no matter which locale the UI is displaying.
fn pull_request_link_label(remote_line: &str) -> Option<LocalizedString> {
    // GitHub: "Create a pull request for 'branch' on GitHub by visiting:"
    // Bitbucket: "Create pull request for branch:"
    if remote_line.contains("Create a pull request") || remote_line.contains("Create pull request")
    {
        return Some(t!("Create Pull Request"));
    }
    // GitLab: "To create a merge request for branch, visit:"
    if remote_line.contains("create a merge request") {
        return Some(t!("Create Merge Request"));
    }
    // GitLab: "View merge request for branch:"
    if remote_line.contains("View merge request") {
        return Some(t!("View Merge Request"));
    }
    None
}

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
    ToastWithLog { output: RemoteCommandOutput },
    PushPrLink { label: LocalizedString, url: String },
}

pub struct SuccessMessage {
    pub message: String,
    pub style: SuccessStyle,
}

fn extract_pull_request_link(output: &RemoteCommandOutput) -> Option<(LocalizedString, String)> {
    let mut pending_label: Option<LocalizedString> = None;

    for line in output.stderr.lines() {
        let Some(remote_line) = line.trim_start().strip_prefix("remote:") else {
            pending_label = None;
            continue;
        };

        if let Some(label) = pull_request_link_label(remote_line) {
            pending_label = Some(label);
        }

        if let Some(url) = extract_url(remote_line)
            && let Some(label) = pending_label.take()
        {
            return Some((label, url));
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
                    message: String::from(t!("Fetch: Already up to date")),
                    style: SuccessStyle::Toast,
                }
            } else {
                let message = match remote {
                    Some(remote) => String::from(t!(
                        "Synchronized with {$remote}",
                        remote = remote.name.clone()
                    )),
                    None => String::from(t!("Synchronized with remotes")),
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
                    message: String::from(t!("Pull: Already up to date")),
                    style: SuccessStyle::Toast,
                }
            } else if output.stdout.starts_with("Updating") {
                let files_changed = get_changes(&output).log_err();
                let message = match files_changed {
                    Some(1) => String::from(t!(
                        "Received 1 file change from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                    Some(files_changed) => String::from(t!(
                        "Received {$count} file changes from {$remote}",
                        count = files_changed,
                        remote = remote_ref.name.clone()
                    )),
                    None => String::from(t!(
                        "Fast forwarded from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.starts_with("Merge") {
                let files_changed = get_changes(&output).log_err();
                let message = match files_changed {
                    Some(1) => String::from(t!(
                        "Merged 1 file change from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                    Some(files_changed) => String::from(t!(
                        "Merged {$count} file changes from {$remote}",
                        count = files_changed,
                        remote = remote_ref.name.clone()
                    )),
                    None => String::from(t!(
                        "Merged from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.contains("Successfully rebased") {
                SuccessMessage {
                    message: String::from(t!(
                        "Successfully rebased from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else {
                SuccessMessage {
                    message: String::from(t!(
                        "Successfully pulled from {$remote}",
                        remote = remote_ref.name.clone()
                    )),
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Push(branch_name, remote_ref) => {
            if output.stderr.ends_with("Everything up-to-date\n") {
                SuccessMessage {
                    message: String::from(t!("Push: Everything is up-to-date")),
                    style: SuccessStyle::Toast,
                }
            } else if let Some((label, url)) = extract_pull_request_link(&output) {
                SuccessMessage {
                    message: String::from(t!(
                        "Pushed {$branch} to {$remote}",
                        branch = branch_name.clone(),
                        remote = remote_ref.name.clone()
                    )),
                    style: SuccessStyle::PushPrLink { label, url },
                }
            } else {
                SuccessMessage {
                    message: String::from(t!(
                        "Pushed {$branch} to {$remote}",
                        branch = branch_name.clone(),
                        remote = remote_ref.name.clone()
                    )),
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
        if let SuccessStyle::PushPrLink { label, url } = msg.style {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
            assert_eq!(label.fallback(), "Create Pull Request");
            assert_eq!(url, "https://example.com/test/test/pull/new/test");
        } else {
            panic!("Expected PushPrLink variant");
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

        if let SuccessStyle::PushPrLink { label, url } = msg.style {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
            assert_eq!(label.fallback(), "Create Merge Request");
            assert_eq!(
                url,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            )
        } else {
            panic!("Expected PushPrLink variant")
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

        let (label, url) = extract_pull_request_link(&output).expect("link should be extracted");
        assert_eq!(label.fallback(), "Create Pull Request");
        assert_eq!(
            url,
            "https://bitbucket.example.com/projects/TEST/repos/test/pull-requests?create&sourceBranch=refs/heads/test"
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

        if let SuccessStyle::PushPrLink { label, url } = msg.style {
            assert_eq!(msg.message, "Pushed test_branch to test_remote");
            assert_eq!(label.fallback(), "View Merge Request");
            assert_eq!(url, "https://example.com/test/test/-/merge_requests/99999");
        } else {
            panic!("Expected PushPrLink variant")
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
