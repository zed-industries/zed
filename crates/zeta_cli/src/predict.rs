use crate::example::{ActualExcerpt, NamedExample};
use crate::headless::ZetaCliAppState;
use crate::paths::{CACHE_DIR, LATEST_EXAMPLE_RUN_DIR, RUN_DIR, print_run_data_dir};
use crate::{
    CacheMode, PredictArguments, PredictionOptions, PredictionProvider, PredictionsOutputFormat,
};
use ::serde::Serialize;
use anyhow::{Context, Result, anyhow};
use cloud_zeta2_prompt::{CURSOR_MARKER, write_codeblock};
use futures::StreamExt as _;
use gpui::{AppContext, AsyncApp, Entity};
use project::Project;
use project::buffer_store::BufferStoreEvent;
use serde::Deserialize;
use std::fs;
use std::io::{IsTerminal, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use zeta::{EvalCache, EvalCacheEntryKind, EvalCacheKey, Zeta};

pub async fn run_predict(
    args: PredictArguments,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) {
    let example = NamedExample::load(args.example_path).unwrap();
    let project = example.setup_project(app_state, cx).await.unwrap();
    let zeta = setup_zeta(args.options.provider, &project, app_state, cx).unwrap();
    let _edited_buffers = example.apply_edit_history(&project, cx).await.unwrap();
    let result = perform_predict(example, project, zeta, None, args.options, cx)
        .await
        .unwrap();
    result.write(args.format, std::io::stdout()).unwrap();

    print_run_data_dir(true, std::io::stdout().is_terminal());
}

pub fn setup_zeta(
    provider: PredictionProvider,
    project: &Entity<Project>,
    app_state: &Arc<ZetaCliAppState>,
    cx: &mut AsyncApp,
) -> Result<Entity<Zeta>> {
    let zeta =
        cx.new(|cx| zeta::Zeta::new(app_state.client.clone(), app_state.user_store.clone(), cx))?;

    zeta.update(cx, |zeta, _cx| {
        let model = match provider {
            PredictionProvider::Zeta1 => zeta::ZetaEditPredictionModel::Zeta1,
            PredictionProvider::Zeta2 => zeta::ZetaEditPredictionModel::Zeta2,
            PredictionProvider::Sweep => zeta::ZetaEditPredictionModel::Sweep,
        };
        zeta.set_edit_prediction_model(model);
    })?;

    let buffer_store = project.read_with(cx, |project, _| project.buffer_store().clone())?;

    cx.subscribe(&buffer_store, {
        let project = project.clone();
        let zeta = zeta.clone();
        move |_, event, cx| match event {
            BufferStoreEvent::BufferAdded(buffer) => {
                zeta.update(cx, |zeta, cx| zeta.register_buffer(&buffer, &project, cx));
            }
            _ => {}
        }
    })?
    .detach();

    anyhow::Ok(zeta)
}

pub async fn perform_predict(
    example: NamedExample,
    project: Entity<Project>,
    zeta: Entity<Zeta>,
    repetition_ix: Option<u16>,
    options: PredictionOptions,
    cx: &mut AsyncApp,
) -> Result<PredictionDetails> {
    let mut cache_mode = options.cache;
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

    let prompt_format = options.zeta2.prompt_format;

    zeta.update(cx, |zeta, _cx| {
        let mut options = zeta.options().clone();
        options.prompt_format = prompt_format.into();
        zeta.set_options(options);
    })?;

    let mut debug_task = gpui::Task::ready(Ok(()));

    if options.provider == crate::PredictionProvider::Zeta2 {
        let mut debug_rx = zeta.update(cx, |zeta, _| zeta.debug_info())?;

        debug_task = cx.background_spawn({
            let result = result.clone();
            async move {
                let mut start_time = None;
                let mut search_queries_generated_at = None;
                let mut search_queries_executed_at = None;
                while let Some(event) = debug_rx.next().await {
                    match event {
                        zeta::ZetaDebugInfo::ContextRetrievalStarted(info) => {
                            start_time = Some(info.timestamp);
                            fs::write(
                                example_run_dir.join("search_prompt.md"),
                                &info.search_prompt,
                            )?;
                        }
                        zeta::ZetaDebugInfo::SearchQueriesGenerated(info) => {
                            search_queries_generated_at = Some(info.timestamp);
                            fs::write(
                                example_run_dir.join("search_queries.json"),
                                serde_json::to_string_pretty(&info.search_queries).unwrap(),
                            )?;
                        }
                        zeta::ZetaDebugInfo::SearchQueriesExecuted(info) => {
                            search_queries_executed_at = Some(info.timestamp);
                        }
                        zeta::ZetaDebugInfo::ContextRetrievalFinished(_info) => {}
                        zeta::ZetaDebugInfo::EditPredictionRequested(request) => {
                            let prediction_started_at = Instant::now();
                            start_time.get_or_insert(prediction_started_at);
                            let prompt = request.local_prompt.unwrap_or_default();
                            fs::write(example_run_dir.join("prediction_prompt.md"), &prompt)?;

                            {
                                let mut result = result.lock().unwrap();
                                result.prompt_len = prompt.chars().count();

                                for included_file in request.inputs.included_files {
                                    let insertions =
                                        vec![(request.inputs.cursor_point, CURSOR_MARKER)];
                                    result.excerpts.extend(included_file.excerpts.iter().map(
                                        |excerpt| ActualExcerpt {
                                            path: included_file.path.components().skip(1).collect(),
                                            text: String::from(excerpt.text.as_ref()),
                                        },
                                    ));
                                    write_codeblock(
                                        &included_file.path,
                                        included_file.excerpts.iter(),
                                        if included_file.path == request.inputs.cursor_path {
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

                            let response =
                                request.response_rx.await?.0.map_err(|err| anyhow!(err))?;
                            let response = zeta::text_from_response(response).unwrap_or_default();
                            let prediction_finished_at = Instant::now();
                            fs::write(example_run_dir.join("prediction_response.md"), &response)?;

                            let mut result = result.lock().unwrap();
                            result.generated_len = response.chars().count();

                            result.planning_search_time =
                                Some(search_queries_generated_at.unwrap() - start_time.unwrap());
                            result.running_search_time = Some(
                                search_queries_executed_at.unwrap()
                                    - search_queries_generated_at.unwrap(),
                            );
                            result.prediction_time = prediction_finished_at - prediction_started_at;
                            result.total_time = prediction_finished_at - start_time.unwrap();

                            break;
                        }
                    }
                }
                anyhow::Ok(())
            }
        });

        zeta.update(cx, |zeta, cx| {
            zeta.refresh_context(project.clone(), cursor_buffer.clone(), cursor_anchor, cx)
        })?
        .await?;
    }

    let prediction = zeta
        .update(cx, |zeta, cx| {
            zeta.request_prediction(
                &project,
                &cursor_buffer,
                cursor_anchor,
                cloud_llm_client::PredictEditsRequestTrigger::Cli,
                cx,
            )
        })?
        .await?;

    debug_task.await?;

    let mut result = Arc::into_inner(result).unwrap().into_inner().unwrap();

    result.diff = prediction
        .and_then(|prediction| {
            let prediction = prediction.prediction.ok()?;
            prediction.edit_preview.as_unified_diff(&prediction.edits)
        })
        .unwrap_or_default();

    anyhow::Ok(result)
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
