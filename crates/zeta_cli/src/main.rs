mod headless;
mod retrieval_stats;
mod source_location;
mod util;

use crate::retrieval_stats::retrieval_stats;
use ::util::paths::PathStyle;
use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};
use cloud_llm_client::predict_edits_v3::{self};
use edit_prediction_context::{
    EditPredictionContextOptions, EditPredictionExcerptOptions, EditPredictionScoreOptions,
};
use gpui::{Application, AsyncApp, prelude::*};
use language::Bias;
use language_model::LlmApiToken;
use project::Project;
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use serde_json::json;
use std::{collections::HashSet, path::PathBuf, process::exit, str::FromStr, sync::Arc};
use zeta::{PerformPredictEditsParams, Zeta};

use crate::headless::ZetaCliAppState;
use crate::source_location::SourceLocation;
use crate::util::{open_buffer, open_buffer_with_language_server};

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
    RetrievalStats {
        #[clap(flatten)]
        zeta2_args: Zeta2Args,
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        extension: Option<String>,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long)]
        skip: Option<usize>,
    },
}

#[derive(Debug, Args)]
#[group(requires = "worktree")]
struct ContextArgs {
    #[arg(long)]
    worktree: PathBuf,
    #[arg(long)]
    cursor: SourceLocation,
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
    #[arg(long, default_value_t = 42)]
    file_indexing_parallelism: usize,
    #[arg(long, default_value_t = false)]
    disable_imports_gathering: bool,
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum PromptFormat {
    MarkedExcerpt,
    LabeledSections,
    OnlySnippets,
    #[default]
    NumberedLines,
}

impl Into<predict_edits_v3::PromptFormat> for PromptFormat {
    fn into(self) -> predict_edits_v3::PromptFormat {
        match self {
            Self::MarkedExcerpt => predict_edits_v3::PromptFormat::MarkedExcerpt,
            Self::LabeledSections => predict_edits_v3::PromptFormat::LabeledSections,
            Self::OnlySnippets => predict_edits_v3::PromptFormat::OnlySnippets,
            Self::NumberedLines => predict_edits_v3::PromptFormat::NumberedLines,
        }
    }
}

#[derive(clap::ValueEnum, Default, Debug, Clone)]
enum OutputFormat {
    #[default]
    Prompt,
    Request,
    Full,
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

    let mut ready_languages = HashSet::default();
    let (_lsp_open_handle, buffer) = if use_language_server {
        let (lsp_open_handle, _, buffer) = open_buffer_with_language_server(
            project.clone(),
            worktree.clone(),
            cursor.path.clone(),
            &mut ready_languages,
            cx,
        )
        .await?;
        (Some(lsp_open_handle), buffer)
    } else {
        let buffer =
            open_buffer(project.clone(), worktree.clone(), cursor.path.clone(), cx).await?;
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
        // wait for worktree scan before starting zeta2 so that wait_for_initial_indexing waits for
        // the whole worktree.
        worktree
            .read_with(cx, |worktree, _cx| {
                worktree.as_local().unwrap().scan_complete()
            })?
            .await;
        let output = cx
            .update(|cx| {
                let zeta = cx.new(|cx| {
                    zeta2::Zeta::new(app_state.client.clone(), app_state.user_store.clone(), cx)
                });
                let indexing_done_task = zeta.update(cx, |zeta, cx| {
                    zeta.set_options(zeta2_args.to_options(true));
                    zeta.register_buffer(&buffer, &project, cx);
                    zeta.wait_for_initial_indexing(&project, cx)
                });
                cx.spawn(async move |cx| {
                    indexing_done_task.await?;
                    let request = zeta
                        .update(cx, |zeta, cx| {
                            let cursor = buffer.read(cx).snapshot().anchor_before(clipped_cursor);
                            zeta.cloud_request_for_zeta_cli(&project, &buffer, cursor, cx)
                        })?
                        .await?;

                    let planned_prompt = cloud_zeta2_prompt::PlannedPrompt::populate(&request)?;
                    let (prompt_string, section_labels) = planned_prompt.to_prompt_string()?;

                    match zeta2_args.output_format {
                        OutputFormat::Prompt => anyhow::Ok(prompt_string),
                        OutputFormat::Request => {
                            anyhow::Ok(serde_json::to_string_pretty(&request)?)
                        }
                        OutputFormat::Full => anyhow::Ok(serde_json::to_string_pretty(&json!({
                            "request": request,
                            "prompt": prompt_string,
                            "section_labels": section_labels,
                        }))?),
                    }
                })
            })?
            .await?;
        Ok(GetContextOutput::Zeta2(output))
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

impl Zeta2Args {
    fn to_options(&self, omit_excerpt_overlaps: bool) -> zeta2::ZetaOptions {
        zeta2::ZetaOptions {
            context: EditPredictionContextOptions {
                use_imports: !self.disable_imports_gathering,
                excerpt: EditPredictionExcerptOptions {
                    max_bytes: self.max_excerpt_bytes,
                    min_bytes: self.min_excerpt_bytes,
                    target_before_cursor_over_total_bytes: self
                        .target_before_cursor_over_total_bytes,
                },
                score: EditPredictionScoreOptions {
                    omit_excerpt_overlaps,
                },
            },
            max_diagnostic_bytes: self.max_diagnostic_bytes,
            max_prompt_bytes: self.max_prompt_bytes,
            prompt_format: self.prompt_format.clone().into(),
            file_indexing_parallelism: self.file_indexing_parallelism,
        }
    }
}

fn main() {
    zlog::init();
    zlog::init_output_stderr();
    let args = ZetaCliArgs::parse();
    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    app.run(move |cx| {
        let app_state = Arc::new(headless::init(cx));
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
                Commands::RetrievalStats {
                    zeta2_args,
                    worktree,
                    extension,
                    limit,
                    skip,
                } => {
                    retrieval_stats(
                        worktree,
                        app_state,
                        extension,
                        limit,
                        skip,
                        (&zeta2_args).to_options(false),
                        cx,
                    )
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
