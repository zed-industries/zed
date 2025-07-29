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

pub enum SuccessStyle {
    Toast,
    ToastWithLog { output: RemoteCommandOutput },
    PushPrLink { link: String },
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
                    message: "Already up to date".into(),
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

            if output.stderr.starts_with("Everything up to date") {
                SuccessMessage {
                    message: output.stderr.trim().to_owned(),
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
            if output.stderr.contains("* [new branch]") {
                let pr_hints = [
                    // GitHub
                    "Create a pull request",
                    // Bitbucket
                    "Create pull request",
                    // GitLab
                    "create a merge request",
                ];
                let style = if pr_hints
                    .iter()
                    .any(|indicator| output.stderr.contains(indicator))
                {
                    let finder = LinkFinder::new();
                    let first_link = finder
                        .links(&output.stderr)
                        .filter(|link| *link.kind() == LinkKind::Url)
                        .map(|link| link.start()..link.end())
                        .next();
                    if let Some(link) = first_link {
                        let link = output.stderr[link].to_string();
                        SuccessStyle::PushPrLink { link }
                    } else {
                        SuccessStyle::ToastWithLog { output }
                    }
                } else {
                    SuccessStyle::ToastWithLog { output }
                };
                SuccessMessage {
                    message: format!("Published {} to {}", branch_name, remote_ref.name),
                    style,
                }
            } else if output.stderr.starts_with("Everything up to date") {
                SuccessMessage {
                    message: output.stderr.trim().to_owned(),
                    style: SuccessStyle::Toast,
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

    #[test]
    fn test_push_new_branch_pull_request() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: String::from(
                "
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: Create a pull request for 'test' on GitHub by visiting:
                remote:      https://example.com/test/test/pull/new/test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                ",
            ),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { link } = &msg.style {
            assert_eq!(link, "https://example.com/test/test/pull/new/test");
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: String::from("
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: To create a merge request for test, visit:
                remote:   https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { link } = &msg.style {
            assert_eq!(
                link,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            );
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_no_link() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: String::from(
                "
                To http://example.com/test/test.git
                 * [new branch]      test -> test
                ",
            ),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::ToastWithLog { output } = &msg.style {
            assert_eq!(
                output.stderr,
                "
                To http://example.com/test/test.git
                 * [new branch]      test -> test
                "
            );
        } else {
            panic!("Expected ToastWithLog variant");
        }
    }
}
