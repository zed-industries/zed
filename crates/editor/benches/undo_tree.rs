use criterion::{Criterion, black_box, criterion_group, criterion_main};
use editor::{Editor, EditorMode, MultiBuffer, actions::ShowUndoTree};
use gpui::{AppContext, Focusable as _, TestAppContext, TestDispatcher};
use settings::SettingsStore;
use std::time::Duration;
use text::{Buffer as TextBuffer, BufferId, ReplicaId};

const LARGE_UNDO_HISTORY_EDIT_COUNT: usize = 30_000;

fn build_text_buffer_with_history(edit_count: usize) -> TextBuffer {
    let mut buffer = TextBuffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).expect("valid local buffer id"),
        "",
    );
    buffer.set_group_interval(Duration::ZERO);
    for _ in 0..edit_count {
        let end = buffer.len();
        buffer.edit([(end..end, "x")]);
    }
    buffer
}

fn bench_edit_latency_without_snapshot(c: &mut Criterion) {
    let mut buffer = build_text_buffer_with_history(LARGE_UNDO_HISTORY_EDIT_COUNT);
    c.bench_function("undo_tree/edit_latency_no_snapshot_30k", |bench| {
        bench.iter(|| {
            let end = buffer.len();
            let operation = buffer.edit([(end..end, "y")]);
            black_box(operation);
        });
    });
}

fn bench_snapshot_then_edit(c: &mut Criterion) {
    let mut buffer = build_text_buffer_with_history(LARGE_UNDO_HISTORY_EDIT_COUNT);
    c.bench_function("undo_tree/snapshot_then_edit_30k", |bench| {
        bench.iter(|| {
            let snapshot = buffer.export_undo_history_snapshot();
            let end = buffer.len();
            let operation = buffer.edit([(end..end, "y")]);
            black_box(snapshot.node_count());
            black_box(operation);
        });
    });
}

fn bench_export_snapshot(c: &mut Criterion) {
    let buffer = build_text_buffer_with_history(LARGE_UNDO_HISTORY_EDIT_COUNT);
    c.bench_function("undo_tree/export_snapshot_30k", |bench| {
        bench.iter(|| {
            let snapshot = buffer.export_undo_history_snapshot();
            black_box(snapshot.node_count());
            black_box(snapshot);
        });
    });
}

fn bench_snapshot_into_state(c: &mut Criterion) {
    let buffer = build_text_buffer_with_history(LARGE_UNDO_HISTORY_EDIT_COUNT);
    c.bench_function("undo_tree/snapshot_into_state_30k", |bench| {
        bench.iter(|| {
            let snapshot = buffer.export_undo_history_snapshot();
            black_box(snapshot.into_state());
        });
    });
}

fn bench_visualizer_state_rebuild(c: &mut Criterion) {
    let dispatcher = TestDispatcher::new(1);
    let mut app = TestAppContext::build(dispatcher, None);
    app.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });

    let language_buffer = app.update(|cx| {
        cx.new(|cx| {
            let mut buffer = language::Buffer::local("", cx);
            buffer.set_group_interval(Duration::ZERO);
            for _ in 0..LARGE_UNDO_HISTORY_EDIT_COUNT {
                let end = buffer.len();
                buffer.edit([(end..end, "x")], None, cx);
            }
            buffer
        })
    });
    let multi_buffer = app.update(|cx| cx.new(|cx| MultiBuffer::singleton(language_buffer, cx)));

    let window = app.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(EditorMode::full(), multi_buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor.show_undo_tree(&ShowUndoTree, window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    c.bench_function("undo_tree/visualizer_state_rebuild_30k", |bench| {
        bench.iter(|| {
            window.update(|_window, cx| {
                editor.update(cx, |editor, cx| {
                    black_box(editor.benchmark_undo_tree_visualizer_state(cx));
                });
            });
        });
    });
}

criterion_group!(
    benches,
    bench_edit_latency_without_snapshot,
    bench_snapshot_then_edit,
    bench_export_snapshot,
    bench_snapshot_into_state,
    bench_visualizer_state_rebuild
);
criterion_main!(benches);
