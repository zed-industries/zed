use std::{cell::Cell, fmt, rc::Rc, time::Duration};

use gpui::{
    BenchAppContext, Bounds, Context, InteractiveElement as _, IntoElement, ParentElement as _,
    Pixels, Render, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px, rgb, size,
};

const COLUMN_COUNT: usize = 100;
const NODE_SIZE: f32 = 6.0;
const ROW_HEIGHT: f32 = 24.0;
const PANEL_WIDTH: f32 = 320.0;

#[derive(Default)]
struct RenderWork {
    rendered_frames: Cell<usize>,
    prepainted_children: Cell<usize>,
    invalid_children: Cell<usize>,
}

impl RenderWork {
    fn record_children(&self, bounds: &[Bounds<Pixels>], valid: impl Fn(&Bounds<Pixels>) -> bool) {
        self.prepainted_children
            .set(self.prepainted_children.get() + bounds.len());
        self.invalid_children.set(
            self.invalid_children.get() + bounds.iter().filter(|bounds| !valid(bounds)).count(),
        );
    }
}

struct ClosedInspectorInput {
    scale: usize,
    unit: &'static str,
    open_close_cycles: usize,
    other_window_inspected: bool,
}

impl fmt::Display for ClosedInspectorInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}-{}-{}-cycles",
            self.scale, self.unit, self.open_close_cycles
        )?;
        if self.other_window_inspected {
            write!(formatter, "-other-window")?;
        }
        Ok(())
    }
}

fn closed_inspector_inputs(
    unit: &'static str,
    scales: impl IntoIterator<Item = usize>,
) -> Vec<ClosedInspectorInput> {
    let lifecycles = if cfg!(feature = "inspector") {
        vec![(0, false), (2, false), (0, true), (2, true)]
    } else {
        vec![(0, false)]
    };
    scales
        .into_iter()
        .flat_map(|scale| {
            lifecycles
                .iter()
                .map(
                    move |&(open_close_cycles, other_window_inspected)| ClosedInspectorInput {
                        scale,
                        unit,
                        open_close_cycles,
                        other_window_inspected,
                    },
                )
        })
        .collect()
}

#[gpui::bench(
    inputs = closed_inspector_inputs("nodes", [100, 1_000, 10_000]),
    input_name = "case",
    group = "closed_inspector_grid",
    fps = 120
)]
fn bare_grid(input: &ClosedInspectorInput, cx: &mut BenchAppContext) {
    assert_eq!(input.scale % COLUMN_COUNT, 0);
    measure_closed_inspector(
        input,
        cx,
        |work| BareGrid {
            node_count: input.scale,
            alternate: false,
            work,
        },
        |grid| grid.alternate = !grid.alternate,
    );
}

#[gpui::bench(
    inputs = closed_inspector_inputs("rows", [50, 500]),
    input_name = "case",
    group = "closed_inspector_panel",
    fps = 120
)]
fn panel(input: &ClosedInspectorInput, cx: &mut BenchAppContext) {
    measure_closed_inspector(
        input,
        cx,
        |work| Panel::new(input.scale, work),
        |panel| panel.selected_row = (panel.selected_row + 1) % panel.labels.len(),
    );
}

fn measure_closed_inspector<V: Render + 'static>(
    input: &ClosedInspectorInput,
    cx: &mut BenchAppContext,
    build_view: impl FnOnce(Rc<RenderWork>) -> V,
    mut update_view: impl FnMut(&mut V),
) {
    assert!(!cfg!(debug_assertions), "use --profile release-fast");
    let expected_children = input.scale;

    let work = Rc::new(RenderWork::default());
    let mut window = cx.add_empty_window();
    let view = window.update(|window, cx| {
        assert!(!window.is_inspector_picking(cx));
        window.replace_root(cx, |_, _| build_view(work.clone()))
    });
    cx.run_until_idle();
    for _ in 0..input.open_close_cycles {
        for open in [true, false] {
            window.update(|window, cx| {
                #[cfg(feature = "inspector")]
                window.toggle_inspector(cx);
                assert_eq!(window.is_inspector_picking(cx), open);
                window.refresh();
            });
            cx.run_until_idle();
        }
    }
    let mut other_window = input.other_window_inspected.then(|| {
        let mut window = cx.add_empty_window();
        window.update(|window, cx| {
            #[cfg(feature = "inspector")]
            window.toggle_inspector(cx);
            assert!(window.is_inspector_picking(cx));
        });
        window
    });
    cx.run_until_idle();
    let other_frames_before = other_window.as_mut().map(|window| {
        window.update(|window, _| {
            window
                .frame_duration_snapshot()
                .draw_duration_histogram
                .len()
        })
    });
    assert!(work.rendered_frames.get() > 0);
    assert_eq!(
        work.prepainted_children.get(),
        work.rendered_frames.get() * expected_children
    );
    assert_eq!(work.invalid_children.get(), 0);
    work.rendered_frames.set(0);
    work.prepainted_children.set(0);

    let frames_before = window.update(|window, _| window.frame_duration_snapshot());
    let mut updated_frames = 0;
    cx.bench_renderer(view, |view, _, cx| {
        update_view(view);
        updated_frames += 1;
        cx.notify();
    });
    let frames_after = window.update(|window, cx| {
        assert!(!window.is_inspector_picking(cx));
        window.frame_duration_snapshot()
    });

    let other_frames_after = other_window.as_mut().map(|window| {
        window.update(|window, cx| {
            assert!(window.is_inspector_picking(cx));
            window
                .frame_duration_snapshot()
                .draw_duration_histogram
                .len()
        })
    });
    assert_eq!(other_frames_after, other_frames_before);
    assert!(updated_frames > 0);
    assert_eq!(work.rendered_frames.get(), updated_frames);
    assert_eq!(
        work.prepainted_children.get(),
        updated_frames * expected_children
    );
    assert_eq!(work.invalid_children.get(), 0);
    assert_eq!(
        frames_after.draw_duration_histogram.len() - frames_before.draw_duration_histogram.len(),
        updated_frames as u64
    );
    assert_eq!(
        frames_after.dirty_to_present_histogram.len()
            - frames_before.dirty_to_present_histogram.len(),
        updated_frames as u64
    );
}

struct BareGrid {
    node_count: usize,
    alternate: bool,
    work: Rc<RenderWork>,
}

impl Render for BareGrid {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.work
            .rendered_frames
            .set(self.work.rendered_frames.get() + 1);
        let background = rgb(if self.alternate { 0x334455 } else { 0x445566 });

        div()
            .id("bare-grid")
            .flex()
            .flex_col()
            .size(px(COLUMN_COUNT as f32 * NODE_SIZE))
            .children((0..self.node_count / COLUMN_COUNT).map(|row_index| {
                let work = self.work.clone();
                div()
                    .on_children_prepainted(move |bounds, _, _| {
                        work.record_children(&bounds, |bounds| {
                            bounds.size == size(px(NODE_SIZE), px(NODE_SIZE))
                        });
                    })
                    .id(("row", row_index))
                    .flex()
                    .h(px(NODE_SIZE))
                    .flex_shrink_0()
                    .children((0..COLUMN_COUNT).map(|column_index| {
                        div()
                            .id(("node", column_index))
                            .size(px(NODE_SIZE))
                            .flex_shrink_0()
                            .bg(background)
                            .border_1()
                            .border_color(rgb(0x8899aa))
                            .rounded(px(2.0))
                            .when(column_index % 2 == 0, |element| element.cursor_pointer())
                    }))
            }))
    }
}

struct Panel {
    labels: Vec<SharedString>,
    counts: Vec<SharedString>,
    selected_row: usize,
    work: Rc<RenderWork>,
}

impl Panel {
    fn new(row_count: usize, work: Rc<RenderWork>) -> Self {
        Panel {
            labels: (0..row_count)
                .map(|index| {
                    SharedString::from(format!("crates/module_{}/src/file_{index}.rs", index % 17))
                })
                .collect(),
            counts: (0..row_count)
                .map(|index| SharedString::from(((index * 7) % 100).to_string()))
                .collect(),
            selected_row: 0,
            work,
        }
    }
}

impl Render for Panel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.work
            .rendered_frames
            .set(self.work.rendered_frames.get() + 1);
        let work = self.work.clone();
        let selected_row = self.selected_row;

        div()
            .on_children_prepainted(move |bounds, _, _| {
                work.record_children(&bounds, |bounds| bounds.size.height == px(ROW_HEIGHT));
            })
            .id("panel")
            .flex()
            .flex_col()
            .w(px(PANEL_WIDTH))
            .h(px(ROW_HEIGHT * self.labels.len() as f32))
            .bg(rgb(0x1b1f24))
            .children(self.labels.iter().zip(&self.counts).enumerate().map(
                |(index, (label, count))| {
                    div()
                        .id(("row", index))
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .h(px(ROW_HEIGHT))
                        .flex_shrink_0()
                        .rounded_sm()
                        .cursor_pointer()
                        .hover(|style| style.bg(rgb(0x2a3038)))
                        .active(|style| style.bg(rgb(0x343c46)))
                        .when(index == selected_row, |row| row.bg(rgb(0x1f4f82)))
                        .on_click(|_, _, _| {})
                        .child(
                            div()
                                .size_4()
                                .flex_shrink_0()
                                .rounded_sm()
                                .bg(rgb(if index % 3 == 0 { 0x6aa1ff } else { 0x8fbf6a })),
                        )
                        .child(
                            div()
                                .flex_1()
                                .overflow_hidden()
                                .text_sm()
                                .text_color(rgb(0xd0d4da))
                                .child(label.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x8a919c))
                                .child(count.clone()),
                        )
                },
            ))
    }
}

gpui::bench_group! {
    name = benches;
    config = criterion::Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
        .without_plots();
    targets = bare_grid, panel
}
gpui::bench_main!(benches);
