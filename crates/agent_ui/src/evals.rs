use std::str::FromStr;

use crate::inline_assistant::test::run_inline_assistant_test;

use eval_utils::{EvalOutput, NoProcessor};
use gpui::TestAppContext;
use language_model::{LanguageModelRegistry, SelectedModel};
use rand::{SeedableRng as _, rngs::StdRng};

fn run_eval_test(
    iterations: usize,
    expected_pass_ratio: f32,
    prompt: String,
    buffer: String,
    message: String,
    output_passes: impl (Fn(&str) -> bool) + Send + Sync + 'static,
) {
    eval_utils::eval(iterations, expected_pass_ratio, NoProcessor, move || {
        run_eval(
            &EvalInput {
                prompt: prompt.clone(),
                buffer: buffer.clone(),
            },
            &|_, output| {
                if output_passes(&output) {
                    EvalOutput {
                        outcome: eval_utils::OutcomeKind::Passed,
                        data: format!("{}: Passed!", message),
                        metadata: (),
                    }
                } else {
                    EvalOutput {
                        outcome: eval_utils::OutcomeKind::Failed,
                        data: format!("{}: {}", message, output),
                        metadata: (),
                    }
                }
            },
        )
    });
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_single_cursor_edit() {
    run_eval_test(
        10,
        1.0,
        "Rename this variable to buffer_text".to_string(),
        indoc::indoc! {"
            struct EvalExampleStruct {
                text: Strˇing,
                prompt: String,
            }
        "}
        .to_string(),
        "Failed to rename variable, output".to_string(),
        |output| {
            output
                == indoc::indoc! {"
            struct EvalExampleStruct {
                buffer_text: String,
                prompt: String,
            }
            "}
        },
    );
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_cant_do() {
    run_eval_test(
        10,
        1.0,
        "Rename the struct to EvalExampleStructNope".to_string(),
        indoc::indoc! {"
            struct EvalExampleStruct {
                text: Strˇing,
                prompt: String,
            }
        "}
        .to_string(),
        "No change should have occurred, but got".to_string(),
        |output| {
            output
                == indoc::indoc! {"
            struct EvalExampleStruct {
                text: String,
                prompt: String,
            }
            "}
        },
    );
}

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_unclear() {
    run_eval_test(
        10,
        1.0,
        "Make exactly the change I want you to make".to_string(),
        indoc::indoc! {"
            struct EvalExampleStruct {
                text: Strˇing,
                prompt: String,
            }
        "}
        .to_string(),
        "No change should have occurred, but got".to_string(),
        |output| {
            output
                == indoc::indoc! {"
            struct EvalExampleStruct {
                text: String,
                prompt: String,
            }
            "}
        },
    );
}

struct EvalInput {
    buffer: String,
    prompt: String,
}

fn run_eval(
    input: &EvalInput,
    judge: &dyn Fn(&EvalInput, &str) -> eval_utils::EvalOutput<()>,
) -> eval_utils::EvalOutput<()> {
    let dispatcher = gpui::TestDispatcher::new(StdRng::from_os_rng());
    let mut cx = TestAppContext::build(dispatcher, None);
    cx.skip_drawing();

    let buffer_text = run_inline_assistant_test(
        input.buffer.clone(),
        input.prompt.clone(),
        |cx| {
            // Reconfigure to use a real model instead of the fake one
            let model_name = std::env::var("ZED_AGENT_MODEL")
                .unwrap_or("anthropic/claude-sonnet-4-latest".into());

            let selected_model = SelectedModel::from_str(&model_name)
                .expect("Invalid model format. Use 'provider/model-id'");

            log::info!("Selected model: {selected_model:?}");

            cx.update(|_, cx| {
                LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                    registry.select_inline_assistant_model(Some(&selected_model), cx);
                });
            });
        },
        |_cx| {
            log::info!("Waiting for actual response from the LLM...");
        },
        &mut cx,
    );

    judge(input, &buffer_text)
}
