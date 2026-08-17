// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::color::ChromaSampling::Cs400;
use crate::context::*;
use crate::encoder::FrameInvariants;
use crate::frame::*;
use crate::tiling::*;
use crate::util::{clamp, msb, CastFromPrimitive, Pixel};

use crate::cpu_features::CpuFeatureLevel;
use std::cmp;

cfg_if::cfg_if! {
  if #[cfg(nasm_x86_64)] {
    pub(crate) use crate::asm::x86::cdef::*;
  } else if #[cfg(asm_neon)] {
    pub(crate) use crate::asm::aarch64::cdef::*;
  } else {
    pub(crate) use self::rust::*;
  }
}

pub const CDEF_VERY_LARGE: u16 = 0x8000;
// These values match dav1d; flags indicating where padding exists
pub const CDEF_HAVE_LEFT: u8 = 1 << 0;
pub const CDEF_HAVE_RIGHT: u8 = 1 << 1;
pub const CDEF_HAVE_TOP: u8 = 1 << 2;
pub const CDEF_HAVE_BOTTOM: u8 = 1 << 3;
pub const CDEF_HAVE_ALL: u8 =
  CDEF_HAVE_LEFT | CDEF_HAVE_RIGHT | CDEF_HAVE_TOP | CDEF_HAVE_BOTTOM;

pub(crate) const CDEF_SEC_STRENGTHS: u8 = 4;

pub struct CdefDirections {
  dir: [[u8; 8]; 8],
  var: [[i32; 8]; 8],
}

pub(crate) mod rust {
  use super::*;

  use simd_helpers::cold_for_target_arch;

  // Instead of dividing by n between 2 and 8, we multiply by 3*5*7*8/n.
  // The output is then 840 times larger, but we don't care for finding
  // the max.
  const CDEF_DIV_TABLE: [i32; 9] = [0, 840, 420, 280, 210, 168, 140, 120, 105];

  /// Returns the position and value of the first instance of the max element in
  /// a slice as a tuple.
  ///
  /// # Arguments
  ///
  /// * `elems` - A non-empty slice of integers
  ///
  /// # Panics
  ///
  /// Panics if `elems` is empty
  #[inline]
  fn first_max_element(elems: &[i32]) -> (usize, i32) {
    // In case of a tie, the first element must be selected.
    let (max_idx, max_value) = elems
      .iter()
      .enumerate()
      .max_by_key(|&(i, v)| (v, -(i as isize)))
      .unwrap();
    (max_idx, *max_value)
  }

  // Detect direction. 0 means 45-degree up-right, 2 is horizontal, and so on.
  // The search minimizes the weighted variance along all the lines in a
  // particular direction, i.e. the squared error between the input and a
  // "predicted" block where each pixel is replaced by the average along a line
  // in a particular direction. Since each direction have the same sum(x^2) term,
  // that term is never computed. See Section 2, step 2, of:
  // http://jmvalin.ca/notes/intra_paint.pdf
  pub fn cdef_find_dir<T: Pixel>(
    img: &PlaneSlice<'_, T>, var: &mut u32, coeff_shift: usize,
    _cpu: CpuFeatureLevel,
  ) -> i32 {
    let mut cost: [i32; 8] = [0; 8];
    let mut partial: [[i32; 15]; 8] = [[0; 15]; 8];
    for i in 0..8 {
      for j in 0..8 {
        let p: i32 = i32::cast_from(img[i][j]);
        // We subtract 128 here to reduce the maximum range of the squared
        // partial sums.
        debug_assert!(p >> coeff_shift <= 255);
        let x = (p >> coeff_shift) - 128;
        partial[0][i + j] += x;
        partial[1][i + j / 2] += x;
        partial[2][i] += x;
        partial[3][3 + i - j / 2] += x;
        partial[4][7 + i - j] += x;
        partial[5][3 - i / 2 + j] += x;
        partial[6][j] += x;
        partial[7][i / 2 + j] += x;
      }
    }
    for i in 0..8 {
      cost[2] += partial[2][i] * partial[2][i];
      cost[6] += partial[6][i] * partial[6][i];
    }
    cost[2] *= CDEF_DIV_TABLE[8];
    cost[6] *= CDEF_DIV_TABLE[8];
    for i in 0..7 {
      cost[0] += (partial[0][i] * partial[0][i]
        + partial[0][14 - i] * partial[0][14 - i])
        * CDEF_DIV_TABLE[i + 1];
      cost[4] += (partial[4][i] * partial[4][i]
        + partial[4][14 - i] * partial[4][14 - i])
        * CDEF_DIV_TABLE[i + 1];
    }
    cost[0] += partial[0][7] * partial[0][7] * CDEF_DIV_TABLE[8];
    cost[4] += partial[4][7] * partial[4][7] * CDEF_DIV_TABLE[8];
    for i in (1..8).step_by(2) {
      for j in 0..5 {
        cost[i] += partial[i][3 + j] * partial[i][3 + j];
      }
      cost[i] *= CDEF_DIV_TABLE[8];
      for j in 0..3 {
        cost[i] += (partial[i][j] * partial[i][j]
          + partial[i][10 - j] * partial[i][10 - j])
          * CDEF_DIV_TABLE[2 * j + 2];
      }
    }

    let (best_dir, best_cost) = first_max_element(&cost);
    // Difference between the optimal variance and the variance along the
    // orthogonal direction. Again, the sum(x^2) terms cancel out.
    // We'd normally divide by 840, but dividing by 1024 is close enough
    // for what we're going to do with this. */
    *var = ((best_cost - cost[(best_dir + 4) & 7]) >> 10) as u32;

    best_dir as i32
  }

  #[inline(always)]
  fn constrain(diff: i32, threshold: i32, damping: i32) -> i32 {
    if threshold != 0 {
      let shift = cmp::max(0, damping - msb(threshold));
      let magnitude = (threshold - (diff.abs() >> shift)).clamp(0, diff.abs());

      if diff < 0 {
        -magnitude
      } else {
        magnitude
      }
    } else {
      0
    }
  }

  pub unsafe fn pad_into_tmp16<T: Pixel>(
    dst: *mut u16, dst_stride: isize, src: *const T, src_stride: isize,
    block_width: usize, block_height: usize, edges: u8,
  ) {
    let mut w = block_width;
    let mut h = block_height;
    let (dst_col, src_col) = if (edges & CDEF_HAVE_LEFT) != 0 {
      w += 2;
      (dst, src.offset(-2))
    } else {
      (dst.offset(2), src)
    };
    if (edges & CDEF_HAVE_RIGHT) != 0 {
      w += 2;
    };

    let (mut dst_ptr, mut src_ptr) = if (edges & CDEF_HAVE_TOP) != 0 {
      h += 2;
      (dst_col, src_col.offset(-2 * src_stride))
    } else {
      (dst_col.offset(2 * dst_stride), src_col)
    };
    if (edges & CDEF_HAVE_BOTTOM) != 0 {
      h += 2;
    };

    for _y in 0..h {
      for x in 0..w {
        *dst_ptr.add(x) = u16::cast_from(*src_ptr.add(x));
      }
      src_ptr = src_ptr.offset(src_stride);
      dst_ptr = dst_ptr.offset(dst_stride);
    }
  }

  #[cold_for_target_arch("x86_64")]
  #[allow(clippy::erasing_op, clippy::identity_op, clippy::neg_multiply)]
  pub(crate) unsafe fn cdef_filter_block<T: Pixel, U: Pixel>(
    dst: &mut PlaneRegionMut<'_, T>, input: *const U, istride: isize,
    pri_strength: i32, sec_strength: i32, dir: usize, damping: i32,
    bit_depth: usize, xdec: usize, ydec: usize, edges: u8,
    _cpu: CpuFeatureLevel,
  ) {
    if edges != CDEF_HAVE_ALL {
      // slowpath for unpadded border[s]
      let tmpstride = 2 + (8 >> xdec) + 2;
      let mut tmp = [CDEF_VERY_LARGE; (2 + 8 + 2) * (2 + 8 + 2)];
      // copy in what pixels we have/are allowed to use
      pad_into_tmp16(
        tmp.as_mut_ptr(), // points to *padding* upper left
        tmpstride,
        input, // points to *block* upper left
        istride,
        8 >> xdec,
        8 >> ydec,
        edges,
      );
      cdef_filter_block(
        dst,
        tmp.as_ptr().offset(2 * tmpstride + 2),
        tmpstride,
        pri_strength,
        sec_strength,
        dir,
        damping,
        bit_depth,
        xdec,
        ydec,
        CDEF_HAVE_ALL,
        _cpu,
      );
    } else {
      let xsize = (8 >> xdec) as isize;
      let ysize = (8 >> ydec) as isize;
      let coeff_shift = bit_depth - 8;
      let cdef_pri_taps = [[4, 2], [3, 3]];
      let cdef_sec_taps = [[2, 1], [2, 1]];
      let pri_taps =
        cdef_pri_taps[((pri_strength >> coeff_shift) & 1) as usize];
      let sec_taps =
        cdef_sec_taps[((pri_strength >> coeff_shift) & 1) as usize];
      let cdef_directions = [
        [-1 * istride + 1, -2 * istride + 2],
        [0 * istride + 1, -1 * istride + 2],
        [0 * istride + 1, 0 * istride + 2],
        [0 * istride + 1, 1 * istride + 2],
        [1 * istride + 1, 2 * istride + 2],
        [1 * istride + 0, 2 * istride + 1],
        [1 * istride + 0, 2 * istride + 0],
        [1 * istride + 0, 2 * istride - 1],
      ];
      for i in 0..ysize {
        for j in 0..xsize {
          let ptr_in = input.offset(i * istride + j);
          let x = i32::cast_from(*ptr_in);
          let mut sum: i32 = 0;
          let mut max = x;
          let mut min = x;
          for k in 0..2usize {
            let cdef_dirs = [
              cdef_directions[dir][k],
              cdef_directions[(dir + 2) & 7][k],
              cdef_directions[(dir + 6) & 7][k],
            ];
            let pri_tap = pri_taps[k];
            let p = [
              i32::cast_from(*ptr_in.offset(cdef_dirs[0])),
              i32::cast_from(*ptr_in.offset(-cdef_dirs[0])),
            ];
            for p_elem in p.iter() {
              sum += pri_tap * constrain(*p_elem - x, pri_strength, damping);
              if *p_elem != CDEF_VERY_LARGE as i32 {
                max = cmp::max(*p_elem, max);
              }
              min = cmp::min(*p_elem, min);
            }

            let s = [
              i32::cast_from(*ptr_in.offset(cdef_dirs[1])),
              i32::cast_from(*ptr_in.offset(-cdef_dirs[1])),
              i32::cast_from(*ptr_in.offset(cdef_dirs[2])),
              i32::cast_from(*ptr_in.offset(-cdef_dirs[2])),
            ];
            let sec_tap = sec_taps[k];
            for s_elem in s.iter() {
              if *s_elem != CDEF_VERY_LARGE as i32 {
                max = cmp::max(*s_elem, max);
              }
              min = cmp::min(*s_elem, min);
              sum += sec_tap * constrain(*s_elem - x, sec_strength, damping);
            }
          }
          let v = x + ((8 + sum - (sum < 0) as i32) >> 4);
          dst[i as usize][j as usize] = T::cast_from(clamp(v, min, max));
        }
      }
    }
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn check_max_element() {
      assert_eq!(first_max_element(&[-1, -1, 1, 2, 3, 4, 6, 6]), (6, 6));
      assert_eq!(first_max_element(&[-1, -1, 1, 2, 3, 4, 7, 6]), (6, 7));
      assert_eq!(first_max_element(&[0, 0]), (0, 0));
    }
  }
}

// We use the variance of an 8x8 block to adjust the effective filter strength.
#[inline]
fn adjust_strength(strength: i32, var: i32) -> i32 {
  let i = if (var >> 6) != 0 { cmp::min(msb(var >> 6), 12) } else { 0 };
  if var != 0 {
    (strength * (4 + i) + 8) >> 4
  } else {
    0
  }
}

#[profiling::function]
pub fn cdef_analyze_superblock_range<T: Pixel>(
  fi: &FrameInvariants<T>, in_frame: &Frame<T>, blocks: &TileBlocks<'_>,
  sb_w: usize, sb_h: usize,
) -> Vec<CdefDirections> {
  let mut ret = Vec::<CdefDirections>::with_capacity(sb_h * sb_w);
  for sby in 0..sb_h {
    for sbx in 0..sb_w {
      let sbo = TileSuperBlockOffset(SuperBlockOffset { x: sbx, y: sby });
      ret.push(cdef_analyze_superblock(fi, in_frame, blocks, sbo));
    }
  }
  ret
}

#[profiling::function]
pub fn cdef_analyze_superblock<T: Pixel>(
  fi: &FrameInvariants<T>, in_frame: &Frame<T>, blocks: &TileBlocks<'_>,
  sbo: TileSuperBlockOffset,
) -> CdefDirections {
  let coeff_shift = fi.sequence.bit_depth - 8;
  let mut dir: CdefDirections =
    CdefDirections { dir: [[0; 8]; 8], var: [[0; 8]; 8] };
  // Each direction block is 8x8 in y, and direction computation only looks at y
  for by in 0..8 {
    for bx in 0..8 {
      let block_offset = sbo.block_offset(bx << 1, by << 1);
      if block_offset.0.x < blocks.cols() && block_offset.0.y < blocks.rows() {
        let skip = blocks[block_offset].skip
          & blocks[sbo.block_offset(2 * bx + 1, 2 * by)].skip
          & blocks[sbo.block_offset(2 * bx, 2 * by + 1)].skip
          & blocks[sbo.block_offset(2 * bx + 1, 2 * by + 1)].skip;

        if !skip {
          let mut var: u32 = 0;
          let in_plane = &in_frame.planes[0];
          let in_po = sbo.plane_offset(&in_plane.cfg);
          let in_slice = in_plane.slice(in_po);
          dir.dir[bx][by] = cdef_find_dir::<T>(
            &in_slice.reslice(8 * bx as isize, 8 * by as isize),
            &mut var,
            coeff_shift,
            fi.cpu_feature_level,
          ) as u8;
          dir.var[bx][by] = var as i32;
        }
      }
    }
  }
  dir
}

//   input: A Frame of reconstructed/deblocked pixels prepared to
//   undergo CDEF. Note that the input is a Frame and not a Tile due to
//   Tiles not allowing [supervised] out-of-rect access for padding
//   pixels.  This will be corrected at some point in the future.

//   tile_sbo: specifies an offset into the output Tile, not an
//   absolute offset in the visible frame.  The Tile's own offset is
//   added to this in order to address into the input Frame.

//   tb: the TileBlocks associated with the filtered region; the
//   provided blocks co-locate with the output region.  The TileBlocks
//   provide by-[super]qblock CDEF parameters.

//   output: TileMut destination for filtered pixels.  The output's
//   rect specifies the region of the input to be processed (x and y
//   are relative to the input Frame's origin).  Note that an
//   additional area of 2 pixels of padding is used for CDEF.  When
//   these pixels are unavailable (beyond the visible frame or at a
//   tile boundary), the filtering process ignores input pixels that
//   don't exist.

/// # Panics
///
/// - If called with invalid parameters
#[profiling::function]
pub fn cdef_filter_superblock<T: Pixel>(
  fi: &FrameInvariants<T>, input: &Frame<T>, output: &mut TileMut<'_, T>,
  blocks: &TileBlocks<'_>, tile_sbo: TileSuperBlockOffset, cdef_index: u8,
  cdef_dirs: &CdefDirections,
) {
  let bit_depth = fi.sequence.bit_depth;
  let coeff_shift = fi.sequence.bit_depth as i32 - 8;
  let cdef_damping = fi.cdef_damping as i32;
  let cdef_y_strength = fi.cdef_y_strengths[cdef_index as usize];
  let cdef_uv_strength = fi.cdef_uv_strengths[cdef_index as usize];
  let cdef_pri_y_strength = (cdef_y_strength / CDEF_SEC_STRENGTHS) as i32;
  let mut cdef_sec_y_strength = (cdef_y_strength % CDEF_SEC_STRENGTHS) as i32;
  let cdef_pri_uv_strength = (cdef_uv_strength / CDEF_SEC_STRENGTHS) as i32;
  let planes = if fi.sequence.chroma_sampling == Cs400 { 1 } else { 3 };
  let mut cdef_sec_uv_strength =
    (cdef_uv_strength % CDEF_SEC_STRENGTHS) as i32;
  if cdef_sec_y_strength == 3 {
    cdef_sec_y_strength += 1;
  }
  if cdef_sec_uv_strength == 3 {
    cdef_sec_uv_strength += 1;
  }

  let tile_rect = *output.planes[0].rect();
  let input_xoffset =
    tile_rect.x + tile_sbo.plane_offset(&input.planes[0].cfg).x;
  let input_yoffset =
    tile_rect.y + tile_sbo.plane_offset(&input.planes[0].cfg).y;
  let input_xavail = input.planes[0].cfg.width as isize - input_xoffset;
  let input_yavail = input.planes[0].cfg.height as isize - input_yoffset;

  /* determine what edge padding we have, and what padding we don't.
   * We don't pad here, but rather tell the filter_block call what it
   * needs to do, then let it handle the specifics (following dav1d's
   * lead).  We make one assumption that's not obvious: Because the
   * cdef clipping area is rounded up to an even 8x8 luma block, we
   * don't need to guard against having only one (as opposed to two)
   * pixels of padding past the current block boundary.  The padding
   * is all-or-nothing. */

  // Slightly harder than in dav1d; we're not always doing full-frame.
  let have_top_p =
    if tile_sbo.0.y as isize + tile_rect.y > 0 { CDEF_HAVE_TOP } else { 0 };
  let have_left_p =
    if tile_sbo.0.x as isize + tile_rect.x > 0 { CDEF_HAVE_LEFT } else { 0 };
  let mut edges = have_top_p | CDEF_HAVE_BOTTOM;

  // Each direction block is 8x8 in y, potentially smaller if subsampled in chroma
  for by in 0..8usize {
    if by + 1 >= (input_yavail as usize >> 3) {
      edges &= !CDEF_HAVE_BOTTOM
    };
    edges &= !CDEF_HAVE_LEFT;
    edges |= have_left_p;
    edges |= CDEF_HAVE_RIGHT;
    for bx in 0..8usize {
      if bx + 1 >= (input_xavail as usize >> 3) {
        edges &= !CDEF_HAVE_RIGHT
      };
      let block_offset = tile_sbo.block_offset(bx << 1, by << 1);
      if block_offset.0.x < blocks.cols() && block_offset.0.y < blocks.rows() {
        let skip = blocks[block_offset].skip
          & blocks[tile_sbo.block_offset(2 * bx + 1, 2 * by)].skip
          & blocks[tile_sbo.block_offset(2 * bx, 2 * by + 1)].skip
          & blocks[tile_sbo.block_offset(2 * bx + 1, 2 * by + 1)].skip;
        let dir = cdef_dirs.dir[bx][by];
        let var = cdef_dirs.var[bx][by];
        for p in 0..planes {
          let out_plane = &mut output.planes[p];
          let in_plane = &input.planes[p];
          let xdec = in_plane.cfg.xdec;
          let ydec = in_plane.cfg.ydec;
          let xsize = 8 >> xdec;
          let ysize = 8 >> ydec;
          let in_po = PlaneOffset {
            x: (input_xoffset >> xdec) + (bx * xsize) as isize,
            y: (input_yoffset >> ydec) + (by * ysize) as isize,
          };
          let in_stride = in_plane.cfg.stride;
          let in_slice = &in_plane.slice(in_po);

          let out_block = &mut out_plane.subregion_mut(Area::BlockRect {
            bo: tile_sbo.block_offset(2 * bx, 2 * by).0,
            width: xsize,
            height: ysize,
          });

          if !skip {
            let local_pri_strength;
            let local_sec_strength;
            let mut local_damping: i32 = cdef_damping + coeff_shift;
            // See `Cdef_Uv_Dir` constant lookup table in Section 7.15.1
            // <https://aomediacodec.github.io/av1-spec/#cdef-block-process>
            let local_dir = if p == 0 {
              local_pri_strength =
                adjust_strength(cdef_pri_y_strength << coeff_shift, var);
              local_sec_strength = cdef_sec_y_strength << coeff_shift;
              if cdef_pri_y_strength != 0 {
                dir as usize
              } else {
                0
              }
            } else {
              local_pri_strength = cdef_pri_uv_strength << coeff_shift;
              local_sec_strength = cdef_sec_uv_strength << coeff_shift;
              local_damping -= 1;
              if cdef_pri_uv_strength != 0 {
                if xdec != ydec {
                  [7, 0, 2, 4, 5, 6, 6, 6][dir as usize]
                } else {
                  dir as usize
                }
              } else {
                0
              }
            };

            // SAFETY: `cdef_filter_block` may call Assembly code.
            // The asserts here verify that we are not calling it
            // with invalid parameters.
            unsafe {
              assert!(
                input.planes[p].cfg.width as isize
                  >= in_po.x
                    + xsize as isize
                    + if edges & CDEF_HAVE_RIGHT > 0 { 2 } else { 0 }
              );
              assert!(
                0 <= in_po.x - if edges & CDEF_HAVE_LEFT > 0 { 2 } else { 0 }
              );
              assert!(
                input.planes[p].cfg.height as isize
                  >= in_po.y
                    + ysize as isize
                    + if edges & CDEF_HAVE_BOTTOM > 0 { 2 } else { 0 }
              );
              assert!(
                0 <= in_po.y - if edges & CDEF_HAVE_TOP > 0 { 2 } else { 0 }
              );

              cdef_filter_block(
                out_block,
                in_slice.as_ptr(),
                in_stride as isize,
                local_pri_strength,
                local_sec_strength,
                local_dir,
                local_damping,
                bit_depth,
                xdec,
                ydec,
                edges,
                fi.cpu_feature_level,
              );
            }
          } else {
            // no filtering, but we need to copy input to output
            for i in 0..ysize {
              for j in 0..xsize {
                out_block[i][j] = in_slice[i][j];
              }
            }
          }
        }
      }
      edges |= CDEF_HAVE_LEFT;
    }
    edges |= CDEF_HAVE_TOP;
  }
}

// The purpose of CDEF is to perform deringing based on the detected
// direction of blocks.  CDEF parameters are stored for each 64 by 64
// block of pixels.  The CDEF filter is applied on each 8 by 8 block
// of pixels.  Reference:
// http://av1-spec.argondesign.com/av1-spec/av1-spec.html#cdef-process

//   input: A Frame of reconstructed/deblocked pixels prepared to
//   undergo CDEF.  cdef_filter_tile acts on a subset of these input
//   pixels, as specified by the PlaneRegion rect of the output. Note
//   that the input is a Frame and not a Tile due to Tiles not
//   allowing [supervised] out-of-rect access for padding pixels.
//   This will be corrected at some point in the future.

//   tb: the TileBlocks associated with the filtered region; the
//   provided blocks co-locate with the output region.

//   output: TileMut destination for filtered pixels.  The output's
//   rect specifies the region of the input to be processed (x and y
//   are relative to the input Frame's origin).  Note that an
//   additional area of 2 pixels of padding is used for CDEF.  When
//   these pixels are unavailable (beyond the visible frame or at a
//   tile boundary), the filtering process ignores input pixels that
//   don't exist.

#[profiling::function]
pub fn cdef_filter_tile<T: Pixel>(
  fi: &FrameInvariants<T>, input: &Frame<T>, tb: &TileBlocks,
  output: &mut TileMut<'_, T>,
) {
  // Each filter block is 64x64, except right and/or bottom for non-multiple-of-64 sizes.
  // FIXME: 128x128 SB support will break this, we need FilterBlockOffset etc.

  // No need to guard against having fewer actual coded blocks than
  // the output.rect() area.  Inner code already guards this case.
  let fb_width = output.planes[0].rect().width.div_ceil(64);
  let fb_height = output.planes[0].rect().height.div_ceil(64);

  // should parallelize this
  for fby in 0..fb_height {
    for fbx in 0..fb_width {
      // tile_sbo is treated as an offset into the Tiles' plane
      // regions, not as an absolute offset in the visible frame.  The
      // Tile's own offset is added to this in order to address into
      // the input Frame.
      let tile_sbo = TileSuperBlockOffset(SuperBlockOffset { x: fbx, y: fby });
      let cdef_index = tb.get_cdef(tile_sbo);
      let cdef_dirs = cdef_analyze_superblock(fi, input, tb, tile_sbo);

      cdef_filter_superblock(
        fi, input, output, tb, tile_sbo, cdef_index, &cdef_dirs,
      );
    }
  }
}
