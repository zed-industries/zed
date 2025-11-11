use crate::PromptFormat;
use crate::example::{ActualExcerpt, ExpectedExcerpt, NamedExample};
use crate::headless::ZetaCliAppState;
use crate::paths::{CACHE_DIR, LOGS_DIR};
use ::serde::Serialize;
use anyhow::{Result, anyhow};
use clap::Args;
use collections::HashMap;
use gpui::http_client::Url;
use language::{Anchor, Buffer, Point};
// use cloud_llm_client::predict_edits_v3::PromptFormat;
use cloud_zeta2_prompt::{CURSOR_MARKER, write_codeblock};
use futures::StreamExt as _;
use gpui::{AppContext, AsyncApp, Entity};
use project::Project;
use serde::Deserialize;
use std::cell::Cell;
use std::fs;
use std::io::Write;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use zeta2::LlmResponseCache;

#[derive(Debug, Args)]
pub struct PredictArguments {
    #[arg(long, value_enum, default_value_t = PromptFormat::default())]
    prompt_format: PromptFormat,
    #[arg(long)]
    use_expected_context: bool,
    #[clap(long, short, value_enum, default_value_t = PredictionsOutputFormat::Md)]
    format: PredictionsOutputFormat,
    example_path: PathBuf,
    #[clap(long)]
    skip_cache: bool,
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
    let result = zeta2_predict(
        example,
        args.skip_cache,
        args.prompt_format,
        args.use_expected_context,
        &app_state,
        cx,
    )
    .await
    .unwrap();
    result.write(args.format, std::io::stdout()).unwrap();
}

thread_local! {
    static AUTHENTICATED: Cell<bool> = const { Cell::new(false) };
}

pub async fn zeta2_predict(
    example: NamedExample,
    skip_cache: bool,
    prompt_format: PromptFormat,
    use_expected_context: bool,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<PredictionDetails> {
    fs::create_dir_all(&*LOGS_DIR)?;
    let worktree_path = example.setup_worktree().await?;

    if !AUTHENTICATED.get() {
        AUTHENTICATED.set(true);

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

    zeta.update(cx, |zeta, _cx| {
        zeta.with_llm_response_cache(Arc::new(Cache { skip_cache }));
    })?;

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

    let result = Arc::new(Mutex::new(PredictionDetails::default()));
    let mut debug_rx = zeta.update(cx, |zeta, _| zeta.debug_info())?;

    let debug_task = cx.background_spawn({
        let result = result.clone();
        async move {
            let mut start_time = None;
            let mut search_queries_generated_at = None;
            let mut search_queries_executed_at = None;
            while let Some(event) = debug_rx.next().await {
                match event {
                    zeta2::ZetaDebugInfo::ContextRetrievalStarted(info) => {
                        start_time = Some(info.timestamp);
                        fs::write(LOGS_DIR.join("search_prompt.md"), &info.search_prompt)?;
                    }
                    zeta2::ZetaDebugInfo::SearchQueriesGenerated(info) => {
                        search_queries_generated_at = Some(info.timestamp);
                        fs::write(
                            LOGS_DIR.join("search_queries.json"),
                            serde_json::to_string_pretty(&info.search_queries).unwrap(),
                        )?;
                    }
                    zeta2::ZetaDebugInfo::SearchQueriesExecuted(info) => {
                        search_queries_executed_at = Some(info.timestamp);
                    }
                    zeta2::ZetaDebugInfo::ContextRetrievalFinished(_info) => {}
                    zeta2::ZetaDebugInfo::EditPredictionRequested(request) => {
                        let prediction_started_at = Instant::now();
                        start_time.get_or_insert(prediction_started_at);
                        fs::write(
                            LOGS_DIR.join("prediction_prompt.md"),
                            &request.local_prompt.unwrap_or_default(),
                        )?;

                        {
                            let mut result = result.lock().unwrap();

                            for included_file in request.request.included_files {
                                let insertions =
                                    vec![(request.request.cursor_point, CURSOR_MARKER)];
                                result.excerpts.extend(included_file.excerpts.iter().map(
                                    |excerpt| ActualExcerpt {
                                        path: included_file.path.components().skip(1).collect(),
                                        text: String::from(excerpt.text.as_ref()),
                                    },
                                ));
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
                                    &mut result.excerpts_text,
                                );
                            }
                        }

                        let response = request.response_rx.await?.0.map_err(|err| anyhow!(err))?;
                        let response = zeta2::text_from_response(response).unwrap_or_default();
                        let prediction_finished_at = Instant::now();
                        fs::write(LOGS_DIR.join("prediction_response.md"), &response)?;

                        let mut result = result.lock().unwrap();

                        if !use_expected_context {
                            result.planning_search_time =
                                Some(search_queries_generated_at.unwrap() - start_time.unwrap());
                            result.running_search_time = Some(
                                search_queries_executed_at.unwrap()
                                    - search_queries_generated_at.unwrap(),
                            );
                        }
                        result.prediction_time = prediction_finished_at - prediction_started_at;
                        result.total_time = prediction_finished_at - start_time.unwrap();

                        break;
                    }
                }
            }
            anyhow::Ok(())
        }
    });

    zeta.update(cx, |zeta, _cx| {
        let mut options = zeta.options().clone();
        options.prompt_format = prompt_format.into();
        zeta.set_options(options);
    })?;

    if use_expected_context {
        let context_excerpts_tasks = example
            .example
            .expected_context
            .iter()
            .flat_map(|section| {
                section.alternatives[0].excerpts.iter().map(|excerpt| {
                    resolve_context_entry(project.clone(), excerpt.clone(), cx.clone())
                })
            })
            .collect::<Vec<_>>();
        let context_excerpts_vec = futures::future::try_join_all(context_excerpts_tasks).await?;

        let mut context_excerpts = HashMap::default();
        for (buffer, mut excerpts) in context_excerpts_vec {
            context_excerpts
                .entry(buffer)
                .or_insert(Vec::new())
                .append(&mut excerpts);
        }

        zeta.update(cx, |zeta, _cx| {
            zeta.set_context(project.clone(), context_excerpts)
        })?;
    } else {
        zeta.update(cx, |zeta, cx| {
            zeta.refresh_context(project.clone(), cursor_buffer.clone(), cursor_anchor, cx)
        })?
        .await?;
    }

    let prediction = zeta
        .update(cx, |zeta, cx| {
            zeta.request_prediction(&project, &cursor_buffer, cursor_anchor, cx)
        })?
        .await?;

    debug_task.await?;

    let mut result = Arc::into_inner(result).unwrap().into_inner().unwrap();
    result.diff = prediction
        .map(|prediction| {
            let old_text = prediction.snapshot.text();
            let new_text = prediction
                .buffer
                .update(cx, |buffer, cx| {
                    buffer.edit(prediction.edits.iter().cloned(), None, cx);
                    buffer.text()
                })
                .unwrap();
            language::unified_diff(&old_text, &new_text)
        })
        .unwrap_or_default();

    anyhow::Ok(result)
}

async fn resolve_context_entry(
    project: Entity<Project>,
    excerpt: ExpectedExcerpt,
    mut cx: AsyncApp,
) -> Result<(Entity<Buffer>, Vec<Range<Anchor>>)> {
    let buffer = project
        .update(&mut cx, |project, cx| {
            let project_path = project.find_project_path(&excerpt.path, cx).unwrap();
            project.open_buffer(project_path, cx)
        })?
        .await?;

    let ranges = buffer.read_with(&mut cx, |buffer, _| {
        let full_text = buffer.text();
        let offset = full_text
            .find(&excerpt.text)
            .expect("Expected context not found");
        let point = buffer.offset_to_point(offset);
        excerpt
            .required_lines
            .iter()
            .map(|line| {
                let row = point.row + line.0;
                let range = Point::new(row, 0)..Point::new(row + 1, 0);
                buffer.anchor_after(range.start)..buffer.anchor_before(range.end)
            })
            .collect()
    })?;

    Ok((buffer, ranges))
}

struct Cache {
    skip_cache: bool,
}

impl Cache {
    fn path(key: u64) -> PathBuf {
        CACHE_DIR.join(format!("{key:x}.json"))
    }
}

impl LlmResponseCache for Cache {
    fn get_key(&self, url: &Url, body: &str) -> u64 {
        use collections::FxHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = FxHasher::default();
        url.hash(&mut hasher);
        body.hash(&mut hasher);
        hasher.finish()
    }

    fn read_response(&self, key: u64) -> Option<String> {
        let path = Cache::path(key);
        if path.exists() {
            if self.skip_cache {
                log::info!("Skipping existing cached LLM response: {}", path.display());
                None
            } else {
                log::info!("Using LLM response from cache: {}", path.display());
                Some(fs::read_to_string(path).unwrap())
            }
        } else {
            None
        }
    }

    fn write_response(&self, key: u64, value: &str) {
        fs::create_dir_all(&*CACHE_DIR).unwrap();

        let path = Cache::path(key);
        log::info!("Writing LLM response to cache: {}", path.display());
        fs::write(path, value).unwrap();
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PredictionDetails {
    pub diff: String,
    pub excerpts: Vec<ActualExcerpt>,
    pub excerpts_text: String, // TODO: contains the worktree root path. Drop this field and compute it on the fly
    pub planning_search_time: Option<Duration>,
    pub running_search_time: Option<Duration>,
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
        let inference_time = self.planning_search_time.unwrap_or_default() + self.prediction_time;

        format!(
            "## Excerpts\n\n\
            {}\n\n\
            ## Prediction\n\n\
            {}\n\n\
            ## Time\n\n\
            Planning searches: {}ms\n\
            Running searches: {}ms\n\
            Making Prediction: {}ms\n\n\
            -------------------\n\n\
            Total: {}ms\n\
            Inference: {}ms ({:.2}%)\n",
            self.excerpts_text,
            self.diff,
            self.planning_search_time.unwrap_or_default().as_millis(),
            self.running_search_time.unwrap_or_default().as_millis(),
            self.prediction_time.as_millis(),
            self.total_time.as_millis(),
            inference_time.as_millis(),
            (inference_time.as_millis() as f64 / self.total_time.as_millis() as f64) * 100.
        )
    }
}
