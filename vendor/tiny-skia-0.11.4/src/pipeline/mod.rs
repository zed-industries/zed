// Copyright 2016 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/*!
A raster pipeline implementation.

Despite having a lot of changes compared to `SkRasterPipeline`,
the core principles are the same:

1. A pipeline consists of stages.
1. A pipeline has a global context shared by all stages.
   Unlike Skia, were each stage has it's own, possibly shared, context.
1. Each stage has a high precision implementation. See `highp.rs`.
1. Some stages have a low precision implementation. See `lowp.rs`.
1. Each stage calls the "next" stage after its done.
1. During pipeline "compilation", if **all** stages have a lowp implementation,
   the lowp pipeline will be used. Otherwise, the highp variant will be used.
1. The pipeline "compilation" produces a list of function pointer.
   The last pointer is a pointer to the "return" function,
   which simply stops the execution of the pipeline.

This implementation is a bit tricky, but it gives the maximum performance.
A simple and straightforward implementation using traits and loops, like:

```ignore
trait StageTrait {
    fn apply(&mut self, pixels: &mut [Pixel]);
}

let stages: Vec<&mut dyn StageTrait>;
for stage in stages {
    stage.apply(pixels);
}
```

will be at least 20-30% slower. Not really sure why.

Also, since this module is all about performance, any kind of branching is
strictly forbidden. All stage functions must not use `if`, `match` or loops.
There are still some exceptions, which are basically an imperfect implementations
and should be optimized out in the future.
*/

use alloc::vec::Vec;

use arrayvec::ArrayVec;

use tiny_skia_path::NormalizedF32;

use crate::{Color, PremultipliedColor, PremultipliedColorU8, SpreadMode};
use crate::{PixmapRef, Transform};

pub use blitter::RasterPipelineBlitter;

use crate::geom::ScreenIntRect;
use crate::pixmap::SubPixmapMut;
use crate::wide::u32x8;

mod blitter;
#[rustfmt::skip] mod highp;
#[rustfmt::skip] mod lowp;

const MAX_STAGES: usize = 32; // More than enough.

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Stage {
    MoveSourceToDestination = 0,
    MoveDestinationToSource,
    Clamp0,
    ClampA,
    Premultiply,
    UniformColor,
    SeedShader,
    LoadDestination,
    Store,
    LoadDestinationU8,
    StoreU8,
    Gather,
    LoadMaskU8,
    MaskU8,
    ScaleU8,
    LerpU8,
    Scale1Float,
    Lerp1Float,
    DestinationAtop,
    DestinationIn,
    DestinationOut,
    DestinationOver,
    SourceAtop,
    SourceIn,
    SourceOut,
    SourceOver,
    Clear,
    Modulate,
    Multiply,
    Plus,
    Screen,
    Xor,
    ColorBurn,
    ColorDodge,
    Darken,
    Difference,
    Exclusion,
    HardLight,
    Lighten,
    Overlay,
    SoftLight,
    Hue,
    Saturation,
    Color,
    Luminosity,
    SourceOverRgba,
    Transform,
    Reflect,
    Repeat,
    Bilinear,
    Bicubic,
    PadX1,
    ReflectX1,
    RepeatX1,
    Gradient,
    EvenlySpaced2StopGradient,
    XYToRadius,
    XYTo2PtConicalFocalOnCircle,
    XYTo2PtConicalWellBehaved,
    XYTo2PtConicalGreater,
    Mask2PtConicalDegenerates,
    ApplyVectorMask,
}

pub const STAGES_COUNT: usize = Stage::ApplyVectorMask as usize + 1;

impl<'a> PixmapRef<'a> {
    #[inline(always)]
    pub(crate) fn gather(&self, index: u32x8) -> [PremultipliedColorU8; highp::STAGE_WIDTH] {
        let index: [u32; 8] = bytemuck::cast(index);
        let pixels = self.pixels();
        [
            pixels[index[0] as usize],
            pixels[index[1] as usize],
            pixels[index[2] as usize],
            pixels[index[3] as usize],
            pixels[index[4] as usize],
            pixels[index[5] as usize],
            pixels[index[6] as usize],
            pixels[index[7] as usize],
        ]
    }
}

impl<'a> SubPixmapMut<'a> {
    #[inline(always)]
    pub(crate) fn offset(&self, dx: usize, dy: usize) -> usize {
        self.real_width * dy + dx
    }

    #[inline(always)]
    pub(crate) fn slice_at_xy(&mut self, dx: usize, dy: usize) -> &mut [PremultipliedColorU8] {
        let offset = self.offset(dx, dy);
        &mut self.pixels_mut()[offset..]
    }

    #[inline(always)]
    pub(crate) fn slice_mask_at_xy(&mut self, dx: usize, dy: usize) -> &mut [u8] {
        let offset = self.offset(dx, dy);
        &mut self.data[offset..]
    }

    #[inline(always)]
    pub(crate) fn slice4_at_xy(
        &mut self,
        dx: usize,
        dy: usize,
    ) -> &mut [PremultipliedColorU8; highp::STAGE_WIDTH] {
        arrayref::array_mut_ref!(self.pixels_mut(), self.offset(dx, dy), highp::STAGE_WIDTH)
    }

    #[inline(always)]
    pub(crate) fn slice16_at_xy(
        &mut self,
        dx: usize,
        dy: usize,
    ) -> &mut [PremultipliedColorU8; lowp::STAGE_WIDTH] {
        arrayref::array_mut_ref!(self.pixels_mut(), self.offset(dx, dy), lowp::STAGE_WIDTH)
    }

    #[inline(always)]
    pub(crate) fn slice16_mask_at_xy(
        &mut self,
        dx: usize,
        dy: usize,
    ) -> &mut [u8; lowp::STAGE_WIDTH] {
        arrayref::array_mut_ref!(self.data, self.offset(dx, dy), lowp::STAGE_WIDTH)
    }
}

#[derive(Default, Debug)]
pub struct AAMaskCtx {
    pub pixels: [u8; 2],
    pub stride: u32,  // can be zero
    pub shift: usize, // mask offset/position in pixmap coordinates
}

impl AAMaskCtx {
    #[inline(always)]
    pub fn copy_at_xy(&self, dx: usize, dy: usize, tail: usize) -> [u8; 2] {
        let offset = (self.stride as usize * dy + dx) - self.shift;
        // We have only 3 variants, so unroll them.
        match (offset, tail) {
            (0, 1) => [self.pixels[0], 0],
            (0, 2) => [self.pixels[0], self.pixels[1]],
            (1, 1) => [self.pixels[1], 0],
            _ => [0, 0], // unreachable
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MaskCtx<'a> {
    pub data: &'a [u8],
    pub real_width: u32,
}

impl MaskCtx<'_> {
    #[inline(always)]
    fn offset(&self, dx: usize, dy: usize) -> usize {
        self.real_width as usize * dy + dx
    }
}

#[derive(Default)]
pub struct Context {
    pub current_coverage: f32,
    pub sampler: SamplerCtx,
    pub uniform_color: UniformColorCtx,
    pub evenly_spaced_2_stop_gradient: EvenlySpaced2StopGradientCtx,
    pub gradient: GradientCtx,
    pub two_point_conical_gradient: TwoPointConicalGradientCtx,
    pub limit_x: TileCtx,
    pub limit_y: TileCtx,
    pub transform: Transform,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct SamplerCtx {
    pub spread_mode: SpreadMode,
    pub inv_width: f32,
    pub inv_height: f32,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct UniformColorCtx {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub rgba: [u16; 4], // [0,255] in a 16-bit lane.
}

// A gradient color is an unpremultiplied RGBA not in a 0..1 range.
// It basically can have any float value.
#[derive(Copy, Clone, Default, Debug)]
pub struct GradientColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl GradientColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        GradientColor { r, g, b, a }
    }
}

impl From<Color> for GradientColor {
    fn from(c: Color) -> Self {
        GradientColor {
            r: c.red(),
            g: c.green(),
            b: c.blue(),
            a: c.alpha(),
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct EvenlySpaced2StopGradientCtx {
    pub factor: GradientColor,
    pub bias: GradientColor,
}

#[derive(Clone, Default, Debug)]
pub struct GradientCtx {
    /// This value stores the actual colors count.
    /// `factors` and `biases` must store at least 16 values,
    /// since this is the length of a lowp pipeline stage.
    /// So any any value past `len` is just zeros.
    pub len: usize,
    pub factors: Vec<GradientColor>,
    pub biases: Vec<GradientColor>,
    pub t_values: Vec<NormalizedF32>,
}

impl GradientCtx {
    pub fn push_const_color(&mut self, color: GradientColor) {
        self.factors.push(GradientColor::new(0.0, 0.0, 0.0, 0.0));
        self.biases.push(color);
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TwoPointConicalGradientCtx {
    // This context is used only in highp, where we use Tx4.
    pub mask: u32x8,
    pub p0: f32,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TileCtx {
    pub scale: f32,
    pub inv_scale: f32, // cache of 1/scale
}

pub struct RasterPipelineBuilder {
    stages: ArrayVec<Stage, MAX_STAGES>,
    force_hq_pipeline: bool,
    pub ctx: Context,
}

impl RasterPipelineBuilder {
    pub fn new() -> Self {
        RasterPipelineBuilder {
            stages: ArrayVec::new(),
            force_hq_pipeline: false,
            ctx: Context::default(),
        }
    }

    pub fn set_force_hq_pipeline(&mut self, hq: bool) {
        self.force_hq_pipeline = hq;
    }

    pub fn push(&mut self, stage: Stage) {
        self.stages.push(stage);
    }

    pub fn push_transform(&mut self, ts: Transform) {
        if ts.is_finite() && !ts.is_identity() {
            self.stages.push(Stage::Transform);
            self.ctx.transform = ts;
        }
    }

    pub fn push_uniform_color(&mut self, c: PremultipliedColor) {
        let r = c.red();
        let g = c.green();
        let b = c.blue();
        let a = c.alpha();
        let rgba = [
            (r * 255.0 + 0.5) as u16,
            (g * 255.0 + 0.5) as u16,
            (b * 255.0 + 0.5) as u16,
            (a * 255.0 + 0.5) as u16,
        ];

        let ctx = UniformColorCtx { r, g, b, a, rgba };

        self.stages.push(Stage::UniformColor);
        self.ctx.uniform_color = ctx;
    }

    pub fn compile(self) -> RasterPipeline {
        if self.stages.is_empty() {
            return RasterPipeline {
                kind: RasterPipelineKind::High {
                    functions: ArrayVec::new(),
                    tail_functions: ArrayVec::new(),
                },
                ctx: Context::default(),
            };
        }

        let is_lowp_compatible = self
            .stages
            .iter()
            .all(|stage| !lowp::fn_ptr_eq(lowp::STAGES[*stage as usize], lowp::null_fn));

        if self.force_hq_pipeline || !is_lowp_compatible {
            let mut functions: ArrayVec<_, MAX_STAGES> = self
                .stages
                .iter()
                .map(|stage| highp::STAGES[*stage as usize] as highp::StageFn)
                .collect();
            functions.push(highp::just_return as highp::StageFn);

            // I wasn't able to reproduce Skia's load_8888_/store_8888_ performance.
            // Skia uses fallthrough switch, which is probably the reason.
            // In Rust, any branching in load/store code drastically affects the performance.
            // So instead, we're using two "programs": one for "full stages" and one for "tail stages".
            // While the only difference is the load/store methods.
            let mut tail_functions = functions.clone();
            for fun in &mut tail_functions {
                if highp::fn_ptr(*fun) == highp::fn_ptr(highp::load_dst) {
                    *fun = highp::load_dst_tail as highp::StageFn;
                } else if highp::fn_ptr(*fun) == highp::fn_ptr(highp::store) {
                    *fun = highp::store_tail as highp::StageFn;
                } else if highp::fn_ptr(*fun) == highp::fn_ptr(highp::load_dst_u8) {
                    *fun = highp::load_dst_u8_tail as highp::StageFn;
                } else if highp::fn_ptr(*fun) == highp::fn_ptr(highp::store_u8) {
                    *fun = highp::store_u8_tail as highp::StageFn;
                } else if highp::fn_ptr(*fun) == highp::fn_ptr(highp::source_over_rgba) {
                    // SourceOverRgba calls load/store manually, without the pipeline,
                    // therefore we have to switch it too.
                    *fun = highp::source_over_rgba_tail as highp::StageFn;
                }
            }

            RasterPipeline {
                kind: RasterPipelineKind::High {
                    functions,
                    tail_functions,
                },
                ctx: self.ctx,
            }
        } else {
            let mut functions: ArrayVec<_, MAX_STAGES> = self
                .stages
                .iter()
                .map(|stage| lowp::STAGES[*stage as usize] as lowp::StageFn)
                .collect();
            functions.push(lowp::just_return as lowp::StageFn);

            // See above.
            let mut tail_functions = functions.clone();
            for fun in &mut tail_functions {
                if lowp::fn_ptr(*fun) == lowp::fn_ptr(lowp::load_dst) {
                    *fun = lowp::load_dst_tail as lowp::StageFn;
                } else if lowp::fn_ptr(*fun) == lowp::fn_ptr(lowp::store) {
                    *fun = lowp::store_tail as lowp::StageFn;
                } else if lowp::fn_ptr(*fun) == lowp::fn_ptr(lowp::load_dst_u8) {
                    *fun = lowp::load_dst_u8_tail as lowp::StageFn;
                } else if lowp::fn_ptr(*fun) == lowp::fn_ptr(lowp::store_u8) {
                    *fun = lowp::store_u8_tail as lowp::StageFn;
                } else if lowp::fn_ptr(*fun) == lowp::fn_ptr(lowp::source_over_rgba) {
                    // SourceOverRgba calls load/store manually, without the pipeline,
                    // therefore we have to switch it too.
                    *fun = lowp::source_over_rgba_tail as lowp::StageFn;
                }
            }

            RasterPipeline {
                kind: RasterPipelineKind::Low {
                    functions,
                    tail_functions,
                },
                ctx: self.ctx,
            }
        }
    }
}

pub enum RasterPipelineKind {
    High {
        functions: ArrayVec<highp::StageFn, MAX_STAGES>,
        tail_functions: ArrayVec<highp::StageFn, MAX_STAGES>,
    },
    Low {
        functions: ArrayVec<lowp::StageFn, MAX_STAGES>,
        tail_functions: ArrayVec<lowp::StageFn, MAX_STAGES>,
    },
}

pub struct RasterPipeline {
    kind: RasterPipelineKind,
    pub ctx: Context,
}

impl RasterPipeline {
    pub fn run(
        &mut self,
        rect: &ScreenIntRect,
        aa_mask_ctx: AAMaskCtx,
        mask_ctx: MaskCtx,
        pixmap_src: PixmapRef,
        pixmap_dst: &mut SubPixmapMut,
    ) {
        match self.kind {
            RasterPipelineKind::High {
                ref functions,
                ref tail_functions,
            } => {
                highp::start(
                    functions.as_slice(),
                    tail_functions.as_slice(),
                    rect,
                    aa_mask_ctx,
                    mask_ctx,
                    &mut self.ctx,
                    pixmap_src,
                    pixmap_dst,
                );
            }
            RasterPipelineKind::Low {
                ref functions,
                ref tail_functions,
            } => {
                lowp::start(
                    functions.as_slice(),
                    tail_functions.as_slice(),
                    rect,
                    aa_mask_ctx,
                    mask_ctx,
                    &mut self.ctx,
                    // lowp doesn't support pattern, so no `pixmap_src` for it.
                    pixmap_dst,
                );
            }
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod blend_tests {
    // Test blending modes.
    //
    // Skia has two kinds of a raster pipeline: high and low precision.
    // "High" uses f32 and "low" uses u16.
    // And for basic operations we don't need f32 and u16 simply faster.
    // But those modes are not identical. They can produce slightly different results
    // due rounding.

    use super::*;
    use crate::{BlendMode, Color, Pixmap, PremultipliedColorU8};
    use crate::geom::IntSizeExt;

    macro_rules! test_blend {
        ($name:ident, $mode:expr, $is_highp:expr, $r:expr, $g:expr, $b:expr, $a:expr) => {
            #[test]
            fn $name() {
                let mut pixmap = Pixmap::new(1, 1).unwrap();
                pixmap.fill(Color::from_rgba8(50, 127, 150, 200));

                let pixmap_src = PixmapRef::from_bytes(&[0, 0, 0, 0], 1, 1).unwrap();

                let mut p = RasterPipelineBuilder::new();
                p.set_force_hq_pipeline($is_highp);
                p.push_uniform_color(Color::from_rgba8(220, 140, 75, 180).premultiply());
                p.push(Stage::LoadDestination);
                p.push($mode.to_stage().unwrap());
                p.push(Stage::Store);
                let mut p = p.compile();
                let rect = pixmap.size().to_screen_int_rect(0, 0);
                p.run(&rect, AAMaskCtx::default(), MaskCtx::default(), pixmap_src,
                      &mut pixmap.as_mut().as_subpixmap());

                assert_eq!(
                    pixmap.as_ref().pixel(0, 0).unwrap(),
                    PremultipliedColorU8::from_rgba($r, $g, $b, $a).unwrap()
                );
            }
        };
    }

    macro_rules! test_blend_lowp {
        ($name:ident, $mode:expr, $r:expr, $g:expr, $b:expr, $a:expr) => (
            test_blend!{$name, $mode, false, $r, $g, $b, $a}
        )
    }

    macro_rules! test_blend_highp {
        ($name:ident, $mode:expr, $r:expr, $g:expr, $b:expr, $a:expr) => (
            test_blend!{$name, $mode, true, $r, $g, $b, $a}
        )
    }

    test_blend_lowp!(clear_lowp,              BlendMode::Clear,                 0,   0,   0,   0);
    // Source is a no-op
    test_blend_lowp!(destination_lowp,        BlendMode::Destination,          39, 100, 118, 200);
    test_blend_lowp!(source_over_lowp,        BlendMode::SourceOver,          167, 129,  88, 239);
    test_blend_lowp!(destination_over_lowp,   BlendMode::DestinationOver,      73, 122, 130, 239);
    test_blend_lowp!(source_in_lowp,          BlendMode::SourceIn,            122,  78,  42, 141);
    test_blend_lowp!(destination_in_lowp,     BlendMode::DestinationIn,        28,  71,  83, 141);
    test_blend_lowp!(source_out_lowp,         BlendMode::SourceOut,            34,  22,  12,  39);
    test_blend_lowp!(destination_out_lowp,    BlendMode::DestinationOut,       12,  30,  35,  59);
    test_blend_lowp!(source_atop_lowp,        BlendMode::SourceAtop,          133, 107,  76, 200);
    test_blend_lowp!(destination_atop_lowp,   BlendMode::DestinationAtop,      61,  92,  95, 180);
    test_blend_lowp!(xor_lowp,                BlendMode::Xor,                  45,  51,  46,  98);
    test_blend_lowp!(plus_lowp,               BlendMode::Plus,                194, 199, 171, 255);
    test_blend_lowp!(modulate_lowp,           BlendMode::Modulate,             24,  39,  25, 141);
    test_blend_lowp!(screen_lowp,             BlendMode::Screen,              170, 160, 146, 239);
    test_blend_lowp!(overlay_lowp,            BlendMode::Overlay,              92, 128, 106, 239);
    test_blend_lowp!(darken_lowp,             BlendMode::Darken,               72, 121,  88, 239);
    test_blend_lowp!(lighten_lowp,            BlendMode::Lighten,             166, 128, 129, 239);
    // ColorDodge in not available for lowp.
    // ColorBurn in not available for lowp.
    test_blend_lowp!(hard_light_lowp,         BlendMode::HardLight,           154, 128,  95, 239);
    // SoftLight in not available for lowp.
    test_blend_lowp!(difference_lowp,         BlendMode::Difference,          138,  57,  87, 239);
    test_blend_lowp!(exclusion_lowp,          BlendMode::Exclusion,           146, 121, 121, 239);
    test_blend_lowp!(multiply_lowp,           BlendMode::Multiply,             69,  90,  71, 238);
    // Hue in not available for lowp.
    // Saturation in not available for lowp.
    // Color in not available for lowp.
    // Luminosity in not available for lowp.

    test_blend_highp!(clear_highp,            BlendMode::Clear,                 0,   0,   0,   0);
    // Source is a no-op
    test_blend_highp!(destination_highp,      BlendMode::Destination,          39, 100, 118, 200);
    test_blend_highp!(source_over_highp,      BlendMode::SourceOver,          167, 128,  88, 239);
    test_blend_highp!(destination_over_highp, BlendMode::DestinationOver,      72, 121, 129, 239);
    test_blend_highp!(source_in_highp,        BlendMode::SourceIn,            122,  78,  42, 141);
    test_blend_highp!(destination_in_highp,   BlendMode::DestinationIn,        28,  71,  83, 141);
    test_blend_highp!(source_out_highp,       BlendMode::SourceOut,            33,  21,  11,  39);
    test_blend_highp!(destination_out_highp,  BlendMode::DestinationOut,       11,  29,  35,  59);
    test_blend_highp!(source_atop_highp,      BlendMode::SourceAtop,          133, 107,  76, 200);
    test_blend_highp!(destination_atop_highp, BlendMode::DestinationAtop,      61,  92,  95, 180);
    test_blend_highp!(xor_highp,              BlendMode::Xor,                  45,  51,  46,  98);
    test_blend_highp!(plus_highp,             BlendMode::Plus,                194, 199, 171, 255);
    test_blend_highp!(modulate_highp,         BlendMode::Modulate,             24,  39,  24, 141);
    test_blend_highp!(screen_highp,           BlendMode::Screen,              171, 160, 146, 239);
    test_blend_highp!(overlay_highp,          BlendMode::Overlay,              92, 128, 106, 239);
    test_blend_highp!(darken_highp,           BlendMode::Darken,               72, 121,  88, 239);
    test_blend_highp!(lighten_highp,          BlendMode::Lighten,             167, 128, 129, 239);
    test_blend_highp!(color_dodge_highp,      BlendMode::ColorDodge,          186, 192, 164, 239);
    test_blend_highp!(color_burn_highp,       BlendMode::ColorBurn,            54,  63,  46, 239);
    test_blend_highp!(hard_light_highp,       BlendMode::HardLight,           155, 128,  95, 239);
    test_blend_highp!(soft_light_highp,       BlendMode::SoftLight,            98, 124, 115, 239);
    test_blend_highp!(difference_highp,       BlendMode::Difference,          139,  58,  88, 239);
    test_blend_highp!(exclusion_highp,        BlendMode::Exclusion,           147, 121, 122, 239);
    test_blend_highp!(multiply_highp,         BlendMode::Multiply,             69,  89,  71, 239);
    test_blend_highp!(hue_highp,              BlendMode::Hue,                 128, 103,  74, 239);
    test_blend_highp!(saturation_highp,       BlendMode::Saturation,           59, 126, 140, 239);
    test_blend_highp!(color_highp,            BlendMode::Color,               139, 100,  60, 239);
    test_blend_highp!(luminosity_highp,       BlendMode::Luminosity,          100, 149, 157, 239);
}
