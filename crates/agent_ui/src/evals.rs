use std::str::FromStr;
use std::sync::Arc;

use crate::{InlineAssistant, context_store::ContextStore};

use client::{Client, UserStore};
use clock::FakeSystemClock;
use editor::{Editor, MultiBuffer};
use futures::channel::mpsc;
use gpui::{AppContext, TestAppContext, UpdateGlobal};
use http_client::FakeHttpClient;
use language::Buffer;
use language_model::{LanguageModelRegistry, SelectedModel, fake_provider::FakeLanguageModel};
use project::{FakeFs, Project};
use prompt_store::PromptBuilder;
use smol::stream::StreamExt;
use workspace::Workspace;

#[gpui::test]
#[cfg_attr(not(feature = "unit-eval"), ignore)]
async fn eval_inline_assistant(cx: &mut TestAppContext) {
    let use_real_model = std::env::var("USE_REAL_MODEL").is_ok();
    if use_real_model {
        cx.executor().allow_parking();
    }

    let fs = FakeFs::new(cx.executor());
    let app_state = cx.update(|cx| workspace::AppState::test(cx));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let http = Arc::new(reqwest_client::ReqwestClient::user_agent("agent tests").unwrap());
    let client = cx.update(|cx| {
        cx.set_http_client(http);
        Client::production(cx)
    });
    let mut inline_assistant = InlineAssistant::new(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
    );

    let mut completion_rx = {
        let (tx, rx) = mpsc::unbounded();
        inline_assistant.set_completion_receiver(tx);
        rx
    };

    // Initialize settings and client
    cx.update(|cx| {
        gpui_tokio::init(cx);
        settings::init(cx);
        client::init(&client, cx);
        workspace::init(app_state.clone(), cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store, client.clone(), cx);

        cx.set_global(inline_assistant);
    });

    // Initialize required systems and set up language model
    let fake_model = cx.update(|cx| {
        if use_real_model {
            // Reconfigure to use a real model instead of the fake one
            let model_name = std::env::var("ZED_AGENT_MODEL")
                .unwrap_or("anthropic/claude-sonnet-4-latest".into());

            let selected_model = SelectedModel::from_str(&model_name)
                .expect("Invalid model format. Use 'provider/model-id'");

            println!("MODEL {selected_model:?}");
            println!("Using real model: {}", model_name);
            println!("NOTE: Real models require authentication/API keys to be configured");
            println!("      Set ANTHROPIC_API_KEY, OPENAI_API_KEY, etc. in your environment");

            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry.select_inline_assistant_model(Some(&selected_model), cx);
            });

            None
        } else {
            LanguageModelRegistry::global(cx).update(cx, |registry, _| Some(registry.fake_model()))
        }
    });

    let project = Project::test(fs.clone(), [], cx).await;

    // Create workspace with window
    let (workspace, window_cx) = cx.add_window_view(|window, cx| {
        window.activate_window();
        Workspace::new(None, project.clone(), app_state.clone(), window, cx)
    });

    // Create all entities and call assist within the window context to avoid borrowing issues
    let (_editor, buffer) = window_cx.update(|window, cx| {
        let buffer = cx.new(|cx| Buffer::local("// Test buffer content\n", cx));
        let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
        let editor = cx.new(|cx| Editor::for_multibuffer(multibuffer, None, window, cx));
        let context_store = cx.new(|_cx| ContextStore::new(project.downgrade()));

        // Add editor to workspace
        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        });

        // Call assist method
        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            let assist_id = inline_assistant
                .assist(
                    &editor,
                    workspace.downgrade(),
                    context_store,
                    project.downgrade(),
                    None, // prompt_store
                    None, // thread_store
                    Some("Add another comment line".to_string()),
                    window,
                    cx,
                )
                .unwrap();
            inline_assistant.start_assist(assist_id, window, cx);
        });

        (editor, buffer)
    });

    // Run until parked to allow the assist to start
    cx.run_until_parked();

    // If using fake model, simulate the language model responding
    if let Some(fake_model) = fake_model {
        let fake = fake_model.as_fake();
        // let fake = fake_model;
        fake.send_last_completion_stream_text_chunk(
            "// This is a helpful comment\n// explaining what this code does\n",
        );
        fake.end_last_completion_stream();

        // Run again to process the model's response
        cx.run_until_parked();
    } else {
        println!("Using real model - waiting for actual response...");

        cx.executor()
            .block_test(async { completion_rx.next().await });
    }

    // Step 0. Get actual prompting working in our psuedo-harness
    // Step 1. Think up a nice eval that does something simple
    // Step 2. Pull out the `eval()` method, we're all going to use "iterations" and "expected passrate"
    // Step 3. Implement an EvalInput for inline assistant, that does the stuff we did earlier

    let buffer_text = buffer.read_with(cx, |buffer, _| buffer.text());

    let original_text = "// Test buffer content\n";
    if buffer_text != original_text {
        println!("\n=== Changes detected! ===");
        println!("Buffer text:\n{}", buffer_text);
    } else {
        println!("\n=== No changes made ===");
        println!("Note: Codegen may still be running or language model may not be responding");
    }
}
