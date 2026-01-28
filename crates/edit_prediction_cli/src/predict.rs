use crate::{
    BatchProvider, FormatPromptArgs, PredictArgs, PredictionProvider, TeacherBackend,
    anthropic_client::AnthropicClient,
    example::{Example, ExamplePrediction, ExamplePrompt},
    format_prompt::{TeacherPrompt, run_format_prompt},
    headless::EpAppState,
    llm_client::{LlmClient, model_for_backend},
    load_project::run_load_project,
    openai_client::OpenAiClient,
    paths::{LATEST_EXAMPLE_RUN_DIR, RUN_DIR},
    progress::{ExampleProgress, InfoStyle, Step},
    qa,
    repair::{build_repair_prompt_for_prediction, needs_repair_qa, parse_repair_response},
    retrieve_context::run_context_retrieval,
};
use anyhow::Context as _;
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

static ANTHROPIC_CLIENT: OnceLock<AnthropicClient> = OnceLock::new();
static OPENAI_CLIENT: OnceLock<OpenAiClient> = OnceLock::new();

pub async fn run_prediction(
    example: &mut Example,
    args: &PredictArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    mut cx: AsyncApp,
) -> anyhow::Result<()> {
    let repetition_count = args.repetitions;

    if let Some(existing_prediction) = example.predictions.first() {
        let has_prediction = existing_prediction.actual_patch.is_some()
            || !existing_prediction.actual_output.is_empty();
        if has_prediction {
            match args.provider {
                None => return Ok(()),
                Some(provider) if existing_prediction.provider == provider => return Ok(()),
                Some(_) => example.predictions.clear(),
            }
        }
    }

    let Some(provider) = args.provider else {
        anyhow::bail!(
            "No existing predictions found. Use --provider to specify which model to use for prediction."
        );
    };

    run_context_retrieval(example, app_state.clone(), example_progress, cx.clone()).await?;

    if let PredictionProvider::Teacher(backend) | PredictionProvider::TeacherNonBatching(backend) =
        provider
    {
        let _step_progress = example_progress.start(Step::Predict);

        run_format_prompt(
            example,
            &FormatPromptArgs { provider },
            app_state.clone(),
            example_progress,
            cx,
        )
        .await?;

        let batched = matches!(provider, PredictionProvider::Teacher(..));
        return predict_teacher(example, backend, batched, repetition_count).await;
    }

    if let PredictionProvider::RepairedTeacher(backend) = provider {
        let _step_progress = example_progress.start(Step::Predict);

        run_format_prompt(
            example,
            &FormatPromptArgs { provider },
            app_state.clone(),
            example_progress,
            cx,
        )
        .await?;

        return predict_repaired_teacher(example, backend, repetition_count).await;
    }

    run_load_project(example, app_state.clone(), example_progress, cx.clone()).await?;

    let step_progress = example_progress.start(Step::Predict);

    if matches!(
        provider,
        PredictionProvider::Zeta1 | PredictionProvider::Zeta2(_)
    ) {
        step_progress.set_substatus("authenticating");
        static AUTHENTICATED: OnceLock<Shared<Task<()>>> = OnceLock::new();
        AUTHENTICATED
            .get_or_init(|| {
                let client = app_state.client.clone();
                cx.spawn(async move |cx| {
                    if let Err(e) = client.sign_in_with_optional_connect(true, cx).await {
                        eprintln!("Authentication failed: {}", e);
                    }
                })
                .shared()
            })
            .clone()
            .await;
    }

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx))
        .context("EditPredictionStore not initialized")?;

    ep_store.update(&mut cx, |store, _cx| {
        let model = match provider {
            PredictionProvider::Zeta1 => edit_prediction::EditPredictionModel::Zeta1,
            PredictionProvider::Zeta2(version) => {
                edit_prediction::EditPredictionModel::Zeta2 { version }
            }
            PredictionProvider::Sweep => edit_prediction::EditPredictionModel::Sweep,
            PredictionProvider::Mercury => edit_prediction::EditPredictionModel::Mercury,
            PredictionProvider::Teacher(..)
            | PredictionProvider::TeacherNonBatching(..)
            | PredictionProvider::RepairedTeacher(..)
            | PredictionProvider::Repair => {
                unreachable!()
            }
        };
        store.set_edit_prediction_model(model);
    });
    step_progress.set_substatus("configuring model");
    let state = example.state.as_ref().context("state must be set")?;
    let run_dir = RUN_DIR.join(&example.spec.name);

    let updated_example = Arc::new(Mutex::new(example.clone()));
    let current_run_ix = Arc::new(AtomicUsize::new(0));

    let mut debug_rx = ep_store.update(&mut cx, |store, cx| store.debug_info(&state.project, cx));
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
                            if matches!(provider, PredictionProvider::Zeta2(_)) {
                                updated_example.prompt.get_or_insert(ExamplePrompt {
                                    input: prompt,
                                    expected_output: String::new(),
                                    rejected_output: None,
                                    provider,
                                });
                            }
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

        fs::create_dir_all(&run_dir)?;
        if LATEST_EXAMPLE_RUN_DIR.is_symlink() {
            fs::remove_file(&*LATEST_EXAMPLE_RUN_DIR)?;
        }
        #[cfg(unix)]
        std::os::unix::fs::symlink(&run_dir, &*LATEST_EXAMPLE_RUN_DIR)?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&run_dir, &*LATEST_EXAMPLE_RUN_DIR)?;

        updated_example
            .lock()
            .unwrap()
            .predictions
            .push(ExamplePrediction {
                actual_patch: None,
                actual_output: String::new(),
                error: None,
                provider,
            });

        step_progress.set_substatus("requesting prediction");
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
            .await?;

        let actual_patch = prediction.and_then(|prediction| {
            let prediction = prediction.prediction.ok()?;
            prediction
                .edit_preview
                .as_unified_diff(prediction.snapshot.file(), &prediction.edits)
        });

        let has_prediction = actual_patch.as_ref().is_some_and(|p| !p.is_empty());

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
            step_progress.set_info(info, style);
        }
    }

    ep_store.update(&mut cx, |store, _| {
        store.remove_project(&state.project);
    });
    debug_task.await?;

    *example = Arc::into_inner(updated_example)
        .ok_or_else(|| anyhow::anyhow!("Failed to unwrap Arc"))?
        .into_inner()
        .map_err(|_| anyhow::anyhow!("Failed to unwrap Mutex"))?;
    Ok(())
}

async fn predict_teacher(
    example: &mut Example,
    backend: TeacherBackend,
    batched: bool,
    repetition_count: usize,
) -> anyhow::Result<()> {
    match backend {
        TeacherBackend::Sonnet45 => {
            predict_anthropic(example, backend, batched, repetition_count).await
        }
        TeacherBackend::Gpt52 => predict_openai(example, backend, batched, repetition_count).await,
    }
}

async fn predict_anthropic(
    example: &mut Example,
    backend: TeacherBackend,
    batched: bool,
    repetition_count: usize,
) -> anyhow::Result<()> {
    let llm_model_name = backend.model_name();
    let max_tokens = 16384;
    let llm_client = ANTHROPIC_CLIENT.get_or_init(|| {
        let client = if batched {
            AnthropicClient::batch(&crate::paths::LLM_CACHE_DB)
        } else {
            AnthropicClient::plain()
        };
        client.expect("Failed to create Anthropic client")
    });

    let prompt = example.prompt.as_ref().context("Prompt is required")?;

    for ix in 0..repetition_count {
        let messages = vec![anthropic::Message {
            role: anthropic::Role::User,
            content: vec![anthropic::RequestContent::Text {
                text: prompt.input.clone(),
                cache_control: None,
            }],
        }];

        let seed = if repetition_count > 1 { Some(ix) } else { None };
        let Some(response) = llm_client
            .generate(llm_model_name, max_tokens, messages, seed)
            .await?
        else {
            // Request stashed for batched processing
            return Ok(());
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

        let actual_patch = TeacherPrompt::parse(example, &actual_output)?;

        let prediction = ExamplePrediction {
            actual_patch: Some(actual_patch),
            actual_output,
            error: None,
            provider: if batched {
                PredictionProvider::Teacher(backend)
            } else {
                PredictionProvider::TeacherNonBatching(backend)
            },
        };

        example.predictions.push(prediction);
    }
    Ok(())
}

async fn predict_openai(
    example: &mut Example,
    backend: TeacherBackend,
    batched: bool,
    repetition_count: usize,
) -> anyhow::Result<()> {
    let llm_model_name = backend.model_name();
    let max_tokens = 16384;
    let llm_client = OPENAI_CLIENT.get_or_init(|| {
        let client = if batched {
            OpenAiClient::batch(&crate::paths::LLM_CACHE_DB)
        } else {
            OpenAiClient::plain()
        };
        client.expect("Failed to create OpenAI client")
    });

    let prompt = example.prompt.as_ref().context("Prompt is required")?;

    for ix in 0..repetition_count {
        let messages = vec![open_ai::RequestMessage::User {
            content: open_ai::MessageContent::Plain(prompt.input.clone()),
        }];

        let seed = if repetition_count > 1 { Some(ix) } else { None };
        let Some(response) = llm_client
            .generate(llm_model_name, max_tokens, messages, seed)
            .await?
        else {
            // Request stashed for batched processing
            return Ok(());
        };

        let actual_output = response
            .choices
            .into_iter()
            .filter_map(|choice| match choice.message {
                open_ai::RequestMessage::Assistant { content, .. } => content.map(|c| match c {
                    open_ai::MessageContent::Plain(text) => text,
                    open_ai::MessageContent::Multipart(parts) => parts
                        .into_iter()
                        .filter_map(|p| match p {
                            open_ai::MessagePart::Text { text } => Some(text),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join(""),
                }),
                _ => None,
            })
            .collect::<Vec<String>>()
            .join("\n");

        let actual_patch = TeacherPrompt::parse(example, &actual_output)?;

        let prediction = ExamplePrediction {
            actual_patch: Some(actual_patch),
            actual_output,
            error: None,
            provider: if batched {
                PredictionProvider::Teacher(backend)
            } else {
                PredictionProvider::TeacherNonBatching(backend)
            },
        };

        example.predictions.push(prediction);
    }
    Ok(())
}

/// Default confidence threshold for repair
const DEFAULT_REPAIR_CONFIDENCE_THRESHOLD: u8 = 3;

/// Predict using teacher model, then run QA evaluation on all predictions,
/// and replace predictions that need repair.
///
/// This is a non-batched flow that processes each step synchronously.
/// - Predictions that pass QA keep their original Teacher provider
/// - Predictions that fail QA are replaced with repaired versions (RepairedTeacher provider)
/// - QA results are not stored because they would be outdated after replacement
async fn predict_repaired_teacher(
    example: &mut Example,
    backend: TeacherBackend,
    repetition_count: usize,
) -> anyhow::Result<()> {
    // Step 1: Run teacher prediction (non-batched for immediate results)
    predict_teacher(example, backend, false, repetition_count).await?;

    if example.predictions.is_empty() {
        return Ok(());
    }

    let batch_provider = match backend {
        TeacherBackend::Sonnet45 => BatchProvider::Anthropic,
        TeacherBackend::Gpt52 => BatchProvider::Openai,
    };
    let llm_client = LlmClient::new(batch_provider, false)?;
    let model = model_for_backend(batch_provider);

    // Step 2: Run QA for all predictions and repair those that need it
    let mut final_predictions = Vec::with_capacity(example.predictions.len());
    let mut final_qa = Vec::with_capacity(example.predictions.len());

    for prediction in &example.predictions {
        // Skip QA if no actual patch was generated
        if prediction.actual_patch.is_none() {
            final_predictions.push(prediction.clone());
            final_qa.push(None);
            continue;
        }

        // Run QA evaluation for this prediction
        let qa_result =
            if let Some(qa_prompt) = qa::build_prompt_for_prediction(example, prediction) {
                match llm_client.generate(model, 1024, &qa_prompt).await? {
                    Some(response_text) => Some(qa::parse_response(&response_text)),
                    None => None,
                }
            } else {
                None
            };

        // Check if repair is needed
        let needs_repair = qa_result
            .as_ref()
            .map(|qa| needs_repair_qa(qa, DEFAULT_REPAIR_CONFIDENCE_THRESHOLD))
            .unwrap_or(false);

        if needs_repair {
            let qa = qa_result
                .as_ref()
                .expect("qa_result must be Some if needs_repair is true");
            // Step 3: Run repair for this prediction
            if let Some(repair_prompt) = build_repair_prompt_for_prediction(example, prediction, qa)
            {
                if let Some(response_text) =
                    llm_client.generate(model, 16384, &repair_prompt).await?
                {
                    match parse_repair_response(example, &response_text) {
                        Ok(mut repaired_prediction) => {
                            repaired_prediction.provider =
                                PredictionProvider::RepairedTeacher(backend);
                            final_predictions.push(repaired_prediction);
                            final_qa.push(qa_result);
                        }
                        Err(e) => {
                            final_predictions.push(ExamplePrediction {
                                actual_patch: None,
                                actual_output: response_text,
                                error: Some(format!("Failed to parse repair response: {}", e)),
                                provider: PredictionProvider::RepairedTeacher(backend),
                            });
                            final_qa.push(qa_result);
                        }
                    }
                } else {
                    // Repair generation returned None, keep original
                    final_predictions.push(prediction.clone());
                    final_qa.push(qa_result);
                }
            } else {
                // Couldn't build repair prompt, keep original
                final_predictions.push(prediction.clone());
                final_qa.push(qa_result);
            }
        } else {
            // No repair needed, keep original (with Teacher provider)
            final_predictions.push(prediction.clone());
            final_qa.push(qa_result);
        }
    }

    example.predictions = final_predictions;
    example.qa = final_qa;

    Ok(())
}

pub async fn sync_batches(provider: Option<&PredictionProvider>) -> anyhow::Result<()> {
    match provider {
        Some(PredictionProvider::Teacher(backend)) => match backend {
            TeacherBackend::Sonnet45 => {
                let llm_client = ANTHROPIC_CLIENT.get_or_init(|| {
                    AnthropicClient::batch(&crate::paths::LLM_CACHE_DB)
                        .expect("Failed to create Anthropic client")
                });
                llm_client
                    .sync_batches()
                    .await
                    .context("Failed to sync Anthropic batches")?;
            }
            TeacherBackend::Gpt52 => {
                let llm_client = OPENAI_CLIENT.get_or_init(|| {
                    OpenAiClient::batch(&crate::paths::LLM_CACHE_DB)
                        .expect("Failed to create OpenAI client")
                });
                llm_client
                    .sync_batches()
                    .await
                    .context("Failed to sync OpenAI batches")?;
            }
        },
        _ => (),
    };
    Ok(())
}
