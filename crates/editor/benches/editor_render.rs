use criterion::{Bencher, BenchmarkId};
use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
};
use gpui::{AppContext as _, Focusable as _, TestAppContext, TestDispatcher};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;
use ui::IntoElement;
use util::RandomCharIter;

// Reproduces issue #32051: place a cursor on every line of an N-line buffer, then type and delete.
// Parameterized over the cursor count so the per-keystroke cost can be tracked as N grows.
fn editor_multi_cursor_input(bencher: &mut Bencher<'_>, args: &(usize, TestAppContext)) {
    let (line_count, cx) = args;
    let mut cx = cx.clone();

    let text = "line:\n".repeat(*line_count);
    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

    let cx = cx.add_empty_window();
    let editor = cx.update(|window, cx| {
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor.select_all(&SelectAll, window, cx);
            editor.split_selection_into_lines(
                &SplitSelectionIntoLines {
                    keep_selections: true,
                },
                window,
                cx,
            );
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    bencher.iter(|| {
        cx.update(|window, cx| {
            editor.update(cx, |editor, cx| {
                editor.handle_input("hello world", window, cx);
                editor.delete_to_previous_word_start(
                    &DeleteToPreviousWordStart {
                        ignore_newlines: false,
                        ignore_brackets: false,
                    },
                    window,
                    cx,
                );
                editor.delete_to_previous_word_start(
                    &DeleteToPreviousWordStart {
                        ignore_newlines: false,
                        ignore_brackets: false,
                    },
                    window,
                    cx,
                );
            });
        })
    });
}

fn open_editor_with_one_long_line(bencher: &mut Bencher<'_>, args: &(String, TestAppContext)) {
    let (text, cx) = args;
    let mut cx = cx.clone();

    bencher.iter(|| {
        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));

        let cx = cx.add_empty_window();
        cx.update(|window, cx| {
            let editor = cx.new(|cx| {
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.set_style(editor::EditorStyle::default(), window, cx);
                editor
            });
            window.focus(&editor.focus_handle(cx), cx);
            editor
        });
    });
}

fn editor_render(bencher: &mut Bencher<'_>, cx: &TestAppContext) {
    let mut cx = cx.clone();
    let buffer = cx.update(|cx| {
        let mut rng = StdRng::seed_from_u64(1);
        let text_len = rng.random_range(10000..90000);
        if rng.random() {
            let text = RandomCharIter::new(&mut rng)
                .take(text_len)
                .collect::<String>();
            MultiBuffer::build_simple(&text, cx)
        } else {
            MultiBuffer::build_random(&mut rng, cx)
        }
    });

    let cx = cx.add_empty_window();
    let editor = cx.update(|window, cx| {
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    bencher.iter(|| {
        cx.update(|window, cx| {
            let mut view = editor.clone().into_any_element();
            let _ = view.request_layout(window, cx);
            let _ = view.prepaint(window, cx);
            view.paint(window, cx);
        });
    })
}

fn init_test_context(cx: &TestAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
}

fn criterion_benches(criterion: &mut criterion::Criterion) {
    let dispatcher = TestDispatcher::new(1);
    let cx = gpui::TestAppContext::build(dispatcher, None);
    init_test_context(&cx);

    let mut group = criterion.benchmark_group("Time to render");
    group.bench_with_input(
        BenchmarkId::new("editor_render", "TestAppContext"),
        &cx,
        editor_render,
    );
    group.finish();

    let text = String::from_iter(["char"; 1000]);
    let input = (text, cx.clone());
    let mut group = criterion.benchmark_group("Build buffer with one long line");
    group.bench_with_input(
        BenchmarkId::new("editor_with_one_long_line", "(String, TestAppContext )"),
        &input,
        open_editor_with_one_long_line,
    );
    group.finish();

    // The 100k case is slow (and is the "hang" regime from the issue), so it is opt-in via
    // ZED_BENCH_HUGE=1 to avoid burdening regular runs.
    let mut line_counts = vec![1000usize, 10000];
    if std::env::var("ZED_BENCH_HUGE").is_ok() {
        line_counts.push(100000);
    }
    let mut group = criterion.benchmark_group("Multi-cursor input");
    group.sample_size(10);
    for line_count in line_counts {
        group.bench_with_input(
            BenchmarkId::new("cursors", line_count),
            &(line_count, cx.clone()),
            editor_multi_cursor_input,
        );
    }
    group.finish();
}

gpui::bench_group!(benches, criterion_benches);
gpui::bench_main!(benches);
