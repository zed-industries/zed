use criterion::Bencher;
use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
};
use gpui::{AppContext as _, BenchAppContext, Focusable as _};
use settings::SettingsStore;

#[gpui::bench]
fn editor_input_with_1000_cursors(bencher: &mut Bencher<'_>, cx: &mut BenchAppContext) {
    init_context(cx);

    let text = String::from_iter(["line:\n"; 1000]);
    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

    let mut cx = cx.add_empty_window();
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

fn init_context(cx: &mut BenchAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
}

gpui::bench_group!(benches, editor_input_with_1000_cursors);
gpui::bench_main!(benches);
