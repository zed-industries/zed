use std::{borrow::Cow, cell::Cell, fmt, rc::Rc, time::Duration};

use gpui::{
    AppContext as _, BenchAppContext, Bounds, Context, InlineObject, IntoElement, ParentElement,
    Pixels, Render, SharedString, Styled, StyledText, TextRun, Window, WindowBounds, WindowOptions,
    canvas, div, fill, font, hsla, point, px, size,
};

const IBM_PLEX_SANS: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");

#[derive(Clone, Copy)]
enum LayoutMode {
    PlainWarm,
    PlainReflow,
    InlineObjectsWarm,
    InlineObjectsReflow,
    FlexWordsWarm,
    FlexWordsReflow,
}

#[derive(Clone, Copy)]
struct BenchCase {
    mode: LayoutMode,
    paragraphs: usize,
}

impl fmt::Display for BenchCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self.mode {
            LayoutMode::PlainWarm => "plain-warm",
            LayoutMode::PlainReflow => "plain-reflow",
            LayoutMode::InlineObjectsWarm => "inline-objects-warm",
            LayoutMode::InlineObjectsReflow => "inline-objects-reflow",
            LayoutMode::FlexWordsWarm => "flex-words-warm",
            LayoutMode::FlexWordsReflow => "flex-words-reflow",
        };
        write!(formatter, "{mode}-{}-paragraphs", self.paragraphs)
    }
}

fn plain_cases() -> Vec<BenchCase> {
    [12, 48]
        .into_iter()
        .flat_map(|paragraphs| {
            [
                BenchCase {
                    mode: LayoutMode::PlainWarm,
                    paragraphs,
                },
                BenchCase {
                    mode: LayoutMode::PlainReflow,
                    paragraphs,
                },
            ]
        })
        .collect()
}

fn replaced_cases() -> Vec<BenchCase> {
    [12, 48]
        .into_iter()
        .flat_map(|paragraphs| {
            [
                BenchCase {
                    mode: LayoutMode::InlineObjectsWarm,
                    paragraphs,
                },
                BenchCase {
                    mode: LayoutMode::FlexWordsWarm,
                    paragraphs,
                },
                BenchCase {
                    mode: LayoutMode::InlineObjectsReflow,
                    paragraphs,
                },
                BenchCase {
                    mode: LayoutMode::FlexWordsReflow,
                    paragraphs,
                },
            ]
        })
        .collect()
}

#[derive(Clone)]
struct ObjectSpec {
    width: Pixels,
    ascent: Pixels,
    descent: Pixels,
}

#[derive(Clone)]
enum Segment {
    Word(SharedString),
    Space,
    Object {
        spec: ObjectSpec,
        trailing_punctuation: Option<SharedString>,
    },
}

#[derive(Clone)]
struct ParagraphFixture {
    plain_variants: [SharedString; 4],
    inline_text_variants: [SharedString; 4],
    inline_objects_variants: [Vec<InlineObject>; 4],
    flex_segments_variants: [Vec<Segment>; 4],
    expected_objects: usize,
}

fn build_paragraphs(count: usize) -> Rc<[ParagraphFixture]> {
    const WORDS: &[&str] = &[
        "Given",
        "continuous",
        "operators",
        "over",
        "Hilbert",
        "spaces",
        "converge",
        "uniformly",
        "whenever",
        "bounded",
        "eigenvalues",
        "satisfy",
        "spectral",
        "stability",
        "conditions",
        "under",
        "compact",
        "perturbations",
        "across",
        "boundary",
        "intervals",
        "without",
        "losing",
        "orthogonality",
    ];
    let object_specs = [
        ObjectSpec {
            width: px(44.),
            ascent: px(19.),
            descent: px(8.),
        },
        ObjectSpec {
            width: px(68.),
            ascent: px(24.),
            descent: px(11.),
        },
        ObjectSpec {
            width: px(36.),
            ascent: px(16.),
            descent: px(5.),
        },
        ObjectSpec {
            width: px(54.),
            ascent: px(21.),
            descent: px(9.),
        },
    ];

    (0..count)
        .map(|paragraph_index| {
            let mut plain_variants: [SharedString; 4] = Default::default();
            let mut inline_text_variants: [SharedString; 4] = Default::default();
            let mut inline_objects_variants: [Vec<InlineObject>; 4] = Default::default();
            let mut flex_segments_variants: [Vec<Segment>; 4] = Default::default();
            let mut expected_objects = 0;

            for variant in 0..4 {
                let prefix = format!("[{paragraph_index}:{variant}] ");
                let mut plain = prefix.clone();
                let mut inline_text = prefix.clone();
                let mut inline_objects = Vec::with_capacity(4);
                let mut flex_segments = Vec::with_capacity(96);
                flex_segments.push(Segment::Word(SharedString::from(
                    prefix.trim_end().to_owned(),
                )));

                for word_index in 0..48 {
                    plain.push(' ');
                    inline_text.push(' ');
                    flex_segments.push(Segment::Space);

                    if word_index % 12 == 5 {
                        let spec = object_specs
                            [(paragraph_index + word_index / 12) % object_specs.len()]
                        .clone();
                        let punctuation = (word_index % 24 == 5).then_some(",");

                        plain.push_str("expr_value");
                        if let Some(punctuation) = punctuation {
                            plain.push_str(punctuation);
                        }

                        let start = inline_text.len();
                        inline_text.push('\u{fffc}');
                        let end = inline_text.len();
                        if let Some(punctuation) = punctuation {
                            inline_text.push_str(punctuation);
                        }
                        inline_objects.push(InlineObject {
                            range: start..end,
                            width: spec.width,
                            ascent: spec.ascent,
                            descent: spec.descent,
                        });
                        flex_segments.push(Segment::Object {
                            spec,
                            trailing_punctuation: punctuation.map(SharedString::from),
                        });
                    } else {
                        let word =
                            WORDS[(paragraph_index * 7 + word_index + variant) % WORDS.len()];
                        plain.push_str(word);
                        inline_text.push_str(word);
                        flex_segments.push(Segment::Word(SharedString::from(word)));
                    }
                }
                plain.push('.');
                inline_text.push('.');
                flex_segments.push(Segment::Word(SharedString::from(".")));

                expected_objects = inline_objects.len();
                plain_variants[variant] = SharedString::from(plain);
                inline_text_variants[variant] = SharedString::from(inline_text);
                inline_objects_variants[variant] = inline_objects;
                flex_segments_variants[variant] = flex_segments;
            }

            ParagraphFixture {
                plain_variants,
                inline_text_variants,
                inline_objects_variants,
                flex_segments_variants,
                expected_objects,
            }
        })
        .collect::<Vec<_>>()
        .into()
}

struct InlineObjectsBenchView {
    mode: LayoutMode,
    paragraphs: Rc<[ParagraphFixture]>,
    step: usize,
    painted_objects: Rc<Cell<usize>>,
}

impl InlineObjectsBenchView {
    fn is_reflow(&self) -> bool {
        matches!(
            self.mode,
            LayoutMode::PlainReflow | LayoutMode::InlineObjectsReflow | LayoutMode::FlexWordsReflow
        )
    }
}

impl Render for InlineObjectsBenchView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.painted_objects.set(0);
        let variant = if self.is_reflow() { self.step % 4 } else { 0 };
        let container_width = if self.is_reflow() {
            px(540. + ((self.step % 6) as f32) * 24.)
        } else {
            px(600.)
        };
        let font_size = px(15.);
        let line_height = px(21.);
        let text_color = hsla(0., 0., 0.12, 1.);
        let object_fill = hsla(0.58, 0.65, 0.45, 0.25);
        let base_font = font("IBM Plex Sans");

        let mut root = div()
            .w(container_width)
            .flex()
            .flex_col()
            .gap(px(8.))
            .text_size(font_size)
            .line_height(line_height)
            .text_color(text_color);

        for paragraph in self.paragraphs.iter() {
            match self.mode {
                LayoutMode::PlainWarm | LayoutMode::PlainReflow => {
                    let text = paragraph.plain_variants[variant].clone();
                    let run = TextRun {
                        len: text.len(),
                        font: base_font.clone(),
                        color: text_color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    };
                    root = root.child(StyledText::new(text).with_runs(vec![run]));
                }
                LayoutMode::InlineObjectsWarm | LayoutMode::InlineObjectsReflow => {
                    let text = paragraph.inline_text_variants[variant].clone();
                    let run = TextRun {
                        len: text.len(),
                        font: base_font.clone(),
                        color: text_color,
                        background_color: None,
                        underline: None,
                        strikethrough: None,
                    };
                    let objects = paragraph.inline_objects_variants[variant].clone();
                    let painted_objects = self.painted_objects.clone();
                    root = root.child(
                        StyledText::new(text)
                            .with_runs(vec![run])
                            .with_inline_objects(objects)
                            .on_paint_inline_object(move |_, placement, window, _| {
                                painted_objects.set(painted_objects.get() + 1);
                                window.paint_quad(fill(placement.bounds, object_fill));
                            }),
                    );
                }
                LayoutMode::FlexWordsWarm | LayoutMode::FlexWordsReflow => {
                    let mut row = div().w_full().flex().flex_wrap().items_center();
                    for segment in &paragraph.flex_segments_variants[variant] {
                        match segment {
                            Segment::Word(word) => {
                                let run = TextRun {
                                    len: word.len(),
                                    font: base_font.clone(),
                                    color: text_color,
                                    background_color: None,
                                    underline: None,
                                    strikethrough: None,
                                };
                                row = row.child(StyledText::new(word.clone()).with_runs(vec![run]));
                            }
                            Segment::Space => {
                                row = row.child(div().w(px(4.)).h(line_height));
                            }
                            Segment::Object {
                                spec,
                                trailing_punctuation,
                            } => {
                                let painted_objects = self.painted_objects.clone();
                                let height = spec.ascent + spec.descent;
                                let width = spec.width;
                                let box_canvas = canvas(
                                    |_, _, _| {},
                                    move |bounds: Bounds<Pixels>, _, window, _| {
                                        painted_objects.set(painted_objects.get() + 1);
                                        window.paint_quad(fill(bounds, object_fill));
                                    },
                                )
                                .w(width)
                                .h(height);
                                if let Some(punctuation) = trailing_punctuation {
                                    let run = TextRun {
                                        len: punctuation.len(),
                                        font: base_font.clone(),
                                        color: text_color,
                                        background_color: None,
                                        underline: None,
                                        strikethrough: None,
                                    };
                                    row = row.child(
                                        div().flex().items_center().child(box_canvas).child(
                                            StyledText::new(punctuation.clone())
                                                .with_runs(vec![run]),
                                        ),
                                    );
                                } else {
                                    row = row.child(box_canvas);
                                }
                            }
                        }
                    }
                    root = root.child(row);
                }
            }
        }

        root
    }
}

fn run_case(case: &BenchCase, cx: &mut BenchAppContext) {
    assert!(!cfg!(debug_assertions), "use --profile release-fast");
    let paragraphs = build_paragraphs(case.paragraphs);
    let expected_objects: usize = match case.mode {
        LayoutMode::PlainWarm | LayoutMode::PlainReflow => 0,
        _ => paragraphs
            .iter()
            .map(|paragraph| paragraph.expected_objects)
            .sum(),
    };
    let painted_objects = Rc::new(Cell::new(0));

    let (window, root) = cx.update(|app| {
        let paragraphs = paragraphs.clone();
        let painted_objects = painted_objects.clone();
        let mut root_entity = None;
        let handle = app
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(0.), px(0.)),
                        size(px(1024.), px(12000.)),
                    ))),
                    ..Default::default()
                },
                |window, cx| {
                    window
                        .text_system()
                        .add_fonts(vec![Cow::Borrowed(IBM_PLEX_SANS)])
                        .expect("IBM Plex Sans benchmark font should register");
                    let view = cx.new(|_| InlineObjectsBenchView {
                        mode: case.mode,
                        paragraphs,
                        step: 0,
                        painted_objects,
                    });
                    root_entity = Some(view.clone());
                    view
                },
            )
            .expect("failed to open benchmark window");
        app.update_window(handle.into(), |_, window, _| window.activate_window())
            .expect("failed to activate benchmark window");
        (handle, root_entity.expect("benchmark root entity"))
    });

    cx.run_until_idle();
    cx.update(|app| {
        window
            .update(app, |_, window, _| window.refresh())
            .expect("refresh benchmark window");
    });
    cx.run_until_idle();
    assert_eq!(painted_objects.get(), expected_objects);

    cx.bench_renderer(root, |view, _, cx| {
        view.step = view.step.wrapping_add(1);
        cx.notify();
    });

    assert_eq!(painted_objects.get(), expected_objects);
    cx.update(|app| {
        window
            .update(app, |_, window, _| window.remove_window())
            .expect("close benchmark window");
    });
    cx.run_until_idle();
}

#[gpui::bench(
    inputs = plain_cases(),
    input_name = "case",
    group = "TextLayout/plain",
    fps = 120
)]
fn bench_plain_text(case: &BenchCase, cx: &mut BenchAppContext) {
    run_case(case, cx);
}

#[gpui::bench(
    inputs = replaced_cases(),
    input_name = "case",
    group = "TextLayout/replaced",
    fps = 120
)]
fn bench_replaced_text(case: &BenchCase, cx: &mut BenchAppContext) {
    run_case(case, cx);
}

gpui::bench_group!(
    name = benches;
    config = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .sample_size(20)
        .without_plots();
    targets = bench_plain_text, bench_replaced_text
);
gpui::bench_main!(benches);
