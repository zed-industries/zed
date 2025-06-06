use anyhow::Context as _;
use git::repository::{Remote, RemoteCommandOutput};
use linkify::{LinkFinder, LinkKind};
use ui::SharedString;
use util::ResultExt as _;

#[derive(Clone)]
pub enum RemoteAction {
    Fetch,
    Pull(Remote),
    Push(SharedString, Remote),
}

impl RemoteAction {
    pub fn name(&self) -> &'static str {
        match self {
            RemoteAction::Fetch => "fetch",
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
        RemoteAction::Fetch => {
            if output.stderr.is_empty() {
                SuccessMessage {
                    message: "Already up to date".into(),
                    style: SuccessStyle::Toast,
                }
            } else {
                SuccessMessage {
                    message: "SynchroniCodeOrbit with remotes".into(),
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
                let style = if output.stderr.contains("Create a pull request") {
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
