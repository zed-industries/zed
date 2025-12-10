use crate::{
    PredictionProvider,
    example::{Example, ExamplePrediction},
    headless::EpAppState,
    paths::{LATEST_EXAMPLE_RUN_DIR, RUN_DIR},
    retrieve_context::run_context_retrieval,
};
use edit_prediction::{DebugEvent, EditPredictionStore};
use futures::StreamExt as _;
use gpui::{AppContext as _, AsyncApp};
use std::{
    fs,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};

pub async fn run_prediction(
    example: &mut Example,
    provider: Option<PredictionProvider>,
    repetition_count: usize,
    app_state: Arc<EpAppState>,
    mut cx: AsyncApp,
) {
    if !example.predictions.is_empty() {
        return;
    }

    run_context_retrieval(example, app_state, cx.clone()).await;

    let provider = provider.unwrap();

    let ep_store = cx
        .update(|cx| EditPredictionStore::try_global(cx).unwrap())
        .unwrap();

    ep_store
        .update(&mut cx, |store, _cx| {
            let model = match provider {
                PredictionProvider::Zeta2 => edit_prediction::EditPredictionModel::Zeta2,
                PredictionProvider::Sweep => edit_prediction::EditPredictionModel::Sweep,
                PredictionProvider::Mercury => edit_prediction::EditPredictionModel::Mercury,
                PredictionProvider::AnthropicBatched => todo!(),
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
                assert_eq!(updated_example.predictions.len(), run_ix + 1);

                let run_dir = if repetition_count > 1 {
                    run_dir.join(format!("{:03}", run_ix))
                } else {
                    run_dir.clone()
                };

                match event {
                    DebugEvent::EditPredictionStarted(request) => {
                        if let Some(prompt) = request.prompt {
                            fs::write(run_dir.join("prediction_prompt.md"), &prompt)?;
                        }
                    }
                    DebugEvent::EditPredictionFinished(request) => {
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

        updated_example
            .lock()
            .unwrap()
            .predictions
            .last_mut()
            .unwrap()
            .actual_patch = prediction
            .and_then(|prediction| {
                let prediction = prediction.prediction.ok()?;
                prediction.edit_preview.as_unified_diff(&prediction.edits)
            })
            .unwrap_or_default();
    }

    debug_task.await.unwrap();

    *example = Arc::into_inner(updated_example)
        .unwrap()
        .into_inner()
        .unwrap();
}
