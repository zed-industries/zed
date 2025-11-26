use std::{str::FromStr, sync::Arc};
// use std::sync::Arc;

use crate::inline_assistant::test::run_inline_assistant_test;

use eval_utils::EvalOutput;
use gpui::TestAppContext;
use language_model::{LanguageModelRegistry, SelectedModel};
use rand::{SeedableRng as _, rngs::StdRng};

#[test]
fn eval_single_cursor_edit() {
    eval_utils::eval(
        1,
        1.0,
        0.0,
        Arc::new(|tx| {
            run_eval(
                &EvalInput {
                    prompt: "Rename this variable to buffer_text".to_string(),
                    text: indoc::indoc! {"
                        struct EvalExampleStruct {
                            text: Strˇing,
                            prompt: String,
                        }
                    "}
                    .to_string(),
                },
                tx,
                &|_, output| {
                    EvalOutput::assert(
                        format!("Failed to rename variable, output: {}", output),
                        output
                            == indoc::indoc! {"
                            struct EvalExampleStruct {
                                buffer_text: String,
                                prompt: String,
                            }
                        "},
                    )
                },
            );
        }),
    );
}

struct EvalInput {
    text: String,
    prompt: String,
}

fn run_eval(
    input: &EvalInput,
    tx: std::sync::mpsc::Sender<eval_utils::EvalOutput>,
    judge: &dyn Fn(&EvalInput, &str) -> eval_utils::EvalOutput,
) {
    let dispatcher = gpui::TestDispatcher::new(StdRng::from_os_rng());
    let mut cx = TestAppContext::build(dispatcher, None);

    let buffer_text = run_inline_assistant_test(
        input.text.clone(),
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

    let output = judge(input, &buffer_text);
    tx.send(output).ok();
}
