use std::str::FromStr;

use crate::inline_assistant::test::run_inline_assistant_test;

use eval_utils::{EvalOutput, NoProcessor};
use gpui::TestAppContext;
use language_model::{LanguageModelRegistry, SelectedModel};
use rand::{SeedableRng as _, rngs::StdRng};

#[test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
fn eval_single_cursor_edit() {
    eval_utils::eval(20, 1.0, NoProcessor, move || {
        run_eval(
            &EvalInput {
                prompt: "Rename this variable to buffer_text".to_string(),
                buffer: indoc::indoc! {"
                    struct EvalExampleStruct {
                        text: Strˇing,
                        prompt: String,
                    }
                "}
                .to_string(),
            },
            &|_, output| {
                let expected = indoc::indoc! {"
                    struct EvalExampleStruct {
                        buffer_text: String,
                        prompt: String,
                    }
                    "};
                if output == expected {
                    EvalOutput {
                        outcome: eval_utils::OutcomeKind::Passed,
                        data: "Passed!".to_string(),
                        metadata: (),
                    }
                } else {
                    EvalOutput {
                        outcome: eval_utils::OutcomeKind::Failed,
                        data: format!("Failed to rename variable, output: {}", output),
                        metadata: (),
                    }
                }
            },
        )
    });
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
