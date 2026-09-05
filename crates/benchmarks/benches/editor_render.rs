use editor::{
    Editor, EditorMode, MultiBuffer,
    actions::{DeleteToPreviousWordStart, SelectAll, SplitSelectionIntoLines},
};
use gpui::{AppContext as _, BenchAppContext, Focusable as _};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use settings::SettingsStore;
use util::RandomCharIter;
use zed_actions::editor::{MoveDown, MoveUp};

#[gpui::bench(
    inputs = multi_cursor_line_counts(),
    group = "Multi-cursor input",
    input_name = "cursors",
    sample_size = 10
)]
fn editor_multi_cursor_input(line_count: &usize, cx: &mut BenchAppContext) {
    init_context(cx);

    let text = "line:\n".repeat(*line_count);
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

#[gpui::bench]
fn open_editor_with_one_long_line(cx: &mut BenchAppContext) {
    init_context(cx);

    let text = String::from_iter(["char"; 1000]);
    cx.bench_iter(move |cx| {
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

        let mut window = cx.add_empty_window();
        window.update(|window, cx| {
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

#[gpui::bench(inputs = [1usize, 3], group = "Editor panes", input_name = "panes", sample_size = 20)]
fn editor_render_panes(pane_count: &usize, cx: &mut BenchAppContext) {
    use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};

    struct Panes(Vec<Entity<Editor>>);
    impl Render for Panes {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().flex().children(
                self.0
                    .iter()
                    .map(|editor| div().flex_1().min_w_0().h_full().child(editor.clone())),
            )
        }
    }

    init_context(cx);
    let text = "fn example(value: usize) -> usize { value + 1 }\n".repeat(1000);
    let mut window = cx.add_empty_window();
    let active_editor = window.update(|window, cx| {
        let editors: Vec<_> = (0..*pane_count)
            .map(|_| {
                let buffer = MultiBuffer::build_simple(&text, cx);
                cx.new(|cx| {
                    let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
                    editor.set_style(editor::EditorStyle::default(), window, cx);
                    editor
                })
            })
            .collect();
        let active = editors.first().expect("at least one pane").clone();
        window.replace_root(cx, |_, _| Panes(editors));
        window.focus(&active.focus_handle(cx), cx);
        active
    });
    for step in 0..8 {
        cx.run_until_idle();
        window.update(|window, cx| {
            active_editor.update(cx, |editor, cx| {
                if step % 2 == 0 {
                    editor.move_down(&MoveDown, window, cx);
                } else {
                    editor.move_up(&MoveUp, window, cx);
                }
            });
        });
    }
    window.update(|window, _| {
        if let Some(stats) = window.retained_node_stats() {
            eprintln!("editor panes ({pane_count}) after warmup: {stats:?}");
        }
    });
    let mut move_down = true;
    cx.bench_renderer(active_editor, move |editor, window, cx| {
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

fn multi_cursor_line_counts() -> Vec<usize> {
    let mut line_counts = vec![1000, 10000];
    if std::env::var("ZED_BENCH_HUGE").is_ok() {
        line_counts.push(100000);
    }
    line_counts
}

gpui::bench_group!(
    benches,
    editor_multi_cursor_input,
    open_editor_with_one_long_line,
    editor_render,
    editor_render_panes
);
gpui::bench_main!(benches);
