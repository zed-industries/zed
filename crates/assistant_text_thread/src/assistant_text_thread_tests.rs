use crate::{
    CacheStatus, InvokedSlashCommandId, MessageCacheMetadata, MessageId, MessageStatus, TextThread,
    TextThreadEvent, TextThreadId, TextThreadOperation, TextThreadSummary,
};
use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent, SlashCommandOutput,
    SlashCommandOutputSection, SlashCommandRegistry, SlashCommandResult, SlashCommandWorkingSet,
};
use assistant_slash_commands::FileSlashCommand;
use collections::{HashMap, HashSet};
use fs::FakeFs;
use futures::{
    channel::mpsc,
    stream::{self, StreamExt},
};
use gpui::{App, Entity, SharedString, Task, TestAppContext, WeakEntity, prelude::*};
use language::{Buffer, BufferSnapshot, LanguageRegistry, LspAdapterDelegate};
use language_model::{
    ConfiguredModel, LanguageModelCacheConfiguration, LanguageModelRegistry, Role,
    fake_provider::{FakeLanguageModel, FakeLanguageModelProvider},
};
use parking_lot::Mutex;
use pretty_assertions::assert_eq;
use prompt_store::PromptBuilder;
use rand::prelude::*;
use serde_json::json;
use settings::SettingsStore;
use std::{
    cell::RefCell,
    env,
    ops::Range,
    path::Path,
    rc::Rc,
    sync::{Arc, atomic::AtomicBool},
};
use text::{ReplicaId, ToOffset, network::Network};
use ui::{IconName, Window};
use unindent::Unindent;
use util::RandomCharIter;
use workspace::Workspace;

#[gpui::test]
fn test_inserting_and_removing_messages(cx: &mut App) {
    init_test(cx);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry,
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });
    let buffer = text_thread.read(cx).buffer().clone();

    let message_1 = text_thread.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&text_thread, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    let message_2 = text_thread.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..1),
            (message_2.id, Role::Assistant, 1..1)
        ]
    );

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
    });
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..3)
        ]
    );

    let message_3 = text_thread.update(cx, |context, cx| {
        context
            .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..2),
            (message_2.id, Role::Assistant, 2..4),
            (message_3.id, Role::User, 4..4)
        ]
    );

    let message_4 = text_thread.update(cx, |context, cx| {
        context
            .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&text_thread, cx),
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
        messages(&text_thread, cx),
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
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_3.id, Role::User, 3..4),
        ]
    );

    // Undoing the deletion should also undo the merge.
    buffer.update(cx, |buffer, cx| buffer.undo(cx));
    assert_eq!(
        messages(&text_thread, cx),
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
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_3.id, Role::User, 3..4),
        ]
    );

    // Ensure we can still insert after a merged message.
    let message_5 = text_thread.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..3),
            (message_5.id, Role::System, 3..4),
            (message_3.id, Role::User, 4..5)
        ]
    );
}

#[gpui::test]
fn test_message_splitting(cx: &mut App) {
    init_test(cx);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));

    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry.clone(),
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });
    let buffer = text_thread.read(cx).buffer().clone();

    let message_1 = text_thread.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&text_thread, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "aaa\nbbb\nccc\nddd\n")], None, cx)
    });

    let (_, message_2) =
        text_thread.update(cx, |text_thread, cx| text_thread.split_message(3..3, cx));
    let message_2 = message_2.unwrap();

    // We recycle newlines in the middle of a split message
    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..16),
        ]
    );

    let (_, message_3) =
        text_thread.update(cx, |text_thread, cx| text_thread.split_message(3..3, cx));
    let message_3 = message_3.unwrap();

    // We don't recycle newlines at the end of a split message
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..17),
        ]
    );

    let (_, message_4) =
        text_thread.update(cx, |text_thread, cx| text_thread.split_message(9..9, cx));
    let message_4 = message_4.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..9),
            (message_4.id, Role::User, 9..17),
        ]
    );

    let (_, message_5) =
        text_thread.update(cx, |text_thread, cx| text_thread.split_message(9..9, cx));
    let message_5 = message_5.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\nddd\n");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_3.id, Role::User, 4..5),
            (message_2.id, Role::User, 5..9),
            (message_4.id, Role::User, 9..10),
            (message_5.id, Role::User, 10..18),
        ]
    );

    let (message_6, message_7) =
        text_thread.update(cx, |text_thread, cx| text_thread.split_message(14..16, cx));
    let message_6 = message_6.unwrap();
    let message_7 = message_7.unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\ndd\nd\n");
    assert_eq!(
        messages(&text_thread, cx),
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
fn test_messages_for_offsets(cx: &mut App) {
    init_test(cx);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry,
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });
    let buffer = text_thread.read(cx).buffer().clone();

    let message_1 = text_thread.read(cx).message_anchors[0].clone();
    assert_eq!(
        messages(&text_thread, cx),
        vec![(message_1.id, Role::User, 0..0)]
    );

    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
    let message_2 = text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(message_1.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbb")], None, cx));

    let message_3 = text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.edit([(8..8, "ccc")], None, cx));

    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..8),
            (message_3.id, Role::User, 8..11)
        ]
    );

    assert_eq!(
        message_ids_for_offsets(&text_thread, &[0, 4, 9], cx),
        [message_1.id, message_2.id, message_3.id]
    );
    assert_eq!(
        message_ids_for_offsets(&text_thread, &[0, 1, 11], cx),
        [message_1.id, message_3.id]
    );

    let message_4 = text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(message_3.id, Role::User, MessageStatus::Done, cx)
        })
        .unwrap();
    assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\n");
    assert_eq!(
        messages(&text_thread, cx),
        vec![
            (message_1.id, Role::User, 0..4),
            (message_2.id, Role::User, 4..8),
            (message_3.id, Role::User, 8..12),
            (message_4.id, Role::User, 12..12)
        ]
    );
    assert_eq!(
        message_ids_for_offsets(&text_thread, &[0, 4, 8, 12], cx),
        [message_1.id, message_2.id, message_3.id, message_4.id]
    );

    fn message_ids_for_offsets(
        context: &Entity<TextThread>,
        offsets: &[usize],
        cx: &App,
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
    cx.update(init_test);

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
    slash_command_registry.register_command(FileSlashCommand, false);

    let registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry.clone(),
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });

    #[derive(Default)]
    struct ContextRanges {
        parsed_commands: HashSet<Range<language::Anchor>>,
        command_outputs: HashMap<InvokedSlashCommandId, Range<language::Anchor>>,
        output_sections: HashSet<Range<language::Anchor>>,
    }

    let context_ranges = Rc::new(RefCell::new(ContextRanges::default()));
    text_thread.update(cx, |_, cx| {
        cx.subscribe(&text_thread, {
            let context_ranges = context_ranges.clone();
            move |text_thread, _, event, _| {
                let mut context_ranges = context_ranges.borrow_mut();
                match event {
                    TextThreadEvent::InvokedSlashCommandChanged { command_id } => {
                        let command = text_thread.invoked_slash_command(command_id).unwrap();
                        context_ranges
                            .command_outputs
                            .insert(*command_id, command.range.clone());
                    }
                    TextThreadEvent::ParsedSlashCommandsUpdated { removed, updated } => {
                        for range in removed {
                            context_ranges.parsed_commands.remove(range);
                        }
                        for command in updated {
                            context_ranges
                                .parsed_commands
                                .insert(command.source_range.clone());
                        }
                    }
                    TextThreadEvent::SlashCommandOutputSectionAdded { section } => {
                        context_ranges.output_sections.insert(section.range.clone());
                    }
                    _ => {}
                }
            }
        })
        .detach();
    });

    let buffer = text_thread.read_with(cx, |text_thread, _| text_thread.buffer().clone());

    // Insert a slash command
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "/file src/lib.rs")], None, cx);
    });
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        «/file src/lib.rs»"
            .unindent(),
        cx,
    );

    // Edit the argument of the slash command.
    buffer.update(cx, |buffer, cx| {
        let edit_offset = buffer.text().find("lib.rs").unwrap();
        buffer.edit([(edit_offset..edit_offset + "lib".len(), "main")], None, cx);
    });
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        «/file src/main.rs»"
            .unindent(),
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
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        /unknown src/main.rs"
            .unindent(),
        cx,
    );

    // Undoing the insertion of an non-existent slash command resorts the previous one.
    buffer.update(cx, |buffer, cx| buffer.undo(cx));
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        «/file src/main.rs»"
            .unindent(),
        cx,
    );

    let (command_output_tx, command_output_rx) = mpsc::unbounded();
    text_thread.update(cx, |text_thread, cx| {
        let command_source_range = text_thread.parsed_slash_commands[0].source_range.clone();
        text_thread.insert_command_output(
            command_source_range,
            "file",
            Task::ready(Ok(command_output_rx.boxed())),
            true,
            cx,
        );
    });
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        ⟦«/file src/main.rs»
        …⟧
        "
        .unindent(),
        cx,
    );

    command_output_tx
        .unbounded_send(Ok(SlashCommandEvent::StartSection {
            icon: IconName::Ai,
            label: "src/main.rs".into(),
            metadata: None,
        }))
        .unwrap();
    command_output_tx
        .unbounded_send(Ok(SlashCommandEvent::Content("src/main.rs".into())))
        .unwrap();
    cx.run_until_parked();
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        ⟦«/file src/main.rs»
        src/main.rs…⟧
        "
        .unindent(),
        cx,
    );

    command_output_tx
        .unbounded_send(Ok(SlashCommandEvent::Content("\nfn main() {}".into())))
        .unwrap();
    cx.run_until_parked();
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        ⟦«/file src/main.rs»
        src/main.rs
        fn main() {}…⟧
        "
        .unindent(),
        cx,
    );

    command_output_tx
        .unbounded_send(Ok(SlashCommandEvent::EndSection))
        .unwrap();
    cx.run_until_parked();
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        ⟦«/file src/main.rs»
        ⟪src/main.rs
        fn main() {}⟫…⟧
        "
        .unindent(),
        cx,
    );

    drop(command_output_tx);
    cx.run_until_parked();
    assert_text_and_context_ranges(
        &buffer,
        &context_ranges,
        &"
        ⟦⟪src/main.rs
        fn main() {}⟫⟧
        "
        .unindent(),
        cx,
    );

    #[track_caller]
    fn assert_text_and_context_ranges(
        buffer: &Entity<Buffer>,
        ranges: &RefCell<ContextRanges>,
        expected_marked_text: &str,
        cx: &mut TestAppContext,
    ) {
        let mut actual_marked_text = String::new();
        buffer.update(cx, |buffer, _| {
            struct Endpoint {
                offset: usize,
                marker: char,
            }

            let ranges = ranges.borrow();
            let mut endpoints = Vec::new();
            for range in ranges.command_outputs.values() {
                endpoints.push(Endpoint {
                    offset: range.start.to_offset(buffer),
                    marker: '⟦',
                });
            }
            for range in ranges.parsed_commands.iter() {
                endpoints.push(Endpoint {
                    offset: range.start.to_offset(buffer),
                    marker: '«',
                });
            }
            for range in ranges.output_sections.iter() {
                endpoints.push(Endpoint {
                    offset: range.start.to_offset(buffer),
                    marker: '⟪',
                });
            }

            for range in ranges.output_sections.iter() {
                endpoints.push(Endpoint {
                    offset: range.end.to_offset(buffer),
                    marker: '⟫',
                });
            }
            for range in ranges.parsed_commands.iter() {
                endpoints.push(Endpoint {
                    offset: range.end.to_offset(buffer),
                    marker: '»',
                });
            }
            for range in ranges.command_outputs.values() {
                endpoints.push(Endpoint {
                    offset: range.end.to_offset(buffer),
                    marker: '⟧',
                });
            }

            endpoints.sort_by_key(|endpoint| endpoint.offset);
            let mut offset = 0;
            for endpoint in endpoints {
                actual_marked_text.extend(buffer.text_for_range(offset..endpoint.offset));
                actual_marked_text.push(endpoint.marker);
                offset = endpoint.offset;
            }
            actual_marked_text.extend(buffer.text_for_range(offset..buffer.len()));
        });

        assert_eq!(actual_marked_text, expected_marked_text);
    }
}

#[gpui::test]
async fn test_serialization(cx: &mut TestAppContext) {
    cx.update(init_test);

    let registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry.clone(),
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });
    let buffer = text_thread.read_with(cx, |text_thread, _| text_thread.buffer().clone());
    let message_0 = text_thread.read_with(cx, |text_thread, _| text_thread.message_anchors[0].id);
    let message_1 = text_thread.update(cx, |text_thread, cx| {
        text_thread
            .insert_message_after(message_0, Role::Assistant, MessageStatus::Done, cx)
            .unwrap()
    });
    let message_2 = text_thread.update(cx, |text_thread, cx| {
        text_thread
            .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "a"), (1..1, "b\nc")], None, cx);
        buffer.finalize_last_transaction();
    });
    let _message_3 = text_thread.update(cx, |text_thread, cx| {
        text_thread
            .insert_message_after(message_2.id, Role::System, MessageStatus::Done, cx)
            .unwrap()
    });
    buffer.update(cx, |buffer, cx| buffer.undo(cx));
    assert_eq!(buffer.read_with(cx, |buffer, _| buffer.text()), "a\nb\nc\n");
    assert_eq!(
        cx.read(|cx| messages(&text_thread, cx)),
        [
            (message_0, Role::User, 0..2),
            (message_1.id, Role::Assistant, 2..6),
            (message_2.id, Role::System, 6..6),
        ]
    );

    let serialized_context = text_thread.read_with(cx, |text_thread, cx| text_thread.serialize(cx));
    let deserialized_context = cx.new(|cx| {
        TextThread::deserialize(
            serialized_context,
            Path::new("").into(),
            registry.clone(),
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            None,
            None,
            cx,
        )
    });
    let deserialized_buffer =
        deserialized_context.read_with(cx, |text_thread, _| text_thread.buffer().clone());
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

#[gpui::test(iterations = 25)]
async fn test_random_context_collaboration(cx: &mut TestAppContext, mut rng: StdRng) {
    cx.update(init_test);

    let min_peers = env::var("MIN_PEERS")
        .map(|i| i.parse().expect("invalid `MIN_PEERS` variable"))
        .unwrap_or(2);
    let max_peers = env::var("MAX_PEERS")
        .map(|i| i.parse().expect("invalid `MAX_PEERS` variable"))
        .unwrap_or(5);
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(50);

    let slash_commands = cx.update(SlashCommandRegistry::default_global);
    slash_commands.register_command(FakeSlashCommand("cmd-1".into()), false);
    slash_commands.register_command(FakeSlashCommand("cmd-2".into()), false);
    slash_commands.register_command(FakeSlashCommand("cmd-3".into()), false);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor.clone()));
    let network = Arc::new(Mutex::new(Network::new(rng.clone())));
    let mut text_threads = Vec::new();

    let num_peers = rng.random_range(min_peers..=max_peers);
    let context_id = TextThreadId::new();
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    for i in 0..num_peers {
        let context = cx.new(|cx| {
            TextThread::new(
                context_id.clone(),
                ReplicaId::new(i as u16),
                language::Capability::ReadWrite,
                registry.clone(),
                prompt_builder.clone(),
                Arc::new(SlashCommandWorkingSet::default()),
                None,
                None,
                cx,
            )
        });

        cx.update(|cx| {
            cx.subscribe(&context, {
                let network = network.clone();
                move |_, event, _| {
                    if let TextThreadEvent::Operation(op) = event {
                        network
                            .lock()
                            .broadcast(ReplicaId::new(i as u16), vec![op.to_proto()]);
                    }
                }
            })
            .detach();
        });

        text_threads.push(context);
        network.lock().add_peer(ReplicaId::new(i as u16));
    }

    let mut mutation_count = operations;

    while mutation_count > 0
        || !network.lock().is_idle()
        || network.lock().contains_disconnected_peers()
    {
        let context_index = rng.random_range(0..text_threads.len());
        let text_thread = &text_threads[context_index];

        match rng.random_range(0..100) {
            0..=29 if mutation_count > 0 => {
                log::info!("Context {}: edit buffer", context_index);
                text_thread.update(cx, |text_thread, cx| {
                    text_thread
                        .buffer()
                        .update(cx, |buffer, cx| buffer.randomly_edit(&mut rng, 1, cx));
                });
                mutation_count -= 1;
            }
            30..=44 if mutation_count > 0 => {
                text_thread.update(cx, |text_thread, cx| {
                    let range = text_thread.buffer().read(cx).random_byte_range(0, &mut rng);
                    log::info!("Context {}: split message at {:?}", context_index, range);
                    text_thread.split_message(range, cx);
                });
                mutation_count -= 1;
            }
            45..=59 if mutation_count > 0 => {
                text_thread.update(cx, |text_thread, cx| {
                    if let Some(message) = text_thread.messages(cx).choose(&mut rng) {
                        let role = *[Role::User, Role::Assistant, Role::System]
                            .choose(&mut rng)
                            .unwrap();
                        log::info!(
                            "Context {}: insert message after {:?} with {:?}",
                            context_index,
                            message.id,
                            role
                        );
                        text_thread.insert_message_after(message.id, role, MessageStatus::Done, cx);
                    }
                });
                mutation_count -= 1;
            }
            60..=74 if mutation_count > 0 => {
                text_thread.update(cx, |text_thread, cx| {
                    let command_text = "/".to_string()
                        + slash_commands
                            .command_names()
                            .choose(&mut rng)
                            .unwrap()
                            .clone()
                            .as_ref();

                    let command_range = text_thread.buffer().update(cx, |buffer, cx| {
                        let offset = buffer.random_byte_range(0, &mut rng).start;
                        buffer.edit(
                            [(offset..offset, format!("\n{}\n", command_text))],
                            None,
                            cx,
                        );
                        offset + 1..offset + 1 + command_text.len()
                    });

                    let output_text = RandomCharIter::new(&mut rng)
                        .filter(|c| *c != '\r')
                        .take(10)
                        .collect::<String>();

                    let mut events = vec![Ok(SlashCommandEvent::StartMessage {
                        role: Role::User,
                        merge_same_roles: true,
                    })];

                    let num_sections = rng.random_range(0..=3);
                    let mut section_start = 0;
                    for _ in 0..num_sections {
                        let section_end = output_text.floor_char_boundary(
                            rng.random_range(section_start..=output_text.len()),
                        );
                        events.push(Ok(SlashCommandEvent::StartSection {
                            icon: IconName::Ai,
                            label: "section".into(),
                            metadata: None,
                        }));
                        events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                            text: output_text[section_start..section_end].to_string(),
                            run_commands_in_text: false,
                        })));
                        events.push(Ok(SlashCommandEvent::EndSection));
                        section_start = section_end;
                    }

                    if section_start < output_text.len() {
                        events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                            text: output_text[section_start..].to_string(),
                            run_commands_in_text: false,
                        })));
                    }

                    log::info!(
                        "Context {}: insert slash command output at {:?} with {:?} events",
                        context_index,
                        command_range,
                        events.len()
                    );

                    let command_range = text_thread
                        .buffer()
                        .read(cx)
                        .anchor_after(command_range.start)
                        ..text_thread
                            .buffer()
                            .read(cx)
                            .anchor_after(command_range.end);
                    text_thread.insert_command_output(
                        command_range,
                        "/command",
                        Task::ready(Ok(stream::iter(events).boxed())),
                        true,
                        cx,
                    );
                });
                cx.run_until_parked();
                mutation_count -= 1;
            }
            75..=84 if mutation_count > 0 => {
                text_thread.update(cx, |text_thread, cx| {
                    if let Some(message) = text_thread.messages(cx).choose(&mut rng) {
                        let new_status = match rng.random_range(0..3) {
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
                        text_thread.update_metadata(message.id, cx, |metadata| {
                            metadata.status = new_status;
                        });
                    }
                });
                mutation_count -= 1;
            }
            _ => {
                let replica_id = ReplicaId::new(context_index as u16);
                if network.lock().is_disconnected(replica_id) {
                    network.lock().reconnect_peer(replica_id, ReplicaId::new(0));

                    let (ops_to_send, ops_to_receive) = cx.read(|cx| {
                        let host_context = &text_threads[0].read(cx);
                        let guest_context = text_thread.read(cx);
                        (
                            guest_context.serialize_ops(&host_context.version(cx), cx),
                            host_context.serialize_ops(&guest_context.version(cx), cx),
                        )
                    });
                    let ops_to_send = ops_to_send.await;
                    let ops_to_receive = ops_to_receive
                        .await
                        .into_iter()
                        .map(TextThreadOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                        .unwrap();
                    log::info!(
                        "Context {}: reconnecting. Sent {} operations, received {} operations",
                        context_index,
                        ops_to_send.len(),
                        ops_to_receive.len()
                    );

                    network.lock().broadcast(replica_id, ops_to_send);
                    text_thread.update(cx, |text_thread, cx| {
                        text_thread.apply_ops(ops_to_receive, cx)
                    });
                } else if rng.random_bool(0.1) && replica_id != ReplicaId::new(0) {
                    log::info!("Context {}: disconnecting", context_index);
                    network.lock().disconnect_peer(replica_id);
                } else if network.lock().has_unreceived(replica_id) {
                    log::info!("Context {}: applying operations", context_index);
                    let ops = network.lock().receive(replica_id);
                    let ops = ops
                        .into_iter()
                        .map(TextThreadOperation::from_proto)
                        .collect::<Result<Vec<_>>>()
                        .unwrap();
                    text_thread.update(cx, |text_thread, cx| text_thread.apply_ops(ops, cx));
                }
            }
        }
    }

    cx.read(|cx| {
        let first_context = text_threads[0].read(cx);
        for text_thread in &text_threads[1..] {
            let text_thread = text_thread.read(cx);
            assert!(text_thread.pending_ops.is_empty(), "pending ops: {:?}", text_thread.pending_ops);
            assert_eq!(
                text_thread.buffer().read(cx).text(),
                first_context.buffer().read(cx).text(),
                "Context {:?} text != Context 0 text",
                text_thread.buffer().read(cx).replica_id()
            );
            assert_eq!(
                text_thread.message_anchors,
                first_context.message_anchors,
                "Context {:?} messages != Context 0 messages",
                text_thread.buffer().read(cx).replica_id()
            );
            assert_eq!(
                text_thread.messages_metadata,
                first_context.messages_metadata,
                "Context {:?} message metadata != Context 0 message metadata",
                text_thread.buffer().read(cx).replica_id()
            );
            assert_eq!(
                text_thread.slash_command_output_sections,
                first_context.slash_command_output_sections,
                "Context {:?} slash command output sections != Context 0 slash command output sections",
                text_thread.buffer().read(cx).replica_id()
            );
        }
    });
}

#[gpui::test]
fn test_mark_cache_anchors(cx: &mut App) {
    init_test(cx);

    let registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let text_thread = cx.new(|cx| {
        TextThread::local(
            registry,
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });
    let buffer = text_thread.read(cx).buffer().clone();

    // Create a test cache configuration
    let cache_configuration = &Some(LanguageModelCacheConfiguration {
        max_cache_anchors: 3,
        should_speculate: true,
        min_total_token: 10,
    });

    let message_1 = text_thread.read(cx).message_anchors[0].clone();

    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, false, cx)
    });

    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .filter(|(_, cache)| cache.as_ref().is_some_and(|cache| cache.is_anchor))
            .count(),
        0,
        "Empty messages should not have any cache anchors."
    );

    buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
    let message_2 = text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(message_1.id, Role::User, MessageStatus::Pending, cx)
        })
        .unwrap();

    buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbbbbbb")], None, cx));
    let message_3 = text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(message_2.id, Role::User, MessageStatus::Pending, cx)
        })
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.edit([(12..12, "cccccc")], None, cx));

    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, false, cx)
    });
    assert_eq!(buffer.read(cx).text(), "aaa\nbbbbbbb\ncccccc");
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .filter(|(_, cache)| cache.as_ref().is_some_and(|cache| cache.is_anchor))
            .count(),
        0,
        "Messages should not be marked for cache before going over the token minimum."
    );
    text_thread.update(cx, |text_thread, _| {
        text_thread.token_count = Some(20);
    });

    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, true, cx)
    });
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .map(|(_, cache)| cache.as_ref().is_some_and(|cache| cache.is_anchor))
            .collect::<Vec<bool>>(),
        vec![true, true, false],
        "Last message should not be an anchor on speculative request."
    );

    text_thread
        .update(cx, |text_thread, cx| {
            text_thread.insert_message_after(
                message_3.id,
                Role::Assistant,
                MessageStatus::Pending,
                cx,
            )
        })
        .unwrap();

    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, false, cx)
    });
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .map(|(_, cache)| cache.as_ref().is_some_and(|cache| cache.is_anchor))
            .collect::<Vec<bool>>(),
        vec![false, true, true, false],
        "Most recent message should also be cached if not a speculative request."
    );
    text_thread.update(cx, |text_thread, cx| {
        text_thread.update_cache_status_for_completion(cx)
    });
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .map(|(_, cache)| cache
                .as_ref()
                .map_or(None, |cache| Some(cache.status.clone())))
            .collect::<Vec<Option<CacheStatus>>>(),
        vec![
            Some(CacheStatus::Cached),
            Some(CacheStatus::Cached),
            Some(CacheStatus::Cached),
            None
        ],
        "All user messages prior to anchor should be marked as cached."
    );

    buffer.update(cx, |buffer, cx| buffer.edit([(14..14, "d")], None, cx));
    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, false, cx)
    });
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .map(|(_, cache)| cache
                .as_ref()
                .map_or(None, |cache| Some(cache.status.clone())))
            .collect::<Vec<Option<CacheStatus>>>(),
        vec![
            Some(CacheStatus::Cached),
            Some(CacheStatus::Cached),
            Some(CacheStatus::Pending),
            None
        ],
        "Modifying a message should invalidate it's cache but leave previous messages."
    );
    buffer.update(cx, |buffer, cx| buffer.edit([(2..2, "e")], None, cx));
    text_thread.update(cx, |text_thread, cx| {
        text_thread.mark_cache_anchors(cache_configuration, false, cx)
    });
    assert_eq!(
        messages_cache(&text_thread, cx)
            .iter()
            .map(|(_, cache)| cache
                .as_ref()
                .map_or(None, |cache| Some(cache.status.clone())))
            .collect::<Vec<Option<CacheStatus>>>(),
        vec![
            Some(CacheStatus::Pending),
            Some(CacheStatus::Pending),
            Some(CacheStatus::Pending),
            None
        ],
        "Modifying a message should invalidate all future messages."
    );
}

#[gpui::test]
async fn test_summarization(cx: &mut TestAppContext) {
    let (text_thread, fake_model) = setup_context_editor_with_fake_model(cx);

    // Initial state should be pending
    text_thread.read_with(cx, |text_thread, _| {
        assert!(matches!(text_thread.summary(), TextThreadSummary::Pending));
        assert_eq!(
            text_thread.summary().or_default(),
            TextThreadSummary::DEFAULT
        );
    });

    let message_1 = text_thread.read_with(cx, |text_thread, _cx| {
        text_thread.message_anchors[0].clone()
    });
    text_thread.update(cx, |context, cx| {
        context
            .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
            .unwrap();
    });

    // Send a message
    text_thread.update(cx, |text_thread, cx| {
        text_thread.assist(cx);
    });

    simulate_successful_response(&fake_model, cx);

    // Should start generating summary when there are >= 2 messages
    text_thread.read_with(cx, |text_thread, _| {
        assert!(!text_thread.summary().content().unwrap().done);
    });

    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Brief");
    fake_model.send_last_completion_stream_text_chunk(" Introduction");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Summary should be set
    text_thread.read_with(cx, |text_thread, _| {
        assert_eq!(text_thread.summary().or_default(), "Brief Introduction");
    });

    // We should be able to manually set a summary
    text_thread.update(cx, |text_thread, cx| {
        text_thread.set_custom_summary("Brief Intro".into(), cx);
    });

    text_thread.read_with(cx, |text_thread, _| {
        assert_eq!(text_thread.summary().or_default(), "Brief Intro");
    });
}

#[gpui::test]
async fn test_thread_summary_error_set_manually(cx: &mut TestAppContext) {
    let (text_thread, fake_model) = setup_context_editor_with_fake_model(cx);

    test_summarize_error(&fake_model, &text_thread, cx);

    // Now we should be able to set a summary
    text_thread.update(cx, |text_thread, cx| {
        text_thread.set_custom_summary("Brief Intro".into(), cx);
    });

    text_thread.read_with(cx, |text_thread, _| {
        assert_eq!(text_thread.summary().or_default(), "Brief Intro");
    });
}

#[gpui::test]
async fn test_thread_summary_error_retry(cx: &mut TestAppContext) {
    let (text_thread, fake_model) = setup_context_editor_with_fake_model(cx);

    test_summarize_error(&fake_model, &text_thread, cx);

    // Sending another message should not trigger another summarize request
    text_thread.update(cx, |text_thread, cx| {
        text_thread.assist(cx);
    });

    simulate_successful_response(&fake_model, cx);

    text_thread.read_with(cx, |text_thread, _| {
        // State is still Error, not Generating
        assert!(matches!(text_thread.summary(), TextThreadSummary::Error));
    });

    // But the summarize request can be invoked manually
    text_thread.update(cx, |text_thread, cx| {
        text_thread.summarize(true, cx);
    });

    text_thread.read_with(cx, |text_thread, _| {
        assert!(!text_thread.summary().content().unwrap().done);
    });

    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("A successful summary");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    text_thread.read_with(cx, |text_thread, _| {
        assert_eq!(text_thread.summary().or_default(), "A successful summary");
    });
}

fn test_summarize_error(
    model: &Arc<FakeLanguageModel>,
    text_thread: &Entity<TextThread>,
    cx: &mut TestAppContext,
) {
    let message_1 = text_thread.read_with(cx, |text_thread, _cx| {
        text_thread.message_anchors[0].clone()
    });
    text_thread.update(cx, |text_thread, cx| {
        text_thread
            .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
            .unwrap();
    });

    // Send a message
    text_thread.update(cx, |text_thread, cx| {
        text_thread.assist(cx);
    });

    simulate_successful_response(model, cx);

    text_thread.read_with(cx, |text_thread, _| {
        assert!(!text_thread.summary().content().unwrap().done);
    });

    // Simulate summary request ending
    cx.run_until_parked();
    model.end_last_completion_stream();
    cx.run_until_parked();

    // State is set to Error and default message
    text_thread.read_with(cx, |text_thread, _| {
        assert_eq!(*text_thread.summary(), TextThreadSummary::Error);
        assert_eq!(
            text_thread.summary().or_default(),
            TextThreadSummary::DEFAULT
        );
    });
}

fn setup_context_editor_with_fake_model(
    cx: &mut TestAppContext,
) -> (Entity<TextThread>, Arc<FakeLanguageModel>) {
    let registry = Arc::new(LanguageRegistry::test(cx.executor()));

    let fake_provider = Arc::new(FakeLanguageModelProvider::default());
    let fake_model = Arc::new(fake_provider.test_model());

    cx.update(|cx| {
        init_test(cx);
        LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
            let configured_model = ConfiguredModel {
                provider: fake_provider.clone(),
                model: fake_model.clone(),
            };
            registry.set_default_model(Some(configured_model.clone()), cx);
            registry.set_thread_summary_model(Some(configured_model), cx);
        })
    });

    let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
    let context = cx.new(|cx| {
        TextThread::local(
            registry,
            None,
            None,
            prompt_builder.clone(),
            Arc::new(SlashCommandWorkingSet::default()),
            cx,
        )
    });

    (context, fake_model)
}

fn simulate_successful_response(fake_model: &FakeLanguageModel, cx: &mut TestAppContext) {
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Assistant response");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
}

fn messages(context: &Entity<TextThread>, cx: &App) -> Vec<(MessageId, Role, Range<usize>)> {
    context
        .read(cx)
        .messages(cx)
        .map(|message| (message.id, message.role, message.offset_range))
        .collect()
}

fn messages_cache(
    context: &Entity<TextThread>,
    cx: &App,
) -> Vec<(MessageId, Option<MessageCacheMetadata>)> {
    context
        .read(cx)
        .messages(cx)
        .map(|message| (message.id, message.cache))
        .collect()
}

fn init_test(cx: &mut App) {
    let settings_store = SettingsStore::test(cx);
    prompt_store::init(cx);
    LanguageModelRegistry::test(cx);
    cx.set_global(settings_store);
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
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(vec![]))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<SlashCommandResult> {
        Task::ready(Ok(SlashCommandOutput {
            text: format!("Executed fake command: {}", self.0),
            sections: vec![],
            run_commands_in_text: false,
        }
        .into_event_stream()))
    }
}
