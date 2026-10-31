use std::{fmt, rc::Rc, time::Duration};

use gpui::{
    Background, BenchAppContext, Bounds, Context, IntoElement, Pixels, Render, Scene, Styled,
    Window, canvas, fill, hsla, point, px, size,
};

#[derive(Clone, Copy)]
enum Pattern {
    Grid,
    Stripes,
    Overlap,
    Nested,
    Clusters,
    Overlay,
}

impl Pattern {
    fn name(self) -> &'static str {
        match self {
            Self::Grid => "grid",
            Self::Stripes => "stripes",
            Self::Overlap => "overlap",
            Self::Nested => "nested",
            Self::Clusters => "clusters",
            Self::Overlay => "overlay",
        }
    }
}

struct Case {
    pattern: Pattern,
    count: usize,
}

impl fmt::Display for Case {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}-{}", self.pattern.name(), self.count)
    }
}

fn cases() -> Vec<Case> {
    [256, 1_024, 4_096]
        .into_iter()
        .flat_map(|count| {
            [
                Pattern::Grid,
                Pattern::Stripes,
                Pattern::Overlap,
                Pattern::Nested,
                Pattern::Clusters,
                Pattern::Overlay,
            ]
            .into_iter()
            .map(move |pattern| Case { pattern, count })
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Rectangle {
    bounds: Bounds<Pixels>,
    background: Background,
}

fn rectangles(case: &Case, extent: f32) -> Rc<[Rectangle]> {
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
                Pattern::Overlay => Bounds::new(grid_origin, size(px(cell * 0.4), px(cell * 0.4))),
            };
            Rectangle {
                bounds,
                background: hsla(index as f32 / case.count as f32, 1., 0.5, 1.).into(),
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
                    window.paint_quad(fill(rectangle.bounds, rectangle.background));
                }
            },
        )
        .size_full()
    }
}

fn build_scene(scene: &mut Scene, rectangles: &[Rectangle]) {
    scene.clear();
    let mask = gpui::ContentMask {
        bounds: Bounds::new(point(px(0.), px(0.)), size(px(1_024.), px(1_024.))).scale(1.),
    };
    for rectangle in rectangles {
        scene.insert_primitive(gpui::Quad {
            order: 0,
            border_style: Default::default(),
            bounds: rectangle.bounds.scale(1.),
            content_mask: mask,
            background: rectangle.background,
            border_color: Default::default(),
            corner_radii: Default::default(),
            border_widths: Default::default(),
        });
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
            // --list and a paint-only filter must not build all 18 scenes.
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
