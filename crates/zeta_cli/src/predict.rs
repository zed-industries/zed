use crate::PromptFormat;
use crate::example::{ActualExcerpt, ExpectedExcerpt, NamedExample};
use crate::headless::ZetaCliAppState;
use crate::paths::{CACHE_DIR, LATEST_EXAMPLE_RUN_DIR, RUN_DIR, print_run_data_dir};
use ::serde::Serialize;
use anyhow::{Context, Result, anyhow};
use clap::{Args, ValueEnum};
use cloud_zeta2_prompt::{CURSOR_MARKER, write_codeblock};
use collections::HashMap;
use futures::StreamExt as _;
use gpui::{AppContext, AsyncApp, Entity};
use language::{Anchor, Buffer, Point};
use project::Project;
use serde::Deserialize;
use std::fs;
use std::io::{IsTerminal, Write};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use zeta2::{EvalCache, EvalCacheEntryKind, EvalCacheKey, Zeta};

#[derive(Debug, Args)]
pub struct PredictArguments {
    #[arg(long, value_enum, default_value_t = PromptFormat::default())]
    prompt_format: PromptFormat,
    #[arg(long)]
    use_expected_context: bool,
    #[clap(long, short, value_enum, default_value_t = PredictionsOutputFormat::Md)]
    format: PredictionsOutputFormat,
    example_path: PathBuf,
    #[clap(long, value_enum, default_value_t = CacheMode::default())]
    cache: CacheMode,
}

#[derive(Debug, ValueEnum, Default, Clone, Copy, PartialEq)]
pub enum CacheMode {
    /// Use cached LLM requests and responses, except when multiple repetitions are requested
    #[default]
    Auto,
    /// Use cached LLM requests and responses, based on the hash of the prompt and the endpoint.
    #[value(alias = "request")]
    Requests,
    /// Ignore existing cache entries for both LLM and search.
    Skip,
    /// Use cached LLM responses AND search results for full determinism. Fails if they haven't been cached yet.
    /// Useful for reproducing results and fixing bugs outside of search queries
    Force,
}

impl CacheMode {
    fn use_cached_llm_responses(&self) -> bool {
        self.assert_not_auto();
        matches!(self, CacheMode::Requests | CacheMode::Force)
    }

    fn use_cached_search_results(&self) -> bool {
        self.assert_not_auto();
        matches!(self, CacheMode::Force)
    }

    fn assert_not_auto(&self) {
        assert_ne!(
            *self,
            CacheMode::Auto,
            "Cache mode should not be auto at this point!"
        );
    }
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
    let (project, mut zetas, _edited_buffers) =
        example.setup_project(app_state, 1, cx).await.unwrap();
    let result = zeta2_predict(
        example,
        project,
        zetas.remove(0),
        None,
        args.prompt_format,
        args.use_expected_context,
        args.cache,
        cx,
    )
    .await
    .unwrap();
    result.write(args.format, std::io::stdout()).unwrap();

    print_run_data_dir(true, std::io::stdout().is_terminal());
}

pub async fn zeta2_predict(
    example: NamedExample,
    project: Entity<Project>,
    zeta: Entity<Zeta>,
    repetition_ix: Option<u16>,
    prompt_format: PromptFormat,
    use_expected_context: bool,
    mut cache_mode: CacheMode,
    cx: &mut AsyncApp,
) -> Result<PredictionDetails> {
    if repetition_ix.is_some() {
        if cache_mode != CacheMode::Auto && cache_mode != CacheMode::Skip {
            panic!("Repetitions are not supported in Auto cache mode");
        } else {
            cache_mode = CacheMode::Skip;
        }
    } else if cache_mode == CacheMode::Auto {
        cache_mode = CacheMode::Requests;
    }

    let mut example_run_dir = RUN_DIR.join(&example.file_name());
    if let Some(repetition_ix) = repetition_ix {
        example_run_dir = example_run_dir.join(format!("{:03}", repetition_ix));
    }
    fs::create_dir_all(&example_run_dir)?;
    if LATEST_EXAMPLE_RUN_DIR.is_symlink() {
        fs::remove_file(&*LATEST_EXAMPLE_RUN_DIR)?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(&example_run_dir, &*LATEST_EXAMPLE_RUN_DIR)
        .context("creating latest link")?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&example_run_dir, &*LATEST_EXAMPLE_RUN_DIR)
        .context("creating latest link")?;

    zeta.update(cx, |zeta, _cx| {
        zeta.with_eval_cache(Arc::new(RunCache {
            example_run_dir: example_run_dir.clone(),
            cache_mode,
        }));
    })?;

    let (cursor_buffer, cursor_anchor) = example.cursor_position(&project, cx).await?;

    let result = Arc::new(Mutex::new(PredictionDetails::new(example_run_dir.clone())));
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
                        fs::write(
                            example_run_dir.join("search_prompt.md"),
                            &info.search_prompt,
                        )?;
                    }
                    zeta2::ZetaDebugInfo::SearchQueriesGenerated(info) => {
                        search_queries_generated_at = Some(info.timestamp);
                        fs::write(
                            example_run_dir.join("search_queries.json"),
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
                        let prompt = request.local_prompt.unwrap_or_default();
                        fs::write(example_run_dir.join("prediction_prompt.md"), &prompt)?;

                        {
                            let mut result = result.lock().unwrap();
                            result.prompt_len = prompt.chars().count();

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
                        fs::write(example_run_dir.join("prediction_response.md"), &response)?;

                        let mut result = result.lock().unwrap();
                        result.generated_len = response.chars().count();

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
                    let branch = buffer.branch(cx);
                    branch.update(cx, |branch, cx| {
                        branch.edit(prediction.edits.iter().cloned(), None, cx);
                        branch.text()
                    })
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

struct RunCache {
    cache_mode: CacheMode,
    example_run_dir: PathBuf,
}

impl RunCache {
    fn output_cache_path((kind, key): &EvalCacheKey) -> PathBuf {
        CACHE_DIR.join(format!("{kind}_out_{key:x}.json",))
    }

    fn input_cache_path((kind, key): &EvalCacheKey) -> PathBuf {
        CACHE_DIR.join(format!("{kind}_in_{key:x}.json",))
    }

    fn link_to_run(&self, key: &EvalCacheKey) {
        let output_link_path = self.example_run_dir.join(format!("{}_out.json", key.0));
        fs::hard_link(Self::output_cache_path(key), &output_link_path).unwrap();

        let input_link_path = self.example_run_dir.join(format!("{}_in.json", key.0));
        fs::hard_link(Self::input_cache_path(key), &input_link_path).unwrap();
    }
}

impl EvalCache for RunCache {
    fn read(&self, key: EvalCacheKey) -> Option<String> {
        let path = RunCache::output_cache_path(&key);

        if path.exists() {
            let use_cache = match key.0 {
                EvalCacheEntryKind::Search => self.cache_mode.use_cached_search_results(),
                EvalCacheEntryKind::Context | EvalCacheEntryKind::Prediction => {
                    self.cache_mode.use_cached_llm_responses()
                }
            };
            if use_cache {
                log::info!("Using cache entry: {}", path.display());
                self.link_to_run(&key);
                Some(fs::read_to_string(path).unwrap())
            } else {
                log::trace!("Skipping cached entry: {}", path.display());
                None
            }
        } else if matches!(self.cache_mode, CacheMode::Force) {
            panic!(
                "No cached entry found for {:?}. Run without `--cache force` at least once.",
                key.0
            );
        } else {
            None
        }
    }

    fn write(&self, key: EvalCacheKey, input: &str, output: &str) {
        fs::create_dir_all(&*CACHE_DIR).unwrap();

        let input_path = RunCache::input_cache_path(&key);
        fs::write(&input_path, input).unwrap();

        let output_path = RunCache::output_cache_path(&key);
        log::trace!("Writing cache entry: {}", output_path.display());
        fs::write(&output_path, output).unwrap();

        self.link_to_run(&key);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PredictionDetails {
    pub diff: String,
    pub excerpts: Vec<ActualExcerpt>,
    pub excerpts_text: String, // TODO: contains the worktree root path. Drop this field and compute it on the fly
    pub planning_search_time: Option<Duration>,
    pub running_search_time: Option<Duration>,
    pub prediction_time: Duration,
    pub total_time: Duration,
    pub run_example_dir: PathBuf,
    pub prompt_len: usize,
    pub generated_len: usize,
}

impl PredictionDetails {
    pub fn new(run_example_dir: PathBuf) -> Self {
        Self {
            diff: Default::default(),
            excerpts: Default::default(),
            excerpts_text: Default::default(),
            planning_search_time: Default::default(),
            running_search_time: Default::default(),
            prediction_time: Default::default(),
            total_time: Default::default(),
            run_example_dir,
            prompt_len: 0,
            generated_len: 0,
        }
    }

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
