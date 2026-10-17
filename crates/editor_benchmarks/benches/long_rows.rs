use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::Range,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use editor::{
    Editor, EditorMode, MultiBuffer, SelectionEffects,
    display_map::{DisplayPoint, DisplayRow, DisplaySnapshot, PendingRowGeometry},
};
use gpui::{
    AnyWindowHandle, AppContext as _, AsyncApp, BenchAppContext, Context, Entity, Focusable as _,
    FontRun, IntoElement, LineLayout, ParentElement as _, Render, Styled as _, Task, Window, div,
    point,
};
use language::{Buffer, LineEnding, Point, Rope, language_settings::SoftWrap};
use zed_actions::editor::MoveDown;

#[gpui::bench(
    inputs = long_row_characters(),
    group = "Scroll an unwrapped long row",
    input_name = "character",
    sample_size = 10
)]
fn editor_scroll_long_row(character: &char, cx: &mut BenchAppContext) {
    bench_scroll_long_row(ScrollRow::Repeated(*character), cx);
}

#[gpui::bench(
    inputs = [4_096, 32_768, 262_144],
    group = "Scroll and repaint an unwrapped combining cluster",
    input_name = "marks",
    sample_size = 10
)]
fn editor_scroll_combining_cluster(marks: &usize, cx: &mut BenchAppContext) {
    bench_scroll_long_row(ScrollRow::Combining(*marks), cx);
}

fn bench_scroll_long_row(row: ScrollRow, cx: &mut BenchAppContext) {
    init_context(cx);

    let content = match row {
        ScrollRow::Repeated(character) => character.to_string().repeat(LONG_ROW_CHARACTERS),
        ScrollRow::Combining(marks) => format!(
            "{}a{}{}",
            "x".repeat(2_048),
            "\u{301}".repeat(marks),
            "b".repeat(2_048),
        ),
    };
    let text = Rope::from(content.as_str());
    let oracle_text = match row {
        ScrollRow::Repeated(character) => character.to_string().repeat(256),
        ScrollRow::Combining(_) => content,
    };
    let mut session_window = None::<AnyWindowHandle>;
    let mut oracle_reported = false;
    cx.bench_renderer_session(
        LONG_ROW_SESSION_TIMEOUT,
        move |cx| {
            let window =
                *session_window.get_or_insert_with(|| cx.add_empty_window().window_handle());
            let text = text.clone();
            let editor_slot = Rc::new(RefCell::new(None));
            let renders = Rc::new(Cell::new(0));
            let oracle = cx
                .update_window(window, |_, window, cx| {
                    let editor = cx.new(|cx| unwrapped_editor(text.clone(), window, cx));
                    editor_slot.replace(Some(editor.clone()));
                    window.replace_root(cx, |_, _| LongRowRoot {
                        editor: editor_slot.clone(),
                        renders: renders.clone(),
                    });
                    window.focus(&editor.focus_handle(cx), cx);
                    editor.update(cx, |editor, cx| {
                        let style = editor.style(cx);
                        let font_size = style.text.font_size.to_pixels(window.rem_size());
                        let wrapper = cx.text_system().line_wrapper(style.text.font(), font_size);
                        let (layout, positions, width) = wrapper.layout_line_with_precise_positions(
                            &oracle_text,
                            &[FontRun {
                                len: oracle_text.len(),
                                font_id: wrapper.font_id(),
                            }],
                        );
                        let em_width = f64::from(
                            window.text_system().em_layout_width(wrapper.font_id(), font_size),
                        );
                        assert!(em_width.is_finite() && em_width > 0.);
                        let character = match row {
                            ScrollRow::Repeated(character) => character,
                            ScrollRow::Combining(marks) => {
                                let cluster = 2_048..2_049 + 2 * marks as u32;
                                let carets = [
                                    cluster.start - 1,
                                    cluster.start,
                                    cluster.end,
                                    cluster.end + 1,
                                    text.len() as u32 - 1,
                                    text.len() as u32,
                                ]
                                .map(|column| {
                                    let x = layout
                                        .runs
                                        .iter()
                                        .flat_map(|run| &run.glyphs)
                                        .zip(&positions)
                                        .find(|(glyph, _)| glyph.index >= column as usize)
                                        .map_or(width, |(_, x)| *x);
                                    (column, x)
                                });
                                let before = carets[1].1 / em_width;
                                let after = carets[2].1 / em_width;
                                let ink = native_ink(&layout, &positions, &cluster);
                                let em_advance = f64::from(
                                    cx.text_system().em_advance(wrapper.font_id(), font_size)
                                        .expect("the native font must have an em advance"),
                                );
                                return ScrollOracle::Combining {
                                    cluster,
                                    carets,
                                    ink,
                                    em_width,
                                    em_advance,
                                    scroll_positions: [before - 32., before - 8., after + 32., before - 32.],
                                };
                            }
                        };
                        let advance = width / 256.;
                        let columns_per_character = advance / em_width;
                        assert!(columns_per_character.is_finite() && columns_per_character > 0.);
                        let paint_width = layout
                            .runs
                            .iter()
                            .flat_map(|run| {
                                let width = f64::from(
                                    cx.text_system().bounding_box(run.font_id, font_size).size.width,
                                );
                                run.glyphs.iter().map(move |glyph| (glyph.index, width))
                            })
                            .zip(positions)
                            .map(|((index, width), x)| {
                                x + width - advance * (index / character.len_utf8()) as f64
                            })
                            .fold(0., f64::max);
                        let leading_characters = (paint_width / advance).ceil() + 1.;
                        if !oracle_reported {
                            eprintln!(
                                "native scroll oracle {character:?}: advance={advance}px, \
                                 paint_width={paint_width}px, maximum_leading_characters={leading_characters}"
                            );
                            oracle_reported = true;
                        }
                        ScrollOracle::Repeated {
                            bytes_per_character: character.len_utf8() as u32,
                            columns_per_character,
                            character_advance: advance,
                            leading_characters,
                        }
                    })
                })
                .expect("the benchmark window must remain open");
            let stopped = Arc::new(AtomicBool::new(false));
            let session = LongRowSession {
                editor: editor_slot,
                renders,
                stopped: stopped.clone(),
                oracle,
                expected_text: text,
                app: cx.update(|cx| cx.to_async()),
                requested_scroll: None,
                visible_start_character: 0,
                turn: 0,
                geometry_ready: Rc::new(Cell::new(true)),
                geometry_task: None,
            };
            (session, window, stopped)
        },
        |session, _, window, cx| {
            if !session.geometry_ready.get() {
                return;
            }
            let turn = session.turn;
            let Some(editor) = session.editor.borrow().clone() else {
                return;
            };
            editor.update(cx, |editor, cx| {
                let display_snapshot = editor.display_snapshot(cx);
                if editor.painted_viewport(&display_snapshot, window, cx).is_none() {
                    return;
                }
                assert_eq!(
                    display_snapshot.max_point(),
                    DisplayPoint::new(DisplayRow(0), session.expected_text.len() as u32),
                    "the complete line must remain unwrapped at turn {turn}"
                );
                if turn > 0 {
                    let requested_scroll = session
                        .requested_scroll
                        .as_ref()
                        .expect("each scroll turn must follow a scroll request");
                    if session.renders.get() <= requested_scroll.renders {
                        return;
                    }
                    if let Err(pending) = session.validate_scroll(
                        requested_scroll.clone(), turn, editor, &display_snapshot, window, cx,
                    ) {
                        assert!(editor.request_row_geometry(pending.clone(), cx));
                        session.geometry_ready.set(false);
                        let ready = session.geometry_ready.clone();
                        let window = window.window_handle();
                        session.geometry_task = Some(cx.spawn(async move |_, cx| {
                            pending.wait().await;
                            cx.update_window(window, |_, window, _| window.refresh())
                                .expect("the benchmark window must remain open");
                            ready.set(true);
                        }));
                        return;
                    }
                    session.requested_scroll.take();
                }
                if turn == LONG_ROW_SCROLL_TURNS {
                    session.stopped.store(true, Ordering::Release);
                    return;
                }
                let scroll_x = match &session.oracle {
                    ScrollOracle::Repeated { .. } => {
                        LONG_ROW_SCROLL_START + LONG_ROW_SCROLL_STEP * (turn + 1) as f64
                    }
                    ScrollOracle::Combining { cluster, scroll_positions, .. } => {
                        let column = if turn % 2 == 0 { cluster.start } else { cluster.end };
                        editor.change_selections(
                            SelectionEffects::no_scroll(),
                            window,
                            cx,
                            |selections| {
                                let point = Point::new(0, column);
                                selections.select_ranges([point..point]);
                            },
                        );
                        window.refresh();
                        scroll_positions[turn as usize / 2]
                    }
                };
                let position = point(scroll_x, 0.);
                editor.set_scroll_position(position, window, cx);
                assert_eq!(
                    editor.scroll_position(cx),
                    position,
                    "scroll turn {turn} did not accept the scroll request"
                );
                session.requested_scroll = Some(RequestedScroll {
                    x: position.x,
                    renders: session.renders.get(),
                });
                session.turn += 1;
            });
        },
    );
}

#[gpui::bench(
    inputs = long_row_byte_counts(),
    group = "Cold open, edit, move and type in unwrapped rows",
    input_name = "bytes_per_row",
    sample_size = 10
)]
fn editor_cold_long_rows(bytes_per_row: &usize, cx: &mut BenchAppContext) {
    bench_cold_long_rows("x".repeat(*bytes_per_row), 2, cx);
}

#[gpui::bench(
    inputs = ["hello_world", "digits_punctuation", "contextual_ascii", "ltr_indic"],
    group = "Cold open, edit, move and type in unwrapped rows",
    input_name = "corpus_20_mib",
    sample_size = 10
)]
fn editor_cold_long_row_corpora(corpus: &str, cx: &mut BenchAppContext) {
    let pattern = match corpus {
        "hello_world" => "hello world ",
        "digits_punctuation" => "0123456789 , ( ) [ ] { } ; + - * / = : . ",
        "contextual_ascii" => r#"{"path":"//->...++","value":123,"ok":true} "#,
        "ltr_indic" => "नमस्ते hello दुनिया 0123456789 ",
        _ => unreachable!("unknown long-row corpus: {corpus}"),
    };
    let bytes_per_row = 20 * 1024 * 1024;
    let prefix_bytes = bytes_per_row - 2;
    let mut row = String::with_capacity(bytes_per_row);
    row.extend(std::iter::repeat_n(pattern, prefix_bytes / pattern.len()));
    row.extend(std::iter::repeat_n(' ', prefix_bytes % pattern.len()));
    row.push_str("xx");
    assert_eq!(row.len(), bytes_per_row);
    bench_cold_long_rows(row, 2, cx);
}

#[gpui::bench(
    inputs = [4_096, 32_768, 262_144],
    group = "Cold open, edit, move and type in unwrapped rows",
    input_name = "short_rows",
    sample_size = 10
)]
fn editor_cold_short_rows(rows: &usize, cx: &mut BenchAppContext) {
    bench_cold_long_rows("x".repeat(80), *rows, cx);
}

fn bench_cold_long_rows(row: String, rows: usize, cx: &mut BenchAppContext) {
    init_context(cx);
    assert!(rows >= 2);
    assert!(row.ends_with("xx"));
    let content = std::iter::repeat_n(row.as_str(), rows)
        .collect::<Vec<_>>()
        .join("\n");
    let text = Rope::from(content.as_str());
    let mut expected_text = content;
    expected_text.insert(2 * row.len(), '!');
    expected_text.insert(2 * row.len() - 1, '!');
    expected_text.insert(row.len(), 'x');
    let expected_text = Arc::<str>::from(expected_text);
    let row_length = row.len() as u32;
    drop(row);
    let mut session_window = None::<AnyWindowHandle>;
    cx.bench_renderer_session(
        Duration::from_secs(240),
        |cx| {
            let window =
                *session_window.get_or_insert_with(|| cx.add_empty_window().window_handle());
            let editor = Rc::new(RefCell::new(None));
            let renders = Rc::new(Cell::new(0));
            cx.update_window(window, |_, window, cx| {
                window.replace_root(cx, |_, _| LongRowRoot {
                    editor: editor.clone(),
                    renders: renders.clone(),
                });
            })
            .expect("the benchmark window must remain open");
            let stopped = Arc::new(AtomicBool::new(false));
            let session = ColdLongRowSession {
                editor,
                renders,
                ready: Rc::new(Cell::new(true)),
                typed: Rc::new(Cell::new(false)),
                stopped: stopped.clone(),
                task: None,
                app: cx.update(|cx| cx.to_async()),
                expected_text: expected_text.clone(),
                row_length,
                checked_render: 0,
            };
            (session, window, stopped)
        },
        |session, _, window, cx| {
            if !session.ready.replace(false) {
                return;
            }
            let editor = session.editor.borrow().clone();
            if let Some(editor) = &editor {
                let render = session.renders.get();
                if render == session.checked_render {
                    session.ready.set(true);
                    return;
                }
                session.checked_render = render;
                let painted = editor.update(cx, |editor, cx| {
                    let snapshot = editor.display_snapshot(cx);
                    editor.painted_viewport(&snapshot, window, cx).is_some()
                });
                if session.renders.get() == 0 || !painted {
                    session.ready.set(true);
                    return;
                }
                if session.typed.get() {
                    session.stopped.store(true, Ordering::Release);
                    return;
                }
            }
            if editor.is_none() {
                session.renders.set(0);
            }
            session.checked_render = 0;
            let editor_slot = session.editor.clone();
            let renders = session.renders.clone();
            let ready = session.ready.clone();
            let typed = session.typed.clone();
            let text = text.clone();
            let window = window.window_handle();
            session.task = Some(cx.spawn(async move |cx| {
                if let Some(editor) = editor {
                    cx.update_window(window, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.buffer().update(cx, |buffer, cx| {
                                let end = Point::new(0, row_length);
                                buffer.edit([(end..end, "x")], None, cx);
                            });
                            editor.move_down(&MoveDown, window, cx);
                            editor.input_completion(cx)
                        })
                    })
                    .expect("the benchmark window must remain open")
                    .await
                    .expect("vertical movement must complete");
                    cx.update_window(window, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            let snapshot = editor.display_snapshot(cx);
                            assert_eq!(
                                editor
                                    .selections
                                    .all::<Point>(&snapshot)
                                    .into_iter()
                                    .map(|selection| selection.range())
                                    .collect::<Vec<_>>(),
                                [row_length - 2, row_length - 1].map(|column| {
                                    let point = Point::new(1, column);
                                    point..point
                                })
                            );
                            editor.handle_input("!", window, cx);
                            editor.input_completion(cx)
                        })
                    })
                    .expect("the benchmark window must remain open")
                    .await
                    .expect("typing must complete");
                    renders.set(0);
                    typed.set(true);
                } else {
                    cx.update_window(window, |_, window, cx| {
                        let editor = cx.new(|cx| unwrapped_editor(text, window, cx));
                        editor.update(cx, |editor, cx| {
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |selections| {
                                    selections.select_ranges([row_length - 2, row_length - 1].map(
                                        |column| {
                                            let point = Point::new(0, column);
                                            point..point
                                        },
                                    ));
                                },
                            );
                        });
                        window.focus(&editor.focus_handle(cx), cx);
                        editor_slot.replace(Some(editor));
                    })
                    .expect("the benchmark window must remain open");
                }
                cx.update_window(window, |_, window, _| window.refresh())
                    .expect("the benchmark window must remain open");
                ready.set(true);
            }));
        },
    );
}

struct ColdLongRowSession {
    editor: Rc<RefCell<Option<Entity<Editor>>>>,
    renders: Rc<Cell<u64>>,
    ready: Rc<Cell<bool>>,
    typed: Rc<Cell<bool>>,
    stopped: Arc<AtomicBool>,
    task: Option<Task<()>>,
    app: AsyncApp,
    expected_text: Arc<str>,
    row_length: u32,
    checked_render: u64,
}

impl Drop for ColdLongRowSession {
    fn drop(&mut self) {
        let editor = self.editor.borrow_mut().take();
        if std::thread::panicking() {
            return;
        }
        assert!(self.stopped.load(Ordering::Acquire));
        assert!(self.renders.get() > 0, "typing must render");
        self.app.update(|cx| {
            editor
                .expect("the cold-open workload must create an editor")
                .update(cx, |editor, cx| {
                    let snapshot = editor.display_snapshot(cx);
                    let (last_row, last_line) = self
                        .expected_text
                        .split('\n')
                        .enumerate()
                        .last()
                        .expect("the expected buffer must have at least one row");
                    assert_eq!(
                        snapshot.max_point(),
                        DisplayPoint::new(DisplayRow(last_row as u32), last_line.len() as u32)
                    );
                    assert_eq!(snapshot.line_len(DisplayRow(0)), self.row_length + 1);
                    assert_eq!(editor.selections.count(), 2);
                    assert_eq!(
                        editor
                            .selections
                            .all::<Point>(&snapshot)
                            .into_iter()
                            .map(|selection| selection.range())
                            .collect::<Vec<_>>(),
                        [self.row_length - 1, self.row_length + 1].map(|column| {
                            let point = Point::new(1, column);
                            point..point
                        })
                    );
                    assert_eq!(editor.text(cx).as_str(), self.expected_text.as_ref());
                });
        });
    }
}

enum ScrollRow {
    Repeated(char),
    Combining(usize),
}

enum ScrollOracle {
    Repeated {
        bytes_per_character: u32,
        columns_per_character: f64,
        character_advance: f64,
        leading_characters: f64,
    },
    Combining {
        cluster: Range<u32>,
        carets: [(u32, f64); 6],
        ink: Vec<(Range<u32>, Range<f64>)>,
        em_width: f64,
        em_advance: f64,
        scroll_positions: [f64; 4],
    },
}

struct LongRowSession {
    editor: Rc<RefCell<Option<Entity<Editor>>>>,
    renders: Rc<Cell<u64>>,
    stopped: Arc<AtomicBool>,
    oracle: ScrollOracle,
    expected_text: Rope,
    app: AsyncApp,
    requested_scroll: Option<RequestedScroll>,
    visible_start_character: u32,
    turn: u64,
    geometry_ready: Rc<Cell<bool>>,
    geometry_task: Option<Task<()>>,
}

impl LongRowSession {
    fn validate_scroll(
        &mut self,
        requested_scroll: RequestedScroll,
        turn: u64,
        editor: &Editor,
        display_snapshot: &DisplaySnapshot,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Result<(), PendingRowGeometry> {
        assert!(
            self.renders.get() > requested_scroll.renders,
            "no render followed the scroll request preceding turn {turn}"
        );
        let pixel_tolerance = 0.5 / f64::from(window.scale_factor());
        let (bytes_per_character, columns_per_character, character_advance, leading_characters) =
            match &self.oracle {
                ScrollOracle::Repeated {
                    bytes_per_character,
                    columns_per_character,
                    character_advance,
                    leading_characters,
                } => (
                    *bytes_per_character,
                    *columns_per_character,
                    *character_advance,
                    *leading_characters,
                ),
                ScrollOracle::Combining {
                    cluster,
                    carets,
                    ink,
                    em_width,
                    em_advance,
                    ..
                } => {
                    assert!(
                        (editor.scroll_position(cx).x - requested_scroll.x).abs() * em_width
                            <= pixel_tolerance,
                        "scroll drift at turn {turn}"
                    );
                    let details = editor.text_layout_details(window, cx);
                    for &(column, expected_x) in carets {
                        let actual_x = display_snapshot.x_for_display_point(
                            DisplayPoint::new(DisplayRow(0), column),
                            &details,
                        )?;
                        assert!(
                            (actual_x - expected_x).abs() <= pixel_tolerance,
                            "native position drift at column {column}, turn {turn}: \
                         {actual_x}px != {expected_x}px, bound={pixel_tolerance}px"
                        );
                        assert_eq!(
                            display_snapshot.display_column_for_x(
                                DisplayRow(0),
                                expected_x,
                                &details
                            )?,
                            column,
                            "native hit test at column {column}, turn {turn}"
                        );
                    }
                    let column = if turn % 2 == 1 {
                        cluster.start
                    } else {
                        cluster.end
                    };
                    let point = Point::new(0, column);
                    assert_eq!(
                        editor
                            .selections
                            .all::<Point>(display_snapshot)
                            .into_iter()
                            .map(|selection| selection.range())
                            .collect::<Vec<_>>(),
                        [point..point]
                    );
                    let ranges = editor.visible_text_ranges(display_snapshot, cx)?;
                    let [range] = ranges.as_slice() else {
                        panic!("expected one visible range at turn {turn}, got {ranges:?}");
                    };
                    assert_eq!((range.start.row, range.end.row), (0, 0));
                    let viewport_start = editor.scroll_position(cx).x * em_width;
                    let viewport_end = viewport_start
                        + editor
                            .visible_column_count()
                            .expect("the editor must have a viewport")
                            * em_advance;
                    let required = ink
                        .iter()
                        .filter(|(_, bounds)| {
                            bounds.start < viewport_end - pixel_tolerance
                                && bounds.end > viewport_start + pixel_tolerance
                        })
                        .map(|(columns, _)| columns.clone())
                        .reduce(|left, right| left.start.min(right.start)..left.end.max(right.end))
                        .expect("the scrolled viewport must intersect native ink");
                    assert!(
                        range.start.column <= required.start && range.end.column >= required.end,
                        "visible range {range:?} omits native ink dependencies {required:?} \
                         in viewport {viewport_start}..{viewport_end} at turn {turn}"
                    );
                    for point in [range.start, range.end] {
                        assert!(point.column <= cluster.start || point.column >= cluster.end);
                    }
                    return Ok(());
                }
            };
        let expected = requested_scroll.x / columns_per_character;
        let actual_scroll = editor.scroll_position(cx).x / columns_per_character;
        assert!(
            (actual_scroll - expected).abs() * character_advance <= pixel_tolerance,
            "scroll drift at turn {turn}: {actual_scroll} != {expected} characters"
        );
        let details = editor.text_layout_details(window, cx);
        for character in [expected.ceil() as u32, LONG_ROW_CHARACTERS as u32 - 1] {
            let column = character * bytes_per_character;
            let expected_x = f64::from(character) * character_advance;
            let actual_x = display_snapshot
                .x_for_display_point(DisplayPoint::new(DisplayRow(0), column), &details)?;
            assert!(
                (actual_x - expected_x).abs() <= pixel_tolerance,
                "native position drift at character {character}, turn {turn}: \
                 {actual_x}px != {expected_x}px, bound={pixel_tolerance}px"
            );
            for fraction in [-0.25, 0.25] {
                assert_eq!(
                    display_snapshot.display_column_for_x(
                        DisplayRow(0),
                        (f64::from(character) + fraction) * character_advance,
                        &details,
                    )?,
                    column,
                    "native hit test at character {character} + {fraction}, turn {turn}"
                );
            }
        }
        let visible_ranges = editor.visible_text_ranges(display_snapshot, cx)?;
        let [visible_range] = visible_ranges.as_slice() else {
            panic!("expected one visible range at turn {turn}, got {visible_ranges:?}");
        };
        assert!(
            visible_range.start.row == 0
                && visible_range.end.row == 0
                && visible_range.start.column < visible_range.end.column
                && visible_range.start.column % bytes_per_character == 0
                && visible_range.end.column % bytes_per_character == 0,
            "invalid visible range {visible_range:?} at turn {turn}"
        );
        let visible_start_character = visible_range.start.column / bytes_per_character;
        assert!(
            visible_start_character > self.visible_start_character
                && f64::from(visible_start_character) >= expected - leading_characters
                && f64::from(visible_start_character) <= expected + LONG_ROW_CHARACTER_TOLERANCE,
            "visible range at turn {turn} starts at character {visible_start_character}, \
             expected after character {} and within {}..={} from native font bounds",
            self.visible_start_character,
            expected - leading_characters,
            expected + LONG_ROW_CHARACTER_TOLERANCE,
        );
        self.visible_start_character = visible_start_character;
        Ok(())
    }
}

#[derive(Clone)]
struct RequestedScroll {
    x: f64,
    renders: u64,
}

impl Drop for LongRowSession {
    fn drop(&mut self) {
        let editor = self.editor.borrow_mut().take();
        if std::thread::panicking() {
            return;
        }
        assert!(self.stopped.load(Ordering::Acquire));
        assert_eq!(self.turn, LONG_ROW_SCROLL_TURNS);
        assert!(self.requested_scroll.is_none());
        if let ScrollOracle::Combining { .. } = &self.oracle {
            self.app.update(|cx| {
                let editor = editor.expect("the scroll workload must retain its editor");
                let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
                assert_eq!(snapshot.len().0, self.expected_text.len());
                assert!(
                    snapshot
                        .chars_at(Point::new(0, 0))
                        .eq(self.expected_text.chars())
                );
            });
        }
    }
}

struct LongRowRoot {
    editor: Rc<RefCell<Option<Entity<Editor>>>>,
    renders: Rc<Cell<u64>>,
}

impl Render for LongRowRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
        div().size_full().children(self.editor.borrow().clone())
    }
}

const LONG_ROW_CHARACTERS: usize = 20_000_000;
const LONG_ROW_SCROLL_TURNS: u64 = 8;
const LONG_ROW_SCROLL_START: f64 = 1_000_000.;
const LONG_ROW_SCROLL_STEP: f64 = 200.;
const LONG_ROW_CHARACTER_TOLERANCE: f64 = 1.;
const LONG_ROW_SESSION_TIMEOUT: Duration = Duration::from_secs(30);

fn long_row_characters() -> Vec<char> {
    vec!['x', 'é', '漢']
}

fn long_row_byte_counts() -> Vec<usize> {
    vec![20 * 1024, 2 * 1024 * 1024, 20 * 1024 * 1024]
}

fn native_ink(
    layout: &LineLayout,
    positions: &[f64],
    cluster: &Range<u32>,
) -> Vec<(Range<u32>, Range<f64>)> {
    let mut columns = layout
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.index as u32))
        .collect::<Vec<_>>();
    assert_eq!(columns.len(), positions.len());
    columns.push(layout.len as u32);
    columns.sort_unstable();
    columns.dedup();
    let text_system = gpui_platform::bench_text_system();
    let mut glyph_bounds = HashMap::new();
    let mut ink = Vec::new();
    for ((font_id, glyph), x) in layout
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter().map(move |glyph| (run.font_id, glyph)))
        .zip(positions)
    {
        let start = glyph.index as u32;
        let end = *columns
            .get(columns.partition_point(|column| *column <= start))
            .expect("each native glyph must have a source dependency");
        let start = if cluster.contains(&start) {
            cluster.start
        } else {
            start
        };
        let end = if cluster.start < end && end < cluster.end {
            cluster.end
        } else {
            end
        };
        let bounds = glyph_bounds.entry((font_id, glyph.id)).or_insert_with(|| {
            let bounds = text_system
                .typographic_bounds(font_id, glyph.id)
                .expect("native glyph ink bounds must be available");
            let scale = f64::from(layout.font_size)
                / f64::from(text_system.font_metrics(font_id).units_per_em);
            f64::from(bounds.left()) * scale..f64::from(bounds.right()) * scale
        });
        if bounds.start < bounds.end {
            ink.push((start..end, x + bounds.start..x + bounds.end));
        }
    }
    ink.sort_unstable_by(|left, right| {
        left.0
            .start
            .cmp(&right.0.start)
            .then(left.0.end.cmp(&right.0.end))
            .then(left.1.start.total_cmp(&right.1.start))
    });
    let mut merged = Vec::<(Range<u32>, Range<f64>)>::new();
    for (columns, bounds) in ink {
        if let Some((previous_columns, previous_bounds)) = merged.last_mut()
            && *previous_columns == columns
            && bounds.start <= previous_bounds.end
        {
            previous_bounds.end = previous_bounds.end.max(bounds.end);
        } else {
            merged.push((columns, bounds));
        }
    }
    merged
}

fn unwrapped_editor(text: Rope, window: &mut Window, cx: &mut Context<Editor>) -> Editor {
    let buffer = cx.new(|cx| Buffer::local_normalized(text, LineEnding::Unix, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let mut editor = Editor::new(EditorMode::full(), buffer, None, window, cx);
    editor.set_soft_wrap_mode(SoftWrap::None, cx);
    editor
}

fn init_context(cx: &mut BenchAppContext) {
    cx.update(|cx| {
        settings::init(cx);
        assets::Assets.load_test_fonts(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
}

gpui::bench_group!(
    benches,
    editor_scroll_long_row,
    editor_scroll_combining_cluster,
    editor_cold_long_rows,
    editor_cold_long_row_corpora,
    editor_cold_short_rows
);
gpui::bench_main!(benches);
