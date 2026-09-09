use std::{borrow::Cow, hint::black_box};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gpui::{
    AtlasKey, AtlasTile, ContentMask, DevicePixels, MonochromeSprite, Path, Quad, RenderSvgParams,
    ScaledPixels, Scene, SharedString, Size, TransformationMatrix, bounds, hsla, linear_color_stop,
    linear_gradient, point, px, size,
};
use gpui_software::{FontCorrection, SoftwareRenderer};

fn device_size(width: i32, height: i32) -> Size<DevicePixels> {
    Size {
        width: DevicePixels(width),
        height: DevicePixels(height),
    }
}

fn content_mask(width: i32, height: i32) -> ContentMask<ScaledPixels> {
    ContentMask {
        bounds: bounds(
            point(ScaledPixels(0.0), ScaledPixels(0.0)),
            size(ScaledPixels(width as f32), ScaledPixels(height as f32)),
        ),
    }
}

fn glyph_tile(renderer: &SoftwareRenderer, variant: usize) -> AtlasTile {
    let glyph_size = device_size(6, 10);
    let key = AtlasKey::Svg(RenderSvgParams {
        path: SharedString::from(format!("software-benchmark-glyph-{variant}")),
        size: glyph_size,
    });
    let coverage = (0..60)
        .map(|index| {
            if (index * 17 + variant * 7) % 11 < 4 {
                96
            } else {
                220
            }
        })
        .collect::<Vec<_>>();
    renderer
        .atlas()
        .get_or_insert_with(&key, &mut || {
            Ok(Some((glyph_size, Cow::Borrowed(&coverage))))
        })
        .expect("benchmark glyph atlas insertion failed")
        .expect("benchmark glyph builder returned no tile")
}

fn editor_scene(
    window_size: Size<DevicePixels>,
    tiles: &[AtlasTile],
    changed_line: bool,
    scroll_offset: i32,
) -> Scene {
    let mut scene = Scene::default();
    let mask = content_mask(window_size.width.0, window_size.height.0);
    scene.insert_primitive(Quad {
        bounds: mask.bounds,
        content_mask: mask,
        background: hsla(0.62, 0.20, 0.10, 1.0).into(),
        ..Default::default()
    });
    for index in 0..200 {
        let x = ((index * 97) % window_size.width.0.max(1) as usize) as f32;
        let y = ((index * 53) % window_size.height.0.max(1) as usize) as f32;
        scene.insert_primitive(Quad {
            bounds: bounds(
                point(ScaledPixels(x), ScaledPixels(y)),
                size(ScaledPixels(120.0), ScaledPixels(22.0)),
            ),
            content_mask: mask,
            background: hsla(0.62, 0.12, 0.16, 1.0).into(),
            ..Default::default()
        });
    }
    for index in 0..20 {
        let x = 900.0 + (index % 5) as f32 * 48.0;
        let y = 80.0 + (index / 5) as f32 * 48.0;
        let mut path = Path::new(point(px(x), px(y)));
        path.line_to(point(px(x + 28.0), px(y + 12.0)));
        path.line_to(point(px(x + 8.0), px(y + 30.0)));
        path.color = hsla(0.55, 0.5, 0.55, 1.0).into();
        path.content_mask = ContentMask {
            bounds: bounds(
                point(px(0.0), px(0.0)),
                size(
                    px(window_size.width.0 as f32),
                    px(window_size.height.0 as f32),
                ),
            ),
        };
        scene.insert_primitive(path.scale(1.0));
    }
    for index in 0..(window_size.height.0 / 14 + 4) * 100 {
        let column = index % 100;
        let row = index / 100;
        let y = 40 + row * 14 - scroll_offset;
        if y < -10 || y >= window_size.height.0 {
            continue;
        }
        let color = if changed_line && row == 20 {
            hsla(0.12, 0.8, 0.65, 1.0)
        } else {
            hsla((row % 13) as f32 / 13.0, 0.3, 0.82, 1.0)
        };
        scene.insert_primitive(MonochromeSprite {
            order: 0,
            pad: 0,
            bounds: bounds(
                point(
                    ScaledPixels(24.0 + column as f32 * 8.0),
                    ScaledPixels(y as f32),
                ),
                size(ScaledPixels(6.0), ScaledPixels(10.0)),
            ),
            content_mask: mask,
            color,
            tile: tiles[(index * 7 + row * 3) as usize % tiles.len()],
            transformation: TransformationMatrix::unit(),
        });
    }
    scene.finish();
    scene
}

fn frame_benchmarks(criterion: &mut Criterion) {
    let thread_count = rayon::current_num_threads();
    for (name, window_size) in [
        ("1080p", device_size(1920, 1080)),
        ("4k", device_size(3840, 2160)),
    ] {
        let mut renderer = SoftwareRenderer::new(window_size, FontCorrection::default());
        let tiles = (0..16)
            .map(|variant| glyph_tile(&renderer, variant))
            .collect::<Vec<_>>();
        let scene = editor_scene(window_size, &tiles, false, 0);
        renderer.draw(&scene, true);
        capture(&renderer, &format!("editor-{name}"));
        criterion.bench_with_input(
            BenchmarkId::new(format!("full-{name}"), thread_count),
            &thread_count,
            |bencher, _| {
                bencher.iter(|| black_box(renderer.draw(black_box(&scene), true)));
            },
        );

        let base = editor_scene(window_size, &tiles, false, 0);
        let changed = editor_scene(window_size, &tiles, true, 0);
        validate_pair(&mut renderer, &base, &changed, false);
        renderer.draw(&base, true);
        let mut alternate = false;
        criterion.bench_with_input(
            BenchmarkId::new(format!("single-line-{name}"), thread_count),
            &thread_count,
            |bencher, _| {
                bencher.iter(|| {
                    alternate = !alternate;
                    let scene = if alternate { &changed } else { &base };
                    black_box(renderer.draw(black_box(scene), false))
                });
            },
        );

        let scrolled = editor_scene(window_size, &tiles, false, 14);
        validate_pair(&mut renderer, &base, &scrolled, true);
        renderer.draw(&base, true);
        criterion.bench_with_input(
            BenchmarkId::new(format!("scroll-{name}"), thread_count),
            &thread_count,
            |bencher, _| {
                bencher.iter(|| {
                    alternate = !alternate;
                    let scene = if alternate { &scrolled } else { &base };
                    black_box(renderer.draw(black_box(scene), false))
                });
            },
        );
        let smooth = editor_scene(window_size, &tiles, false, 3);
        validate_pair(&mut renderer, &base, &smooth, true);
        criterion.bench_function(&format!("smooth-scroll-{name}/{thread_count}"), |bencher| {
            bencher.iter(|| {
                alternate = !alternate;
                black_box(renderer.draw(black_box(if alternate { &smooth } else { &base }), false))
            });
        });
        criterion.bench_function(&format!("unchanged-{name}/{thread_count}"), |bencher| {
            renderer.draw(&base, true);
            assert!(renderer.draw(&base, false).is_empty());
            bencher.iter(|| black_box(renderer.draw(black_box(&base), false)));
        });
    }
    let mut renderer = SoftwareRenderer::new(device_size(1024, 768), FontCorrection::default());
    let mut scene = Scene::default();
    let mut path = Path::new(point(px(64.0), px(64.0)));
    for index in 0..64 {
        let angle = index as f32 * std::f32::consts::TAU / 64.0;
        path.line_to(point(
            px(512.0 + 420.0 * angle.cos()),
            px(384.0 + 300.0 * angle.sin()),
        ));
    }
    path.color = hsla(0.55, 0.5, 0.55, 0.6).into();
    path.content_mask = ContentMask {
        bounds: bounds(point(px(0.0), px(0.0)), size(px(1024.0), px(768.0))),
    };
    scene.insert_primitive(path.scale(1.0));
    scene.finish();
    renderer.draw(&scene, true);
    capture(&renderer, "large-path");
    assert!(
        renderer
            .framebuffer()
            .pixels()
            .iter()
            .any(|pixel| *pixel != 0xff00_0000)
    );
    criterion.bench_function(&format!("large-path/{thread_count}"), |bencher| {
        bencher.iter(|| black_box(renderer.draw(black_box(&scene), true)));
    });

    let mut scene = Scene::default();
    for index in 0..64 {
        scene.insert_primitive(Quad {
            bounds: bounds(
                point(
                    ScaledPixels((index % 8 * 128) as f32),
                    ScaledPixels((index / 8 * 96) as f32),
                ),
                size(ScaledPixels(128.0), ScaledPixels(96.0)),
            ),
            content_mask: content_mask(1024, 768),
            background: linear_gradient(
                110.0,
                linear_color_stop(hsla((index % 4) as f32 * 0.25, 0.6, 0.3, 0.5), 0.0),
                linear_color_stop(hsla(0.6, 0.8, 0.7, 1.0), 1.0),
            ),
            ..Default::default()
        });
    }
    scene.finish();
    renderer.draw(&scene, true);
    capture(&renderer, "gradients");
    criterion.bench_function(&format!("gradients/{thread_count}"), |bencher| {
        bencher.iter(|| black_box(renderer.draw(black_box(&scene), true)));
    });

    let hidden = |hue| {
        let mut scene = Scene::default();
        for index in 0..100 {
            scene.insert_primitive(Quad {
                bounds: content_mask(1024, 768).bounds,
                content_mask: content_mask(1024, 768),
                background: hsla(hue + index as f32 / 100.0, 0.5, 0.4, 0.5).into(),
                ..Default::default()
            });
        }
        scene.insert_primitive(Quad {
            bounds: content_mask(1024, 768).bounds,
            content_mask: content_mask(1024, 768),
            background: hsla(0.6, 0.5, 0.4, 1.0).into(),
            ..Default::default()
        });
        scene.finish();
        scene
    };
    let base = hidden(0.0);
    let changed = hidden(0.2);
    renderer.draw(&base, true);
    let expected = renderer.framebuffer().pixels().to_vec();
    renderer.draw(&changed, false);
    assert_eq!(expected, renderer.framebuffer().pixels());
    let mut alternate = false;
    criterion.bench_function(&format!("occluded/{thread_count}"), |bencher| {
        bencher.iter(|| {
            alternate = !alternate;
            black_box(renderer.draw(black_box(if alternate { &base } else { &changed }), false))
        });
    });
}

fn capture(renderer: &SoftwareRenderer, name: &str) {
    if let Some(directory) = std::env::var_os("GPUI_SOFTWARE_CAPTURE_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).expect("create capture directory");
        let bytes: Vec<_> = renderer
            .framebuffer()
            .pixels()
            .iter()
            .flat_map(|pixel| pixel.to_le_bytes())
            .collect();
        std::fs::write(directory.join(format!("{name}.argb")), bytes).expect("write frame capture");
    }
}

fn validate_pair(renderer: &mut SoftwareRenderer, base: &Scene, changed: &Scene, scrolling: bool) {
    renderer.draw(base, true);
    let before = renderer.framebuffer().pixels().to_vec();
    let damage = renderer.draw(changed, false);
    assert!(!damage.is_empty());
    let incremental = renderer.framebuffer().pixels().to_vec();
    assert_ne!(incremental, before);
    if scrolling {
        let stride = renderer.framebuffer().size().width.0 as usize;
        assert_ne!(
            &incremental[100 * stride..200 * stride],
            &before[100 * stride..200 * stride]
        );
    }
    renderer.draw(changed, true);
    assert_eq!(incremental, renderer.framebuffer().pixels());
    renderer.draw(base, false);
    assert_eq!(before, renderer.framebuffer().pixels());
}

criterion_group!(benches, frame_benchmarks);
criterion_main!(benches);
