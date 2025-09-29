mod headless;

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};
use cloud_llm_client::predict_edits_v3;
use edit_prediction_context::EditPredictionExcerptOptions;
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
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use util::paths::PathStyle;
use util::rel_path::RelPath;
use zeta::{PerformPredictEditsParams, Zeta};

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
    Zeta2Context {
        #[clap(flatten)]
        zeta2_args: Zeta2Args,
        #[clap(flatten)]
        context_args: ContextArgs,
    },
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

#[derive(Debug, Args)]
struct Zeta2Args {
    #[arg(long, default_value_t = 8192)]
    max_prompt_bytes: usize,
    #[arg(long, default_value_t = 2048)]
    max_excerpt_bytes: usize,
    #[arg(long, default_value_t = 1024)]
    min_excerpt_bytes: usize,
    #[arg(long, default_value_t = 0.66)]
    target_before_cursor_over_total_bytes: f32,
    #[arg(long, default_value_t = 1024)]
    max_diagnostic_bytes: usize,
    #[arg(long, value_enum, default_value_t = PromptFormat::default())]
    prompt_format: PromptFormat,
    #[arg(long, value_enum, default_value_t = Default::default())]
    output_format: OutputFormat,
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum PromptFormat {
    #[default]
    MarkedExcerpt,
    LabeledSections,
    OnlySnippets,
}

impl Into<predict_edits_v3::PromptFormat> for PromptFormat {
    fn into(self) -> predict_edits_v3::PromptFormat {
        match self {
            Self::MarkedExcerpt => predict_edits_v3::PromptFormat::MarkedExcerpt,
            Self::LabeledSections => predict_edits_v3::PromptFormat::LabeledSections,
            Self::OnlySnippets => predict_edits_v3::PromptFormat::OnlySnippets,
        }
    }
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum OutputFormat {
    #[default]
    Prompt,
    Request,
    Both,
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
    path: Arc<RelPath>,
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

        let path = RelPath::new(Path::new(&parts[0]), PathStyle::local())?.into_arc();
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

enum GetContextOutput {
    Zeta1(zeta::GatherContextOutput),
    Zeta2(String),
}

async fn get_context(
    zeta2_args: Option<Zeta2Args>,
    args: ContextArgs,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<GetContextOutput> {
    let ContextArgs {
        worktree: worktree_path,
        cursor,
        use_language_server,
        events,
    } = args;

    let worktree_path = worktree_path.canonicalize()?;

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
        let buffer = open_buffer(&project, &worktree, &cursor.path, cx).await?;
        (None, buffer)
    };

    let full_path_str = worktree
        .read_with(cx, |worktree, _| worktree.root_name().join(&cursor.path))?
        .display(PathStyle::local())
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

    if let Some(zeta2_args) = zeta2_args {
        Ok(GetContextOutput::Zeta2(
            cx.update(|cx| {
                let zeta = cx.new(|cx| {
                    zeta2::Zeta::new(app_state.client.clone(), app_state.user_store.clone(), cx)
                });
                zeta.update(cx, |zeta, cx| {
                    zeta.register_buffer(&buffer, &project, cx);
                    zeta.set_options(zeta2::ZetaOptions {
                        excerpt: EditPredictionExcerptOptions {
                            max_bytes: zeta2_args.max_excerpt_bytes,
                            min_bytes: zeta2_args.min_excerpt_bytes,
                            target_before_cursor_over_total_bytes: zeta2_args
                                .target_before_cursor_over_total_bytes,
                        },
                        max_diagnostic_bytes: zeta2_args.max_diagnostic_bytes,
                        max_prompt_bytes: zeta2_args.max_prompt_bytes,
                        prompt_format: zeta2_args.prompt_format.into(),
                    })
                });
                // TODO: Actually wait for indexing.
                let timer = cx.background_executor().timer(Duration::from_secs(5));
                cx.spawn(async move |cx| {
                    timer.await;
                    let request = zeta
                        .update(cx, |zeta, cx| {
                            let cursor = buffer.read(cx).snapshot().anchor_before(clipped_cursor);
                            zeta.cloud_request_for_zeta_cli(&project, &buffer, cursor, cx)
                        })?
                        .await?;

                    let planned_prompt = cloud_zeta2_prompt::PlannedPrompt::populate(&request)?;
                    let prompt_string = planned_prompt.to_prompt_string()?.0;
                    match zeta2_args.output_format {
                        OutputFormat::Prompt => anyhow::Ok(prompt_string),
                        OutputFormat::Request => {
                            anyhow::Ok(serde_json::to_string_pretty(&request)?)
                        }
                        OutputFormat::Both => anyhow::Ok(serde_json::to_string_pretty(&json!({
                            "request": request,
                            "prompt": prompt_string,
                        }))?),
                    }
                })
            })?
            .await?,
        ))
    } else {
        let prompt_for_events = move || (events, 0);
        Ok(GetContextOutput::Zeta1(
            cx.update(|cx| {
                zeta::gather_context(
                    full_path_str,
                    &snapshot,
                    clipped_cursor,
                    prompt_for_events,
                    cx,
                )
            })?
            .await?,
        ))
    }
}

pub async fn open_buffer(
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    path: &RelPath,
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
        worktree_id: worktree.id(),
        path: path.into(),
    })?;

    project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
        .await
}

pub async fn open_buffer_with_language_server(
    project: &Entity<Project>,
    worktree: &Entity<Worktree>,
    path: &RelPath,
    cx: &mut AsyncApp,
) -> Result<(Entity<Entity<Buffer>>, Entity<Buffer>)> {
    let buffer = open_buffer(project, worktree, path, cx).await?;

    let (lsp_open_handle, path_style) = project.update(cx, |project, cx| {
        (
            project.register_buffer_with_language_servers(&buffer, cx),
            project.path_style(cx),
        )
    })?;

    let log_prefix = path.display(path_style);
    wait_for_lang_server(&project, &buffer, log_prefix.into_owned(), cx).await?;

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
        let is_zeta2_context_command = matches!(args.command, Commands::Zeta2Context { .. });
        cx.spawn(async move |cx| {
            let result = match args.command {
                Commands::Zeta2Context {
                    zeta2_args,
                    context_args,
                } => match get_context(Some(zeta2_args), context_args, &app_state, cx).await {
                    Ok(GetContextOutput::Zeta1 { .. }) => unreachable!(),
                    Ok(GetContextOutput::Zeta2(output)) => Ok(output),
                    Err(err) => Err(err),
                },
                Commands::Context(context_args) => {
                    match get_context(None, context_args, &app_state, cx).await {
                        Ok(GetContextOutput::Zeta1(output)) => {
                            Ok(serde_json::to_string_pretty(&output.body).unwrap())
                        }
                        Ok(GetContextOutput::Zeta2 { .. }) => unreachable!(),
                        Err(err) => Err(err),
                    }
                }
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
                                match get_context(None, context_args, &app_state, cx).await? {
                                    GetContextOutput::Zeta1(output) => output.body,
                                    GetContextOutput::Zeta2 { .. } => unreachable!(),
                                }
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
                    // TODO: Remove this once the 5 second delay is properly replaced.
                    if is_zeta2_context_command {
                        eprintln!("Note that zeta_cli doesn't yet wait for indexing, instead waits 5 seconds.");
                    }
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
