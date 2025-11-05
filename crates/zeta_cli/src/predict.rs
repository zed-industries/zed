use crate::example::{ActualExcerpt, NamedExample};
use crate::headless::ZetaCliAppState;
use ::serde::Serialize;
use anyhow::{Context as _, Result, anyhow};
use clap::Args;
use cloud_zeta2_prompt::{CURSOR_MARKER, write_codeblock};
use futures::StreamExt as _;
use gpui::AsyncApp;
use language_model::LanguageModelRegistry;
use project::Project;
use serde::Deserialize;
use std::cell::Cell;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Args)]
pub struct PredictArguments {
    example_path: PathBuf,
    #[clap(long, short, value_enum, default_value_t = PredictionsOutputFormat::Md)]
    format: PredictionsOutputFormat,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum PredictionsOutputFormat {
    Json,
    Md,
    Diff,
}
pub async fn run_zeta2_predict(
    args: PredictArguments,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) {
    let example = NamedExample::load(args.example_path).unwrap();
    let result = zeta2_predict(example, &app_state, cx).await.unwrap();
    result.write(args.format, std::io::stdout()).unwrap();
}

thread_local! {
    static AUTHENTICATED: Cell<bool> = const { Cell::new(false) };
}

pub async fn zeta2_predict(
    example: NamedExample,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<PredictionDetails> {
    let worktree_path = example.setup_worktree().await?;

    if !AUTHENTICATED.get() {
        AUTHENTICATED.set(true);

        cx.update(|cx| {
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry
                    .provider(&zeta2::related_excerpts::MODEL_PROVIDER_ID)
                    .unwrap()
                    .authenticate(cx)
            })
        })?
        .await?;

        app_state
            .client
            .sign_in_with_optional_connect(true, cx)
            .await?;
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

    let buffer_store = project.read_with(cx, |project, _| project.buffer_store().clone())?;

    let worktree = project
        .update(cx, |project, cx| {
            project.create_worktree(&worktree_path, true, cx)
        })?
        .await?;
    worktree
        .read_with(cx, |worktree, _cx| {
            worktree.as_local().unwrap().scan_complete()
        })?
        .await;

    let zeta = cx.update(|cx| zeta2::Zeta::global(&app_state.client, &app_state.user_store, cx))?;

    cx.subscribe(&buffer_store, {
        let project = project.clone();
        move |_, event, cx| match event {
            project::buffer_store::BufferStoreEvent::BufferAdded(buffer) => {
                zeta2::Zeta::try_global(cx)
                    .unwrap()
                    .update(cx, |zeta, cx| zeta.register_buffer(&buffer, &project, cx));
            }
            _ => {}
        }
    })?
    .detach();

    let _edited_buffers = example.apply_edit_history(&project, cx).await?;
    let (cursor_buffer, cursor_anchor) = example.cursor_position(&project, cx).await?;

    let mut debug_rx = zeta.update(cx, |zeta, _| zeta.debug_info())?;

    let refresh_task = zeta.update(cx, |zeta, cx| {
        zeta.refresh_context(project.clone(), cursor_buffer.clone(), cursor_anchor, cx)
    })?;

    let mut context_retrieval_started_at = None;
    let mut context_retrieval_finished_at = None;
    let mut search_queries_generated_at = None;
    let mut search_queries_executed_at = None;
    let mut prediction_started_at = None;
    let mut prediction_finished_at = None;
    let mut excerpts_text = String::new();
    let mut prediction_task = None;
    let mut result = PredictionDetails::default();
    while let Some(event) = debug_rx.next().await {
        match event {
            zeta2::ZetaDebugInfo::ContextRetrievalStarted(info) => {
                context_retrieval_started_at = Some(info.timestamp);
            }
            zeta2::ZetaDebugInfo::SearchQueriesGenerated(info) => {
                search_queries_generated_at = Some(info.timestamp);
            }
            zeta2::ZetaDebugInfo::SearchQueriesExecuted(info) => {
                search_queries_executed_at = Some(info.timestamp);
            }
            zeta2::ZetaDebugInfo::ContextRetrievalFinished(info) => {
                context_retrieval_finished_at = Some(info.timestamp);

                prediction_task = Some(zeta.update(cx, |zeta, cx| {
                    zeta.request_prediction(&project, &cursor_buffer, cursor_anchor, cx)
                })?);
            }
            zeta2::ZetaDebugInfo::EditPredicted(request) => {
                prediction_started_at = Some(Instant::now());
                request.response_rx.await?.0.map_err(|err| anyhow!(err))?;
                prediction_finished_at = Some(Instant::now());

                for included_file in request.request.included_files {
                    let insertions = vec![(request.request.cursor_point, CURSOR_MARKER)];
                    result
                        .excerpts
                        .extend(included_file.excerpts.iter().map(|excerpt| ActualExcerpt {
                            path: included_file.path.components().skip(1).collect(),
                            text: String::from(excerpt.text.as_ref()),
                        }));
                    write_codeblock(
                        &included_file.path,
                        included_file.excerpts.iter(),
                        if included_file.path == request.request.excerpt_path {
                            &insertions
                        } else {
                            &[]
                        },
                        included_file.max_row,
                        false,
                        &mut excerpts_text,
                    );
                }
                break;
            }
            _ => {}
        }
    }

    refresh_task.await.context("context retrieval failed")?;
    let prediction = prediction_task.unwrap().await?;

    result.diff = prediction
        .map(|prediction| {
            let old_text = prediction.snapshot.text();
            let new_text = prediction.buffer.update(cx, |buffer, cx| {
                buffer.edit(prediction.edits.iter().cloned(), None, cx);
                buffer.text()
            })?;
            anyhow::Ok(language::unified_diff(&old_text, &new_text))
        })
        .transpose()?
        .unwrap_or_default();
    result.excerpts_text = excerpts_text;

    result.planning_search_time =
        search_queries_generated_at.unwrap() - context_retrieval_started_at.unwrap();
    result.running_search_time =
        search_queries_executed_at.unwrap() - search_queries_generated_at.unwrap();
    result.filtering_search_time =
        context_retrieval_finished_at.unwrap() - search_queries_executed_at.unwrap();
    result.prediction_time = prediction_finished_at.unwrap() - prediction_started_at.unwrap();
    result.total_time = prediction_finished_at.unwrap() - context_retrieval_started_at.unwrap();

    anyhow::Ok(result)
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PredictionDetails {
    pub diff: String,
    pub excerpts: Vec<ActualExcerpt>,
    pub excerpts_text: String, // TODO: contains the worktree root path. Drop this field and compute it on the fly
    pub planning_search_time: Duration,
    pub filtering_search_time: Duration,
    pub running_search_time: Duration,
    pub prediction_time: Duration,
    pub total_time: Duration,
}

impl PredictionDetails {
    pub fn write(&self, format: PredictionsOutputFormat, mut out: impl Write) -> Result<()> {
        let formatted = match format {
            PredictionsOutputFormat::Md => self.to_markdown(),
            PredictionsOutputFormat::Json => serde_json::to_string_pretty(self)?,
            PredictionsOutputFormat::Diff => self.diff.clone(),
        };

        Ok(out.write_all(formatted.as_bytes())?)
    }

    pub fn to_markdown(&self) -> String {
        let inference_time =
            self.planning_search_time + self.filtering_search_time + self.prediction_time;

        format!(
            "## Excerpts\n\n\
            {}\n\n\
            ## Prediction\n\n\
            {}\n\n\
            ## Time\n\n\
            Planning searches: {}ms\n\
            Running searches: {}ms\n\
            Filtering context results: {}ms\n\
            Making Prediction: {}ms\n\n\
            -------------------\n\n\
            Total: {}ms\n\
            Inference: {}ms ({:.2}%)\n",
            self.excerpts_text,
            self.diff,
            self.planning_search_time.as_millis(),
            self.running_search_time.as_millis(),
            self.filtering_search_time.as_millis(),
            self.prediction_time.as_millis(),
            self.total_time.as_millis(),
            inference_time.as_millis(),
            (inference_time.as_millis() as f64 / self.total_time.as_millis() as f64) * 100.
        )
    }
}
