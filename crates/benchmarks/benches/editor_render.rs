use criterion::{Bencher, BenchmarkId};
use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
};
use gpui::{AppContext as _, BenchAppContext, Focusable as _, TestAppContext, TestDispatcher};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;
use util::RandomCharIter;
use zed_actions::editor::{MoveDown, MoveUp};

#[gpui::bench]
fn editor_input_with_1000_cursors(cx: &mut BenchAppContext) {
    init_context(cx);

    let text = String::from_iter(["line:\n"; 1000]);
    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
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

    cx.bench_iter(|_| {
        window.update(|window, cx| {
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

#[gpui::bench]
fn editor_render(cx: &mut BenchAppContext) {
    init_context(cx);

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

    let mut window = cx.add_empty_window();
    let editor = window.update(|window, cx| {
        let editor = window.replace_root(cx, |window, cx| {
            let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
            editor.set_style(editor::EditorStyle::default(), window, cx);
            editor
        });
        window.focus(&editor.focus_handle(cx), cx);
        editor
    });

    let mut move_down = true;
    cx.bench_renderer(editor, move |editor, window, cx| {
        if move_down {
            editor.move_down(&MoveDown, window, cx);
        } else {
            editor.move_up(&MoveUp, window, cx);
        }
        move_down = !move_down;
    });
}

fn init_context(cx: &mut BenchAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
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

    let text = String::from_iter(["char"; 1000]);
    let input = (text, cx);
    let mut group = criterion.benchmark_group("Build buffer with one long line");
    group.bench_with_input(
        BenchmarkId::new("editor_with_one_long_line", "(String, TestAppContext )"),
        &input,
        open_editor_with_one_long_line,
    );
    group.finish();
}

gpui::bench_group!(
    benches,
    editor_input_with_1000_cursors,
    editor_render,
    criterion_benches
);
gpui::bench_main!(benches);
