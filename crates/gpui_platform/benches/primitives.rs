//! Painting many primitives: draw-order assignment (`BoundsTree`), `Scene::finish`
//! sorting and batching, and, through a window, everything from paint to GPU submission.
//!
//! The `random` cases are the general workload: seeded rectangles of every size,
//! scattered and overlapping, painted as a mix of quads, shadows, underlines, filled paths
//! and text. `SEED` and `ITERATIONS` choose the seeds as for `#[gpui::test]`. The fixed
//! patterns isolate the geometry that decides an ordering structure's best and worst
//! cases, such as many primitives overlapping one another (`overlap`, `nested`) or sharing
//! one axis (`stripes`), so a change in `random` can be traced to the shape that caused it.

use std::{fmt, rc::Rc, time::Duration};

use gpui::{
    App, AtlasTextureId, AtlasTextureKind, AtlasTile, Background, BenchAppContext, Bounds,
    BoxShadow, ContentMask, Context, Corners, DevicePixels, Hsla, IntoElement, PaintedPath, Path,
    PathBuilder, Pixels, Render, Scene, SharedString, Styled, TextAlign, TextRun, TileId,
    TransformationMatrix, UnderlineStyle, Window, canvas, fill, hsla, point,
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

#[derive(Clone)]
enum Kind {
    Quad,
    Shadow,
    Underline,
    /// Built and binned before timing, since binning would otherwise dominate what these
    /// cases are for. The window paints `path`, and the scene takes a copy of `painted`.
    Path {
        path: Path,
        painted: PaintedPath,
    },
    Text(SharedString),
}

#[derive(Clone)]
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
            let bounds = Bounds::new(origin, size(px(width), px(height)));
            let color = hsla(rng.random_range(0.0..1.0), 0.8, 0.5, 1.);
            let kind = match rng.random_range(0..20) {
                0..9 => Kind::Quad,
                9..11 => Kind::Shadow,
                11..13 => Kind::Underline,
                13..16 => triangle(bounds).map_or(Kind::Quad, |path| Kind::Path {
                    painted: PaintedPath::new(path.clone(), 1., scene_content_mask(), color.into()),
                    path,
                }),
                _ => {
                    let length = rng.random_range(1..12);
                    Kind::Text("Ag3 Wq7 fixture"[..length].to_string().into())
                }
            };
            Rectangle {
                bounds,
                background: color.into(),
                color,
                kind,
            }
        })
        .collect::<Vec<_>>()
        .into()
}

fn triangle(bounds: Bounds<Pixels>) -> Option<Path> {
    let mut path = PathBuilder::fill();
    path.move_to(bounds.origin);
    path.line_to(bounds.top_right());
    path.line_to(bounds.bottom_left() + point(bounds.size.width / 2., px(0.)));
    path.close();
    path.build().ok()
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
            move |_, _, window, cx| {
                for rectangle in rectangles.iter() {
                    paint(rectangle, window, cx);
                }
            },
        )
        .size_full()
    }
}

fn paint(rectangle: &Rectangle, window: &mut Window, cx: &mut App) {
    match &rectangle.kind {
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
        Kind::Path { path, .. } => window.paint_path(path.clone(), rectangle.color),
        // Shaped every frame, as the text element does; the line layout cache makes
        // repeated shaping a lookup.
        Kind::Text(text) => {
            let font_size = rectangle.bounds.size.height.clamp(px(8.), px(32.));
            let run = TextRun {
                len: text.len(),
                font: window.text_style().font(),
                color: rectangle.color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let line = window
                .text_system()
                .shape_line(text.clone(), font_size, &[run], None);
            if let Err(error) = line.paint(
                rectangle.bounds.origin,
                font_size,
                TextAlign::Left,
                None,
                window,
                cx,
            ) {
                eprintln!("failed to paint text: {error}");
            }
        }
    }
}

/// Stands in for one glyph's atlas tile. Spread over a few textures, as glyphs of
/// different sizes and fonts are, so batching by texture is exercised.
fn glyph_tile(index: usize) -> AtlasTile {
    AtlasTile {
        texture_id: AtlasTextureId {
            index: (index % 3) as u32,
            kind: AtlasTextureKind::Monochrome,
        },
        tile_id: TileId(index as u32),
        padding: 0,
        bounds: Bounds::new(
            point(DevicePixels(0), DevicePixels(0)),
            size(DevicePixels(16), DevicePixels(16)),
        ),
    }
}

fn scene_content_mask() -> ContentMask<Pixels> {
    ContentMask {
        bounds: Bounds::new(point(px(0.), px(0.)), size(px(1_024.), px(1_024.))),
    }
}

fn build_scene(scene: &mut Scene, rectangles: &[Rectangle]) {
    insert_primitives(scene, rectangles);
    scene.finish();
}

fn insert_primitives(scene: &mut Scene, rectangles: &[Rectangle]) {
    scene.clear();
    let content_mask = scene_content_mask().scale(1.);
    for (index, rectangle) in rectangles.iter().enumerate() {
        let bounds = rectangle.bounds.scale(1.);
        match &rectangle.kind {
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
                bounds: bounds.dilate(gpui::ScaledPixels(18.)),
                corner_radii: Corners::default(),
                content_mask,
                color: rectangle.color,
                shape_bounds: bounds,
                shape_corner_radii: Corners::all(gpui::ScaledPixels(4.)),
                inset: false.into(),
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
            Kind::Path { painted, .. } => scene.insert_primitive(painted.clone()),
            Kind::Text(text) => {
                let glyph_size = bounds.size.height.min(gpui::ScaledPixels(32.));
                for glyph in 0..text.len() {
                    let origin = bounds.origin
                        + point(glyph_size * (glyph as f32 * 0.6), gpui::ScaledPixels(0.));
                    scene.insert_primitive(gpui::MonochromeSprite {
                        order: 0,
                        pad: 0,
                        tile: glyph_tile(index + glyph),
                        bounds: Bounds::new(origin, size(glyph_size * 0.6, glyph_size)),
                        content_mask,
                        color: rectangle.color,
                        transformation: TransformationMatrix::unit(),
                    });
                }
            }
        }
    }
}

fn scene_finish(criterion: &mut criterion::Criterion<gpui::BenchMeasurement>) {
    let mut group = criterion.benchmark_group("Primitives/finish");
    for case in cases() {
        let mut fixture = None;
        group.bench_function(case.to_string(), |bencher| {
            let rectangles = fixture.get_or_insert_with(|| rectangles(&case, 512.));
            bencher.iter_batched_ref(
                || {
                    let mut scene = Scene::default();
                    insert_primitives(&mut scene, rectangles);
                    scene
                },
                |scene| scene.finish(),
                criterion::BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

fn scene_ordering(criterion: &mut criterion::Criterion<gpui::BenchMeasurement>) {
    let mut group = criterion.benchmark_group("Primitives/scene");
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
                "scene stats Primitives/scene/{case}: batches={}",
                scene.batches().count()
            );
        }
    }
    group.finish();
}

#[gpui::bench(inputs = cases(), input_name = "case", group = "Primitives/paint", fps = 120)]
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
                "scene stats Primitives/paint/{case}: batches={}",
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
    targets = paint_rectangles, scene_ordering, scene_finish
);
gpui::bench_main!(benches);
