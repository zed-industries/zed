use criterion::{Bencher, BenchmarkId};
use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
};
use gpui::{AppContext, Focusable as _, TestAppContext, TestDispatcher};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;
use ui::IntoElement;
use util::RandomCharIter;

fn editor_input_with_1000_cursors(bencher: &mut Bencher<'_>, cx: &TestAppContext) {
    let mut cx = cx.clone();
    let text = String::from_iter(["line:\n"; 1000]);
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
        window.focus(&editor.focus_handle(cx));
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
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

        let cx = cx.add_empty_window();
        let _ = cx.update(|window, cx| {
            let editor = cx.new(|cx| {
                let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                editor.set_style(editor::EditorStyle::default(), window, cx);
                editor
            });
            window.focus(&editor.focus_handle(cx));
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
        window.focus(&editor.focus_handle(cx));
        editor
    });

    bencher.iter(|| {
        cx.update(|window, cx| {
            // editor.update(cx, |editor, cx| editor.move_down(&MoveDown, window, cx));
            let mut view = editor.clone().into_any_element();
            let _ = view.request_layout(window, cx);
            let _ = view.prepaint(window, cx);
            view.paint(window, cx);
        });
    })
}

pub fn benches() {
    let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(1));
    let cx = gpui::TestAppContext::build(dispatcher, None);
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        // release_channel::init(semver::Version::new(0,0,0), cx);
        editor::init(cx);
    });

    let mut criterion: criterion::Criterion<_> =
        (criterion::Criterion::default()).configure_from_args();

    // setup app context
    let mut group = criterion.benchmark_group("Time to render");
    group.bench_with_input(
        BenchmarkId::new("editor_render", "TestAppContext"),
        &cx,
        editor_render,
    );

    group.finish();

    let text = String::from_iter(["char"; 1000]);
    let mut group = criterion.benchmark_group("Build buffer with one long line");
    group.bench_with_input(
        BenchmarkId::new("editor_with_one_long_line", "(String, TestAppContext )"),
        &(text, cx.clone()),
        open_editor_with_one_long_line,
    );

    group.finish();

    let mut group = criterion.benchmark_group("multi cursor edits");
    group.bench_with_input(
        BenchmarkId::new("editor_input_with_1000_cursors", "TestAppContext"),
        &cx,
        editor_input_with_1000_cursors,
    );
    group.finish();
}

fn main() {
    benches();
    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
