use std::{borrow::Cow, hint::black_box, sync::Arc, time::Duration};

use anyhow::{Context as _, Result};
use bench_metrics::MetricReport;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, BenchMeasurement, Bounds, ContentMask,
    Corners, DevicePixels, Hsla, ImageId, MonochromeSprite, PaddedBool32, PlatformAtlas,
    PlatformHeadlessRenderer, Point, PolychromeSprite, RenderImageParams, RenderSvgParams,
    ScaledPixels, Scene, SharedString, Size, TransformationMatrix, size,
};

struct Entry {
    key: AtlasKey,
    bytes: Vec<u8>,
}

struct Fixture {
    renderer: Box<dyn PlatformHeadlessRenderer>,
    atlas: Arc<dyn PlatformAtlas>,
    dimensions: Size<DevicePixels>,
    seed: Entry,
    entries: Vec<Entry>,
}

struct Batch<'a> {
    fixture: &'a Fixture,
    texture_id: AtlasTextureId,
    misses: bool,
    builder_calls: usize,
    tiles: Vec<AtlasTile>,
}

fn atlas_upload(criterion: &mut Criterion<BenchMeasurement>) {
    assert!(cfg!(target_os = "macos"), "this benchmark requires Metal");
    assert!(!cfg!(debug_assertions), "use --profile release-fast");
    let mut group = criterion.benchmark_group("atlas_upload");
    for (kind, format) in [
        (AtlasTextureKind::Monochrome, "a8"),
        (AtlasTextureKind::Polychrome, "bgra8"),
    ] {
        for (width, height, count) in [(1, 1, 128), (3, 2, 128), (16, 16, 128), (256, 256, 8)] {
            let dimensions = size(DevicePixels(width), DevicePixels(height));
            let mut fixture = Fixture::new(kind, dimensions, count).expect("create Metal fixture");
            fixture.validate_pixels().expect("validate atlas pixels");
            group.throughput(Throughput::Elements(count as u64));
            for (misses, mode) in [(true, "miss"), (false, "hit")] {
                let report = MetricReport::new();
                let id = BenchmarkId::new(
                    format!("{mode}_{format}"),
                    format!("{width}x{height}_{count}"),
                );
                group.bench_function(id, |bencher| {
                    let mut iterations = 0;
                    report.discard_pending();
                    bencher.iter_batched_ref(
                        || {
                            iterations += 1;
                            fixture.prepare(misses).expect("prepare atlas batch")
                        },
                        |batch| batch.run().expect("insert atlas batch"),
                        BatchSize::PerIteration,
                    );
                    report.record_sample(iterations * count as u64);
                });
                eprintln!("{mode}_{format}/{width}x{height}: secondary metrics per atlas lookup");
                report.print("  ");
            }
        }
    }
    group.finish();
}

impl Fixture {
    fn new(kind: AtlasTextureKind, dimensions: Size<DevicePixels>, count: usize) -> Result<Self> {
        let renderer = gpui_platform::current_headless_renderer()?.context("Metal unavailable")?;
        let atlas = renderer.sprite_atlas();
        Ok(Self {
            renderer,
            atlas,
            dimensions,
            seed: make_entry(kind, dimensions, count),
            entries: (0..count)
                .map(|index| make_entry(kind, dimensions, index))
                .collect(),
        })
    }

    fn insert(&self, entry: &Entry, builder_calls: &mut usize) -> Result<AtlasTile> {
        self.atlas
            .get_or_insert_with(black_box(entry.key.clone()), &mut || {
                *builder_calls += 1;
                Ok(Some((
                    black_box(self.dimensions),
                    Cow::Borrowed(black_box(&entry.bytes)),
                )))
            })?
            .context("missing atlas tile")
    }

    fn prepare(&self, misses: bool) -> Result<Batch<'_>> {
        for entry in &self.entries {
            self.atlas.remove(&entry.key);
        }
        self.atlas.remove(&self.seed.key);
        let mut builder_calls = 0;
        let seed = self.insert(&self.seed, &mut builder_calls)?;
        assert_eq!(builder_calls, 1);
        if !misses {
            for entry in &self.entries {
                let tile = self.insert(entry, &mut builder_calls)?;
                assert_eq!(tile.texture_id, seed.texture_id);
            }
            assert_eq!(builder_calls, self.entries.len() + 1);
        }
        Ok(Batch {
            fixture: self,
            texture_id: seed.texture_id,
            misses,
            builder_calls: 0,
            tiles: Vec::with_capacity(self.entries.len()),
        })
    }

    fn validate_pixels(&mut self) -> Result<()> {
        let tiles = {
            let mut batch = self.prepare(true)?;
            batch.run()?;
            batch.tiles.clone()
        };
        for (entry, tile) in self.entries.iter().zip(tiles) {
            let scene = tile_scene(tile);
            let image = self
                .renderer
                .render_scene_to_image(&scene, self.dimensions)?;
            let expected = match tile.texture_id.kind {
                AtlasTextureKind::Monochrome => entry
                    .bytes
                    .iter()
                    .flat_map(|&alpha| [alpha, alpha, alpha, 255])
                    .collect::<Vec<_>>(),
                AtlasTextureKind::Polychrome => entry
                    .bytes
                    .chunks_exact(4)
                    .flat_map(|pixel| [pixel[2], pixel[1], pixel[0], 255])
                    .collect(),
                AtlasTextureKind::Subpixel => unreachable!(),
            };
            assert_eq!(image.as_raw(), &expected);
        }
        let mut batch = self.prepare(false)?;
        batch.run()?;
        Ok(())
    }
}

impl Batch<'_> {
    fn run(&mut self) -> Result<()> {
        for entry in &self.fixture.entries {
            self.tiles
                .push(self.fixture.insert(entry, &mut self.builder_calls)?);
        }
        black_box(&self.tiles);
        Ok(())
    }
}

impl Drop for Batch<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        let count = self.fixture.entries.len();
        assert_eq!(self.tiles.len(), count);
        assert_eq!(self.builder_calls, if self.misses { count } else { 0 });
        for tile in &self.tiles {
            assert_eq!(tile.texture_id, self.texture_id);
            assert_eq!(tile.bounds.size, self.fixture.dimensions);
        }
    }
}

fn make_entry(kind: AtlasTextureKind, dimensions: Size<DevicePixels>, index: usize) -> Entry {
    let key = match kind {
        AtlasTextureKind::Monochrome => AtlasKey::Svg(RenderSvgParams {
            path: SharedString::from(format!("atlas-upload-{index}")),
            size: dimensions,
        }),
        AtlasTextureKind::Polychrome => AtlasKey::Image(RenderImageParams {
            image_id: ImageId(index),
            frame_index: 0,
        }),
        AtlasTextureKind::Subpixel => unreachable!(),
    };
    let mut bytes = Vec::new();
    for row in 0..dimensions.height.0 {
        for column in 0..dimensions.width.0 {
            let value = (index as u8)
                .wrapping_mul(73)
                .wrapping_add((row as u8).wrapping_mul(31))
                .wrapping_add((column as u8).wrapping_mul(17));
            match kind {
                AtlasTextureKind::Monochrome => bytes.push(value),
                AtlasTextureKind::Polychrome => {
                    bytes.extend([value, value.wrapping_add(53), value.wrapping_add(107), 255])
                }
                AtlasTextureKind::Subpixel => unreachable!(),
            }
        }
    }
    Entry { key, bytes }
}

fn tile_scene(tile: AtlasTile) -> Scene {
    let bounds = Bounds {
        origin: Point::default(),
        size: size(
            ScaledPixels(tile.bounds.size.width.0 as f32),
            ScaledPixels(tile.bounds.size.height.0 as f32),
        ),
    };
    let content_mask = ContentMask { bounds };
    let mut scene = Scene::default();
    match tile.texture_id.kind {
        AtlasTextureKind::Monochrome => scene.insert_primitive(MonochromeSprite {
            order: 0,
            pad: 0,
            tile,
            bounds,
            content_mask,
            color: Hsla {
                h: 0.0,
                s: 0.0,
                l: 1.0,
                a: 1.0,
            },
            transformation: TransformationMatrix::default(),
        }),
        AtlasTextureKind::Polychrome => scene.insert_primitive(PolychromeSprite {
            order: 0,
            pad: 0,
            tile,
            grayscale: PaddedBool32::from(false),
            opacity: 1.0,
            bounds,
            content_mask,
            corner_radii: Corners::default(),
        }),
        AtlasTextureKind::Subpixel => unreachable!(),
    }
    scene.finish();
    scene
}

gpui::bench_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2))
        .without_plots();
    targets = atlas_upload
}
gpui::bench_main!(benches);
