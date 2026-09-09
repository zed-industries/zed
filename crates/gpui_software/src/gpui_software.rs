mod atlas;
mod bin_pass;
mod damage;
mod framebuffer;
mod kernels;
mod lower;
mod paths;
mod raster;
mod stats;
mod text_correction;

use std::sync::Arc;

use gpui::{DevicePixels, GpuSpecs, PlatformAtlas, Scene, Size};

pub use atlas::SoftwareAtlas;
pub use damage::Damage;
pub use framebuffer::Framebuffer;
pub use text_correction::FontCorrection;

use bin_pass::BinGrid;
use lower::{LoweringCache, lower_scene};

pub struct SoftwareRenderer {
    framebuffer: Framebuffer,
    atlas: Arc<SoftwareAtlas>,
    font_correction: FontCorrection,
    lowering_cache: LoweringCache,
    bins: BinGrid,
    previous_hashes: Vec<u64>,
    previous_grid_size: (usize, usize),
    force_full: bool,
}

impl SoftwareRenderer {
    pub fn new(size: Size<DevicePixels>, font_correction: FontCorrection) -> Self {
        Self {
            framebuffer: Framebuffer::new(size),
            atlas: Arc::new(SoftwareAtlas::new()),
            font_correction,
            lowering_cache: LoweringCache::default(),
            bins: BinGrid::default(),
            previous_hashes: Vec::new(),
            previous_grid_size: (0, 0),
            force_full: true,
        }
    }

    pub fn atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub fn resize(&mut self, size: Size<DevicePixels>) {
        if self.framebuffer.size() != size {
            self.framebuffer.resize(size);
            self.force_full = true;
        }
    }

    pub fn set_font_correction(&mut self, correction: FontCorrection) {
        if self.font_correction != correction {
            self.font_correction = correction;
            self.force_full = true;
        }
    }

    pub fn draw(&mut self, scene: &Scene, force_full: bool) -> Damage {
        let frame_start = std::time::Instant::now();
        let lowered = lower_scene(
            scene,
            self.font_correction,
            std::mem::take(&mut self.lowering_cache),
        );
        let lower_elapsed = frame_start.elapsed();
        let atlas = self.atlas.lock();
        self.bins
            .update(self.framebuffer.size(), &lowered.ops, &atlas);
        let bins = &self.bins;
        let bin_elapsed = frame_start.elapsed() - lower_elapsed;
        let force_full = force_full
            || self.force_full
            || self.previous_grid_size != (bins.columns(), bins.rows());
        let damage = damage::compute_damage(
            self.framebuffer.size(),
            bins,
            &self.previous_hashes,
            force_full,
        );
        let damage_elapsed = frame_start.elapsed() - lower_elapsed - bin_elapsed;

        let raster_start = std::time::Instant::now();
        if !damage.rects.is_empty() {
            raster::rasterize(&mut self.framebuffer, &lowered, bins, &damage, &atlas);
        }
        let raster_elapsed = raster_start.elapsed();

        self.previous_hashes.clear();
        self.previous_hashes
            .extend(bins.cells().iter().map(|cell| cell.hash));
        self.previous_grid_size = (bins.columns(), bins.rows());
        self.force_full = false;
        stats::log_frame(
            lower_elapsed,
            bin_elapsed,
            damage_elapsed,
            raster_elapsed,
            lowered.ops.len(),
            &damage,
        );
        self.lowering_cache = lowered.cache;
        self.lowering_cache.spare_ops = lowered.ops;
        self.lowering_cache.trim();
        damage
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    pub fn gpu_specs(&self) -> GpuSpecs {
        GpuSpecs {
            is_software_emulated: false,
            device_name: "GPUI software renderer".to_owned(),
            driver_name: "gpui_software".to_owned(),
            driver_info: format!(
                "{}, {} threads",
                kernels::simd_level(),
                rayon::current_num_threads()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use gpui::{
        AtlasKey, ContentMask, FontId, GlyphId, ImageId, MonochromeSprite, Path, PolychromeSprite,
        Quad, RenderGlyphParams, RenderImageParams, RenderSvgParams, ScaledPixels, Scene,
        SharedString, SubpixelSprite, TransformationMatrix, bounds, hsla, linear_color_stop,
        linear_gradient, point, px, size,
    };

    use super::*;

    fn device_size(width: i32, height: i32) -> Size<DevicePixels> {
        Size {
            width: DevicePixels(width),
            height: DevicePixels(height),
        }
    }

    fn mask(width: f32, height: f32) -> ContentMask<ScaledPixels> {
        ContentMask {
            bounds: bounds(
                point(ScaledPixels(0.0), ScaledPixels(0.0)),
                size(ScaledPixels(width), ScaledPixels(height)),
            ),
        }
    }

    fn damage_scene(color: gpui::Hsla) -> Scene {
        let mut scene = Scene::default();
        scene.insert_primitive(Quad {
            bounds: mask(128.0, 64.0).bounds,
            content_mask: mask(128.0, 64.0),
            background: hsla(0.0, 0.0, 0.0, 1.0).into(),
            ..Default::default()
        });
        scene.insert_primitive(Quad {
            bounds: bounds(
                point(ScaledPixels(68.0), ScaledPixels(4.0)),
                size(ScaledPixels(8.0), ScaledPixels(8.0)),
            ),
            content_mask: mask(128.0, 64.0),
            background: color.into(),
            ..Default::default()
        });
        scene.finish();
        scene
    }

    #[test]
    fn tracks_damage_and_forces_full_redraw_after_resize() {
        let mut renderer = SoftwareRenderer::new(device_size(128, 64), FontCorrection::default());
        let first = renderer.draw(&damage_scene(hsla(0.0, 1.0, 0.5, 1.0)), false);
        assert_eq!(first.rects.len(), 1);
        assert!(
            renderer
                .draw(&damage_scene(hsla(0.0, 1.0, 0.5, 1.0)), false)
                .is_empty()
        );

        let changed = renderer.draw(&damage_scene(hsla(0.6, 1.0, 0.5, 1.0)), false);
        assert_eq!(changed.rects.len(), 1);
        assert_eq!(changed.rects[0].origin.x, DevicePixels(64));
        assert_eq!(changed.rects[0].size.width, DevicePixels(64));

        renderer.resize(device_size(192, 96));
        let resized = renderer.draw(&damage_scene(hsla(0.6, 1.0, 0.5, 1.0)), false);
        assert_eq!(resized.rects.len(), 1);
        assert!(resized.rects.iter().all(|rect| {
            rect.origin.x == DevicePixels(0) && rect.size.width == DevicePixels(192)
        }));
    }

    #[test]
    fn renders_descending_gradient_stops_without_hiding_content() {
        let mut scene = Scene::default();
        scene.insert_primitive(Quad {
            bounds: mask(64.0, 1.0).bounds,
            content_mask: mask(64.0, 1.0),
            background: hsla(0.0, 0.0, 1.0, 1.0).into(),
            ..Default::default()
        });
        scene.insert_primitive(Quad {
            bounds: mask(64.0, 1.0).bounds,
            content_mask: mask(64.0, 1.0),
            background: linear_gradient(
                90.0,
                linear_color_stop(hsla(0.0, 0.0, 0.0, 1.0), 0.7),
                linear_color_stop(hsla(0.0, 0.0, 0.0, 0.0), 0.0),
            ),
            ..Default::default()
        });
        scene.finish();

        let mut renderer = SoftwareRenderer::new(device_size(64, 1), FontCorrection::default());
        renderer.draw(&scene, false);
        let pixels = renderer.framebuffer().pixels();

        assert!(pixels[0] > pixels[63]);
    }

    #[test]
    fn composites_monochrome_and_polychrome_atlas_tiles() {
        let mut renderer = SoftwareRenderer::new(device_size(2, 1), FontCorrection::default());
        let mono_key = AtlasKey::Svg(RenderSvgParams {
            path: SharedString::from("test"),
            size: device_size(2, 1),
        });
        let mono_tile = renderer
            .atlas()
            .get_or_insert_with(&mono_key, &mut || {
                Ok(Some((device_size(2, 1), Cow::Borrowed(&[0u8, 255u8]))))
            })
            .expect("monochrome atlas insertion failed")
            .expect("monochrome builder returned no tile");
        let mut scene = Scene::default();
        scene.insert_primitive(Quad {
            bounds: mask(2.0, 1.0).bounds,
            content_mask: mask(2.0, 1.0),
            background: hsla(0.0, 0.0, 1.0, 1.0).into(),
            ..Default::default()
        });
        scene.insert_primitive(MonochromeSprite {
            order: 0,
            pad: 0,
            bounds: mask(2.0, 1.0).bounds,
            content_mask: mask(2.0, 1.0),
            color: hsla(0.0, 0.0, 0.0, 1.0),
            tile: mono_tile,
            transformation: TransformationMatrix::unit(),
        });
        scene.finish();
        renderer.draw(&scene, false);
        assert_eq!(renderer.framebuffer().pixels(), &[0xffff_ffff, 0xff00_0000]);

        let mut renderer = SoftwareRenderer::new(device_size(1, 1), FontCorrection::default());
        let image_key = AtlasKey::Image(RenderImageParams {
            image_id: ImageId(1),
            frame_index: 0,
        });
        let image_tile = renderer
            .atlas()
            .get_or_insert_with(&image_key, &mut || {
                Ok(Some((
                    device_size(1, 1),
                    Cow::Borrowed(&[0u8, 0, 255, 128]),
                )))
            })
            .expect("image atlas insertion failed")
            .expect("image builder returned no tile");
        let mut scene = Scene::default();
        scene.insert_primitive(PolychromeSprite {
            order: 0,
            pad: 0,
            grayscale: false.into(),
            opacity: 0.5,
            bounds: mask(1.0, 1.0).bounds,
            content_mask: mask(1.0, 1.0),
            corner_radii: Default::default(),
            tile: image_tile,
        });
        scene.finish();
        renderer.draw(&scene, false);
        assert_eq!(renderer.framebuffer().pixels(), &[0xff40_0000]);
    }

    fn subpixel_framebuffer(is_bgr: bool) -> u32 {
        let correction = FontCorrection {
            is_bgr,
            ..Default::default()
        };
        let mut renderer = SoftwareRenderer::new(device_size(1, 1), correction);
        let glyph_key = AtlasKey::Glyph(RenderGlyphParams {
            font_id: FontId(0),
            glyph_id: GlyphId(1),
            font_size: px(12.0),
            subpixel_variant: point(0, 0),
            scale_factor: 1.0,
            is_emoji: false,
            subpixel_rendering: true,
            dilation: 0,
        });
        let tile = renderer
            .atlas()
            .get_or_insert_with(&glyph_key, &mut || {
                Ok(Some((device_size(1, 1), Cow::Borrowed(&[255u8, 0, 0, 0]))))
            })
            .expect("subpixel atlas insertion failed")
            .expect("subpixel builder returned no tile");
        let mut scene = Scene::default();
        scene.insert_primitive(Quad {
            bounds: mask(1.0, 1.0).bounds,
            content_mask: mask(1.0, 1.0),
            background: hsla(0.0, 0.0, 1.0, 1.0).into(),
            ..Default::default()
        });
        scene.insert_primitive(SubpixelSprite {
            order: 0,
            pad: 0,
            bounds: mask(1.0, 1.0).bounds,
            content_mask: mask(1.0, 1.0),
            color: hsla(0.0, 0.0, 0.0, 1.0),
            tile,
            transformation: TransformationMatrix::unit(),
        });
        scene.finish();
        renderer.draw(&scene, false);
        renderer.framebuffer().pixels()[0]
    }

    #[test]
    fn applies_subpixel_channel_order() {
        assert_eq!(subpixel_framebuffer(false), 0xff00_ffff);
        assert_eq!(subpixel_framebuffer(true), 0xffff_ff00);
    }

    #[test]
    fn fills_paths_through_vello_cpu() {
        let mut path = Path::new(point(px(1.0), px(1.0)));
        path.line_to(point(px(7.0), px(1.0)));
        path.line_to(point(px(7.0), px(7.0)));
        path.color = hsla(0.0, 1.0, 0.5, 1.0).into();
        path.content_mask = ContentMask {
            bounds: bounds(point(px(0.0), px(0.0)), size(px(8.0), px(8.0))),
        };
        let mut scene = Scene::default();
        scene.insert_primitive(path.scale(1.0));
        scene.finish();
        let mut renderer = SoftwareRenderer::new(device_size(8, 8), FontCorrection::default());
        renderer.draw(&scene, false);
        assert!(
            renderer
                .framebuffer()
                .pixels()
                .iter()
                .any(|pixel| *pixel != 0xff00_0000)
        );
    }
    #[test]
    fn atlas_reuse_invalidates_damage() {
        let mut renderer = SoftwareRenderer::new(device_size(8, 1), FontCorrection::default());
        let atlas = renderer.atlas();
        for index in 0..260 {
            let key = AtlasKey::Image(RenderImageParams {
                image_id: ImageId(index),
                frame_index: 0,
            });
            let bytes = if index % 2 == 0 {
                [0u8, 0, 255, 255].repeat(8)
            } else {
                [255u8, 0, 0, 255].repeat(8)
            };
            let tile = atlas
                .get_or_insert_with(&key, &mut || {
                    Ok(Some((device_size(8, 1), Cow::Borrowed(&bytes))))
                })
                .expect("insert")
                .expect("tile");
            let mut scene = Scene::default();
            scene.insert_primitive(PolychromeSprite {
                order: 0,
                pad: 0,
                grayscale: false.into(),
                opacity: 1.0,
                bounds: mask(8.0, 1.0).bounds,
                content_mask: mask(8.0, 1.0),
                corner_radii: Default::default(),
                tile,
            });
            scene.finish();
            assert!(
                !renderer.draw(&scene, false).is_empty(),
                "replacement {index}"
            );
            let expected = if index % 2 == 0 {
                0xffff_0000
            } else {
                0xff00_00ff
            };
            assert!(
                renderer
                    .framebuffer()
                    .pixels()
                    .iter()
                    .all(|pixel| *pixel == expected)
            );
            assert!(renderer.draw(&scene, false).is_empty());
            atlas.remove(&key);
        }
    }
    #[test]
    fn borders_stay_inside_quad() {
        for thickness in [1.0, 2.0, 20.0] {
            let mut scene = Scene::default();
            scene.insert_primitive(Quad {
                bounds: bounds(
                    point(ScaledPixels(0.0), ScaledPixels(0.0)),
                    size(ScaledPixels(10.0), ScaledPixels(1.0)),
                ),
                content_mask: mask(16.0, 16.0),
                border_color: hsla(0.0, 0.0, 1.0, 0.5),
                border_widths: gpui::Edges::all(ScaledPixels(thickness)),
                ..Default::default()
            });
            scene.finish();
            let mut renderer =
                SoftwareRenderer::new(device_size(16, 16), FontCorrection::default());
            renderer.draw(&scene, true);
            assert!(
                renderer.framebuffer().pixels()[16..]
                    .iter()
                    .all(|pixel| *pixel == 0xff00_0000)
            );
            assert!(
                renderer.framebuffer().pixels()[..10]
                    .iter()
                    .all(|pixel| *pixel == 0xff80_8080)
            );
        }
    }

    fn rectangle_path(left: f32, right: f32) -> Path<ScaledPixels> {
        let mut path = Path::new(point(px(left), px(1.0)));
        path.line_to(point(px(right), px(1.0)));
        path.line_to(point(px(right), px(9.0)));
        path.line_to(point(px(left), px(9.0)));
        path.color = hsla(0.0, 0.0, 1.0, 1.0).into();
        path.content_mask = ContentMask {
            bounds: bounds(point(px(0.0), px(0.0)), size(px(16.0), px(16.0))),
        };
        path.scale(1.0)
    }

    #[test]
    fn thin_paths_remain_visible() {
        for (left, right) in [(0.35, 0.75), (0.1, 0.4)] {
            let mut scene = Scene::default();
            scene.insert_primitive(rectangle_path(left, right));
            scene.finish();
            let mut renderer =
                SoftwareRenderer::new(device_size(16, 16), FontCorrection::default());
            renderer.draw(&scene, true);
            assert!(
                renderer
                    .framebuffer()
                    .pixels()
                    .iter()
                    .any(|pixel| *pixel != 0xff00_0000)
            );
        }
    }

    #[test]
    fn path_gradients_preserve_opacity() {
        for descending in [false, true] {
            let mut path = rectangle_path(1.0, 9.0);
            path.color = linear_gradient(
                90.0,
                linear_color_stop(hsla(0.0, 0.0, 1.0, 0.0), if descending { 1.0 } else { 0.0 }),
                linear_color_stop(hsla(0.0, 0.0, 1.0, 1.0), if descending { 0.0 } else { 1.0 }),
            );
            let mut scene = Scene::default();
            scene.insert_primitive(path);
            scene.finish();
            let mut renderer =
                SoftwareRenderer::new(device_size(16, 16), FontCorrection::default());
            renderer.draw(&scene, true);
            let pixels = renderer.framebuffer().pixels();
            if descending {
                assert!(pixels[4 * 16 + 1] > pixels[4 * 16 + 8]);
            } else {
                assert!(pixels[4 * 16 + 1] < pixels[4 * 16 + 8]);
            }
        }
    }

    #[test]
    fn fractional_gradient_path_bounds_invalidate_damage() {
        let mut path = rectangle_path(1.0, 9.0);
        path.color = linear_gradient(
            90.0,
            linear_color_stop(hsla(0.0, 0.0, 0.0, 1.0), 0.0),
            linear_color_stop(hsla(0.0, 0.0, 1.0, 1.0), 1.0),
        );
        let mut first = Scene::default();
        first.insert_primitive(path.clone());
        first.finish();
        path.bounds.size.width.0 -= 0.25;
        let mut second = Scene::default();
        second.insert_primitive(path);
        second.finish();
        let mut renderer = SoftwareRenderer::new(device_size(16, 16), FontCorrection::default());
        renderer.draw(&first, true);
        let original = renderer.framebuffer().pixels().to_vec();
        renderer.draw(&second, false);
        let incremental = renderer.framebuffer().pixels().to_vec();
        renderer.draw(&second, true);
        assert_ne!(original, renderer.framebuffer().pixels());
        assert_eq!(incremental, renderer.framebuffer().pixels());
    }

    #[test]
    fn singular_sprites_are_empty() {
        let mut renderer = SoftwareRenderer::new(device_size(16, 16), FontCorrection::default());
        let tile = renderer
            .atlas()
            .get_or_insert_with(
                &AtlasKey::Svg(RenderSvgParams {
                    path: SharedString::from("singular"),
                    size: device_size(2, 2),
                }),
                &mut || Ok(Some((device_size(2, 2), Cow::Borrowed(&[255; 4])))),
            )
            .expect("insert")
            .expect("tile");
        let mut scene = Scene::default();
        scene.insert_primitive(MonochromeSprite {
            order: 0,
            pad: 0,
            bounds: mask(2.0, 2.0).bounds,
            content_mask: mask(16.0, 16.0),
            color: hsla(0.0, 0.0, 1.0, 1.0),
            tile,
            transformation: TransformationMatrix {
                rotation_scale: [[1.0, 1.0], [1.0, 1.0]],
                translation: [0.0; 2],
            },
        });
        scene.finish();
        renderer.draw(&scene, true);
        assert!(
            renderer
                .framebuffer()
                .pixels()
                .iter()
                .all(|pixel| *pixel == 0xff00_0000)
        );
    }

    #[test]
    fn incremental_frames_match_full_across_scene_changes() {
        use rand::{Rng, SeedableRng, rngs::StdRng};
        let mut random = StdRng::seed_from_u64(0x5ced_2026);
        let mut renderer = SoftwareRenderer::new(device_size(193, 97), FontCorrection::default());
        for frame in 0..100 {
            if frame % 13 == 0 {
                renderer.resize(device_size(
                    random.random_range(65..260),
                    random.random_range(33..130),
                ));
            }
            let mut scene = Scene::default();
            for index in 0..35 {
                let color = hsla(random.random(), 0.7, 0.6, random.random());
                let background = if index % 3 == 0 {
                    linear_gradient(
                        random.random_range(-360.0..360.0),
                        linear_color_stop(color, 0.0),
                        linear_color_stop(hsla(0.5, 0.6, 0.4, 0.8), 1.0),
                    )
                } else {
                    color.into()
                };
                scene.insert_primitive(Quad {
                    bounds: bounds(
                        point(
                            ScaledPixels(random.random_range(-20.0..200.0)),
                            ScaledPixels(random.random_range(-10.0..100.0)),
                        ),
                        size(
                            ScaledPixels(random.random_range(0.0..100.0)),
                            ScaledPixels(random.random_range(0.0..60.0)),
                        ),
                    ),
                    content_mask: mask(220.0, 120.0),
                    background,
                    ..Default::default()
                });
            }
            let mut path = rectangle_path(
                random.random_range(0.0..2.0),
                random.random_range(3.0..15.0),
            );
            path.color = hsla(random.random(), 0.7, 0.5, random.random()).into();
            scene.insert_primitive(path);
            scene.finish();
            renderer.draw(&scene, false);
            let incremental = renderer.framebuffer().pixels().to_vec();
            renderer.draw(&scene, true);
            assert_eq!(
                incremental,
                renderer.framebuffer().pixels(),
                "frame {frame}"
            );
            assert!(renderer.draw(&scene, false).is_empty());
        }
    }

    #[test]
    fn fully_occluded_changes_do_not_damage_the_frame() {
        let scene = |hue| {
            let mut scene = damage_scene(hsla(hue, 0.5, 0.5, 1.0));
            scene.insert_primitive(Quad {
                bounds: mask(128.0, 64.0).bounds,
                content_mask: mask(128.0, 64.0),
                background: hsla(0.5, 0.5, 0.5, 1.0).into(),
                ..Default::default()
            });
            scene.finish();
            scene
        };
        let mut renderer = SoftwareRenderer::new(device_size(128, 64), FontCorrection::default());
        renderer.draw(&scene(0.0), false);
        let before = renderer.framebuffer().pixels().to_vec();
        assert!(renderer.draw(&scene(0.5), false).is_empty());
        assert_eq!(before, renderer.framebuffer().pixels());
    }

    #[test]
    fn unrelated_path_insertion_preserves_clean_cells() {
        let scene = |insert| {
            let mut scene = Scene::default();
            if insert {
                scene.insert_primitive(rectangle_path(1.0, 9.0));
            }
            let mut path = rectangle_path(70.0, 80.0);
            path.content_mask = mask(128.0, 64.0);
            scene.insert_primitive(path);
            scene.finish();
            scene
        };
        let mut renderer = SoftwareRenderer::new(device_size(128, 64), FontCorrection::default());
        renderer.draw(&scene(false), false);
        let damage = renderer.draw(&scene(true), false);
        assert!(!damage.is_empty());
        assert!(
            damage
                .rects
                .iter()
                .all(|rect| rect.origin.x.0 + rect.size.width.0 <= 64)
        );
        let incremental = renderer.framebuffer().pixels().to_vec();
        renderer.draw(&scene(true), true);
        assert_eq!(incremental, renderer.framebuffer().pixels());
    }
}
