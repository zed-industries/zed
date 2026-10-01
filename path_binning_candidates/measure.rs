//! Binner measurements on the painting example's fixtures and Zed-shaped ones.
//! `hooks.patch` mounts this as `gpui::path_binning_measure`; see README.md.
//!
//! - `measure`: per fixture, the whole build and the binning alone (median µs),
//!   then tiles, device pixels shaded and curves visited. Binning and shading
//!   are at the display scale `PATH_SCALE` (default 1.5).
//! - `profile`: bins one fixture (`FIXTURE`) in a loop for `PROFILE_SECONDS`,
//!   so a sampling profiler sees mostly binning.

use crate::{
    Bounds, LineJoin, Path, PathBuilder, PathStyle, PathTiles, Pixels, Point, StrokeOptions,
    point, px, size,
};
use std::time::{Duration, Instant};

static SCALE: std::sync::LazyLock<f32> = std::sync::LazyLock::new(|| {
    std::env::var("PATH_SCALE")
        .ok()
        .and_then(|scale| scale.parse().ok())
        .unwrap_or(1.5)
});

/// The scale to bin for, `PATH_BIN_SCALE`, which defaults to `SCALE`. Binning
/// for 1.5 and shading at another scale reproduces the old fixed-scale binner.
static BIN_SCALE: std::sync::LazyLock<f32> = std::sync::LazyLock::new(|| {
    std::env::var("PATH_BIN_SCALE")
        .ok()
        .and_then(|scale| scale.parse().ok())
        .unwrap_or(*SCALE)
});

fn square(left: f32) -> PathBuilder {
    let mut builder = PathBuilder::fill();
    builder.move_to(point(px(left), px(50.)));
    builder.line_to(point(px(left + 80.), px(50.)));
    builder.line_to(point(px(left + 80.), px(130.)));
    builder.line_to(point(px(left), px(130.)));
    builder.close();
    builder
}

fn circle(center: Point<Pixels>, radius: Pixels) -> PathBuilder {
    let mut builder = PathBuilder::fill();
    builder.move_to(point(center.x + radius, center.y));
    builder.arc_to(point(radius, radius), px(0.), false, false, point(center.x - radius, center.y));
    builder.arc_to(point(radius, radius), px(0.), false, false, point(center.x + radius, center.y));
    builder.close();
    builder
}

fn rust_logo() -> PathBuilder {
    let mut builder = lyon_path::Path::svg_builder();
    lyon_extra::rust_logo::build_logo_path(&mut builder);
    let mut builder: PathBuilder = builder.into();
    builder.translate(point(px(10.), px(200.)));
    builder.scale(0.9);
    builder
}

fn polygon(points: &[(f32, f32)], close: bool) -> PathBuilder {
    let mut builder = PathBuilder::fill();
    let points: Vec<_> = points.iter().map(|&(x, y)| point(px(x), px(y))).collect();
    builder.add_polygon(&points, close);
    builder
}

fn star() -> PathBuilder {
    let mut builder = PathBuilder::fill();
    builder.move_to(point(px(350.), px(200.)));
    for (x, y) in [
        (370., 260.),
        (430., 260.),
        (380., 300.),
        (400., 360.),
        (350., 320.),
        (300., 360.),
        (320., 300.),
        (270., 260.),
        (330., 260.),
        (350., 200.),
    ] {
        builder.line_to(point(px(x), px(y)));
    }
    builder
}

fn gradient_shape() -> PathBuilder {
    let bounds = Bounds {
        origin: point(px(450.), px(200.)),
        size: size(px(200.), px(80.)),
    };
    let horizontal_offset = bounds.size.height;
    let vertical_offset = px(30.);
    let mut builder = PathBuilder::fill();
    builder.move_to(bounds.bottom_left());
    builder.curve_to(
        bounds.origin + point(horizontal_offset, vertical_offset),
        bounds.origin + point(px(0.0), vertical_offset),
    );
    builder.line_to(bounds.top_right() + point(-horizontal_offset, vertical_offset));
    builder.curve_to(
        bounds.bottom_right(),
        bounds.top_right() + point(px(0.0), vertical_offset),
    );
    builder.line_to(bounds.bottom_left());
    builder
}

fn pie() -> Vec<PathBuilder> {
    let radius = point(px(96.), px(96.));
    let pie_center = point(px(775.), px(255.));
    [
        ((871., 255.), (747., 163.)),
        ((747., 163.), (679., 263.)),
        ((679., 263.), (754., 349.)),
        ((754., 349.), (854., 310.)),
        ((854., 310.), (871., 255.)),
    ]
    .into_iter()
    .map(|(start, end)| {
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(start.0), px(start.1)));
        builder.arc_to(radius, px(0.), false, false, point(px(end.0), px(end.1)));
        builder.line_to(pie_center);
        builder.close();
        builder
    })
    .collect()
}

fn wave() -> PathBuilder {
    let options = StrokeOptions {
        line_width: 1.,
        line_join: LineJoin::Bevel,
        ..Default::default()
    };
    let mut builder = PathBuilder::stroke(px(1.)).with_style(PathStyle::Stroke(options));
    builder.move_to(point(px(40.), px(420.)));
    for i in 1..50 {
        builder.line_to(point(
            px(40.0 + i as f32 * 10.0),
            px(420.0 + (i as f32 * 10.0).sin() * 40.0),
        ));
    }
    builder
}

/// A polygon whose corners are rounded the way the editor rounds
/// highlighted ranges: a quadratic curve through each corner.
fn rounded(points: &[(f32, f32)], radius: f32) -> PathBuilder {
    let mut builder = PathBuilder::fill();
    let count = points.len();
    let toward = |from: (f32, f32), to: (f32, f32)| {
        let (dx, dy) = (to.0 - from.0, to.1 - from.1);
        let length = (dx * dx + dy * dy).sqrt();
        let distance = radius.min(length / 2.0);
        point(px(from.0 + dx / length * distance), px(from.1 + dy / length * distance))
    };
    for index in 0..count {
        let corner = points[index];
        let previous = points[(index + count - 1) % count];
        let next = points[(index + 1) % count];
        let entry = toward(corner, previous);
        if index == 0 {
            builder.move_to(entry);
        } else {
            builder.line_to(entry);
        }
        builder.curve_to(toward(corner, next), point(px(corner.0), px(corner.1)));
    }
    builder.close();
    builder
}

/// An editor selection over lines spanning `(start, end)` columns, with
/// Zed's 20 px lines and 0.15 line-height corner radius.
fn editor_selection(lines: &[(f32, f32)]) -> PathBuilder {
    let line_height = 20.0;
    let top = 100.0;
    let mut outline = Vec::new();
    for (index, &(_, end)) in lines.iter().enumerate() {
        let y = top + index as f32 * line_height;
        outline.push((end, y));
        outline.push((end, y + line_height));
    }
    for (index, &(start, _)) in lines.iter().enumerate().rev() {
        let y = top + index as f32 * line_height;
        outline.push((start, y + line_height));
        outline.push((start, y));
    }
    outline.dedup();
    rounded(&outline, 0.15 * line_height)
}

/// Git graph lanes as `git_graph.rs` draws them: 1.5 px strokes, each
/// straight segment its own subpath, with a curve between lanes.
fn git_lanes() -> Vec<PathBuilder> {
    let row_height = 24.0;
    (0..8)
        .map(|lane| {
            let x = 20.0 + lane as f32 * 16.0;
            let mut builder = PathBuilder::stroke(px(1.5));
            let mut y = 12.0 + 3.5;
            builder.move_to(point(px(x), px(y)));
            for row in 0..30 {
                if row % 7 == 3 && lane > 0 {
                    let to_y = y + row_height;
                    builder.curve_to(
                        point(px(x - 16.0), px(to_y)),
                        point(px(x), px(to_y)),
                    );
                    builder.move_to(point(px(x - 16.0), px(to_y)));
                    builder.line_to(point(px(x), px(to_y)));
                    builder.move_to(point(px(x), px(to_y)));
                    y = to_y;
                } else {
                    y += row_height;
                    builder.line_to(point(px(x), px(y)));
                    builder.move_to(point(px(x), px(y)));
                }
            }
            builder
        })
        .collect()
}

fn git_nodes() -> Vec<PathBuilder> {
    (0..30)
        .map(|row| {
            let lane = (row * 5) % 8;
            circle(
                point(px(20.0 + lane as f32 * 16.0), px(12.0 + row as f32 * 24.0)),
                px(3.5),
            )
        })
        .collect()
}

fn divider() -> PathBuilder {
    let mut builder = PathBuilder::stroke(px(1.)).dash_array(&[px(4.), px(2.)]);
    builder.move_to(point(px(20.), px(300.5)));
    builder.line_to(point(px(420.), px(300.5)));
    builder
}

fn strokes(lines: Vec<Vec<Point<Pixels>>>, dashed: bool) -> Vec<PathBuilder> {
    lines
        .into_iter()
        .map(|points| {
            let mut builder = PathBuilder::stroke(px(1.));
            if dashed {
                builder = builder.dash_array(&[px(4.), px(2.)]);
            }
            for (index, point) in points.into_iter().enumerate() {
                if index == 0 {
                    builder.move_to(point);
                } else {
                    builder.line_to(point);
                }
            }
            builder
        })
        .collect()
}

fn scribble_lines() -> Vec<Vec<Point<Pixels>>> {
    let mut state: u32 = 0x9E3779B9;
    let mut random = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state as f32 / u32::MAX as f32
    };
    let mut lines = Vec::new();
    for _ in 0..30 {
        let mut x = 40.0 + random() * 860.0;
        let mut y = 80.0 + random() * 440.0;
        let mut angle = random() * std::f32::consts::TAU;
        let mut points = vec![point(px(x), px(y))];
        for _ in 0..120 {
            angle += (random() - 0.5) * 1.2;
            x += angle.cos() * 12.0;
            y += angle.sin() * 12.0;
            if !(40.0..=900.0).contains(&x) {
                angle = std::f32::consts::PI - angle;
                x = x.clamp(40.0, 900.0);
            }
            if !(80.0..=520.0).contains(&y) {
                angle = -angle;
                y = y.clamp(80.0, 520.0);
            }
            points.push(point(px(x), px(y)));
        }
        lines.push(points);
    }
    lines
}

fn grid_lines() -> Vec<Vec<Point<Pixels>>> {
    let mut lines = Vec::new();
    for i in 0..55 {
        let y = 80.0 + i as f32 * 8.0;
        lines.push(vec![point(px(40.0), px(y)), point(px(900.0), px(y))]);
    }
    for i in 0..108 {
        let x = 40.0 + i as f32 * 8.0;
        lines.push(vec![point(px(x), px(80.0)), point(px(x), px(520.0))]);
    }
    lines
}

/// Each fixture builds its paths afresh, since building consumes them.
pub(crate) fn fixtures() -> Vec<(&'static str, fn() -> Vec<PathBuilder>)> {
    vec![
        ("zed: selection, 1 line", || vec![editor_selection(&[(120., 360.)])]),
        ("zed: selection, 3 lines", || {
            vec![editor_selection(&[(300., 700.), (100., 700.), (100., 450.)])]
        }),
        ("zed: selection, 40 lines", || {
            let mut lines = vec![(300., 760.)];
            lines.extend((0..38).map(|line| (100., 500. + (line % 9) as f32 * 30.)));
            lines.push((100., 420.));
            vec![editor_selection(&lines)]
        }),
        ("zed: git graph nodes (30)", git_nodes),
        ("zed: git graph lanes (8)", git_lanes),
        ("zed: dashed divider", || vec![divider()]),
        ("4 squares", || [40., 150., 250., 350.].map(square).into()),
        ("2 circles", || {
            [530., 570.]
                .map(|x| circle(point(px(x), px(85.)), px(30.)))
                .into()
        }),
        ("rust logo", || vec![rust_logo()]),
        ("bolt", || {
            vec![polygon(
                &[(150., 300.), (200., 225.), (200., 275.), (250., 200.)],
                false,
            )]
        }),
        ("star", || vec![star()]),
        ("gradient shape", || vec![gradient_shape()]),
        ("5 pie slices", pie),
        ("full-screen selection", || {
            vec![rounded(
                &[(100., 40.), (904., 40.), (904., 604.), (100., 604.)],
                4.,
            )]
        }),
        ("wave", || vec![wave()]),
        ("1 hline", || strokes(vec![grid_lines()[0].clone()], false)),
        ("1 vline", || strokes(vec![grid_lines()[60].clone()], false)),
        ("grid", || strokes(grid_lines(), false)),
        ("grid dashed", || strokes(grid_lines(), true)),
        ("scribble", || strokes(scribble_lines(), false)),
        ("scribble dashed", || strokes(scribble_lines(), true)),
    ]
}

/// Median time per call of `run`, in microseconds.
pub(crate) fn time(mut run: impl FnMut()) -> f64 {
    let start = Instant::now();
    let mut iterations = 0u32;
    while start.elapsed().as_millis() < 150 {
        run();
        iterations += 1;
    }
    let batch = (iterations / 15).max(1);
    let mut samples: Vec<f64> = (0..31)
        .map(|_| {
            let start = Instant::now();
            for _ in 0..batch {
                run();
            }
            start.elapsed().as_secs_f64() * 1e6 / batch as f64
        })
        .collect();
    samples.sort_by(|a, b| a.total_cmp(b));
    samples[samples.len() / 2]
}

/// Tiles, device pixels shaded, and curves visited at `SCALE`, emulating
/// the shader's early exit.
fn shader_work(path: &PathTiles) -> (usize, u64, u64) {
    let scale = *SCALE;
    let mut pixels = 0;
    let mut visits = 0;
    let half_pixel = 0.5 / scale;
    for tile in path.tiles() {
        let left = (tile.top_left.x * scale - 0.5).ceil() as i64;
        let right = (tile.bottom_right.x * scale - 0.5).ceil() as i64;
        let top = (tile.top_left.y * scale - 0.5).ceil() as i64;
        let bottom = (tile.bottom_right.y * scale - 0.5).ceil() as i64;
        let start = tile.curve_start as usize;
        let curves = &path.curves()[start..start + tile.curve_count as usize];
        for y in top..bottom {
            for x in left..right {
                pixels += 1;
                let center = point((x as f32 + 0.5) / scale, (y as f32 + 0.5) / scale);
                for curve in curves {
                    if curve.p0.x.min(curve.p1.x) >= center.x + half_pixel + 2.0 / 256.0 {
                        break;
                    }
                    visits += 1;
                }
            }
        }
    }
    (path.tiles().len(), pixels, visits)
}

#[test]
#[ignore]
fn measure() {
    let filter = std::env::var("FIXTURES").unwrap_or_default();
    println!("fixture\tcurves\tbuild_us\tbin_us\ttiles\tpixels\tvisits");
    for (name, make) in fixtures() {
        if !filter.is_empty() && !name.contains(filter.as_str()) {
            continue;
        }
        let outlines: Vec<_> = make().into_iter().map(|builder| builder.outline()).collect();
        let curves: usize = outlines.iter().map(|(curves, _)| curves.len()).sum();
        let build = time(|| {
            for builder in make() {
                std::hint::black_box(builder.build().ok());
            }
        });
        let paths: Vec<_> = outlines
            .iter()
            .map(|(curves, fill_rule)| Path::from_curves(curves.clone(), *fill_rule))
            .collect();
        let bin = time(|| {
            for path in &paths {
                std::hint::black_box(path.bin(*BIN_SCALE));
            }
        });
        let mut work = (0, 0, 0);
        for path in &paths {
            let (tiles, pixels, visits) = shader_work(&path.bin(*BIN_SCALE));
            work.0 += tiles;
            work.1 += pixels;
            work.2 += visits;
        }
        println!(
            "{name}\t{curves}\t{build:.2}\t{bin:.2}\t{}\t{}\t{}",
            work.0, work.1, work.2
        );
    }
}

/// Per fixture, actual listings and shader visits against the cost model's
/// estimates of them, at whatever cell size the binner picks (force one with
/// `PATH_SWEEP`'s fourth field).
#[test]
#[ignore]
fn calibrate() {
    const MARGIN: f32 = 0.5 + 1.0 / 256.0;
    let filter = std::env::var("FIXTURES").unwrap_or_default();
    println!("fixture\tpaths\tcells\tlistings\tlistings_model\tvisits\tvisits_model\tbin_us");
    for (name, make) in fixtures() {
        if !filter.is_empty() && !name.contains(filter.as_str()) {
            continue;
        }
        let outlines: Vec<_> = make().into_iter().map(|builder| builder.outline()).collect();
        let forced: f32 = std::env::var("PATH_SWEEP")
            .ok()
            .and_then(|sweep| sweep.split(',').nth(3)?.trim().parse().ok())
            .unwrap_or(0.0);
        let (mut listings_model, mut visits_model, mut visits, mut cells) = (0.0, 0.0, 0, 0);
        let paths: Vec<_> = outlines
            .iter()
            .map(|(curves, fill_rule)| Path::from_curves(curves.clone(), *fill_rule))
            .collect();
        let before = crate::path_winding::LISTINGS.load(std::sync::atomic::Ordering::Relaxed);
        for ((curves, _), path) in outlines.iter().zip(&paths) {
            visits += shader_work(&path.bin(*BIN_SCALE)).2;
            let (mut min, mut max, mut perimeter) = (point(f32::MAX, f32::MAX), point(f32::MIN, f32::MIN), 0.0);
            for curve in curves {
                let (low, high) = (curve.p0.x.min(curve.p1.x), curve.p0.x.max(curve.p1.x));
                min = point(min.x.min(low), min.y.min(curve.p0.y));
                max = point(max.x.max(high), max.y.max(curve.p1.y));
                perimeter += high - low + curve.p1.y - curve.p0.y;
            }
            let left = (min.x - MARGIN).floor();
            let top = (min.y - MARGIN).floor();
            let size = point(max.x + MARGIN - left, max.y + MARGIN - top);
            let extent = size.x.max(size.y);
            let mut cell = if forced > 0.0 { forced } else { f32::NAN };
            while ((size.x / cell) as usize + 1) * ((size.y / cell) as usize + 1) > 4096 {
                cell *= 2.0;
            }
            cells += ((size.x / cell) as usize + 1) * ((size.y / cell) as usize + 1);
            let listings = if cell >= extent { curves.len() as f32 } else { curves.len() as f32 + perimeter / cell };
            listings_model += listings;
            visits_model += listings * cell.min(size.x) * cell.min(size.y) / 2.0 * *SCALE * *SCALE;
        }
        let listings = crate::path_winding::LISTINGS.load(std::sync::atomic::Ordering::Relaxed) - before;
        let bin = time(|| {
            for path in &paths {
                std::hint::black_box(path.bin(*BIN_SCALE));
            }
        });
        println!(
            "{name}\t{}\t{cells}\t{listings}\t{listings_model:.0}\t{visits}\t{visits_model:.0}\t{bin:.2}",
            outlines.len()
        );
    }
}

#[test]
#[ignore]
fn profile() {
    let fixture = std::env::var("FIXTURE").unwrap_or_else(|_| "scribble".into());
    let seconds: f64 = std::env::var("PROFILE_SECONDS")
        .ok()
        .and_then(|seconds| seconds.parse().ok())
        .unwrap_or(5.0);
    let Some((_, make)) = fixtures().into_iter().find(|(name, _)| *name == fixture) else {
        panic!("unknown fixture {fixture:?}");
    };
    let paths: Vec<_> = make()
        .into_iter()
        .map(|builder| {
            let (curves, fill_rule) = builder.outline();
            Path::from_curves(curves, fill_rule)
        })
        .collect();
    for _ in 0..20 {
        for path in &paths {
            std::hint::black_box(path.bin(*BIN_SCALE));
        }
    }
    let start = Instant::now();
    let mut iterations = 0u32;
    let mut best = Duration::MAX;
    while start.elapsed().as_secs_f64() < seconds {
        let began = Instant::now();
        for path in &paths {
            std::hint::black_box(path.bin(*BIN_SCALE));
        }
        best = best.min(began.elapsed());
        iterations += 1;
    }
    println!(
        "{fixture}: {iterations} iterations, mean {:.2} us, best {:.2} us",
        (start.elapsed() / iterations).as_secs_f64() * 1e6,
        best.as_secs_f64() * 1e6,
    );
}
