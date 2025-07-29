mod headless;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, Application, AsyncApp};
use gpui::{Entity, Task};
use language::Bias;
use language::Buffer;
use language::Point;
use project::{Project, ProjectPath};
use reqwest_client::ReqwestClient;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use zeta::{GatherContextOutput, gather_context};

use crate::headless::ZetaCliAppState;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GetContext {
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        cursor: CursorPosition,
        #[arg(long)]
        use_language_server: bool,
    },
}

#[derive(Debug, Clone)]
struct CursorPosition {
    path: PathBuf,
    point: Point,
}

impl FromStr for CursorPosition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid cursor format. Expected 'file.rs:line:column', got '{}'",
                s
            ));
        }

        let path = PathBuf::from(parts[0]);
        let line: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid line number: '{}'", parts[1]))?;
        let column: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid column number: '{}'", parts[2]))?;

        // Convert from 1-based to 0-based indexing
        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(CursorPosition { path, point })
    }
}

async fn get_context(
    worktree_path: &Path,
    cursor: &CursorPosition,
    use_language_server: bool,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<GatherContextOutput> {
    let worktree_path = worktree_path.canonicalize()?;
    if cursor.path.is_absolute() {
        return Err(anyhow!("Absolute paths are not supported in --cursor"));
    }

    let (project, _lsp_open_handle, buffer) = if use_language_server {
        let (project, lsp_open_handle, buffer) =
            open_buffer_with_language_server(&worktree_path, &cursor.path, app_state.clone(), cx)
                .await?;
        (Some(project), Some(lsp_open_handle), buffer)
    } else {
        let abs_path = worktree_path.join(&cursor.path);
        let content = std::fs::read_to_string(&abs_path)?;
        let buffer = cx.new(|cx| Buffer::local(content, cx))?;
        (None, None, buffer)
    };

    let worktree_name = worktree_path
        .file_name()
        .ok_or_else(|| anyhow!("--worktree path must end with a folder name"))?;
    let full_path_str = PathBuf::from(worktree_name)
        .join(&cursor.path)
        .to_string_lossy()
        .to_string();

    let snapshot = cx.update(|cx| buffer.read(cx).snapshot())?;
    let clipped_cursor = snapshot.clip_point(cursor.point, Bias::Left);
    if clipped_cursor != cursor.point {
        let max_row = snapshot.max_point().row;
        if cursor.point.row < max_row {
            return Err(anyhow!(
                "Cursor position {:?} is out of bounds (line length is {})",
                cursor.point,
                snapshot.line_len(cursor.point.row)
            ));
        } else {
            return Err(anyhow!(
                "Cursor position {:?} is out of bounds (max row is {})",
                cursor.point,
                max_row
            ));
        }
    }
    let cursor_anchor = snapshot.anchor_before(cursor.point);

    let events = VecDeque::new();
    let can_collect_data = false;
    cx.update(|cx| {
        gather_context(
            project.as_ref(),
            full_path_str,
            &snapshot,
            cursor_anchor,
            events,
            can_collect_data,
            cx,
        )
    })?
    .await
}

pub async fn open_buffer_with_language_server(
    worktree_path: &Path,
    path: &Path,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<(Entity<Project>, Entity<Entity<Buffer>>, Entity<Buffer>)> {
    let project = cx.update(|cx| {
        Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            cx,
        )
    })?;

    let worktree = project
        .update(cx, |project, cx| {
            project.create_worktree(worktree_path, true, cx)
        })?
        .await?;

    let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
        worktree_id: worktree.id(),
        path: path.to_path_buf().into(),
    })?;

    let buffer = project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
        .await?;

    let lsp_open_handle = project.update(cx, |project, cx| {
        project.register_buffer_with_language_servers(&buffer, cx)
    })?;

    let log_prefix = path.to_string_lossy().to_string();
    wait_for_lang_server(&project, &buffer, log_prefix, cx).await?;

    Ok((project, lsp_open_handle, buffer))
}

// TODO: Dedupe with similar function in crates/eval/src/instance.rs
pub fn wait_for_lang_server(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    log_prefix: String,
    cx: &mut AsyncApp,
) -> Task<Result<()>> {
    println!("{}⏵ Waiting for language server", log_prefix);

    let (mut tx, mut rx) = mpsc::channel(1);

    let lsp_store = project
        .read_with(cx, |project, _| project.lsp_store())
        .unwrap();

    let has_lang_server = buffer
        .update(cx, |buffer, cx| {
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store
                    .language_servers_for_local_buffer(&buffer, cx)
                    .next()
                    .is_some()
            })
        })
        .unwrap_or(false);

    if has_lang_server {
        project
            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
            .unwrap()
            .detach();
    }

    let subscriptions = [
        cx.subscribe(&lsp_store, {
            let log_prefix = log_prefix.clone();
            move |_, event, _| match event {
                project::LspStoreEvent::LanguageServerUpdate {
                    message:
                        client::proto::update_language_server::Variant::WorkProgress(
                            client::proto::LspWorkProgress {
                                message: Some(message),
                                ..
                            },
                        ),
                    ..
                } => println!("{}⟲ {message}", log_prefix),
                _ => {}
            }
        }),
        cx.subscribe(&project, {
            let buffer = buffer.clone();
            move |project, event, cx| match event {
                project::Event::LanguageServerAdded(_, _, _) => {
                    let buffer = buffer.clone();
                    project
                        .update(cx, |project, cx| project.save_buffer(buffer, cx))
                        .detach();
                }
                project::Event::DiskBasedDiagnosticsFinished { .. } => {
                    tx.try_send(()).ok();
                }
                _ => {}
            }
        }),
    ];

    cx.spawn(async move |cx| {
        let timeout = cx.background_executor().timer(Duration::new(60 * 5, 0));
        let result = futures::select! {
            _ = rx.next() => {
                println!("{}⚑ Language server idle", log_prefix);
                anyhow::Ok(())
            },
            _ = timeout.fuse() => {
                anyhow::bail!("LSP wait timed out after 5 minutes");
            }
        };
        drop(subscriptions);
        result
    })
}

fn main() -> Result<()> {
    let args = Args::parse();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    match args.command {
        Commands::GetContext {
            worktree,
            cursor,
            use_language_server,
        } => {
            app.run(move |cx| {
                let app_state = Arc::new(headless::init(cx));
                cx.spawn(async move |cx| {
                    let result =
                        get_context(&worktree, &cursor, use_language_server, app_state, cx).await;
                    match result {
                        Ok(output) => {
                            println!("{}", serde_json::to_string_pretty(&output.body).unwrap());
                            let _ = cx.update(|cx| cx.quit());
                        }
                        Err(e) => {
                            eprintln!("Failed to get context:\n{:?}", e);
                            exit(1);
                        }
                    }
                })
                .detach();
            });
        }
    }

    Ok(())
}
