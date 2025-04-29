use std::{path::Path, process::Stdio, sync::Arc};

use gpui::{AppContext, Entity, Task};
use itertools::Itertools as _;
use project::{Worktree, project_settings::ProjectSettings};
use serde::Deserialize as _;
use settings::Settings;
use smol::{
    channel::Receiver,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use ui::App;
use util::ResultExt;

use cargo_metadata::{Artifact, Message, PackageId, diagnostic::Diagnostic};

use crate::ProjectDiagnosticsEditor;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum JsonMessage {
    Cargo(Message),
    Rustc(Diagnostic),
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum CargoCheckMessage {
    CompilerArtifact(Artifact),
    Diagnostic {
        diagnostic: Diagnostic,
        package_id: Option<Arc<PackageId>>,
    },
}

pub fn worktrees_for_diagnostics_fetch(
    editor: Entity<ProjectDiagnosticsEditor>,
    cx: &App,
) -> Vec<Entity<Worktree>> {
    let fetch_cargo_diagnostics = ProjectSettings::get_global(cx)
        .diagnostics
        .fetch_cargo_diagnostics();
    if !fetch_cargo_diagnostics {
        return Vec::new();
    }
    editor
        .read(cx)
        .project
        .read(cx)
        .worktrees(cx)
        .filter(|worktree| worktree.read(cx).entry_for_path("Cargo.toml").is_some())
        .collect()
}

pub fn fetch_worktree_diagnostics(
    worktree_root: &Path,
    cx: &App,
) -> Option<(Task<()>, Receiver<CargoCheckMessage>)> {
    let diagnostics_settings = ProjectSettings::get_global(cx)
        .diagnostics
        .cargo
        .as_ref()
        .filter(|settings| settings.fetch_cargo_diagnostics)?;
    let command_string = diagnostics_settings
        .diagnostics_fetch_command
        .iter()
        .join(" ");
    let mut command_parts = diagnostics_settings.diagnostics_fetch_command.iter();
    let mut command = Command::new(command_parts.next()?)
        .args(command_parts)
        .envs(diagnostics_settings.env.clone())
        .current_dir(worktree_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .log_err()?;

    let stdout = command.stdout.take()?;
    let mut reader = BufReader::new(stdout);
    let (tx, rx) = smol::channel::unbounded();
    let error_threshold = 10;

    let cargo_diagnostics_fetch_task = cx.background_spawn(async move {
        let mut errors = 0;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    return;
                },
                Ok(_) => {
                    errors = 0;
                    let mut deserializer = serde_json::Deserializer::from_str(&line);
                    deserializer.disable_recursion_limit();
                    let cargo_check_message =
                        JsonMessage::deserialize(&mut deserializer).map(|json_message| {
                            match json_message {
                                JsonMessage::Cargo(message) => match message {
                                    Message::CompilerArtifact(artifact) if !artifact.fresh => {
                                        Some(CargoCheckMessage::CompilerArtifact(artifact))
                                    }
                                    Message::CompilerMessage(msg) => {
                                        Some(CargoCheckMessage::Diagnostic {
                                            diagnostic: msg.message,
                                            package_id: Some(Arc::new(msg.package_id)),
                                        })
                                    }
                                    _ => None,
                                },
                                JsonMessage::Rustc(message) => Some(CargoCheckMessage::Diagnostic {
                                    diagnostic: message,
                                    package_id: None,
                                }),
                            }
                        });

                    match cargo_check_message {
                        Ok(Some(message)) => {
                            if tx.send(message).await.is_err() {
                                return;
                            }
                        }
                        Ok(None) => {}
                        Err(e) => log::error!("Failed to parse cargo diagnostics from line '{line}': {e}"),
                    };
                },
                Err(e) => {
                    log::error!("Failed to read line from {command_string} command output when fetching cargo diagnostics: {e}");
                    errors += 1;
                    if errors >= error_threshold {
                        log::error!("Failed {error_threshold} times, aborting the diagnostics fetch");
                        return;
                    }
                },
            }
        }
    });

    Some((cargo_diagnostics_fetch_task, rx))
}
