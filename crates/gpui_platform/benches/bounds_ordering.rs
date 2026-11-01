//! Painter-order assignment (`BoundsTree`) and `Scene::finish` for many primitives.
//!
//! The `random` cases are the general workload: seeded rectangles of every size, scattered
//! and overlapping, painted as a mix of quads, shadows and underlines. `SEED` and
//! `ITERATIONS` choose the seeds as for `#[gpui::test]`. The fixed patterns isolate the
//! geometry that decides an ordering structure's best and worst cases, such as many
//! primitives overlapping one another (`overlap`, `nested`) or sharing one axis
//! (`stripes`), so a change in `random` can be traced to the shape that caused it.

use std::{fmt, rc::Rc, time::Duration};

use gpui::{
    Background, BenchAppContext, Bounds, BoxShadow, Context, Corners, Hsla, IntoElement, Pixels,
    Render, Scene, Styled, UnderlineStyle, Window, canvas, fill, hsla, point,
    private::rand::{Rng as _, SeedableRng as _, rngs::StdRng},
    px, size,
};

#[derive(Clone, Copy)]
enum Pattern {
    Grid,
    Stripes,
    Overlap,
    Nested,
    Clusters,
    Overlay,
    Random { seed: u64 },
}

struct Case {
    pattern: Pattern,
    count: usize,
}

impl fmt::Display for Case {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.pattern {
            Pattern::Grid => "grid",
            Pattern::Stripes => "stripes",
            Pattern::Overlap => "overlap",
            Pattern::Nested => "nested",
            Pattern::Clusters => "clusters",
            Pattern::Overlay => "overlay",
            Pattern::Random { seed } => {
                return write!(formatter, "random-{}-seed-{seed}", self.count);
            }
        };
        write!(formatter, "{name}-{}", self.count)
    }
}

fn cases() -> Vec<Case> {
    let (seeds, _) = gpui::calculate_seeds(2, &[]);
    let seeds = seeds.collect::<Vec<_>>();
    [256, 1_024, 4_096]
        .into_iter()
        .flat_map(|count| {
            seeds
                .iter()
                .map(|&seed| Pattern::Random { seed })
                .chain([
                    Pattern::Grid,
                    Pattern::Stripes,
                    Pattern::Overlap,
                    Pattern::Nested,
                    Pattern::Clusters,
                    Pattern::Overlay,
                ])
                .map(move |pattern| Case { pattern, count })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Kind {
    Quad,
    Shadow,
    Underline,
}

#[derive(Clone, Copy)]
struct Rectangle {
    bounds: Bounds<Pixels>,
    background: Background,
    color: Hsla,
    kind: Kind,
}

fn random_rectangles(seed: u64, count: usize, extent: f32) -> Rc<[Rectangle]> {
    let mut rng = StdRng::seed_from_u64(seed);
    // Log-uniform sizes: mostly small primitives, with a long tail of large ones that
    // overlap many others, as in a real window.
    let (smallest, largest) = (2f32.ln(), (extent * 0.6).ln());
    (0..count)
        .map(|_| {
            let width = rng.random_range(smallest..largest).exp();
            let height = rng.random_range(smallest..largest).exp();
            let origin = point(
                px(32. + rng.random_range(0.0..extent - width)),
                px(32. + rng.random_range(0.0..extent - height)),
            );
            let color = hsla(rng.random_range(0.0..1.0), 0.8, 0.5, 1.);
            let kind = match rng.random_range(0..20) {
                0..14 => Kind::Quad,
                14..17 => Kind::Shadow,
                _ => Kind::Underline,
            };
            Rectangle {
                bounds: Bounds::new(origin, size(px(width), px(height))),
                background: color.into(),
                color,
                kind,
            }
        })
        .collect::<Vec<_>>()
        .into()
}

fn rectangles(case: &Case, extent: f32) -> Rc<[Rectangle]> {
    if let Pattern::Random { seed } = case.pattern {
        return random_rectangles(seed, case.count, extent);
    }
    let columns = (case.count as f32).sqrt().ceil() as usize;
    let cell = extent / columns as f32;
    let stripe_count = (extent / 4.).floor() as usize;
    (0..case.count)
        .map(|index| {
            let grid_origin = point(
                px(32. + (index % columns) as f32 * cell),
                px(32. + (index / columns) as f32 * cell),
            );
            let bounds = match case.pattern {
                Pattern::Grid => Bounds::new(grid_origin, size(px(cell * 0.4), px(cell * 0.4))),
                Pattern::Stripes => Bounds::new(
                    point(px(32.), px(32. + (index % stripe_count) as f32 * 4.)),
                    size(px(extent), px(2.)),
                ),
                Pattern::Overlap => {
                    Bounds::new(point(px(32.), px(32.)), size(px(extent), px(extent)))
                }
                Pattern::Nested => {
                    let inset = extent * 0.3 * index as f32 / case.count as f32;
                    Bounds::new(
                        point(px(32. + inset), px(32. + inset)),
                        size(px(extent - 2. * inset), px(extent - 2. * inset)),
                    )
                }
                Pattern::Clusters => {
                    let origin = 32. + (index % 2) as f32 * extent * 0.55;
                    Bounds::new(
                        point(px(origin), px(origin)),
                        size(px(extent * 0.4), px(extent * 0.4)),
                    )
                }
                Pattern::Overlay if index % 64 == 0 => {
                    Bounds::new(point(px(32.), px(32.)), size(px(extent), px(extent)))
                }
                Pattern::Overlay | Pattern::Random { .. } => {
                    Bounds::new(grid_origin, size(px(cell * 0.4), px(cell * 0.4)))
                }
            };
            let color = hsla(index as f32 / case.count as f32, 1., 0.5, 1.);
            Rectangle {
                bounds,
                background: color.into(),
                color,
                kind: Kind::Quad,
            }
        })
        .collect::<Vec<_>>()
        .into()
}

struct OrderingView {
    rectangles: Rc<[Rectangle]>,
}

impl Render for OrderingView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let rectangles = self.rectangles.clone();
        canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                for rectangle in rectangles.iter() {
                    paint(rectangle, window);
                }
            },
        )
        .size_full()
    }
}

fn paint(rectangle: &Rectangle, window: &mut Window) {
    match rectangle.kind {
        Kind::Quad => window.paint_quad(fill(rectangle.bounds, rectangle.background)),
        Kind::Shadow => window.paint_drop_shadows(
            rectangle.bounds,
            Corners::all(px(4.)),
            &[BoxShadow {
                color: rectangle.color,
                offset: point(px(0.), px(2.)),
                blur_radius: px(6.),
                spread_radius: px(0.),
                inset: false,
            }],
        ),
        Kind::Underline => window.paint_underline(
            rectangle.bounds.origin,
            rectangle.bounds.size.width,
            &UnderlineStyle {
                thickness: rectangle.bounds.size.height.min(px(4.)),
                color: Some(rectangle.color),
                wavy: false,
            },
        ),
    }
}

fn build_scene(scene: &mut Scene, rectangles: &[Rectangle]) {
    scene.clear();
    let content_mask = gpui::ContentMask {
        bounds: Bounds::new(point(px(0.), px(0.)), size(px(1_024.), px(1_024.))).scale(1.),
    };
    for rectangle in rectangles {
        let bounds = rectangle.bounds.scale(1.);
        match rectangle.kind {
            Kind::Quad => scene.insert_primitive(gpui::Quad {
                order: 0,
                border_style: Default::default(),
                bounds,
                content_mask,
                background: rectangle.background,
                border_color: Default::default(),
                corner_radii: Default::default(),
                border_widths: Default::default(),
            }),
            Kind::Shadow => scene.insert_primitive(gpui::Shadow {
                order: 0,
                blur_radius: gpui::ScaledPixels(6.),
                bounds,
                corner_radii: Corners::all(gpui::ScaledPixels(4.)),
                content_mask,
                color: rectangle.color,
                element_bounds: bounds,
                element_corner_radii: Corners::all(gpui::ScaledPixels(4.)),
                inset: 0,
                pad: 0,
            }),
            Kind::Underline => scene.insert_primitive(gpui::Underline {
                order: 0,
                pad: 0,
                bounds,
                content_mask,
                color: rectangle.color,
                thickness: bounds.size.height.min(gpui::ScaledPixels(4.)),
                wavy: Default::default(),
            }),
        }
    }
    scene.finish();
}

fn scene_ordering(criterion: &mut criterion::Criterion<gpui::BenchMeasurement>) {
    let mut group = criterion.benchmark_group("BoundsOrdering/scene");
    for case in cases() {
        let mut fixture = None;
        let report = bench_metrics::MetricReport::new();
        group.bench_function(case.to_string(), |bencher| {
            // Criterion invokes this closure only for selected cases. In particular,
            // --list and a paint-only filter must not build every scene.
            let (rectangles, scene) =
                fixture.get_or_insert_with(|| (rectangles(&case, 512.), Scene::default()));
            report.iter(bencher, || {
                build_scene(scene, rectangles);
                std::hint::black_box(&*scene);
            });
        });
        report.print("  ");
        if std::env::var("GPUI_BENCH_SCENE_STATS").as_deref() == Ok("1")
            && let Some((_, scene)) = &fixture
        {
            eprintln!(
                "scene stats BoundsOrdering/scene/{case}: batches={}",
                scene.batches().count()
            );
        }
    }
    group.finish();
}

#[gpui::bench(inputs = cases(), input_name = "case", group = "BoundsOrdering/paint", fps = 120)]
fn paint_rectangles(case: &Case, cx: &mut BenchAppContext) {
    let mut window = cx.add_empty_window();
    let extent = window.update(|window, _| {
        let viewport = window.viewport_size();
        (viewport.width.as_f32().min(viewport.height.as_f32()) - 64.).min(512.)
    });
    let rectangles = rectangles(case, extent);
    let root =
        window.update(|window, cx| window.replace_root(cx, |_, _| OrderingView { rectangles }));
    cx.run_until_idle();
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    cx.bench_renderer(root, |_, _, cx| cx.notify());
    window.update(|window, _| {
        if std::env::var("GPUI_BENCH_SCENE_STATS").as_deref() == Ok("1") {
            eprintln!(
                "scene stats BoundsOrdering/paint/{case}: batches={}",
                window.rendered_scene().batches().count(),
            );
        }
    });
    window.update(|window, _| window.remove_window());
    cx.run_until_idle();
}

gpui::bench_group!(
    name = benches;
    config = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1))
        .sample_size(20);
    targets = paint_rectangles, scene_ordering
);
gpui::bench_main!(benches);
