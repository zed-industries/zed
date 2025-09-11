mod headless;

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};
use futures::channel::mpsc;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{AppContext, Application, AsyncApp};
use gpui::{Entity, Task};
use language::Bias;
use language::Buffer;
use language::Point;
use language_model::LlmApiToken;
use project::{Project, ProjectPath, Worktree};
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use zeta::{GatherContextOutput, PerformPredictEditsParams, Zeta, gather_context};

use crate::headless::ZetaCliAppState;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct ZetaCliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Context(ContextArgs),
    Predict {
        #[arg(long)]
        predict_edits_body: Option<FileOrStdin>,
        #[clap(flatten)]
        context_args: Option<ContextArgs>,
    },
}

#[derive(Debug, Args)]
#[group(requires = "worktree")]
struct ContextArgs {
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    cursor: CursorPosition,
    #[arg(long)]
    use_language_server: bool,
    #[arg(long)]
    events: Option<FileOrStdin>,
}

#[derive(Debug, Clone)]
enum FileOrStdin {
    File(PathBuf),
    Stdin,
}

impl FileOrStdin {
    async fn read_to_string(&self) -> Result<String, std::io::Error> {
        match self {
            FileOrStdin::File(path) => smol::fs::read_to_string(path).await,
            FileOrStdin::Stdin => smol::unblock(|| std::io::read_to_string(std::io::stdin())).await,
        }
    }
}

impl FromStr for FileOrStdin {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Stdin),
            _ => Ok(Self::File(PathBuf::from_str(s)?)),
        }
    }
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
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<GatherContextOutput> {
    let ContextArgs {
        worktree: worktree_path,
        cursor,
        use_language_server,
        events,
    } = args;

    let worktree_path = worktree_path.canonicalize()?;
    if cursor.path.is_absolute() {
        return Err(anyhow!("Absolute paths are not supported in --cursor"));
    }

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
            project.create_worktree(&worktree_path, true, cx)
        })?
        .await?;

    let (_lsp_open_handle, buffer) = if use_language_server {
        let (lsp_open_handle, buffer) =
            open_buffer_with_language_server(&project, &worktree, &cursor.path, cx).await?;
        (Some(lsp_open_handle), buffer)
    } else {
        let abs_path = worktree_path.join(&cursor.path);
        let content = smol::fs::read_to_string(&abs_path).await?;
        let buffer = cx.new(|cx| Buffer::local(content, cx))?;
        (None, buffer)
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

    let events = match events {
        Some(events) => events.read_to_string().await?,
        None => String::new(),
    };
    let prompt_for_events = move || (events, 0);
    cx.update(|cx| {
        gather_context(
            full_path_str,
            &snapshot,
            clipped_cursor,
            prompt_for_events,
            cx,
        )
    })?
    .await
}

pub async fn open_buffer_with_language_server(
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    path: &Path,
    cx: &mut AsyncApp,
) -> Result<(Entity<Entity<Buffer>>, Entity<Buffer>)> {
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

    Ok((lsp_open_handle, buffer))
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
                    .language_servers_for_local_buffer(buffer, cx)
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
            move |_, event, _| {
                if let project::LspStoreEvent::LanguageServerUpdate {
                    message:
                        client::proto::update_language_server::Variant::WorkProgress(
                            client::proto::LspWorkProgress {
                                message: Some(message),
                                ..
                            },
                        ),
                    ..
                } = event
                {
                    println!("{}⟲ {message}", log_prefix)
                }
            }
        }),
        cx.subscribe(project, {
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

fn main() {
    let args = ZetaCliArgs::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = Arc::new(headless::init(cx));
        cx.spawn(async move |cx| {
            let result = match args.command {
                Commands::Context(context_args) => get_context(context_args, &app_state, cx)
                    .await
                    .map(|output| serde_json::to_string_pretty(&output.body).unwrap()),
                Commands::Predict {
                    predict_edits_body,
                    context_args,
                } => {
                    cx.spawn(async move |cx| {
                        let app_version = cx.update(|cx| AppVersion::global(cx))?;
                        app_state.client.sign_in(true, cx).await?;
                        let llm_token = LlmApiToken::default();
                        llm_token.refresh(&app_state.client).await?;

                        let predict_edits_body =
                            if let Some(predict_edits_body) = predict_edits_body {
                                serde_json::from_str(&predict_edits_body.read_to_string().await?)?
                            } else if let Some(context_args) = context_args {
                                get_context(context_args, &app_state, cx).await?.body
                            } else {
                                return Err(anyhow!(
                                    "Expected either --predict-edits-body-file \
                                    or the required args of the `context` command."
                                ));
                            };

                        let (response, _usage) =
                            Zeta::perform_predict_edits(PerformPredictEditsParams {
                                client: app_state.client.clone(),
                                llm_token,
                                app_version,
                                body: predict_edits_body,
                            })
                            .await?;

                        Ok(response.output_excerpt)
                    })
                    .await
                }
            };
            match result {
                Ok(output) => {
                    println!("{}", output);
                    let _ = cx.update(|cx| cx.quit());
                }
                Err(e) => {
                    eprintln!("Failed: {:?}", e);
                    exit(1);
                }
            }
        })
        .detach();
    });
}
