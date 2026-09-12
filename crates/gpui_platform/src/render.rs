//! Rendering asset identifiers shared by `gpui` and its platform backends.

#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use crate::{PlatformAtlas, Scene};
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use anyhow::Result;
use gpui_shared_string::SharedString;
use gpui_types::{DevicePixels, Size};
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use image::RgbaImage;
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use std::sync::Arc;

/// A unique identifier for the image cache
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(pub usize);

#[derive(PartialEq, Eq, Hash, Clone)]
#[expect(missing_docs)]
pub struct RenderImageParams {
    pub image_id: ImageId,
    pub frame_index: usize,
}

#[derive(Clone, PartialEq, Hash, Eq)]
#[expect(missing_docs)]
pub struct RenderSvgParams {
    pub path: SharedString,
    pub size: Size<DevicePixels>,
}

/// A renderer for headless windows that can produce real rendered output.
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
pub trait PlatformHeadlessRenderer {
    /// Render a scene and return the result as an RGBA image.
    fn render_scene_to_image(
        &mut self,
        scene: &Scene,
        size: Size<DevicePixels>,
    ) -> Result<RgbaImage>;

    /// Render a scene to an offscreen target without reading the result back.
    ///
    /// This is the headless analogue of presenting a frame: it performs the
    /// same CPU-side scene encoding and GPU submission as drawing to a real
    /// window, but doesn't block on GPU completion or copy pixels back.
    fn render_scene(&mut self, scene: &Scene, size: Size<DevicePixels>) -> Result<()>;

    /// Returns the sprite atlas used by this renderer.
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;
}

// Adapted from https://github.com/microsoft/terminal/blob/1283c0f5b99a2961673249fa77c6b986efb5086c/src/renderer/atlas/dwrite.cpp
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
/// Compute gamma correction ratios for subpixel text rendering.
#[allow(dead_code)]
pub fn get_gamma_correction_ratios(gamma: f32) -> [f32; 4] {
    const GAMMA_INCORRECT_TARGET_RATIOS: [[f32; 4]; 13] = [
        [0.0000 / 4.0, 0.0000 / 4.0, 0.0000 / 4.0, 0.0000 / 4.0], // gamma = 1.0
        [0.0166 / 4.0, -0.0807 / 4.0, 0.2227 / 4.0, -0.0751 / 4.0], // gamma = 1.1
        [0.0350 / 4.0, -0.1760 / 4.0, 0.4325 / 4.0, -0.1370 / 4.0], // gamma = 1.2
        [0.0543 / 4.0, -0.2821 / 4.0, 0.6302 / 4.0, -0.1876 / 4.0], // gamma = 1.3
        [0.0739 / 4.0, -0.3963 / 4.0, 0.8167 / 4.0, -0.2287 / 4.0], // gamma = 1.4
        [0.0933 / 4.0, -0.5161 / 4.0, 0.9926 / 4.0, -0.2616 / 4.0], // gamma = 1.5
        [0.1121 / 4.0, -0.6395 / 4.0, 1.1588 / 4.0, -0.2877 / 4.0], // gamma = 1.6
        [0.1300 / 4.0, -0.7649 / 4.0, 1.3159 / 4.0, -0.3080 / 4.0], // gamma = 1.7
        [0.1469 / 4.0, -0.8911 / 4.0, 1.4644 / 4.0, -0.3234 / 4.0], // gamma = 1.8
        [0.1627 / 4.0, -1.0170 / 4.0, 1.6051 / 4.0, -0.3347 / 4.0], // gamma = 1.9
        [0.1773 / 4.0, -1.1420 / 4.0, 1.7385 / 4.0, -0.3426 / 4.0], // gamma = 2.0
        [0.1908 / 4.0, -1.2652 / 4.0, 1.8650 / 4.0, -0.3476 / 4.0], // gamma = 2.1
        [0.2031 / 4.0, -1.3864 / 4.0, 1.9851 / 4.0, -0.3501 / 4.0], // gamma = 2.2
    ];

    const NORM13: f32 = ((0x10000 as f64) / (255.0 * 255.0) * 4.0) as f32;
    const NORM24: f32 = ((0x100 as f64) / (255.0) * 4.0) as f32;

    let index = ((gamma * 10.0).round() as usize).clamp(10, 22) - 10;
    let ratios = GAMMA_INCORRECT_TARGET_RATIOS[index];

    [
        ratios[0] * NORM13,
        ratios[1] * NORM24,
        ratios[2] * NORM13,
        ratios[3] * NORM24,
    ]
}
