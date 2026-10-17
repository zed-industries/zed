// Identical real-div pixel regressions run by each GPU renderer.

use super::{assert_same_pixels, pixel, render_div_fixture};
use gpui::{Path, px};

/// The command-palette regression from zed#37480: a rounded palette has a
/// header above the input and list. Its radius belongs only to the palette's
/// outer corners, never to the input's or list row's own clipping rectangle.
#[gpui::test]
fn rounded_overflow_command_palette_rows_keep_square_corners(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};
    let (pixels, side) = render_div_fixture(cx, || {
        let row = |height, color| {
            gpui::div()
                .w_full()
                .h(px(height))
                .flex_shrink_0()
                .overflow_hidden()
                .bg(color)
        };
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .flex()
                .flex_col()
                .size(px(64.))
                .rounded(px(16.))
                .overflow_hidden()
                .child(row(16., gpui::black()))
                .child(row(16., gpui::red()))
                .child(row(8., gpui::blue()))
                .child(row(8., gpui::black()))
                .child(row(16., gpui::green())),
        )
    });
    let sample = |x, y| pixel(&pixels, side, x * side / 64, y * side / 64);
    // Check all four input and selected-row corners, not just their centres.
    for x in [0, 63] {
        for y in [16, 31] {
            assert_eq!(
                sample(x, y),
                [255, 0, 0, 255],
                "input corner acquired the palette radius at ({x}, {y})"
            );
        }
        for y in [32, 39] {
            assert_eq!(
                sample(x, y),
                [0, 0, 255, 255],
                "selected row acquired the palette radius at ({x}, {y})"
            );
        }
        assert_eq!(sample(x, 0), [255, 255, 255, 255]);
        assert_eq!(sample(x, 63), [255, 255, 255, 255]);
    }
    assert_eq!(sample(32, 0), [0, 0, 0, 255]);
    assert_eq!(sample(32, 63), [0, 128, 0, 255]);
}

#[gpui::test]
fn rounded_overflow_deep_stack_preserves_clips(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};
    let (pixels, side) = render_div_fixture(cx, || {
        let mut child = gpui::div().size(px(64.)).bg(gpui::black());
        // Distinct geometry prevents identical-scope deduplication. This also
        // exceeds Metal's 4096-byte inline fragment parameter limit.
        for depth in 0..100 {
            child = gpui::div()
                .size(px(64.))
                .rounded(px(20. + depth as f32 * 0.01))
                .overflow_hidden()
                .child(child);
        }
        gpui::div().size(px(64.)).bg(gpui::white()).child(child)
    });
    assert_eq!(pixel(&pixels, side, 0, 0), [255, 255, 255, 255]);
    assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
}

#[gpui::test]
fn rounded_overflow_transparent_unequal_borders_clip_padding_edge(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};
    let (pixels, side) = render_div_fixture(cx, || {
        let mut parent = gpui::div().size(px(64.)).rounded(px(24.)).overflow_hidden();
        parent.style().border_widths.left = Some(px(8.).into());
        parent.style().border_widths.top = Some(px(4.).into());
        parent.style().border_widths.right = Some(px(2.).into());
        parent.style().border_widths.bottom = Some(px(6.).into());
        gpui::div()
            .size(px(64.))
            .bg(gpui::white())
            .child(parent.child(gpui::div().size(px(64.)).bg(gpui::black())))
    });
    let sample = |x, y| pixel(&pixels, side, x * side / 64, y * side / 64);
    assert_eq!(sample(9, 5), [255, 255, 255, 255]);
    assert_eq!(sample(24, 5), [0, 0, 0, 255]);
    assert_eq!(sample(9, 24), [0, 0, 0, 255]);
    assert_eq!(sample(60, 24), [0, 0, 0, 255]);
    assert_eq!(sample(63, 32), [255, 255, 255, 255]);
    assert_eq!(sample(32, 60), [255, 255, 255, 255]);
}

#[gpui::test]
fn rounded_overflow_empty_padding_edge_hides_descendants(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};
    let (pixels, _) = render_div_fixture(cx, || {
        let mut parent = gpui::div().size(px(64.)).rounded(px(24.)).overflow_hidden();
        parent.style().border_widths.left = Some(px(40.).into());
        parent.style().border_widths.right = Some(px(40.).into());
        gpui::div()
            .size(px(64.))
            .bg(gpui::white())
            .child(parent.child(gpui::div().size(px(64.)).bg(gpui::black())))
    });
    assert!(
        pixels
            .chunks_exact(4)
            .all(|pixel| pixel == [255, 255, 255, 255])
    );
}

pub(super) fn square_fixture() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};
    gpui::div().size(px(64.)).bg(gpui::white()).child(
        gpui::div()
            .size(px(64.))
            .rounded(px(24.))
            .overflow_hidden()
            .child(gpui::div().size(px(64.)).bg(gpui::black())),
    )
}

pub(super) fn sibling_fixture() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};
    gpui::div()
        .flex()
        .flex_wrap()
        .size(px(64.))
        .bg(gpui::white())
        .children((0..64).map(|_| {
            gpui::div()
                .size(px(8.))
                .rounded(px(3.))
                .overflow_hidden()
                .child(gpui::div().size(px(8.)).bg(gpui::black()))
        }))
}

#[gpui::test]
fn rounded_overflow_many_siblings_remain_independent(cx: &mut gpui::TestAppContext) {
    let (pixels, side) = render_div_fixture(cx, sibling_fixture);
    let cell = side / 8;
    for row in 0..8 {
        for col in 0..8 {
            assert_eq!(
                pixel(&pixels, side, col * cell, row * cell),
                [255, 255, 255, 255]
            );
            assert_eq!(
                pixel(&pixels, side, col * cell + cell / 2, row * cell + cell / 2),
                [0, 0, 0, 255]
            );
        }
    }
}

/// Exercise div layout and painting before rasterizing the complete scene.
/// A square child must not cover the cutout of its rounded overflow container.
#[gpui::test]
fn rounded_overflow_clips_square_child(cx: &mut gpui::TestAppContext) {
    let (pixels, side) = render_div_fixture(cx, square_fixture);
    assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
    assert_eq!(
        pixel(&pixels, side, 1, 1),
        [255, 255, 255, 255],
        "square child leaked outside rounded overflow container"
    );
}

#[gpui::test]
fn rounded_overflow_visible_preserves_square_child(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .size(px(64.))
                .rounded(px(24.))
                .child(gpui::div().size(px(64.)).bg(gpui::black())),
        )
    });
    assert_eq!(pixel(&pixels, side, 1, 1), [0, 0, 0, 255]);
}

/// The regression in zed#37480: an inset rectangular clip must not inherit
/// the radius of its parent. Its own corner lies inside the parent's shape.
#[gpui::test]
fn rounded_overflow_does_not_round_inset_child(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .size(px(64.))
                .rounded(px(24.))
                .overflow_hidden()
                .p(px(16.))
                .child(
                    gpui::div()
                        .size(px(32.))
                        .overflow_hidden()
                        .child(gpui::div().size(px(32.)).bg(gpui::black())),
                ),
        )
    });
    let inset = side / 4;
    assert_eq!(
        pixel(&pixels, side, inset + 1, inset + 1),
        [0, 0, 0, 255],
        "inset child acquired its ancestor's rounded corner"
    );
}

/// Cropping a tall rounded container must not move its bottom corners up
/// to the bottom edge of the shorter rectangular ancestor.
#[gpui::test]
fn rounded_overflow_cropped_child_keeps_original_corners(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div().w(px(64.)).h(px(32.)).overflow_hidden().child(
                gpui::div()
                    .w(px(64.))
                    .min_h(px(128.))
                    .rounded(px(24.))
                    .overflow_hidden()
                    .child(gpui::div().w(px(64.)).min_h(px(128.)).bg(gpui::black())),
            ),
        )
    });
    assert_eq!(
        pixel(&pixels, side, 1, side / 2 - 2),
        [0, 0, 0, 255],
        "ancestor clipping moved the child's bottom rounded corner"
    );
    assert_eq!(pixel(&pixels, side, 1, side / 2 + 2), [255, 255, 255, 255]);
    assert_eq!(pixel(&pixels, side, 1, 1), [255, 255, 255, 255]);
}

/// These siblings overlap, but their rounded masks belong to separate
/// branches. The later sibling cannot mask away the earlier one's pixels.
#[gpui::test]
fn rounded_overflow_overlapping_siblings_keep_independent_clips(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div()
            .relative()
            .size(px(64.))
            .bg(gpui::white())
            .child(
                gpui::div()
                    .absolute()
                    .size(px(48.))
                    .rounded(px(16.))
                    .overflow_hidden()
                    .child(gpui::div().size(px(48.)).bg(gpui::red())),
            )
            .child(
                gpui::div()
                    .absolute()
                    .left(px(24.))
                    .top(px(16.))
                    .size(px(32.))
                    .rounded(px(12.))
                    .overflow_hidden()
                    .child(gpui::div().size(px(32.)).bg(gpui::black())),
            )
    });
    assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
    let exposed = pixel(&pixels, side, side * 3 / 8 + 1, side / 4 + 1);
    assert!(
        exposed[0] > 200 && exposed[1] < 100 && exposed[2] < 100 && exposed[3] == 255,
        "later sibling's corner must reveal the earlier red sibling: {exposed:?}"
    );
    assert_eq!(pixel(&pixels, side, 1, 1), [255, 255, 255, 255]);
}

pub(super) fn render_with(
    cx: &mut gpui::TestAppContext,
    build: fn() -> gpui::Div,
    mut rasterize: impl FnMut(&gpui::Scene, gpui::Size<gpui::DevicePixels>) -> Vec<u8>,
) -> (Vec<u8>, usize) {
    use gpui::{AppContext as _, Context, IntoElement, Render};

    struct View(fn() -> gpui::Div);
    impl Render for View {
        fn render(&mut self, _: &mut gpui::Window, _: &mut Context<Self>) -> impl IntoElement {
            (self.0)()
        }
    }
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(|_| View(build)))
            .unwrap()
    });
    cx.update_window(window.into(), |_, window, cx| {
        window.draw(cx).clear(cx);
        let side = (64. * window.scale_factor()) as usize;
        let size = gpui::size(
            gpui::DevicePixels(side as i32),
            gpui::DevicePixels(side as i32),
        );
        (rasterize(window.painted_scene(), size), side)
    })
    .unwrap()
}

#[gpui::test]
fn rounded_overflow_identical_nested_clips_do_not_darken_edges(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (single, side) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .size(px(64.))
                .rounded(px(24.))
                .overflow_hidden()
                .child(gpui::div().size(px(64.)).bg(gpui::black())),
        )
    });
    let (nested, _) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .size(px(64.))
                .rounded(px(24.))
                .overflow_hidden()
                .child(
                    gpui::div()
                        .size(px(64.))
                        .rounded(px(24.))
                        .overflow_hidden()
                        .child(gpui::div().size(px(64.)).bg(gpui::black())),
                ),
        )
    });
    assert_same_pixels(&nested, &single, side);
}

#[gpui::test]
fn rounded_overflow_clips_paths(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div().size(px(64.)).bg(gpui::white()).child(
            gpui::div()
                .size(px(64.))
                .rounded(px(24.))
                .overflow_hidden()
                .child(
                    gpui::canvas(
                        |_, _, _| (),
                        |bounds, (), window, _| {
                            let mut path = Path::new(bounds.origin);
                            path.line_to(bounds.top_right());
                            path.line_to(bounds.bottom_right());
                            path.line_to(bounds.bottom_left());
                            path.line_to(bounds.origin);
                            window.paint_path(path, gpui::black());
                        },
                    )
                    .size(px(64.)),
                ),
        )
    });
    assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
    assert_eq!(pixel(&pixels, side, 1, 1), [255, 255, 255, 255]);
}

/// Disjoint path batches may share one intermediate rasterization group.
/// A clip mask drawn between them must not overwrite the later path's pixels
/// or leave that later path under the child's clip.
#[gpui::test]
fn rounded_overflow_between_path_batches_restores_path_pixels(cx: &mut gpui::TestAppContext) {
    use gpui::{ParentElement as _, Styled as _};

    let (pixels, side) = render_div_fixture(cx, || {
        gpui::div()
            .relative()
            .size(px(64.))
            .bg(gpui::white())
            .child(
                gpui::canvas(
                    |_, _, _| (),
                    |bounds, (), window, _| {
                        let start = bounds.origin;
                        let mut path = Path::new(start);
                        path.line_to(start + gpui::point(px(16.), px(0.)));
                        path.line_to(start + gpui::point(px(16.), px(16.)));
                        path.line_to(start + gpui::point(px(0.), px(16.)));
                        path.line_to(start);
                        window.paint_path(path, gpui::rgb(0xff0000));
                    },
                )
                .absolute()
                .size(px(64.)),
            )
            .child(
                gpui::div()
                    .absolute()
                    .size(px(64.))
                    .rounded(px(24.))
                    .overflow_hidden()
                    .child(gpui::div().size(px(64.)).bg(gpui::black())),
            )
            .child(
                gpui::canvas(
                    |_, _, _| (),
                    |bounds, (), window, _| {
                        let start = bounds.origin + gpui::point(px(48.), px(0.));
                        let mut path = Path::new(start);
                        path.line_to(start + gpui::point(px(16.), px(0.)));
                        path.line_to(start + gpui::point(px(16.), px(16.)));
                        path.line_to(start + gpui::point(px(0.), px(16.)));
                        path.line_to(start);
                        window.paint_path(path, gpui::rgb(0x00ff00));
                    },
                )
                .absolute()
                .size(px(64.)),
            )
    });
    assert_eq!(pixel(&pixels, side, 1, 1), [255, 0, 0, 255]);
    assert_eq!(pixel(&pixels, side, side - 2, 1), [0, 255, 0, 255]);
    assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
}

/// Change only the ancestor's clip while the child's rendered output remains
/// retained. The replayed child must not bring its previous parent's clip back.
pub(super) fn retained_frames(
    cx: &mut gpui::TestAppContext,
    mut rasterize: impl FnMut(&gpui::Scene, gpui::Size<gpui::DevicePixels>) -> Vec<u8>,
) {
    use gpui::{
        AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render, Styled as _,
    };
    use std::{cell::Cell, rc::Rc};
    struct Child(Rc<Cell<usize>>);
    impl Render for Child {
        fn render(&mut self, _: &mut gpui::Window, _: &mut Context<Self>) -> impl IntoElement {
            self.0.set(self.0.get() + 1);
            gpui::div().size(px(64.)).bg(gpui::black())
        }
    }
    struct Parent {
        radius: f32,
        child: Entity<Child>,
    }
    impl Render for Parent {
        fn render(&mut self, _: &mut gpui::Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().size(px(64.)).bg(gpui::white()).child(
                gpui::div()
                    .size(px(64.))
                    .rounded(px(self.radius))
                    .overflow_hidden()
                    .child(self.child.clone().cached(gpui::StyleRefinement::default())),
            )
        }
    }
    let renders = Rc::new(Cell::new(0));
    let window = cx.update(|cx| {
        let child = cx.new(|_| Child(renders.clone()));
        cx.open_window(Default::default(), |_, cx| {
            cx.new(|_| Parent { radius: 24., child })
        })
        .unwrap()
    });
    for radius in [24., 0., 24.] {
        window
            .update(cx, |parent, _, cx| {
                parent.radius = radius;
                cx.notify();
            })
            .unwrap();
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let side = (64. * window.scale_factor()) as usize;
            let extent = gpui::size(
                gpui::DevicePixels(side as i32),
                gpui::DevicePixels(side as i32),
            );
            let pixels = rasterize(window.painted_scene(), extent);
            let corner = if radius == 0. {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };
            assert_eq!(pixel(&pixels, side, 1, 1), corner);
            assert_eq!(pixel(&pixels, side, side / 2, side / 2), [0, 0, 0, 255]);
        })
        .unwrap();
    }
    assert_eq!(
        renders.get(),
        1,
        "child must remain retained while ancestor clip changes"
    );
}
