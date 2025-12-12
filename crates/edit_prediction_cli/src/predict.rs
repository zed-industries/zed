use crate::{
    PredictionProvider, PromptFormat,
    anthropic_client::AnthropicClient,
    example::{Example, ExamplePrediction},
    format_prompt::{TeacherPrompt, run_format_prompt},
    headless::EpAppState,
    load_project::run_load_project,
    paths::{LATEST_EXAMPLE_RUN_DIR, RUN_DIR},
    progress::{InfoStyle, Progress, Step},
    retrieve_context::run_context_retrieval,
};
use edit_prediction::{DebugEvent, EditPredictionStore};
use futures::{FutureExt as _, StreamExt as _, future::Shared};
use gpui::{AppContext as _, AsyncApp, Task};
use std::{
    fs,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};

pub async fn run_prediction(
    example: &mut Example,
    provider: Option<PredictionProvider>,
    repetition_count: usize,
    app_state: Arc<EpAppState>,
    progress: Arc<Progress>,
    mut cx: AsyncApp,
) {
    if !example.predictions.is_empty() {
        return;
    }

    let provider = provider.unwrap();

    run_context_retrieval(example, app_state.clone(), progress.clone(), cx.clone()).await;

    if matches!(
        provider,
        PredictionProvider::Teacher | PredictionProvider::TeacherNonBatching
    ) {
        let _step_progress = progress.start(Step::Predict, &example.name);

        if example.prompt.is_none() {
            run_format_prompt(
                example,
                PromptFormat::Teacher,
                app_state.clone(),
                progress,
                cx,
            )
            .await;
        }

        let batched = matches!(provider, PredictionProvider::Teacher);
        return predict_anthropic(example, repetition_count, batched).await;
    }

    run_load_project(example, app_state.clone(), progress.clone(), cx.clone()).await;

    let _step_progress = progress.start(Step::Predict, &example.name);

    if matches!(
        provider,
        PredictionProvider::Zeta1 | PredictionProvider::Zeta2
    ) {
        static AUTHENTICATED: OnceLock<Shared<Task<()>>> = OnceLock::new();
        AUTHENTICATED
            .get_or_init(|| {
                let client = app_state.client.clone();
                cx.spawn(async move |cx| {
                    client
                        .sign_in_with_optional_connect(true, cx)
                        .await
                        .unwrap();
                })
                .shared()
            })
            .clone()
            .await;
    }

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx).unwrap())
        .unwrap();

    ep_store
        .update(&mut cx, |store, _cx| {
            let model = match provider {
                PredictionProvider::Zeta1 => edit_prediction::EditPredictionModel::Zeta1,
                PredictionProvider::Zeta2 => edit_prediction::EditPredictionModel::Zeta2,
                PredictionProvider::Sweep => edit_prediction::EditPredictionModel::Sweep,
                PredictionProvider::Mercury => edit_prediction::EditPredictionModel::Mercury,
                PredictionProvider::Teacher | PredictionProvider::TeacherNonBatching => {
                    unreachable!()
                }
            };
            store.set_edit_prediction_model(model);
        })
        .unwrap();
    let state = example.state.as_ref().unwrap();
    let run_dir = RUN_DIR.join(&example.name);

    let updated_example = Arc::new(Mutex::new(example.clone()));
    let current_run_ix = Arc::new(AtomicUsize::new(0));

    let mut debug_rx = ep_store
        .update(&mut cx, |store, cx| store.debug_info(&state.project, cx))
        .unwrap();
    let debug_task = cx.background_spawn({
        let updated_example = updated_example.clone();
        let current_run_ix = current_run_ix.clone();
        let run_dir = run_dir.clone();
        async move {
            while let Some(event) = debug_rx.next().await {
                let run_ix = current_run_ix.load(SeqCst);
                let mut updated_example = updated_example.lock().unwrap();

                let run_dir = if repetition_count > 1 {
                    run_dir.join(format!("{:03}", run_ix))
                } else {
                    run_dir.clone()
                };

                match event {
                    DebugEvent::EditPredictionStarted(request) => {
                        assert_eq!(updated_example.predictions.len(), run_ix + 1);

                        if let Some(prompt) = request.prompt {
                            fs::write(run_dir.join("prediction_prompt.md"), &prompt)?;
                        }
                    }
                    DebugEvent::EditPredictionFinished(request) => {
                        assert_eq!(updated_example.predictions.len(), run_ix + 1);

                        if let Some(output) = request.model_output {
                            fs::write(run_dir.join("prediction_response.md"), &output)?;
                            updated_example
                                .predictions
                                .last_mut()
                                .unwrap()
                                .actual_output = output;
                        }
                        if run_ix >= repetition_count {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            anyhow::Ok(())
        }
    });

    for ix in 0..repetition_count {
        current_run_ix.store(ix, SeqCst);
        let run_dir = if repetition_count > 1 {
            run_dir.join(format!("{:03}", ix))
        } else {
            run_dir.clone()
        };

        fs::create_dir_all(&run_dir).unwrap();
        if LATEST_EXAMPLE_RUN_DIR.is_symlink() {
            fs::remove_file(&*LATEST_EXAMPLE_RUN_DIR).unwrap();
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink(&run_dir, &*LATEST_EXAMPLE_RUN_DIR).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&run_dir, &*LATEST_EXAMPLE_RUN_DIR).unwrap();

        updated_example
            .lock()
            .unwrap()
            .predictions
            .push(ExamplePrediction {
                actual_patch: String::new(),
                actual_output: String::new(),
                provider,
            });

        let prediction = ep_store
            .update(&mut cx, |store, cx| {
                store.request_prediction(
                    &state.project,
                    &state.buffer,
                    state.cursor_position,
                    cloud_llm_client::PredictEditsRequestTrigger::Cli,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let actual_patch = prediction
            .and_then(|prediction| {
                let prediction = prediction.prediction.ok()?;
                prediction.edit_preview.as_unified_diff(&prediction.edits)
            })
            .unwrap_or_default();

        let has_prediction = !actual_patch.is_empty();

        updated_example
            .lock()
            .unwrap()
            .predictions
            .last_mut()
            .unwrap()
            .actual_patch = actual_patch;

        if ix == repetition_count - 1 {
            let (info, style) = if has_prediction {
                ("predicted", InfoStyle::Normal)
            } else {
                ("no prediction", InfoStyle::Warning)
            };
            _step_progress.set_info(info, style);
        }
    }

    ep_store
        .update(&mut cx, |store, _| {
            store.remove_project(&state.project);
        })
        .unwrap();
    debug_task.await.unwrap();

    *example = Arc::into_inner(updated_example)
        .unwrap()
        .into_inner()
        .unwrap();
}

async fn predict_anthropic(example: &mut Example, _repetition_count: usize, batched: bool) {
    let llm_model_name = "claude-sonnet-4-5";
    let max_tokens = 16384;
    let llm_client = if batched {
        AnthropicClient::batch(&crate::paths::LLM_CACHE_DB.as_ref())
    } else {
        AnthropicClient::plain()
    };
    let llm_client = llm_client.expect("Failed to create LLM client");

    let prompt = example
        .prompt
        .as_ref()
        .unwrap_or_else(|| panic!("Prompt is required for an example {}", &example.name));

    let messages = vec![anthropic::Message {
        role: anthropic::Role::User,
        content: vec![anthropic::RequestContent::Text {
            text: prompt.input.clone(),
            cache_control: None,
        }],
    }];

    let Some(response) = llm_client
        .generate(llm_model_name, max_tokens, messages)
        .await
        .unwrap()
    else {
        // Request stashed for batched processing
        return;
    };

    let actual_output = response
        .content
        .into_iter()
        .filter_map(|content| match content {
            anthropic::ResponseContent::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("\n");

    let actual_patch = TeacherPrompt::parse(example, &actual_output);

    let prediction = ExamplePrediction {
        actual_patch,
        actual_output,
        provider: PredictionProvider::Teacher,
    };

    example.predictions.push(prediction);
}

pub async fn sync_batches(provider: &PredictionProvider) {
    match provider {
        PredictionProvider::Teacher => {
            let cache_path = crate::paths::LLM_CACHE_DB.as_ref();
            let llm_client =
                AnthropicClient::batch(cache_path).expect("Failed to create LLM client");
            llm_client
                .sync_batches()
                .await
                .expect("Failed to sync batches");
        }
        _ => (),
    }
}
