// Copyright 2018 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/*!
A low precision raster pipeline implementation.

A lowp pipeline uses u16 instead of f32 for math.
Because of that, it doesn't implement stages that require high precision.
The pipeline compiler will automatically decide which one to use.

Skia uses u16x8 (128bit) types for a generic CPU and u16x16 (256bit) for modern x86 CPUs.
But instead of explicit SIMD instructions, it mainly relies on clang's vector extensions.
And since they are unavailable in Rust, we have to do everything manually.

According to our benchmarks, a SIMD-accelerated u16x8 in Rust is almost 2x slower than in Skia.
Not sure why. For example, there are no div instruction for u16x8, so we have to use
a basic scalar version. Which means unnecessary load/store. No idea what clang does in this case.
Surprisingly, a SIMD-accelerated u16x8 is even slower than a scalar one. Again, not sure why.

Therefore we are using scalar u16x16 by default and relying on rustc/llvm auto vectorization instead.
When targeting a generic CPU, we're just 5-10% slower than Skia. While u16x8 is 30-40% slower.
And while `-C target-cpu=haswell` boosts our performance by around 25%,
we are still 40-60% behind Skia built for Haswell.

On ARM AArch64 the story is different and explicit SIMD make our code up to 2-3x faster.
*/

use crate::PremultipliedColorU8;

use crate::pixmap::SubPixmapMut;
use crate::wide::{f32x8, u16x16, f32x16};
use crate::geom::ScreenIntRect;

pub const STAGE_WIDTH: usize = 16;

pub type StageFn = fn(p: &mut Pipeline);

pub struct Pipeline<'a, 'b: 'a> {
    index: usize,
    functions: &'a [StageFn],
    pixmap: &'a mut SubPixmapMut<'b>,
    mask_ctx: super::MaskCtx<'a>,
    aa_mask_ctx: super::AAMaskCtx,
    ctx: &'a mut super::Context,
    r: u16x16,
    g: u16x16,
    b: u16x16,
    a: u16x16,
    dr: u16x16,
    dg: u16x16,
    db: u16x16,
    da: u16x16,
    tail: usize,
    dx: usize,
    dy: usize,
}

impl Pipeline<'_, '_> {
    #[inline(always)]
    fn next_stage(&mut self) {
        let next: fn(&mut Self) = self.functions[self.index];
        self.index += 1;
        next(self);
    }
}


// Must be in the same order as raster_pipeline::Stage
pub const STAGES: &[StageFn; super::STAGES_COUNT] = &[
    move_source_to_destination,
    move_destination_to_source,
    null_fn, // Clamp0
    null_fn, // ClampA
    premultiply,
    uniform_color,
    seed_shader,
    load_dst,
    store,
    load_dst_u8,
    store_u8,
    null_fn, // Gather
    load_mask_u8,
    mask_u8,
    scale_u8,
    lerp_u8,
    scale_1_float,
    lerp_1_float,
    destination_atop,
    destination_in,
    destination_out,
    destination_over,
    source_atop,
    source_in,
    source_out,
    source_over,
    clear,
    modulate,
    multiply,
    plus,
    screen,
    xor,
    null_fn, // ColorBurn
    null_fn, // ColorDodge
    darken,
    difference,
    exclusion,
    hard_light,
    lighten,
    overlay,
    null_fn, // SoftLight
    null_fn, // Hue
    null_fn, // Saturation
    null_fn, // Color
    null_fn, // Luminosity
    source_over_rgba,
    transform,
    null_fn, // Reflect
    null_fn, // Repeat
    null_fn, // Bilinear
    null_fn, // Bicubic
    pad_x1,
    reflect_x1,
    repeat_x1,
    gradient,
    evenly_spaced_2_stop_gradient,
    xy_to_radius,
    null_fn, // XYTo2PtConicalFocalOnCircle
    null_fn, // XYTo2PtConicalWellBehaved
    null_fn, // XYTo2PtConicalGreater
    null_fn, // Mask2PtConicalDegenerates
    null_fn, // ApplyVectorMask
];

pub fn fn_ptr(f: StageFn) -> *const () {
    f as *const ()
}

pub fn fn_ptr_eq(f1: StageFn, f2: StageFn) -> bool {
    core::ptr::eq(f1 as *const (), f2 as *const ())
}

#[inline(never)]
pub fn start(
    functions: &[StageFn],
    functions_tail: &[StageFn],
    rect: &ScreenIntRect,
    aa_mask_ctx: super::AAMaskCtx,
    mask_ctx: super::MaskCtx,
    ctx: &mut super::Context,
    pixmap: &mut SubPixmapMut,
) {
    let mut p = Pipeline {
        index: 0,
        functions: &[],
        pixmap,
        mask_ctx,
        aa_mask_ctx,
        ctx,
        r: u16x16::default(),
        g: u16x16::default(),
        b: u16x16::default(),
        a: u16x16::default(),
        dr: u16x16::default(),
        dg: u16x16::default(),
        db: u16x16::default(),
        da: u16x16::default(),
        tail: 0,
        dx: 0,
        dy: 0,
    };

    for y in rect.y()..rect.bottom() {
        let mut x = rect.x() as usize;
        let end = rect.right() as usize;

        p.functions = functions;
        while x + STAGE_WIDTH <= end {
            p.index = 0;
            p.dx = x;
            p.dy = y as usize;
            p.tail = STAGE_WIDTH;
            p.next_stage();
            x += STAGE_WIDTH;
        }

        if x != end {
            p.index = 0;
            p.functions = functions_tail;
            p.dx = x;
            p.dy = y as usize;
            p.tail = end - x;
            p.next_stage();
        }
    }
}

fn move_source_to_destination(p: &mut Pipeline) {
    p.dr = p.r;
    p.dg = p.g;
    p.db = p.b;
    p.da = p.a;

    p.next_stage();
}

fn move_destination_to_source(p: &mut Pipeline) {
    p.r = p.dr;
    p.g = p.dg;
    p.b = p.db;
    p.a = p.da;

    p.next_stage();
}

fn premultiply(p: &mut Pipeline) {
    p.r = div255(p.r * p.a);
    p.g = div255(p.g * p.a);
    p.b = div255(p.b * p.a);

    p.next_stage();
}

fn uniform_color(p: &mut Pipeline) {
    let ctx = p.ctx.uniform_color;
    p.r = u16x16::splat(ctx.rgba[0]);
    p.g = u16x16::splat(ctx.rgba[1]);
    p.b = u16x16::splat(ctx.rgba[2]);
    p.a = u16x16::splat(ctx.rgba[3]);

    p.next_stage();
}

fn seed_shader(p: &mut Pipeline) {
    let iota = f32x16(
        f32x8::from([0.5,  1.5,  2.5,  3.5,  4.5,  5.5,  6.5,  7.5]),
        f32x8::from([8.5,  9.5, 10.5, 11.5, 12.5, 13.5, 14.5, 15.5]),
    );

    let x = f32x16::splat(p.dx as f32) + iota;
    let y = f32x16::splat(p.dy as f32 + 0.5);
    split(&x, &mut p.r, &mut p.g);
    split(&y, &mut p.b, &mut p.a);

    p.next_stage();
}

pub fn load_dst(p: &mut Pipeline) {
    load_8888(p.pixmap.slice16_at_xy(p.dx, p.dy), &mut p.dr, &mut p.dg, &mut p.db, &mut p.da);
    p.next_stage();
}

pub fn load_dst_tail(p: &mut Pipeline) {
    load_8888_tail(p.tail, p.pixmap.slice_at_xy(p.dx, p.dy), &mut p.dr, &mut p.dg, &mut p.db, &mut p.da);
    p.next_stage();
}

pub fn store(p: &mut Pipeline) {
    store_8888(&p.r, &p.g, &p.b, &p.a, p.pixmap.slice16_at_xy(p.dx, p.dy));
    p.next_stage();
}

pub fn store_tail(p: &mut Pipeline) {
    store_8888_tail(&p.r, &p.g, &p.b, &p.a, p.tail, p.pixmap.slice_at_xy(p.dx, p.dy));
    p.next_stage();
}

pub fn load_dst_u8(p: &mut Pipeline) {
    load_8(p.pixmap.slice16_mask_at_xy(p.dx, p.dy), &mut p.da);
    p.next_stage();
}

pub fn load_dst_u8_tail(p: &mut Pipeline) {
    // Fill a dummy array with `tail` values. `tail` is always in a 1..STAGE_WIDTH-1 range.
    // This way we can reuse the `load_8888__` method and remove any branches.
    let data = p.pixmap.slice_mask_at_xy(p.dx, p.dy);
    let mut tmp = [0u8; STAGE_WIDTH];
    tmp[0..p.tail].copy_from_slice(&data[0..p.tail]);
    load_8(&tmp, &mut p.da);

    p.next_stage();
}

pub fn store_u8(p: &mut Pipeline) {
    let data = p.pixmap.slice16_mask_at_xy(p.dx, p.dy);
    let a = p.a.as_slice();

    data[ 0] = a[ 0] as u8;
    data[ 1] = a[ 1] as u8;
    data[ 2] = a[ 2] as u8;
    data[ 3] = a[ 3] as u8;
    data[ 4] = a[ 4] as u8;
    data[ 5] = a[ 5] as u8;
    data[ 6] = a[ 6] as u8;
    data[ 7] = a[ 7] as u8;
    data[ 8] = a[ 8] as u8;
    data[ 9] = a[ 9] as u8;
    data[10] = a[10] as u8;
    data[11] = a[11] as u8;
    data[12] = a[12] as u8;
    data[13] = a[13] as u8;
    data[14] = a[14] as u8;
    data[15] = a[15] as u8;

    p.next_stage();
}

pub fn store_u8_tail(p: &mut Pipeline) {
    let data = p.pixmap.slice_mask_at_xy(p.dx, p.dy);
    let a = p.a.as_slice();

    // This is better than `for i in 0..tail`, because this way the compiler
    // knows that we have only 16 steps and slices access is guarantee to be valid.
    // This removes bounds checking and a possible panic call.
    for i in 0..STAGE_WIDTH {
        data[i] = a[i] as u8;

        if i + 1 == p.tail {
            break;
        }
    }

    p.next_stage();
}

// Similar to mask_u8, but only loads the mask values without actually masking the pipeline.
fn load_mask_u8(p: &mut Pipeline) {
    let offset = p.mask_ctx.offset(p.dx, p.dy);

    let mut c = u16x16::default();
    for i in 0..p.tail {
        c.0[i] = u16::from(p.mask_ctx.data[offset + i]);
    }

    p.r = u16x16::splat(0);
    p.g = u16x16::splat(0);
    p.b = u16x16::splat(0);
    p.a = c;

    p.next_stage();
}

fn mask_u8(p: &mut Pipeline) {
    let offset = p.mask_ctx.offset(p.dx, p.dy);

    let mut c = u16x16::default();
    for i in 0..p.tail {
        c.0[i] = u16::from(p.mask_ctx.data[offset + i]);
    }

    if c == u16x16::default() {
        return;
    }

    p.r = div255(p.r * c);
    p.g = div255(p.g * c);
    p.b = div255(p.b * c);
    p.a = div255(p.a * c);

    p.next_stage();
}

fn scale_u8(p: &mut Pipeline) {
    // Load u8xTail and cast it to u16x16.
    let data = p.aa_mask_ctx.copy_at_xy(p.dx, p.dy, p.tail);
    let c = u16x16([
        u16::from(data[0]),
        u16::from(data[1]),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]);

    p.r = div255(p.r * c);
    p.g = div255(p.g * c);
    p.b = div255(p.b * c);
    p.a = div255(p.a * c);

    p.next_stage();
}

fn lerp_u8(p: &mut Pipeline) {
    // Load u8xTail and cast it to u16x16.
    let data = p.aa_mask_ctx.copy_at_xy(p.dx, p.dy, p.tail);
    let c = u16x16([
        u16::from(data[0]),
        u16::from(data[1]),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]);

    p.r = lerp(p.dr, p.r, c);
    p.g = lerp(p.dg, p.g, c);
    p.b = lerp(p.db, p.b, c);
    p.a = lerp(p.da, p.a, c);

    p.next_stage();
}

fn scale_1_float(p: &mut Pipeline) {
    let c = from_float(p.ctx.current_coverage);
    p.r = div255(p.r * c);
    p.g = div255(p.g * c);
    p.b = div255(p.b * c);
    p.a = div255(p.a * c);

    p.next_stage();
}

fn lerp_1_float(p: &mut Pipeline) {
    let c = from_float(p.ctx.current_coverage);
    p.r = lerp(p.dr, p.r, c);
    p.g = lerp(p.dg, p.g, c);
    p.b = lerp(p.db, p.b, c);
    p.a = lerp(p.da, p.a, c);

    p.next_stage();
}

macro_rules! blend_fn {
    ($name:ident, $f:expr) => {
        fn $name(p: &mut Pipeline) {
            p.r = $f(p.r, p.dr, p.a, p.da);
            p.g = $f(p.g, p.dg, p.a, p.da);
            p.b = $f(p.b, p.db, p.a, p.da);
            p.a = $f(p.a, p.da, p.a, p.da);

            p.next_stage();
        }
    };
}

blend_fn!(clear,            |_, _,  _,  _| u16x16::splat(0));
blend_fn!(source_atop,      |s, d, sa, da| div255(s * da + d * inv(sa)));
blend_fn!(destination_atop, |s, d, sa, da| div255(d * sa + s * inv(da)));
blend_fn!(source_in,        |s, _,  _, da| div255(s * da));
blend_fn!(destination_in,   |_, d, sa,  _| div255(d * sa));
blend_fn!(source_out,       |s, _,  _, da| div255(s * inv(da)));
blend_fn!(destination_out,  |_, d, sa,  _| div255(d * inv(sa)));
blend_fn!(source_over,      |s, d, sa,  _| s + div255(d * inv(sa)));
blend_fn!(destination_over, |s, d,  _, da| d + div255(s * inv(da)));
blend_fn!(modulate,         |s, d,  _,  _| div255(s * d));
blend_fn!(multiply,         |s, d, sa, da| div255(s * inv(da) + d * inv(sa) + s * d));
blend_fn!(screen,           |s, d,  _,  _| s + d - div255(s * d));
blend_fn!(xor,              |s, d, sa, da| div255(s * inv(da) + d * inv(sa)));

// Wants a type for some reason.
blend_fn!(plus, |s: u16x16, d, _, _| (s + d).min(&u16x16::splat(255)));


macro_rules! blend_fn2 {
    ($name:ident, $f:expr) => {
        fn $name(p: &mut Pipeline) {
            // The same logic applied to color, and source_over for alpha.
            p.r = $f(p.r, p.dr, p.a, p.da);
            p.g = $f(p.g, p.dg, p.a, p.da);
            p.b = $f(p.b, p.db, p.a, p.da);
            p.a = p.a + div255(p.da * inv(p.a));

            p.next_stage();
        }
    };
}

blend_fn2!(darken,      |s: u16x16, d, sa, da| s + d - div255((s * da).max(&(d * sa))));
blend_fn2!(lighten,     |s: u16x16, d, sa, da| s + d - div255((s * da).min(&(d * sa))));
blend_fn2!(exclusion,   |s: u16x16, d,  _,  _| s + d - u16x16::splat(2) * div255(s * d));

blend_fn2!(difference,  |s: u16x16, d, sa, da|
    s + d - u16x16::splat(2) * div255((s * da).min(&(d * sa))));

blend_fn2!(hard_light, |s: u16x16, d: u16x16, sa, da| {
    div255(s * inv(da) + d * inv(sa)
        + (s+s).cmp_le(&sa).blend(
            u16x16::splat(2) * s * d,
            sa * da - u16x16::splat(2) * (sa-s)*(da-d)
        )
    )
});

blend_fn2!(overlay, |s: u16x16, d: u16x16, sa, da| {
    div255(s * inv(da) + d * inv(sa)
        + (d+d).cmp_le(&da).blend(
            u16x16::splat(2) * s * d,
            sa * da - u16x16::splat(2) * (sa-s)*(da-d)
        )
    )
});

pub fn source_over_rgba(p: &mut Pipeline) {
    let pixels = p.pixmap.slice16_at_xy(p.dx, p.dy);
    load_8888(pixels, &mut p.dr, &mut p.dg, &mut p.db, &mut p.da);
    p.r = p.r + div255(p.dr * inv(p.a));
    p.g = p.g + div255(p.dg * inv(p.a));
    p.b = p.b + div255(p.db * inv(p.a));
    p.a = p.a + div255(p.da * inv(p.a));
    store_8888(&p.r, &p.g, &p.b, &p.a, pixels);

    p.next_stage();
}

pub fn source_over_rgba_tail(p: &mut Pipeline) {
    let pixels = p.pixmap.slice_at_xy(p.dx, p.dy);
    load_8888_tail(p.tail, pixels, &mut p.dr, &mut p.dg, &mut p.db, &mut p.da);
    p.r = p.r + div255(p.dr * inv(p.a));
    p.g = p.g + div255(p.dg * inv(p.a));
    p.b = p.b + div255(p.db * inv(p.a));
    p.a = p.a + div255(p.da * inv(p.a));
    store_8888_tail(&p.r, &p.g, &p.b, &p.a, p.tail, pixels);

    p.next_stage();
}

fn transform(p: &mut Pipeline) {
    let ts = &p.ctx.transform;

    let x = join(&p.r, &p.g);
    let y = join(&p.b, &p.a);

    let nx = mad(x, f32x16::splat(ts.sx), mad(y, f32x16::splat(ts.kx), f32x16::splat(ts.tx)));
    let ny = mad(x, f32x16::splat(ts.ky), mad(y, f32x16::splat(ts.sy), f32x16::splat(ts.ty)));

    split(&nx, &mut p.r, &mut p.g);
    split(&ny, &mut p.b, &mut p.a);

    p.next_stage();
}

fn pad_x1(p: &mut Pipeline) {
    let x = join(&p.r, &p.g);
    let x = x.normalize();
    split(&x, &mut p.r, &mut p.g);

    p.next_stage();
}

fn reflect_x1(p: &mut Pipeline) {
    let x = join(&p.r, &p.g);
    let two = |x| x + x;
    let x = (
        (x - f32x16::splat(1.0))
        - two(((x - f32x16::splat(1.0)) * f32x16::splat(0.5)).floor())
        - f32x16::splat(1.0)
    ).abs().normalize();
    split(&x, &mut p.r, &mut p.g);

    p.next_stage();
}

fn repeat_x1(p: &mut Pipeline) {
    let x = join(&p.r, &p.g);
    let x = (x - x.floor()).normalize();
    split(&x, &mut p.r, &mut p.g);

    p.next_stage();
}

fn gradient(p: &mut Pipeline) {
    let ctx = &p.ctx.gradient;

    // N.B. The loop starts at 1 because idx 0 is the color to use before the first stop.
    let t = join(&p.r, &p.g);
    let mut idx = u16x16::splat(0);
    for i in 1..ctx.len {
        let tt = ctx.t_values[i].get();
        let t0: [f32; 8] = t.0.into();
        let t1: [f32; 8] = t.1.into();
        idx.0[ 0] += (t0[0] >= tt) as u16;
        idx.0[ 1] += (t0[1] >= tt) as u16;
        idx.0[ 2] += (t0[2] >= tt) as u16;
        idx.0[ 3] += (t0[3] >= tt) as u16;
        idx.0[ 4] += (t0[4] >= tt) as u16;
        idx.0[ 5] += (t0[5] >= tt) as u16;
        idx.0[ 6] += (t0[6] >= tt) as u16;
        idx.0[ 7] += (t0[7] >= tt) as u16;
        idx.0[ 8] += (t1[0] >= tt) as u16;
        idx.0[ 9] += (t1[1] >= tt) as u16;
        idx.0[10] += (t1[2] >= tt) as u16;
        idx.0[11] += (t1[3] >= tt) as u16;
        idx.0[12] += (t1[4] >= tt) as u16;
        idx.0[13] += (t1[5] >= tt) as u16;
        idx.0[14] += (t1[6] >= tt) as u16;
        idx.0[15] += (t1[7] >= tt) as u16;
    }
    gradient_lookup(ctx, &idx, t, &mut p.r, &mut p.g, &mut p.b, &mut p.a);

    p.next_stage();
}

fn evenly_spaced_2_stop_gradient(p: &mut Pipeline) {
    let ctx = &p.ctx.evenly_spaced_2_stop_gradient;

    let t = join(&p.r, &p.g);
    round_f32_to_u16(
        mad(t, f32x16::splat(ctx.factor.r), f32x16::splat(ctx.bias.r)),
        mad(t, f32x16::splat(ctx.factor.g), f32x16::splat(ctx.bias.g)),
        mad(t, f32x16::splat(ctx.factor.b), f32x16::splat(ctx.bias.b)),
        mad(t, f32x16::splat(ctx.factor.a), f32x16::splat(ctx.bias.a)),
        &mut p.r, &mut p.g, &mut p.b, &mut p.a,
    );

    p.next_stage();
}

fn xy_to_radius(p: &mut Pipeline) {
    let x = join(&p.r, &p.g);
    let y = join(&p.b, &p.a);
    let x = (x*x + y*y).sqrt();
    split(&x, &mut p.r, &mut p.g);
    split(&y, &mut p.b, &mut p.a);

    p.next_stage();
}

// We are using u16 for index, not u32 as Skia, to simplify the code a bit.
// The gradient creation code will not allow that many stops anyway.
fn gradient_lookup(
    ctx: &super::GradientCtx, idx: &u16x16, t: f32x16,
    r: &mut u16x16, g: &mut u16x16, b: &mut u16x16, a: &mut u16x16,
) {
    macro_rules! gather {
        ($d:expr, $c:ident) => {
            // Surprisingly, but bound checking doesn't affect the performance.
            // And since `idx` can contain any number, we should leave it in place.
            f32x16(
                f32x8::from([
                    $d[idx.0[ 0] as usize].$c,
                    $d[idx.0[ 1] as usize].$c,
                    $d[idx.0[ 2] as usize].$c,
                    $d[idx.0[ 3] as usize].$c,
                    $d[idx.0[ 4] as usize].$c,
                    $d[idx.0[ 5] as usize].$c,
                    $d[idx.0[ 6] as usize].$c,
                    $d[idx.0[ 7] as usize].$c,
                ]),
                f32x8::from([
                    $d[idx.0[ 8] as usize].$c,
                    $d[idx.0[ 9] as usize].$c,
                    $d[idx.0[10] as usize].$c,
                    $d[idx.0[11] as usize].$c,
                    $d[idx.0[12] as usize].$c,
                    $d[idx.0[13] as usize].$c,
                    $d[idx.0[14] as usize].$c,
                    $d[idx.0[15] as usize].$c,
                ]),
            )
        };
    }

    let fr = gather!(&ctx.factors, r);
    let fg = gather!(&ctx.factors, g);
    let fb = gather!(&ctx.factors, b);
    let fa = gather!(&ctx.factors, a);

    let br = gather!(&ctx.biases, r);
    let bg = gather!(&ctx.biases, g);
    let bb = gather!(&ctx.biases, b);
    let ba = gather!(&ctx.biases, a);

    round_f32_to_u16(
        mad(t, fr, br),
        mad(t, fg, bg),
        mad(t, fb, bb),
        mad(t, fa, ba),
        r, g, b, a,
    );
}

#[inline(always)]
fn round_f32_to_u16(
    rf: f32x16, gf: f32x16, bf: f32x16, af: f32x16,
    r: &mut u16x16, g: &mut u16x16, b: &mut u16x16, a: &mut u16x16,
) {
    // TODO: may produce a slightly different result to Skia
    //       affects the two_stops_linear_mirror test

    let rf = rf.normalize() * f32x16::splat(255.0) + f32x16::splat(0.5);
    let gf = gf.normalize() * f32x16::splat(255.0) + f32x16::splat(0.5);
    let bf = bf.normalize() * f32x16::splat(255.0) + f32x16::splat(0.5);
    let af = af * f32x16::splat(255.0) + f32x16::splat(0.5);

    rf.save_to_u16x16(r);
    gf.save_to_u16x16(g);
    bf.save_to_u16x16(b);
    af.save_to_u16x16(a);
}

pub fn just_return(_: &mut Pipeline) {
    // Ends the loop.
}

pub fn null_fn(_: &mut Pipeline) {
    // Just for unsupported functions in STAGES.
}

#[inline(always)]
fn load_8888(
    data: &[PremultipliedColorU8; STAGE_WIDTH],
    r: &mut u16x16, g: &mut u16x16, b: &mut u16x16, a: &mut u16x16,
) {
    *r = u16x16([
        data[ 0].red() as u16, data[ 1].red() as u16, data[ 2].red() as u16, data[ 3].red() as u16,
        data[ 4].red() as u16, data[ 5].red() as u16, data[ 6].red() as u16, data[ 7].red() as u16,
        data[ 8].red() as u16, data[ 9].red() as u16, data[10].red() as u16, data[11].red() as u16,
        data[12].red() as u16, data[13].red() as u16, data[14].red() as u16, data[15].red() as u16,
    ]);

    *g = u16x16([
        data[ 0].green() as u16, data[ 1].green() as u16, data[ 2].green() as u16, data[ 3].green() as u16,
        data[ 4].green() as u16, data[ 5].green() as u16, data[ 6].green() as u16, data[ 7].green() as u16,
        data[ 8].green() as u16, data[ 9].green() as u16, data[10].green() as u16, data[11].green() as u16,
        data[12].green() as u16, data[13].green() as u16, data[14].green() as u16, data[15].green() as u16,
    ]);

    *b = u16x16([
        data[ 0].blue() as u16, data[ 1].blue() as u16, data[ 2].blue() as u16, data[ 3].blue() as u16,
        data[ 4].blue() as u16, data[ 5].blue() as u16, data[ 6].blue() as u16, data[ 7].blue() as u16,
        data[ 8].blue() as u16, data[ 9].blue() as u16, data[10].blue() as u16, data[11].blue() as u16,
        data[12].blue() as u16, data[13].blue() as u16, data[14].blue() as u16, data[15].blue() as u16,
    ]);

    *a = u16x16([
        data[ 0].alpha() as u16, data[ 1].alpha() as u16, data[ 2].alpha() as u16, data[ 3].alpha() as u16,
        data[ 4].alpha() as u16, data[ 5].alpha() as u16, data[ 6].alpha() as u16, data[ 7].alpha() as u16,
        data[ 8].alpha() as u16, data[ 9].alpha() as u16, data[10].alpha() as u16, data[11].alpha() as u16,
        data[12].alpha() as u16, data[13].alpha() as u16, data[14].alpha() as u16, data[15].alpha() as u16,
    ]);
}

#[inline(always)]
fn load_8888_tail(
    tail: usize, data: &[PremultipliedColorU8],
    r: &mut u16x16, g: &mut u16x16, b: &mut u16x16, a: &mut u16x16,
) {
    // Fill a dummy array with `tail` values. `tail` is always in a 1..STAGE_WIDTH-1 range.
    // This way we can reuse the `load_8888__` method and remove any branches.
    let mut tmp = [PremultipliedColorU8::TRANSPARENT; STAGE_WIDTH];
    tmp[0..tail].copy_from_slice(&data[0..tail]);
    load_8888(&tmp, r, g, b, a);
}

#[inline(always)]
fn store_8888(
    r: &u16x16, g: &u16x16, b: &u16x16, a: &u16x16,
    data: &mut [PremultipliedColorU8; STAGE_WIDTH],
) {
    let r = r.as_slice();
    let g = g.as_slice();
    let b = b.as_slice();
    let a = a.as_slice();

    data[ 0] = PremultipliedColorU8::from_rgba_unchecked(r[ 0] as u8, g[ 0] as u8, b[ 0] as u8, a[ 0] as u8);
    data[ 1] = PremultipliedColorU8::from_rgba_unchecked(r[ 1] as u8, g[ 1] as u8, b[ 1] as u8, a[ 1] as u8);
    data[ 2] = PremultipliedColorU8::from_rgba_unchecked(r[ 2] as u8, g[ 2] as u8, b[ 2] as u8, a[ 2] as u8);
    data[ 3] = PremultipliedColorU8::from_rgba_unchecked(r[ 3] as u8, g[ 3] as u8, b[ 3] as u8, a[ 3] as u8);
    data[ 4] = PremultipliedColorU8::from_rgba_unchecked(r[ 4] as u8, g[ 4] as u8, b[ 4] as u8, a[ 4] as u8);
    data[ 5] = PremultipliedColorU8::from_rgba_unchecked(r[ 5] as u8, g[ 5] as u8, b[ 5] as u8, a[ 5] as u8);
    data[ 6] = PremultipliedColorU8::from_rgba_unchecked(r[ 6] as u8, g[ 6] as u8, b[ 6] as u8, a[ 6] as u8);
    data[ 7] = PremultipliedColorU8::from_rgba_unchecked(r[ 7] as u8, g[ 7] as u8, b[ 7] as u8, a[ 7] as u8);
    data[ 8] = PremultipliedColorU8::from_rgba_unchecked(r[ 8] as u8, g[ 8] as u8, b[ 8] as u8, a[ 8] as u8);
    data[ 9] = PremultipliedColorU8::from_rgba_unchecked(r[ 9] as u8, g[ 9] as u8, b[ 9] as u8, a[ 9] as u8);
    data[10] = PremultipliedColorU8::from_rgba_unchecked(r[10] as u8, g[10] as u8, b[10] as u8, a[10] as u8);
    data[11] = PremultipliedColorU8::from_rgba_unchecked(r[11] as u8, g[11] as u8, b[11] as u8, a[11] as u8);
    data[12] = PremultipliedColorU8::from_rgba_unchecked(r[12] as u8, g[12] as u8, b[12] as u8, a[12] as u8);
    data[13] = PremultipliedColorU8::from_rgba_unchecked(r[13] as u8, g[13] as u8, b[13] as u8, a[13] as u8);
    data[14] = PremultipliedColorU8::from_rgba_unchecked(r[14] as u8, g[14] as u8, b[14] as u8, a[14] as u8);
    data[15] = PremultipliedColorU8::from_rgba_unchecked(r[15] as u8, g[15] as u8, b[15] as u8, a[15] as u8);
}

#[inline(always)]
fn store_8888_tail(
    r: &u16x16, g: &u16x16, b: &u16x16, a: &u16x16,
    tail: usize, data: &mut [PremultipliedColorU8],
) {
    let r = r.as_slice();
    let g = g.as_slice();
    let b = b.as_slice();
    let a = a.as_slice();

    // This is better than `for i in 0..tail`, because this way the compiler
    // knows that we have only 16 steps and slices access is guarantee to be valid.
    // This removes bounds checking and a possible panic call.
    for i in 0..STAGE_WIDTH {
        data[i] = PremultipliedColorU8::from_rgba_unchecked(
            r[i] as u8, g[i] as u8, b[i] as u8, a[i] as u8,
        );

        if i + 1 == tail {
            break;
        }
    }
}

#[inline(always)]
fn load_8(data: &[u8; STAGE_WIDTH], a: &mut u16x16) {
    *a = u16x16([
        data[ 0] as u16, data[ 1] as u16, data[ 2] as u16, data[ 3] as u16,
        data[ 4] as u16, data[ 5] as u16, data[ 6] as u16, data[ 7] as u16,
        data[ 8] as u16, data[ 9] as u16, data[10] as u16, data[11] as u16,
        data[12] as u16, data[13] as u16, data[14] as u16, data[15] as u16,
    ]);
}

#[inline(always)]
fn div255(v: u16x16) -> u16x16 {
    // Skia uses `vrshrq_n_u16(vrsraq_n_u16(v, v, 8), 8)` here when NEON is available,
    // but it doesn't affect performance much and breaks reproducible result. Ignore it.
    // NOTE: the compiler does not replace the devision with a shift.
    (v + u16x16::splat(255)) >> u16x16::splat(8) // / u16x16::splat(256)
}

#[inline(always)]
fn inv(v: u16x16) -> u16x16 {
    u16x16::splat(255) - v
}

#[inline(always)]
fn from_float(f: f32) -> u16x16 {
    u16x16::splat((f * 255.0 + 0.5) as u16)
}

#[inline(always)]
fn lerp(from: u16x16, to: u16x16, t: u16x16) -> u16x16 {
    div255(from * inv(t) + to * t)
}

#[inline(always)]
fn split(v: &f32x16, lo: &mut u16x16, hi: &mut u16x16) {
    // We're splitting f32x16 (512bit) into two u16x16 (256 bit).
    let data: [u8; 64] = bytemuck::cast(*v);
    let d0: &mut [u8; 32] = bytemuck::cast_mut(&mut lo.0);
    let d1: &mut [u8; 32] = bytemuck::cast_mut(&mut hi.0);

    d0.copy_from_slice(&data[0..32]);
    d1.copy_from_slice(&data[32..64]);
}

#[inline(always)]
fn join(lo: &u16x16, hi: &u16x16) -> f32x16 {
    // We're joining two u16x16 (256 bit) into f32x16 (512bit).

    let d0: [u8; 32] = bytemuck::cast(lo.0);
    let d1: [u8; 32] = bytemuck::cast(hi.0);

    let mut v = f32x16::default();
    let data: &mut [u8; 64] = bytemuck::cast_mut(&mut v);

    data[0..32].copy_from_slice(&d0);
    data[32..64].copy_from_slice(&d1);

    v
}

#[inline(always)]
fn mad(f: f32x16, m: f32x16, a: f32x16) -> f32x16 {
    // NEON vmlaq_f32 doesn't seem to affect performance in any way. Ignore it.
    f * m + a
}
