// Copyright 2011 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use core::convert::TryFrom;
use core::num::NonZeroU16;

use crate::{IntRect, LengthU32, LineCap, Path, Point, Rect};

use crate::alpha_runs::{AlphaRun, AlphaRuns};
use crate::blitter::Blitter;
use crate::color::AlphaU8;
use crate::fixed_point::{fdot16, fdot6, fdot8, FDot16, FDot6, FDot8};
use crate::geom::{IntRectExt, ScreenIntRect};
use crate::line_clipper;
use crate::math::LENGTH_U32_ONE;

#[derive(Copy, Clone, Debug)]
struct FixedRect {
    left: FDot16,
    top: FDot16,
    right: FDot16,
    bottom: FDot16,
}

impl FixedRect {
    fn from_rect(src: &Rect) -> Self {
        FixedRect {
            left: fdot16::from_f32(src.left()),
            top: fdot16::from_f32(src.top()),
            right: fdot16::from_f32(src.right()),
            bottom: fdot16::from_f32(src.bottom()),
        }
    }
}

/// Multiplies value by 0..256, and shift the result down 8
/// (i.e. return (value * alpha256) >> 8)
fn alpha_mul(value: AlphaU8, alpha256: i32) -> u8 {
    let a = (i32::from(value) * alpha256) >> 8;
    debug_assert!(a >= 0 && a <= 255);
    a as u8
}

pub fn fill_rect(rect: &Rect, clip: &ScreenIntRect, blitter: &mut dyn Blitter) {
    let rect = match rect.intersect(&clip.to_rect()) {
        Some(v) => v,
        None => return, // everything was clipped out
    };

    let fr = FixedRect::from_rect(&rect);
    fill_fixed_rect(&fr, blitter);
}

fn fill_fixed_rect(rect: &FixedRect, blitter: &mut dyn Blitter) {
    fill_dot8(
        fdot8::from_fdot16(rect.left),
        fdot8::from_fdot16(rect.top),
        fdot8::from_fdot16(rect.right),
        fdot8::from_fdot16(rect.bottom),
        true,
        blitter,
    )
}

fn fill_dot8(l: FDot8, t: FDot8, r: FDot8, b: FDot8, fill_inner: bool, blitter: &mut dyn Blitter) {
    fn to_alpha(a: i32) -> u8 {
        debug_assert!(a >= 0 && a <= 255);
        a as u8
    }

    // check for empty now that we're in our reduced precision space
    if l >= r || t >= b {
        return;
    }

    let mut top = t >> 8;
    if top == ((b - 1) >> 8) {
        // just one scanline high
        do_scanline(l, top, r, to_alpha(b - t - 1), blitter);
        return;
    }

    if t & 0xFF != 0 {
        do_scanline(l, top, r, to_alpha(256 - (t & 0xFF)), blitter);
        top += 1;
    }

    let bottom = b >> 8;
    let height = bottom - top;
    if let Some(height) = u32::try_from(height).ok().and_then(LengthU32::new) {
        let mut left = l >> 8;
        if left == ((r - 1) >> 8) {
            // just 1-pixel wide
            if let (Ok(left), Ok(top)) = (u32::try_from(left), u32::try_from(top)) {
                blitter.blit_v(left, top, height, to_alpha(r - l - 1));
            } else {
                debug_assert!(false);
            }
        } else {
            if l & 0xFF != 0 {
                if let (Ok(left), Ok(top)) = (u32::try_from(left), u32::try_from(top)) {
                    blitter.blit_v(left, top, height, to_alpha(256 - (l & 0xFF)));
                } else {
                    debug_assert!(false);
                }

                left += 1;
            }

            let right = r >> 8;
            let width = right - left;
            if fill_inner {
                if let Some(width) = u32::try_from(width).ok().and_then(LengthU32::new) {
                    if let (Ok(left), Ok(top)) = (u32::try_from(left), u32::try_from(top)) {
                        let rect = ScreenIntRect::from_xywh_safe(left, top, width, height);
                        blitter.blit_rect(&rect);
                    } else {
                        debug_assert!(false);
                    }
                } else {
                    debug_assert!(false);
                }
            }

            if r & 0xFF != 0 {
                if let (Ok(right), Ok(top)) = (u32::try_from(right), u32::try_from(top)) {
                    blitter.blit_v(right, top, height, to_alpha(r & 0xFF));
                } else {
                    debug_assert!(false);
                }
            }
        }
    }

    if b & 0xFF != 0 {
        do_scanline(l, bottom, r, to_alpha(b & 0xFF), blitter);
    }
}

fn do_scanline(l: FDot8, top: i32, r: FDot8, alpha: AlphaU8, blitter: &mut dyn Blitter) {
    debug_assert!(l < r);

    let one_len = LENGTH_U32_ONE;
    let top = match u32::try_from(top) {
        Ok(n) => n,
        _ => return,
    };

    if (l >> 8) == ((r - 1) >> 8) {
        // 1x1 pixel
        if let Ok(left) = u32::try_from(l >> 8) {
            blitter.blit_v(left, top, one_len, alpha_mul(alpha, r - l));
        }

        return;
    }

    let mut left = l >> 8;

    if l & 0xFF != 0 {
        if let Ok(left) = u32::try_from(l >> 8) {
            blitter.blit_v(left, top, one_len, alpha_mul(alpha, 256 - (l & 0xFF)));
        }

        left += 1;
    }

    let right = r >> 8;
    let width = right - left;
    if let Some(width) = u32::try_from(width).ok().and_then(LengthU32::new) {
        if let Ok(left) = u32::try_from(left) {
            call_hline_blitter(left, Some(top), width, alpha, blitter);
        }
    }

    if r & 0xFF != 0 {
        if let Ok(right) = u32::try_from(right) {
            blitter.blit_v(right, top, one_len, alpha_mul(alpha, r & 0xFF));
        }
    }
}

fn call_hline_blitter(
    mut x: u32,
    y: Option<u32>,
    count: LengthU32,
    alpha: AlphaU8,
    blitter: &mut dyn Blitter,
) {
    const HLINE_STACK_BUFFER: usize = 100;

    let mut runs = [None; HLINE_STACK_BUFFER + 1];
    let mut aa = [0u8; HLINE_STACK_BUFFER];

    let mut count = count.get();
    loop {
        // In theory, we should be able to just do this once (outside of the loop),
        // since aa[] and runs[] are supposed" to be const when we call the blitter.
        // In reality, some wrapper-blitters (e.g. RgnClipBlitter) cast away that
        // constness, and modify the buffers in-place. Hence the need to be defensive
        // here and reseed the aa value.
        aa[0] = alpha;

        let mut n = count;
        if n > HLINE_STACK_BUFFER as u32 {
            n = HLINE_STACK_BUFFER as u32;
        }

        debug_assert!(n <= u16::MAX as u32);
        runs[0] = NonZeroU16::new(n as u16);
        runs[n as usize] = None;
        if let Some(y) = y {
            blitter.blit_anti_h(x, y, &mut aa, &mut runs);
        }
        x += n;

        if n >= count || count == 0 {
            break;
        }

        count -= n;
    }
}

pub fn stroke_path(
    path: &Path,
    line_cap: LineCap,
    clip: &ScreenIntRect,
    blitter: &mut dyn Blitter,
) {
    super::hairline::stroke_path_impl(path, line_cap, clip, anti_hair_line_rgn, blitter);
}

fn anti_hair_line_rgn(points: &[Point], clip: Option<&ScreenIntRect>, blitter: &mut dyn Blitter) {
    let max = 32767.0;
    let fixed_bounds = Rect::from_ltrb(-max, -max, max, max).unwrap();

    let clip_bounds = if let Some(clip) = clip {
        // We perform integral clipping later on, but we do a scalar clip first
        // to ensure that our coordinates are expressible in fixed/integers.
        //
        // antialiased hairlines can draw up to 1/2 of a pixel outside of
        // their bounds, so we need to outset the clip before calling the
        // clipper. To make the numerics safer, we outset by a whole pixel,
        // since the 1/2 pixel boundary is important to the antihair blitter,
        // we don't want to risk numerical fate by chopping on that edge.
        clip.to_rect().outset(1.0, 1.0)
    } else {
        None
    };

    for i in 0..points.len() - 1 {
        let mut pts = [Point::zero(); 2];

        // We have to pre-clip the line to fit in a Fixed, so we just chop the line.
        if !line_clipper::intersect(&[points[i], points[i + 1]], &fixed_bounds, &mut pts) {
            continue;
        }

        if let Some(clip_bounds) = clip_bounds {
            let tmp = pts;
            if !line_clipper::intersect(&tmp, &clip_bounds, &mut pts) {
                continue;
            }
        }

        let x0 = fdot6::from_f32(pts[0].x);
        let y0 = fdot6::from_f32(pts[0].y);
        let x1 = fdot6::from_f32(pts[1].x);
        let y1 = fdot6::from_f32(pts[1].y);

        if let Some(clip) = clip {
            let left = x0.min(x1);
            let top = y0.min(y1);
            let right = x0.max(x1);
            let bottom = y0.max(y1);

            let ir = IntRect::from_ltrb(
                fdot6::floor(left) - 1,
                fdot6::floor(top) - 1,
                fdot6::ceil(right) + 1,
                fdot6::ceil(bottom) + 1,
            );
            let ir = match ir {
                Some(v) => v,
                None => return,
            };

            if clip.to_int_rect().intersect(&ir).is_none() {
                continue;
            }

            if !clip.to_int_rect().contains(&ir) {
                let subclip = ir
                    .intersect(&clip.to_int_rect())
                    .and_then(|r| r.to_screen_int_rect());

                if let Some(subclip) = subclip {
                    do_anti_hairline(x0, y0, x1, y1, Some(subclip), blitter);
                }

                continue;
            }

            // fall through to no-clip case
        }

        do_anti_hairline(x0, y0, x1, y1, None, blitter);
    }
}

#[derive(Copy, Clone, Debug)]
enum BlitterKind {
    HLine,
    Horish,
    VLine,
    Vertish,
}

fn do_anti_hairline(
    mut x0: FDot6,
    mut y0: FDot6,
    mut x1: FDot6,
    mut y1: FDot6,
    mut clip_opt: Option<ScreenIntRect>,
    blitter: &mut dyn Blitter,
) {
    // check for integer NaN (0x80000000) which we can't handle (can't negate it)
    // It appears typically from a huge float (inf or nan) being converted to int.
    // If we see it, just don't draw.
    if any_bad_ints(x0, y0, x1, y1) != 0 {
        return;
    }

    // The caller must clip the line to [-32767.0 ... 32767.0] ahead of time  (in dot6 format)
    debug_assert!(fdot6::can_convert_to_fdot16(x0));
    debug_assert!(fdot6::can_convert_to_fdot16(y0));
    debug_assert!(fdot6::can_convert_to_fdot16(x1));
    debug_assert!(fdot6::can_convert_to_fdot16(y1));

    if (x1 - x0).abs() > fdot6::from_i32(511) || (y1 - y0).abs() > fdot6::from_i32(511) {
        // instead of (x0 + x1) >> 1, we shift each separately. This is less
        // precise, but avoids overflowing the intermediate result if the
        // values are huge. A better fix might be to clip the original pts
        // directly (i.e. do the divide), so we don't spend time subdividing
        // huge lines at all.
        let hx = (x0 >> 1) + (x1 >> 1);
        let hy = (y0 >> 1) + (y1 >> 1);
        do_anti_hairline(x0, y0, hx, hy, clip_opt, blitter);
        do_anti_hairline(hx, hy, x1, y1, clip_opt, blitter);
        return; // we're done
    }

    let mut scale_start;
    let mut scale_stop;
    let mut istart;
    let mut istop;
    let mut fstart;
    let slope;
    let blitter_kind;

    if (x1 - x0).abs() > (y1 - y0).abs() {
        // mostly horizontal

        if x0 > x1 {
            // we want to go left-to-right
            core::mem::swap(&mut x0, &mut x1);
            core::mem::swap(&mut y0, &mut y1);
        }

        istart = fdot6::floor(x0);
        istop = fdot6::ceil(x1);
        fstart = fdot6::to_fdot16(y0);
        if y0 == y1 {
            // completely horizontal, take fast case
            slope = 0;
            blitter_kind = Some(BlitterKind::HLine);
        } else {
            slope = fdot16::fast_div(y1 - y0, x1 - x0);
            debug_assert!(slope >= -fdot16::ONE && slope <= fdot16::ONE);
            fstart += (slope * (32 - (x0 & 63)) + 32) >> 6;
            blitter_kind = Some(BlitterKind::Horish);
        }

        debug_assert!(istop > istart);
        if istop - istart == 1 {
            // we are within a single pixel
            scale_start = x1 - x0;
            debug_assert!(scale_start >= 0 && scale_start <= 64);
            scale_stop = 0;
        } else {
            scale_start = 64 - (x0 & 63);
            scale_stop = x1 & 63;
        }

        if let Some(clip) = clip_opt {
            let clip = clip.to_int_rect();

            if istart >= clip.right() || istop <= clip.left() {
                return; // we're done
            }

            if istart < clip.left() {
                fstart += slope * (clip.left() - istart);
                istart = clip.left();
                scale_start = 64;
                if istop - istart == 1 {
                    // we are within a single pixel
                    scale_start = contribution_64(x1);
                    scale_stop = 0;
                }
            }

            if istop > clip.right() {
                istop = clip.right();
                scale_stop = 0; // so we don't draw this last column
            }

            debug_assert!(istart <= istop);
            if istart == istop {
                return; // we're done
            }

            // now test if our Y values are completely inside the clip
            let (mut top, mut bottom) = if slope >= 0 {
                // T2B
                let top = fdot16::floor_to_i32(fstart - fdot16::HALF);
                let bottom =
                    fdot16::ceil_to_i32(fstart + (istop - istart - 1) * slope + fdot16::HALF);
                (top, bottom)
            } else {
                // B2T
                let bottom = fdot16::ceil_to_i32(fstart + fdot16::HALF);
                let top =
                    fdot16::floor_to_i32(fstart + (istop - istart - 1) * slope - fdot16::HALF);
                (top, bottom)
            };

            top -= 1;
            bottom += 1;

            if top >= clip.bottom() || bottom <= clip.top() {
                return; // we're done
            }

            if clip.top() <= top && clip.bottom() >= bottom {
                clip_opt = None;
            }
        }
    } else {
        // mostly vertical

        if y0 > y1 {
            // we want to go top-to-bottom
            core::mem::swap(&mut x0, &mut x1);
            core::mem::swap(&mut y0, &mut y1);
        }

        istart = fdot6::floor(y0);
        istop = fdot6::ceil(y1);
        fstart = fdot6::to_fdot16(x0);
        if x0 == x1 {
            if y0 == y1 {
                // are we zero length? nothing to do
                return; // we're done
            }

            slope = 0;
            blitter_kind = Some(BlitterKind::VLine);
        } else {
            slope = fdot16::fast_div(x1 - x0, y1 - y0);
            debug_assert!(slope <= fdot16::ONE && slope >= -fdot16::ONE);
            fstart += (slope * (32 - (y0 & 63)) + 32) >> 6;
            blitter_kind = Some(BlitterKind::Vertish);
        }

        debug_assert!(istop > istart);
        if istop - istart == 1 {
            // we are within a single pixel
            scale_start = y1 - y0;
            debug_assert!(scale_start >= 0 && scale_start <= 64);
            scale_stop = 0;
        } else {
            scale_start = 64 - (y0 & 63);
            scale_stop = y1 & 63;
        }

        if let Some(clip) = clip_opt {
            let clip = clip.to_int_rect();

            if istart >= clip.bottom() || istop <= clip.top() {
                return; // we're done
            }

            if istart < clip.top() {
                fstart += slope * (clip.top() - istart);
                istart = clip.top();
                scale_start = 64;
                if istop - istart == 1 {
                    // we are within a single pixel
                    scale_start = contribution_64(y1);
                    scale_stop = 0;
                }
            }
            if istop > clip.bottom() {
                istop = clip.bottom();
                scale_stop = 0; // so we don't draw this last row
            }

            debug_assert!(istart <= istop);
            if istart == istop {
                return; // we're done
            }

            // now test if our X values are completely inside the clip
            let (mut left, mut right) = if slope >= 0 {
                // L2R
                let left = fdot16::floor_to_i32(fstart - fdot16::HALF);
                let right =
                    fdot16::ceil_to_i32(fstart + (istop - istart - 1) * slope + fdot16::HALF);
                (left, right)
            } else {
                // R2L
                let right = fdot16::ceil_to_i32(fstart + fdot16::HALF);
                let left =
                    fdot16::floor_to_i32(fstart + (istop - istart - 1) * slope - fdot16::HALF);
                (left, right)
            };

            left -= 1;
            right += 1;

            if left >= clip.right() || right <= clip.left() {
                return; // we're done
            }

            if clip.left() <= left && clip.right() >= right {
                clip_opt = None;
            }
        }
    }

    let mut clip_blitter;
    let blitter = if let Some(clip) = clip_opt {
        clip_blitter = RectClipBlitter { blitter, clip };
        &mut clip_blitter
    } else {
        blitter
    };

    let blitter_kind = match blitter_kind {
        Some(v) => v,
        None => return,
    };

    // A bit ugly, but looks like this is the only way to have stack allocated object trait.
    let mut hline_blitter;
    let mut horish_blitter;
    let mut vline_blitter;
    let mut vertish_blitter;
    let hair_blitter: &mut dyn AntiHairBlitter = match blitter_kind {
        BlitterKind::HLine => {
            hline_blitter = HLineAntiHairBlitter(blitter);
            &mut hline_blitter
        }
        BlitterKind::Horish => {
            horish_blitter = HorishAntiHairBlitter(blitter);
            &mut horish_blitter
        }
        BlitterKind::VLine => {
            vline_blitter = VLineAntiHairBlitter(blitter);
            &mut vline_blitter
        }
        BlitterKind::Vertish => {
            vertish_blitter = VertishAntiHairBlitter(blitter);
            &mut vertish_blitter
        }
    };

    debug_assert!(istart >= 0);
    let mut istart = istart as u32;

    debug_assert!(istop >= 0);
    let istop = istop as u32;

    fstart = hair_blitter.draw_cap(istart, fstart, slope, scale_start);
    istart += 1;
    let full_spans = istop - istart - (scale_stop > 0) as u32;
    if full_spans > 0 {
        fstart = hair_blitter.draw_line(istart, istart + full_spans, fstart, slope);
    }

    if scale_stop > 0 {
        hair_blitter.draw_cap(istop - 1, fstart, slope, scale_stop);
    }
}

// returns high-bit set if x == 0x8000
fn bad_int(x: i32) -> i32 {
    x & -x
}

fn any_bad_ints(a: i32, b: i32, c: i32, d: i32) -> i32 {
    (bad_int(a) | bad_int(b) | bad_int(c) | bad_int(d)) >> ((core::mem::size_of::<i32>() << 3) - 1)
}

// We want the fractional part of ordinate, but we want multiples of 64 to
// return 64, not 0, so we can't just say (ordinate & 63).
// We basically want to compute those bits, and if they're 0, return 64.
// We can do that w/o a branch with an extra sub and add.
fn contribution_64(ordinate: FDot6) -> i32 {
    let result = ((ordinate - 1) & 63) + 1;
    debug_assert!(result > 0 && result <= 64);
    result
}

trait AntiHairBlitter {
    fn draw_cap(&mut self, x: u32, fy: FDot16, slope: FDot16, mod64: i32) -> FDot16;
    fn draw_line(&mut self, x: u32, stopx: u32, fy: FDot16, slope: FDot16) -> FDot16;
}

struct HLineAntiHairBlitter<'a>(&'a mut dyn Blitter);

impl AntiHairBlitter for HLineAntiHairBlitter<'_> {
    fn draw_cap(&mut self, x: u32, mut fy: FDot16, _: FDot16, mod64: i32) -> FDot16 {
        fy += fdot16::ONE / 2;
        fy = fy.max(0);

        let y = (fy >> 16) as u32;
        let a = i32_to_alpha(fy >> 8);

        // lower line
        let mut ma = fdot6::small_scale(a, mod64);
        if ma != 0 {
            call_hline_blitter(x, Some(y), LENGTH_U32_ONE, ma, self.0);
        }

        // upper line
        ma = fdot6::small_scale(255 - a, mod64);
        if ma != 0 {
            call_hline_blitter(x, y.checked_sub(1), LENGTH_U32_ONE, ma, self.0);
        }

        fy - fdot16::ONE / 2
    }

    fn draw_line(&mut self, x: u32, stop_x: u32, mut fy: FDot16, _: FDot16) -> FDot16 {
        let count = match LengthU32::new(stop_x - x) {
            Some(n) => n,
            None => return fy,
        };

        fy += fdot16::ONE / 2;
        fy = fy.max(0);

        let y = (fy >> 16) as u32;
        let mut a = i32_to_alpha(fy >> 8);

        // lower line
        if a != 0 {
            call_hline_blitter(x, Some(y), count, a, self.0);
        }

        // upper line
        a = 255 - a;
        if a != 0 {
            call_hline_blitter(x, y.checked_sub(1), count, a, self.0);
        }

        fy - fdot16::ONE / 2
    }
}

struct HorishAntiHairBlitter<'a>(&'a mut dyn Blitter);

impl AntiHairBlitter for HorishAntiHairBlitter<'_> {
    fn draw_cap(&mut self, x: u32, mut fy: FDot16, dy: FDot16, mod64: i32) -> FDot16 {
        fy += fdot16::ONE / 2;
        fy = fy.max(0);

        let lower_y = (fy >> 16) as u32;
        let a = i32_to_alpha(fy >> 8);
        let a0 = fdot6::small_scale(255 - a, mod64);
        let a1 = fdot6::small_scale(a, mod64);
        self.0.blit_anti_v2(x, lower_y.max(1) - 1, a0, a1);

        fy + dy - fdot16::ONE / 2
    }

    fn draw_line(&mut self, mut x: u32, stop_x: u32, mut fy: FDot16, dy: FDot16) -> FDot16 {
        debug_assert!(x < stop_x);

        fy += fdot16::ONE / 2;
        loop {
            fy = fy.max(0);
            let lower_y = (fy >> 16) as u32;
            let a = i32_to_alpha(fy >> 8);
            self.0.blit_anti_v2(x, lower_y.max(1) - 1, 255 - a, a);
            fy += dy;

            x += 1;
            if x >= stop_x {
                break;
            }
        }

        fy - fdot16::ONE / 2
    }
}

struct VLineAntiHairBlitter<'a>(&'a mut dyn Blitter);

impl AntiHairBlitter for VLineAntiHairBlitter<'_> {
    fn draw_cap(&mut self, y: u32, mut fx: FDot16, dx: FDot16, mod64: i32) -> FDot16 {
        debug_assert!(dx == 0);
        fx += fdot16::ONE / 2;
        fx = fx.max(0);

        let x = (fx >> 16) as u32;
        let a = i32_to_alpha(fx >> 8);

        let mut ma = fdot6::small_scale(a, mod64);
        if ma != 0 {
            self.0.blit_v(x, y, LENGTH_U32_ONE, ma);
        }

        ma = fdot6::small_scale(255 - a, mod64);
        if ma != 0 {
            self.0.blit_v(x.max(1) - 1, y, LENGTH_U32_ONE, ma);
        }

        fx - fdot16::ONE / 2
    }

    fn draw_line(&mut self, y: u32, stop_y: u32, mut fx: FDot16, dx: FDot16) -> FDot16 {
        debug_assert!(dx == 0);
        let height = match LengthU32::new(stop_y - y) {
            Some(n) => n,
            None => return fx,
        };

        fx += fdot16::ONE / 2;
        fx = fx.max(0);

        let x = (fx >> 16) as u32;
        let mut a = i32_to_alpha(fx >> 8);

        if a != 0 {
            self.0.blit_v(x, y, height, a);
        }

        a = 255 - a;
        if a != 0 {
            self.0.blit_v(x.max(1) - 1, y, height, a);
        }

        fx - fdot16::ONE / 2
    }
}

struct VertishAntiHairBlitter<'a>(&'a mut dyn Blitter);

impl AntiHairBlitter for VertishAntiHairBlitter<'_> {
    fn draw_cap(&mut self, y: u32, mut fx: FDot16, dx: FDot16, mod64: i32) -> FDot16 {
        fx += fdot16::ONE / 2;
        fx = fx.max(0);

        let x = (fx >> 16) as u32;
        let a = i32_to_alpha(fx >> 8);
        self.0.blit_anti_h2(
            x.max(1) - 1,
            y,
            fdot6::small_scale(255 - a, mod64),
            fdot6::small_scale(a, mod64),
        );

        fx + dx - fdot16::ONE / 2
    }

    fn draw_line(&mut self, mut y: u32, stop_y: u32, mut fx: FDot16, dx: FDot16) -> FDot16 {
        debug_assert!(y < stop_y);

        fx += fdot16::ONE / 2;
        loop {
            fx = fx.max(0);
            let x = (fx >> 16) as u32;
            let a = i32_to_alpha(fx >> 8);
            self.0.blit_anti_h2(x.max(1) - 1, y, 255 - a, a);
            fx += dx;

            y += 1;
            if y >= stop_y {
                break;
            }
        }

        fx - fdot16::ONE / 2
    }
}

fn i32_to_alpha(a: i32) -> u8 {
    (a & 0xFF) as u8
}

struct RectClipBlitter<'a> {
    blitter: &'a mut dyn Blitter,
    clip: ScreenIntRect,
}

impl Blitter for RectClipBlitter<'_> {
    fn blit_anti_h(
        &mut self,
        x: u32,
        y: u32,
        mut antialias: &mut [AlphaU8],
        mut runs: &mut [AlphaRun],
    ) {
        fn y_in_rect(y: u32, rect: ScreenIntRect) -> bool {
            (y - rect.top()) < rect.height()
        }

        if !y_in_rect(y, self.clip) || x >= self.clip.right() {
            return;
        }

        let mut x0 = x;
        let mut x1 = x + compute_anti_width(runs);

        if x1 <= self.clip.left() {
            return;
        }

        debug_assert!(x0 < x1);
        if x0 < self.clip.left() {
            let dx = self.clip.left() - x0;
            AlphaRuns::break_at(antialias, runs, dx as i32);
            antialias = &mut antialias[dx as usize..];
            runs = &mut runs[dx as usize..];
            x0 = self.clip.left();
        }

        debug_assert!(x0 < x1 && runs[(x1 - x0) as usize].is_none());
        if x1 > self.clip.right() {
            x1 = self.clip.right();
            AlphaRuns::break_at(antialias, runs, (x1 - x0) as i32);
            runs[(x1 - x0) as usize] = None;
        }

        debug_assert!(x0 < x1 && runs[(x1 - x0) as usize].is_none());
        debug_assert!(compute_anti_width(runs) == x1 - x0);

        self.blitter.blit_anti_h(x0, y, antialias, runs);
    }

    fn blit_v(&mut self, x: u32, y: u32, height: LengthU32, alpha: AlphaU8) {
        fn x_in_rect(x: u32, rect: ScreenIntRect) -> bool {
            (x - rect.left()) < rect.width()
        }

        if !x_in_rect(x, self.clip) {
            return;
        }

        let mut y0 = y;
        let mut y1 = y + height.get();

        if y0 < self.clip.top() {
            y0 = self.clip.top();
        }

        if y1 > self.clip.bottom() {
            y1 = self.clip.bottom();
        }

        if y0 < y1 {
            if let Some(h) = LengthU32::new(y1 - y0) {
                self.blitter.blit_v(x, y0, h, alpha);
            }
        }
    }

    fn blit_anti_h2(&mut self, x: u32, y: u32, alpha0: AlphaU8, alpha1: AlphaU8) {
        self.blit_anti_h(
            x,
            y,
            &mut [alpha0, alpha1],
            &mut [NonZeroU16::new(1), NonZeroU16::new(1), None],
        );
    }

    fn blit_anti_v2(&mut self, x: u32, y: u32, alpha0: AlphaU8, alpha1: AlphaU8) {
        self.blit_anti_h(x, y, &mut [alpha0], &mut [NonZeroU16::new(1), None]);

        self.blit_anti_h(x, y + 1, &mut [alpha1], &mut [NonZeroU16::new(1), None]);
    }
}

fn compute_anti_width(runs: &[AlphaRun]) -> u32 {
    let mut i = 0;
    let mut width = 0;
    while let Some(count) = runs[i] {
        width += u32::from(count.get());
        i += usize::from(count.get());
    }

    width
}
