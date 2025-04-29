use std::{path::Path, process::Stdio};

use cargo_metadata::diagnostic::Diagnostic as CargoDiagnostic;
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

use crate::ProjectDiagnosticsEditor;

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
) -> Option<(Task<()>, Receiver<CargoDiagnostic>)> {
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
                    match CargoDiagnostic::deserialize(&mut deserializer) {
                        Ok(message) => {
                            if tx.send(message).await.is_err() {
                                return;
                            }
                        }
                        Err(_) => log::debug!("Failed to parse cargo diagnostics from line '{line}'"),
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

pub fn cargo_to_lsp(diagnostics: Vec<CargoDiagnostic>) -> lsp::PublishDiagnosticsParams {
    todo!("TODO kb")
}
