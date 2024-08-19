use crate::{
    assistant_panel, prompt_library, slash_command::file_command, workflow::tool, Context,
    ContextEvent, ContextId, ContextOperation, MessageId, MessageStatus, PromptBuilder,
};
use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandRegistry,
};
use collections::HashSet;
use fs::{FakeFs, Fs as _};
use gpui::{AppContext, Model, SharedString, Task, TestAppContext, WeakView};
use indoc::indoc;
use language::{Buffer, LanguageRegistry, LspAdapterDelegate};
use language_model::{LanguageModelRegistry, Role};
use parking_lot::Mutex;
use project::Project;
use rand::prelude::*;
use rope::Point;
use serde_json::json;
use settings::SettingsStore;
use std::{
    cell::RefCell,
    env,
    ops::Range,
    path::Path,
    rc::Rc,
    sync::{atomic::AtomicBool, Arc},
};
use text::{network::Network, OffsetRangeExt as _, ReplicaId, ToPoint as _};
use ui::{Context as _, WindowContext};
use unindent::Unindent;
use util::{test::marked_text_ranges, RandomCharIter};
use workspace::Workspace;

#[gpui::test]
fn test_inserting_and_removing_messages(cx: &mut AppContext) {
    let settings_store = SettingsStore::test(cx);
    LanguageModelRegistry::test(cx);
    cx.set_global(settings_store);
    assistant_panel::init(cx);
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context =
        cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
    let buffer = context.read(cx).buffer.clone();

    let message_1 = context.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&context, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    let message_2 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..1),
            (message_2.id, Role::Assistant, 1..1)
        ]
    );

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..3)
        ]
    );

    let message_3 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..4),
            (message_3.id, Role::User, 4..4)
        ]
    );

    let message_4 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..4),
            (message_4.id, Role::User, 4..5),
            (message_3.id, Role::User, 5..5),
        ]
    );

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(4..4, "C"), (5..5, "D")], None, cx)
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..4),
            (message_4.id, Role::User, 4..6),
            (message_3.id, Role::User, 6..7),
        ]
    );

    // Deleting across message boundaries merges the messages.
    buffer.update(cx, |buffer, cx| buffer.edit([(1..4, "")], None, cx));
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_3.id, Role::User, 3..4),
        ]
    );

    // Undoing the deletion should also undo the merge.
    buffer.update(cx, |buffer, cx| buffer.undo(cx));
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..4),
            (message_4.id, Role::User, 4..6),
            (message_3.id, Role::User, 6..7),
        ]
    );

    // Redoing the deletion should also redo the merge.
    buffer.update(cx, |buffer, cx| buffer.redo(cx));
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_3.id, Role::User, 3..4),
        ]
    );

    // Ensure we can still insert after a merged message.
    let message_5 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_5.id, Role::System, 3..4),
            (message_3.id, Role::User, 4..5)
        ]
    );
}

#[gpui::test]
fn test_message_splitting(cx: &mut AppContext) {
    let settings_store = SettingsStore::test(cx);
    cx.set_global(settings_store);
    LanguageModelRegistry::test(cx);
    assistant_panel::init(cx);
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context =
        cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
    let buffer = context.read(cx).buffer.clone();

    let message_1 = context.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&context, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "aaa\nbbb\nccc\nddd\n")], None, cx)
    });

    let (_, message_2) = context.update(cx, |context, cx| context.split_message(3..3, cx));
    let message_2 = message_2.unwrap();

    // We recycle newlines in the middle of a split message
    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..16),
        ]
    );

    let (_, message_3) = context.update(cx, |context, cx| context.split_message(3..3, cx));
    let message_3 = message_3.unwrap();

    // We don't recycle newlines at the end of a split message
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..17),
        ]
    );

    let (_, message_4) = context.update(cx, |context, cx| context.split_message(9..9, cx));
    let message_4 = message_4.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..9),
            (message_4.id, Role::User, 9..17),
        ]
    );

    let (_, message_5) = context.update(cx, |context, cx| context.split_message(9..9, cx));
    let message_5 = message_5.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\nddd\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..9),
            (message_4.id, Role::User, 9..10),
            (message_5.id, Role::User, 10..18),
        ]
    );

    let (message_6, message_7) =
        context.update(cx, |context, cx| context.split_message(14..16, cx));
    let message_6 = message_6.unwrap();
    let message_7 = message_7.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\ndd\nd\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..9),
            (message_4.id, Role::User, 9..10),
            (message_5.id, Role::User, 10..14),
            (message_6.id, Role::User, 14..17),
            (message_7.id, Role::User, 17..19),
        ]
    );
}

#[gpui::test]
fn test_messages_for_offsets(cx: &mut AppContext) {
    let settings_store = SettingsStore::test(cx);
    LanguageModelRegistry::test(cx);
    cx.set_global(settings_store);
    assistant_panel::init(cx);
    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context =
        cx.new_model(|cx| Context::local(registry, None, None, prompt_builder.clone(), cx));
    let buffer = context.read(cx).buffer.clone();

    let message_1 = context.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&context, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
    let message_2 = context
        .update(cx, |context, cx| {
            context.insert_message_after(message_1.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbb")], None, cx));

    let message_3 = context
        .update(cx, |context, cx| {
            context.insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.edit([(8..8, "ccc")], None, cx));

    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..8),
            (message_3.id, Role::User, 8..11)
        ]
    );

    assert_eq!(
        message_ids_for_offsets(&context, &[0, 4, 9], cx),
        [message_1.id, message_2.id, message_3.id]
    );
    assert_eq!(
        message_ids_for_offsets(&context, &[0, 1, 11], cx),
        [message_1.id, message_3.id]
    );

    let message_4 = context
        .update(cx, |context, cx| {
            context.insert_message_after(message_3.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\n");
    assert_eq!(
        messages(&context, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..8),
            (message_3.id, Role::User, 8..12),
            (message_4.id, Role::User, 12..12)
        ]
    );
    assert_eq!(
        message_ids_for_offsets(&context, &[0, 4, 8, 12], cx),
        [message_1.id, message_2.id, message_3.id, message_4.id]
    );

    fn message_ids_for_offsets(
        context: &Model<Context>,
        offsets: &[usize],
        cx: &AppContext,
    ) -> Vec<MessageId> {
        context
            .read(cx)
            .messages_for_offsets(offsets.iter().copied(), cx)
            .into_iter()
            .map(|message| message.id)
            .collect()
    }
}

#[gpui::test]
async fn test_slash_commands(cx: &mut TestAppContext) {
    let settings_store = cx.update(SettingsStore::test);
    cx.set_global(settings_store);
    cx.update(LanguageModelRegistry::test);
    cx.update(Project::init_settings);
    cx.update(assistant_panel::init);
    let fs = FakeFs::new(cx.background_executor.clone());

    fs.insert_tree(
        "/test",
        json!({
            "src": {
                "lib.rs": "fn one() -> usize { 1 }",
                "main.rs": "
                    use crate::one;
                    fn main() { one(); }
                ".unindent(),
            }
        }),
    )
    .await;

    let slash_command_registry = cx.update(SlashCommandRegistry::default_global);
    slash_command_registry.register_command(file_command::FileSlashCommand, false);

    let registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context =
        cx.new_model(|cx| Context::local(registry.clone(), None, None, prompt_builder.clone(), cx));

    let output_ranges = Rc::new(RefCell::new(HashSet::default()));
    context.update(cx, |_, cx| {
        cx.subscribe(&context, {
            let ranges = output_ranges.clone();
            move |_, _, event, _| match event {
                ContextEvent::PendingSlashCommandsUpdated { removed, updated } => {
                    for range in removed {
                        ranges.borrow_mut().remove(range);
                    }
                    for command in updated {
                        ranges.borrow_mut().insert(command.source_range.clone());
                    }
                }
                _ => {}
            }
        })
        .detach();
    });

    let buffer = context.read_with(cx, |context, _| context.buffer.clone());

    // Insert a slash command
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "/file src/lib.rs")], None, cx);
    });
    assert_text_and_output_ranges(
        &buffer,
        &output_ranges.borrow(),
        "
        «/file src/lib.rs»
        "
        .unindent()
        .trim_end(),
        cx,
    );

    // Edit the argument of the slash command.
    buffer.update(cx, |buffer, cx| {
        let edit_offset = buffer.text().find("lib.rs").unwrap();
        buffer.edit([(edit_offset..edit_offset + "lib".len(), "main")], None, cx);
    });
    assert_text_and_output_ranges(
        &buffer,
        &output_ranges.borrow(),
        "
        «/file src/main.rs»
        "
        .unindent()
        .trim_end(),
        cx,
    );

    // Edit the name of the slash command, using one that doesn't exist.
    buffer.update(cx, |buffer, cx| {
        let edit_offset = buffer.text().find("/file").unwrap();
        buffer.edit(
            [(edit_offset..edit_offset + "/file".len(), "/unknown")],
            None,
            cx,
        );
    });
    assert_text_and_output_ranges(
        &buffer,
        &output_ranges.borrow(),
        "
        /unknown src/main.rs
        "
        .unindent()
        .trim_end(),
        cx,
    );

    #[track_caller]
    fn assert_text_and_output_ranges(
        buffer: &Model<Buffer>,
        ranges: &HashSet<Range<language::Anchor>>,
        expected_marked_text: &str,
        cx: &mut TestAppContext,
    ) {
        let (expected_text, expected_ranges) = marked_text_ranges(expected_marked_text, false);
        let (actual_text, actual_ranges) = buffer.update(cx, |buffer, _| {
            let mut ranges = ranges
                .iter()
                .map(|range| range.to_offset(buffer))
                .collect::<Vec<_>>();
            ranges.sort_by_key(|a| a.start);
            (buffer.text(), ranges)
        });

        assert_eq!(actual_text, expected_text);
        assert_eq!(actual_ranges, expected_ranges);
    }
}

#[gpui::test]
async fn test_workflow_step_parsing(cx: &mut TestAppContext) {
    cx.update(prompt_library::init);
    let settings_store = cx.update(SettingsStore::test);
    cx.set_global(settings_store);
    cx.update(Project::init_settings);
    let fs = FakeFs::new(cx.executor());
    fs.as_fake()
        .insert_tree(
            "/root",
            json!({
                "hello.rs": r#"
                    fn hello() {
                        println!("Hello, World!");
                    }
                "#.unindent()
            }),
        )
        .await;
    let project = Project::test(fs, [Path::new("/root")], cx).await;
    cx.update(LanguageModelRegistry::test);

    let model = cx.read(|cx| {
        LanguageModelRegistry::read_global(cx)
            .active_model()
            .unwrap()
    });
    cx.update(assistant_panel::init);
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    // Create a new context
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context = cx.new_model(|cx| {
        Context::local(
            registry.clone(),
            Some(project),
            None,
            prompt_builder.clone(),
            cx,
        )
    });
    let buffer = context.read_with(cx, |context, _| context.buffer.clone());

    // Simulate user input
    let user_message = indoc! {r#"
        Please add unnecessary complexity to this code:

        ```hello.rs
        fn main() {
            println!("Hello, World!");
        }
        ```
    "#};
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, user_message)], None, cx);
    });

    // Simulate LLM response with edit steps
    let llm_response = indoc! {r#"
        Sure, I can help you with that. Here's a step-by-step process:

        <step>
        First, let's extract the greeting into a separate function:

        ```rust
        fn greet() {
            println!("Hello, World!");
        }

        fn main() {
            greet();
        }
        ```
        </step>

        <step>
        Now, let's make the greeting customizable:

        ```rust
        fn greet(name: &str) {
            println!("Hello, {}!", name);
        }

        fn main() {
            greet("World");
        }
        ```
        </step>

        These changes make the code more modular and flexible.
    "#};

    // Simulate the assist method to trigger the LLM response
    context.update(cx, |context, cx| context.assist(cx));
    cx.run_until_parked();

    // Retrieve the assistant response message's start from the context
    let response_start_row = context.read_with(cx, |context, cx| {
        let buffer = context.buffer.read(cx);
        context.message_anchors[1].start.to_point(buffer).row
    });

    // Simulate the LLM completion
    model
        .as_fake()
        .stream_last_completion_response(llm_response.to_string());
    model.as_fake().end_last_completion_stream();

    // Wait for the completion to be processed
    cx.run_until_parked();

    // Verify that the edit steps were parsed correctly
    context.read_with(cx, |context, cx| {
        assert_eq!(
            workflow_steps(context, cx),
            vec![
                (
                    Point::new(response_start_row + 2, 0)..Point::new(response_start_row + 12, 3),
                    WorkflowStepTestStatus::Pending
                ),
                (
                    Point::new(response_start_row + 14, 0)..Point::new(response_start_row + 24, 3),
                    WorkflowStepTestStatus::Pending
                ),
            ]
        );
    });

    model
        .as_fake()
        .respond_to_last_tool_use(tool::WorkflowStepResolutionTool {
            step_title: "Title".into(),
            suggestions: vec![tool::WorkflowSuggestionTool {
                path: "/root/hello.rs".into(),
                // Simulate a symbol name that's slightly different than our outline query
                kind: tool::WorkflowSuggestionToolKind::Update {
                    symbol: "fn main()".into(),
                    description: "Extract a greeting function".into(),
                },
            }],
        });

    // Wait for tool use to be processed.
    cx.run_until_parked();

    // Verify that the first edit step is not pending anymore.
    context.read_with(cx, |context, cx| {
        assert_eq!(
            workflow_steps(context, cx),
            vec![
                (
                    Point::new(response_start_row + 2, 0)..Point::new(response_start_row + 12, 3),
                    WorkflowStepTestStatus::Resolved
                ),
                (
                    Point::new(response_start_row + 14, 0)..Point::new(response_start_row + 24, 3),
                    WorkflowStepTestStatus::Pending
                ),
            ]
        );
    });

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum WorkflowStepTestStatus {
        Pending,
        Resolved,
        Error,
    }

    fn workflow_steps(
        context: &Context,
        cx: &AppContext,
    ) -> Vec<(Range<Point>, WorkflowStepTestStatus)> {
        context
            .workflow_steps
            .iter()
            .map(|step| {
                let buffer = context.buffer.read(cx);
                let status = match &step.step.read(cx).resolution {
                    None => WorkflowStepTestStatus::Pending,
                    Some(Ok(_)) => WorkflowStepTestStatus::Resolved,
                    Some(Err(_)) => WorkflowStepTestStatus::Error,
                };
                (step.range.to_point(buffer), status)
            })
            .collect()
    }
}

#[gpui::test]
async fn test_serialization(cx: &mut TestAppContext) {
    let settings_store = cx.update(SettingsStore::test);
    cx.set_global(settings_store);
    cx.update(LanguageModelRegistry::test);
    cx.update(assistant_panel::init);
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context =
        cx.new_model(|cx| Context::local(registry.clone(), None, None, prompt_builder.clone(), cx));
    let buffer = context.read_with(cx, |context, _| context.buffer.clone());
    let message_0 = context.read_with(cx, |context, _| context.message_anchors[0].id);
    let message_1 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_0, Role::Assistant, MessageStatus::Done, cx)
            .unwrap()
    });
    let message_2 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "a"), (1..1, "b\nc")], None, cx);
        buffer.finalize_last_transaction();
    });
    let _message_3 = context.update(cx, |context, cx| {
        context
            .insert_message_after(message_2.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    buffer.update(cx, |buffer, cx| buffer.undo(cx));
    assert_eq!(buffer.read_with(cx, |buffer, _| buffer.text()), "a\nb\nc\n");
    assert_eq!(
        cx.read(|cx| messages(&context, cx)),
        [
            (message_0, Role::User, 0..2),
            (message_1.id, Role::Assistant, 2..6),
            (message_2.id, Role::System, 6..6),
        ]
    );

    let serialized_context = context.read_with(cx, |context, cx| context.serialize(cx));
    let deserialized_context = cx.new_model(|cx| {
        Context::deserialize(
            serialized_context,
            Default::default(),
            registry.clone(),
            prompt_builder.clone(),
            None,
            None,
            cx,
        )
    });
    let deserialized_buffer =
        deserialized_context.read_with(cx, |context, _| context.buffer.clone());
    assert_eq!(
        deserialized_buffer.read_with(cx, |buffer, _| buffer.text()),
        "a\nb\nc\n"
    );
    assert_eq!(
        cx.read(|cx| messages(&deserialized_context, cx)),
        [
            (message_0, Role::User, 0..2),
            (message_1.id, Role::Assistant, 2..6),
            (message_2.id, Role::System, 6..6),
        ]
    );
}

#[gpui::test(iterations = 100)]
async fn test_random_context_collaboration(cx: &mut TestAppContext, mut rng: StdRng) {
    let min_peers = env::var("MIN_PEERS")
        .map(|i| i.parse().expect("invalid `MIN_PEERS` variable"))
        .unwrap_or(2);
    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(50);

    let settings_store = cx.update(SettingsStore::test);
    cx.set_global(settings_store);
    cx.update(LanguageModelRegistry::test);

    cx.update(assistant_panel::init);
    let slash_commands = cx.update(SlashCommandRegistry::default_global);
    slash_commands.register_command(FakeSlashCommand("cmd-1".into()), false);
    slash_commands.register_command(FakeSlashCommand("cmd-2".into()), false);
    slash_commands.register_command(FakeSlashCommand("cmd-3".into()), false);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
    let network = Arc::new(Mutex::new(Network::new(rng.clone())));
    let mut contexts = Vec::new();

    let num_peers = rng.gen_range(min_peers..=max_peers);
    let context_id = ContextId::new();
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    for i in 0..num_peers {
        let context = cx.new_model(|cx| {
            Context::new(
                context_id.clone(),
                i as ReplicaId,
                language::Capability::ReadWrite,
                registry.clone(),
                prompt_builder.clone(),
                None,
                None,
                cx,
            )
        });

        cx.update(|cx| {
            cx.subscribe(&context, {
                let network = network.clone();
                move |_, event, _| {
                    if let ContextEvent::Operation(op) = event {
                        network
                            .lock()
                            .broadcast(i as ReplicaId, vec![op.to_proto()]);
                    }
                }
            })
            .detach();
        });

        contexts.push(context);
        network.lock().add_peer(i as ReplicaId);
    }

    let mut mutation_count = operations;

    while mutation_count > 0
        || !network.lock().is_idle()
        || network.lock().contains_disconnected_peers()
    {
        let context_index = rng.gen_range(0..contexts.len());
        let context = &contexts[context_index];

        match rng.gen_range(0..100) {
            0..=29 if mutation_count > 0 => {
                log::info!("Context {}: edit buffer", context_index);
                context.update(cx, |context, cx| {
                    context
                        .buffer
                        .update(cx, |buffer, cx| buffer.randomly_edit(&mut rng, 1, cx));
                });
                mutation_count -= 1;
            }
            30..=44 if mutation_count > 0 => {
                context.update(cx, |context, cx| {
                    let range = context.buffer.read(cx).random_byte_range(0, &mut rng);
                    log::info!("Context {}: split message at {:?}", context_index, range);
                    context.split_message(range, cx);
                });
                mutation_count -= 1;
            }
            45..=59 if mutation_count > 0 => {
                context.update(cx, |context, cx| {
                    if let Some(message) = context.messages(cx).choose(&mut rng) {
                        let role = *[Role::User, Role::Assistant, Role::System]
                            .choose(&mut rng)
                            .unwrap();
                        log::info!(
                            "Context {}: insert message after {:?} with {:?}",
                            context_index,
                            message.id,
                            role
                        );
                        context.insert_message_after(message.id, role, MessageStatus::Done, cx);
                    }
                });
                mutation_count -= 1;
            }
            60..=74 if mutation_count > 0 => {
                context.update(cx, |context, cx| {
                    let command_text = "/".to_string()
                        + slash_commands
                            .command_names()
                            .choose(&mut rng)
                            .unwrap()
                            .clone()
                            .as_ref();

                    let command_range = context.buffer.update(cx, |buffer, cx| {
                        let offset = buffer.random_byte_range(0, &mut rng).start;
                        buffer.edit(
                            [(offset..offset, format!("\n{}\n", command_text))],
                            None,
                            cx,
                        );
                        offset + 1..offset + 1 + command_text.len()
                    });

                    let output_len = rng.gen_range(1..=10);
                    let output_text = RandomCharIter::new(&mut rng)
                        .filter(|c| *c != '\r')
                        .take(output_len)
                        .collect::<String>();

                    let num_sections = rng.gen_range(0..=3);
                    let mut sections = Vec::with_capacity(num_sections);
                    for _ in 0..num_sections {
                        let section_start = rng.gen_range(0..output_len);
                        let section_end = rng.gen_range(section_start..=output_len);
                        sections.push(SlashCommandOutputSection {
                            range: section_start..section_end,
                            icon: ui::IconName::Ai,
                            label: "section".into(),
                        });
                    }

                    log::info!(
                        "Context {}: insert slash command output at {:?} with {:?}",
                        context_index,
                        command_range,
                        sections
                    );

                    let command_range = context.buffer.read(cx).anchor_after(command_range.start)
                        ..context.buffer.read(cx).anchor_after(command_range.end);
                    context.insert_command_output(
                        command_range,
                        Task::ready(Ok(SlashCommandOutput {
                            text: output_text,
                            sections,
                            run_commands_in_text: false,
                        })),
                        true,
                        false,
                        cx,
                    );
                });
                cx.run_until_parked();
                mutation_count -= 1;
            }
            75..=84 if mutation_count > 0 => {
                context.update(cx, |context, cx| {
                    if let Some(message) = context.messages(cx).choose(&mut rng) {
                        let new_status = match rng.gen_range(0..3) {
                            0 => MessageStatus::Done,
                            1 => MessageStatus::Pending,
                            _ => MessageStatus::Error(SharedString::from("Random error")),
                        };
                        log::info!(
                            "Context {}: update message {:?} status to {:?}",
                            context_index,
                            message.id,
                            new_status
                        );
                        context.update_metadata(message.id, cx, |metadata| {
                            metadata.status = new_status;
                        });
                    }
                });
                mutation_count -= 1;
            }
            _ => {
                let replica_id = context_index as ReplicaId;
                if network.lock().is_disconnected(replica_id) {
                    network.lock().reconnect_peer(replica_id, 0);

                    let (ops_to_send, ops_to_receive) = cx.read(|cx| {
                        let host_context = &contexts[0].read(cx);
                        let guest_context = context.read(cx);
                        (
                            guest_context.serialize_ops(&host_context.version(cx), cx),
                            host_context.serialize_ops(&guest_context.version(cx), cx),
                        )
                    });
                    let ops_to_send = ops_to_send.await;
                    let ops_to_receive = ops_to_receive
                        .await
                        .into_iter()
                        .map(ContextOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                        .unwrap();
                    log::info!(
                        "Context {}: reconnecting. Sent {} operations, received {} operations",
                        context_index,
                        ops_to_send.len(),
                        ops_to_receive.len()
                    );

                    network.lock().broadcast(replica_id, ops_to_send);
                    context
                        .update(cx, |context, cx| context.apply_ops(ops_to_receive, cx))
                        .unwrap();
                } else if rng.gen_bool(0.1) && replica_id != 0 {
                    log::info!("Context {}: disconnecting", context_index);
                    network.lock().disconnect_peer(replica_id);
                } else if network.lock().has_unreceived(replica_id) {
                    log::info!("Context {}: applying operations", context_index);
                    let ops = network.lock().receive(replica_id);
                    let ops = ops
                        .into_iter()
                        .map(ContextOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                        .unwrap();
                    context
                        .update(cx, |context, cx| context.apply_ops(ops, cx))
                        .unwrap();
                }
            }
        }
    }

    cx.read(|cx| {
        let first_context = contexts[0].read(cx);
        for context in &contexts[1..] {
            let context = context.read(cx);
            assert!(context.pending_ops.is_empty());
            assert_eq!(
                context.buffer.read(cx).text(),
                first_context.buffer.read(cx).text(),
                "Context {} text != Context 0 text",
                context.buffer.read(cx).replica_id()
            );
            assert_eq!(
                context.message_anchors,
                first_context.message_anchors,
                "Context {} messages != Context 0 messages",
                context.buffer.read(cx).replica_id()
            );
            assert_eq!(
                context.messages_metadata,
                first_context.messages_metadata,
                "Context {} message metadata != Context 0 message metadata",
                context.buffer.read(cx).replica_id()
            );
            assert_eq!(
                context.slash_command_output_sections,
                first_context.slash_command_output_sections,
                "Context {} slash command output sections != Context 0 slash command output sections",
                context.buffer.read(cx).replica_id()
            );
        }
    });
}

fn messages(context: &Model<Context>, cx: &AppContext) -> Vec<(MessageId, Role, Range<usize>)> {
    context
        .read(cx)
        .messages(cx)
        .map(|message| (message.id, message.role, message.offset_range))
        .collect()
}

#[derive(Clone)]
struct FakeSlashCommand(String);

impl SlashCommand for FakeSlashCommand {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn description(&self) -> String {
        format!("Fake slash command: {}", self.0)
    }

    fn menu_text(&self) -> String {
        format!("Run fake command: {}", self.0)
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(vec![]))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        Task::ready(Ok(SlashCommandOutput {
            text: format!("Executed fake command: {}", self.0),
            sections: vec![],
            run_commands_in_text: false,
        }))
    }
}
