// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::*;

use crate::context::*;
use crate::encoder::*;
use crate::me::WriteGuardMEStats;
use crate::util::*;

use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::DerefMut;

pub const MAX_TILE_WIDTH: usize = 4096;
pub const MAX_TILE_AREA: usize = 4096 * 2304;
pub const MAX_TILE_COLS: usize = 64;
pub const MAX_TILE_ROWS: usize = 64;
pub const MAX_TILE_RATE: f64 = 4096f64 * 2176f64 * 60f64 * 1.1;

/// Tiling information
///
/// This stores everything necessary to split a frame into tiles, and write
/// headers fields into the bitstream.
///
/// The method `tile_iter_mut()` actually provides tiled views of `FrameState`
/// and `FrameBlocks`.
#[derive(Debug, Clone, Copy)]
pub struct TilingInfo {
  pub frame_width: usize,
  pub frame_height: usize,
  pub tile_width_sb: usize,
  pub tile_height_sb: usize,
  pub cols: usize, // number of columns of tiles within the whole frame
  pub rows: usize, // number of rows of tiles within the whole frame
  pub tile_cols_log2: usize,
  pub tile_rows_log2: usize,
  pub min_tile_cols_log2: usize,
  pub max_tile_cols_log2: usize,
  pub min_tile_rows_log2: usize,
  pub max_tile_rows_log2: usize,
  pub sb_size_log2: usize,
  pub min_tiles_log2: usize,
}

impl TilingInfo {
  /// # Panics
  ///
  /// Panics if the resulting tile sizes would be too large.
  pub fn from_target_tiles(
    sb_size_log2: usize, frame_width: usize, frame_height: usize,
    frame_rate: f64, tile_cols_log2: usize, tile_rows_log2: usize,
    is_422_p: bool,
  ) -> Self {
    // <https://aomediacodec.github.io/av1-spec/#tile-info-syntax>

    // Frame::new() aligns to the next multiple of 8
    let frame_width = frame_width.align_power_of_two(3);
    let frame_height = frame_height.align_power_of_two(3);
    let frame_width_sb =
      frame_width.align_power_of_two_and_shift(sb_size_log2);
    let frame_height_sb =
      frame_height.align_power_of_two_and_shift(sb_size_log2);
    let sb_cols = frame_width.align_power_of_two_and_shift(sb_size_log2);
    let sb_rows = frame_height.align_power_of_two_and_shift(sb_size_log2);

    // these are bitstream-defined values and must not be changed
    let max_tile_width_sb = MAX_TILE_WIDTH >> sb_size_log2;
    let max_tile_area_sb = MAX_TILE_AREA >> (2 * sb_size_log2);
    let min_tile_cols_log2 =
      Self::tile_log2(max_tile_width_sb, sb_cols).unwrap();
    let max_tile_cols_log2 =
      Self::tile_log2(1, sb_cols.min(MAX_TILE_COLS)).unwrap();
    let max_tile_rows_log2 =
      Self::tile_log2(1, sb_rows.min(MAX_TILE_ROWS)).unwrap();
    let min_tiles_log2 = min_tile_cols_log2
      .max(Self::tile_log2(max_tile_area_sb, sb_cols * sb_rows).unwrap());

    // Implements restriction in Annex A of the spec.
    // Unlike the other restrictions, this one does not change
    // the header coding of the tile rows/cols.
    let min_tiles_ratelimit_log2 = min_tiles_log2.max(
      ((frame_width * frame_height) as f64 * frame_rate / MAX_TILE_RATE)
        .ceil()
        .log2()
        .ceil() as usize,
    );

    let tile_cols_log2 =
      tile_cols_log2.clamp(min_tile_cols_log2, max_tile_cols_log2);
    let tile_width_sb_pre =
      sb_cols.align_power_of_two_and_shift(tile_cols_log2);

    // If this is 4:2:2, our UV horizontal is subsampled but not our
    // vertical.  Loop Restoration Units must be square, so they
    // will always have an even number of horizontal superblocks. For
    // tiles and LRUs to align, tile_width_sb must be even in 4:2:2
    // video.

    // This is only relevant when doing loop restoration RDO inline
    // with block/superblock encoding, that is, where tiles are
    // relevant.  If (when) we introduce optionally delaying loop-filter
    // encode to after the partitioning loop, we won't need to make
    // any 4:2:2 adjustment.

    let tile_width_sb = if is_422_p {
      (tile_width_sb_pre + 1) >> 1 << 1
    } else {
      tile_width_sb_pre
    };

    let cols = frame_width_sb.div_ceil(tile_width_sb);

    // Adjust tile_cols_log2 in case of rounding tile_width_sb to even.
    let tile_cols_log2 = Self::tile_log2(1, cols).unwrap();
    assert!(tile_cols_log2 >= min_tile_cols_log2);

    let min_tile_rows_log2 = min_tiles_log2.saturating_sub(tile_cols_log2);
    let min_tile_rows_ratelimit_log2 =
      min_tiles_ratelimit_log2.saturating_sub(tile_cols_log2);
    let tile_rows_log2 = tile_rows_log2
      .max(min_tile_rows_log2)
      .clamp(min_tile_rows_ratelimit_log2, max_tile_rows_log2);
    let tile_height_sb = sb_rows.align_power_of_two_and_shift(tile_rows_log2);

    let rows = frame_height_sb.div_ceil(tile_height_sb);

    Self {
      frame_width,
      frame_height,
      tile_width_sb,
      tile_height_sb,
      cols,
      rows,
      tile_cols_log2,
      tile_rows_log2,
      min_tile_cols_log2,
      max_tile_cols_log2,
      min_tile_rows_log2,
      max_tile_rows_log2,
      sb_size_log2,
      min_tiles_log2,
    }
  }

  /// Return the smallest value for `k` such that `blkSize << k` is greater than
  /// or equal to `target`.
  ///
  /// <https://aomediacodec.github.io/av1-spec/#tile-size-calculation-function>
  pub fn tile_log2(blk_size: usize, target: usize) -> Option<usize> {
    let mut k = 0;
    while (blk_size.checked_shl(k)?) < target {
      k += 1;
    }
    Some(k as usize)
  }

  #[inline(always)]
  pub const fn tile_count(&self) -> usize {
    self.cols * self.rows
  }

  /// Split frame-level structures into tiles
  ///
  /// Provide mutable tiled views of frame-level structures.
  pub fn tile_iter_mut<'a, T: Pixel>(
    &self, fs: &'a mut FrameState<T>, fb: &'a mut FrameBlocks,
  ) -> TileContextIterMut<'a, T> {
    let afs = fs as *mut _;
    let afb = fb as *mut _;
    let frame_me_stats = fs.frame_me_stats.write().expect("poisoned lock");
    TileContextIterMut { ti: *self, fs: afs, fb: afb, next: 0, frame_me_stats }
  }
}

/// Container for all tiled views
pub struct TileContextMut<'a, T: Pixel> {
  pub ts: TileStateMut<'a, T>,
  pub tb: TileBlocksMut<'a>,
}

/// Iterator over tiled views
pub struct TileContextIterMut<'a, T: Pixel> {
  ti: TilingInfo,
  fs: *mut FrameState<T>,
  fb: *mut FrameBlocks,
  frame_me_stats: WriteGuardMEStats<'a>,
  next: usize,
}

impl<'a, T: Pixel> Iterator for TileContextIterMut<'a, T> {
  type Item = TileContextMut<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.next < self.ti.rows * self.ti.cols {
      let tile_col = self.next % self.ti.cols;
      let tile_row = self.next / self.ti.cols;
      let ctx = TileContextMut {
        ts: {
          // SAFETY: Multiple tiles mutably access this struct.
          // The dimensions must be configured correctly to ensure
          // the tiles do not overlap.
          let fs = unsafe { &mut *self.fs };
          // SAFETY: ditto
          let frame_me_stats = unsafe {
            let len = self.frame_me_stats.len();
            let ptr = self.frame_me_stats.as_mut_ptr();
            std::slice::from_raw_parts_mut(ptr, len)
          };
          let sbo = PlaneSuperBlockOffset(SuperBlockOffset {
            x: tile_col * self.ti.tile_width_sb,
            y: tile_row * self.ti.tile_height_sb,
          });
          let x = sbo.0.x << self.ti.sb_size_log2;
          let y = sbo.0.y << self.ti.sb_size_log2;
          let tile_width = self.ti.tile_width_sb << self.ti.sb_size_log2;
          let tile_height = self.ti.tile_height_sb << self.ti.sb_size_log2;
          let width = tile_width.min(self.ti.frame_width - x);
          let height = tile_height.min(self.ti.frame_height - y);
          TileStateMut::new(
            fs,
            sbo,
            self.ti.sb_size_log2,
            width,
            height,
            frame_me_stats,
          )
        },
        tb: {
          // SAFETY: Multiple tiles mutably access this struct.
          // The dimensions must be configured correctly to ensure
          // the tiles do not overlap.
          let fb = unsafe { &mut *self.fb };
          let tile_width_mi =
            self.ti.tile_width_sb << (self.ti.sb_size_log2 - MI_SIZE_LOG2);
          let tile_height_mi =
            self.ti.tile_height_sb << (self.ti.sb_size_log2 - MI_SIZE_LOG2);
          let x = tile_col * tile_width_mi;
          let y = tile_row * tile_height_mi;
          let cols = tile_width_mi.min(fb.cols - x);
          let rows = tile_height_mi.min(fb.rows - y);
          TileBlocksMut::new(fb, x, y, cols, rows)
        },
      };
      self.next += 1;
      Some(ctx)
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.ti.cols * self.ti.rows - self.next;
    (remaining, Some(remaining))
  }
}

impl<T: Pixel> ExactSizeIterator for TileContextIterMut<'_, T> {}
impl<T: Pixel> FusedIterator for TileContextIterMut<'_, T> {}

#[cfg(test)]
pub mod test {
  use super::*;
  use crate::api::*;
  use crate::lrf::*;
  use crate::mc::MotionVector;
  use crate::predict::PredictionMode;
  use std::sync::Arc;

  #[test]
  fn test_tiling_info_from_tile_count() {
    let sb_size_log2 = 6;
    let (width, height) = (160, 144);
    let frame_rate = 25f64;

    let ti = TilingInfo::from_target_tiles(
      sb_size_log2,
      width,
      height,
      frame_rate,
      0,
      0,
      false,
    );
    assert_eq!(1, ti.cols);
    assert_eq!(1, ti.rows);
    assert_eq!(3, ti.tile_width_sb);
    assert_eq!(3, ti.tile_height_sb);

    let ti = TilingInfo::from_target_tiles(
      sb_size_log2,
      width,
      height,
      frame_rate,
      1,
      1,
      false,
    );
    assert_eq!(2, ti.cols);
    assert_eq!(2, ti.rows);
    assert_eq!(2, ti.tile_width_sb);
    assert_eq!(2, ti.tile_height_sb);

    let ti = TilingInfo::from_target_tiles(
      sb_size_log2,
      width,
      height,
      frame_rate,
      2,
      2,
      false,
    );
    assert_eq!(3, ti.cols);
    assert_eq!(3, ti.rows);
    assert_eq!(1, ti.tile_width_sb);
    assert_eq!(1, ti.tile_height_sb);

    // cannot split more than superblocks
    let ti = TilingInfo::from_target_tiles(
      sb_size_log2,
      width,
      height,
      frame_rate,
      10,
      8,
      false,
    );
    assert_eq!(3, ti.cols);
    assert_eq!(3, ti.rows);
    assert_eq!(1, ti.tile_width_sb);
    assert_eq!(1, ti.tile_height_sb);

    let ti = TilingInfo::from_target_tiles(
      sb_size_log2,
      1024,
      1024,
      frame_rate,
      0,
      0,
      false,
    );
    assert_eq!(1, ti.cols);
    assert_eq!(1, ti.rows);
    assert_eq!(16, ti.tile_width_sb);
    assert_eq!(16, ti.tile_height_sb);
  }

  fn setup(
    width: usize, height: usize,
  ) -> (FrameInvariants<u16>, FrameState<u16>, FrameBlocks, f64) {
    // FrameInvariants aligns to the next multiple of 8, so using other values could make tests confusing
    assert!(width.trailing_zeros() >= 3);
    assert!(height.trailing_zeros() >= 3);
    // We test only for 420 for now
    let chroma_sampling = ChromaSampling::Cs420;
    let config = Arc::new(EncoderConfig {
      width,
      height,
      bit_depth: 8,
      chroma_sampling,
      ..Default::default()
    });
    let mut sequence = Sequence::new(&config);
    // These tests are all assuming SB-sized LRUs, so set that.
    sequence.enable_large_lru = false;
    let frame_rate = config.frame_rate();
    let fi = FrameInvariants::new(config, Arc::new(sequence));
    let fs = FrameState::new(&fi);
    let fb = FrameBlocks::new(fi.w_in_b, fi.h_in_b);

    (fi, fs, fb, frame_rate)
  }

  #[test]
  fn test_tile_iter_len() {
    // frame size 160x144, 40x36 in 4x4-blocks
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    {
      // 2x2 tiles
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        1,
        1,
        false,
      );
      let mut iter = ti.tile_iter_mut(&mut fs, &mut fb);
      assert_eq!(4, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(3, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(2, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(1, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(0, iter.len());
      assert!(iter.next().is_none());
    }

    {
      // 4x4 tiles requested, will actually get 3x3 tiles
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        2,
        2,
        false,
      );
      let mut iter = ti.tile_iter_mut(&mut fs, &mut fb);
      assert_eq!(9, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(8, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(7, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(6, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(5, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(4, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(3, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(2, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(1, iter.len());
      assert!(iter.next().is_some());
      assert_eq!(0, iter.len());
      assert!(iter.next().is_none());
    }
  }

  #[inline]
  fn rect<T: Pixel>(
    region: &PlaneRegionMut<'_, T>,
  ) -> (isize, isize, usize, usize) {
    let &Rect { x, y, width, height } = region.rect();
    (x, y, width, height)
  }

  #[test]
  fn test_tile_area() {
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    // 4x4 tiles requested, will actually get 3x3 tiles
    let ti = TilingInfo::from_target_tiles(
      fi.sb_size_log2(),
      fi.width,
      fi.height,
      frame_rate,
      2,
      2,
      false,
    );
    let iter = ti.tile_iter_mut(&mut fs, &mut fb);
    let tile_states = iter.map(|ctx| ctx.ts).collect::<Vec<_>>();

    // the frame must be split into 9 tiles:
    //
    //       luma (Y)             chroma (U)            chroma (V)
    //   64x64 64x64 32x64     32x32 32x32 16x32     32x32 32x32 16x32
    //   64x64 64x64 32x64     32x32 32x32 16x32     32x32 32x32 16x32
    //   64x16 64x16 32x16     32x 8 32x 8 16x 8     32x 8 32x 8 16x 8

    assert_eq!(9, tile_states.len());

    let tile = &tile_states[0].rec; // the top-left tile
    assert_eq!((0, 0, 64, 64), rect(&tile.planes[0]));
    assert_eq!((0, 0, 32, 32), rect(&tile.planes[1]));
    assert_eq!((0, 0, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[1].rec; // the top-middle tile
    assert_eq!((64, 0, 64, 64), rect(&tile.planes[0]));
    assert_eq!((32, 0, 32, 32), rect(&tile.planes[1]));
    assert_eq!((32, 0, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[2].rec; // the top-right tile
    assert_eq!((128, 0, 64, 64), rect(&tile.planes[0]));
    assert_eq!((64, 0, 32, 32), rect(&tile.planes[1]));
    assert_eq!((64, 0, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[3].rec; // the middle-left tile
    assert_eq!((0, 64, 64, 64), rect(&tile.planes[0]));
    assert_eq!((0, 32, 32, 32), rect(&tile.planes[1]));
    assert_eq!((0, 32, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[4].rec; // the center tile
    assert_eq!((64, 64, 64, 64), rect(&tile.planes[0]));
    assert_eq!((32, 32, 32, 32), rect(&tile.planes[1]));
    assert_eq!((32, 32, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[5].rec; // the middle-right tile
    assert_eq!((128, 64, 64, 64), rect(&tile.planes[0]));
    assert_eq!((64, 32, 32, 32), rect(&tile.planes[1]));
    assert_eq!((64, 32, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[6].rec; // the bottom-left tile
    assert_eq!((0, 128, 64, 64), rect(&tile.planes[0]));
    assert_eq!((0, 64, 32, 32), rect(&tile.planes[1]));
    assert_eq!((0, 64, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[7].rec; // the bottom-middle tile
    assert_eq!((64, 128, 64, 64), rect(&tile.planes[0]));
    assert_eq!((32, 64, 32, 32), rect(&tile.planes[1]));
    assert_eq!((32, 64, 32, 32), rect(&tile.planes[2]));

    let tile = &tile_states[8].rec; // the bottom-right tile
    assert_eq!((128, 128, 64, 64), rect(&tile.planes[0]));
    assert_eq!((64, 64, 32, 32), rect(&tile.planes[1]));
    assert_eq!((64, 64, 32, 32), rect(&tile.planes[2]));
  }

  #[inline]
  const fn b_area(region: &TileBlocksMut<'_>) -> (usize, usize, usize, usize) {
    (region.x(), region.y(), region.cols(), region.rows())
  }

  #[test]
  fn test_tile_blocks_area() {
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    // 4x4 tiles requested, will actually get 3x3 tiles
    let ti = TilingInfo::from_target_tiles(
      fi.sb_size_log2(),
      fi.width,
      fi.height,
      frame_rate,
      2,
      2,
      false,
    );
    let iter = ti.tile_iter_mut(&mut fs, &mut fb);
    let tbs = iter.map(|ctx| ctx.tb).collect::<Vec<_>>();

    // the FrameBlocks must be split into 9 TileBlocks:
    //
    //   16x16 16x16  8x16
    //   16x16 16x16  8x16
    //   16x 4 16x4   8x 4

    assert_eq!(9, tbs.len());

    assert_eq!((0, 0, 16, 16), b_area(&tbs[0]));
    assert_eq!((16, 0, 16, 16), b_area(&tbs[1]));
    assert_eq!((32, 0, 8, 16), b_area(&tbs[2]));

    assert_eq!((0, 16, 16, 16), b_area(&tbs[3]));
    assert_eq!((16, 16, 16, 16), b_area(&tbs[4]));
    assert_eq!((32, 16, 8, 16), b_area(&tbs[5]));

    assert_eq!((0, 32, 16, 4), b_area(&tbs[6]));
    assert_eq!((16, 32, 16, 4), b_area(&tbs[7]));
    assert_eq!((32, 32, 8, 4), b_area(&tbs[8]));
  }

  #[test]
  fn test_tile_write() {
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    {
      // 4x4 tiles requested, will actually get 3x3 tiles
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        2,
        2,
        false,
      );
      let iter = ti.tile_iter_mut(&mut fs, &mut fb);
      let mut tile_states = iter.map(|ctx| ctx.ts).collect::<Vec<_>>();

      {
        // row 12 of Y-plane of the top-left tile
        let tile_plane = &mut tile_states[0].rec.planes[0];
        let row = &mut tile_plane[12];
        assert_eq!(64, row.len());
        row[35..41].copy_from_slice(&[4, 42, 12, 18, 15, 31]);
      }

      {
        // row 8 of U-plane of the middle-right tile
        let tile_plane = &mut tile_states[5].rec.planes[1];
        let row = &mut tile_plane[8];
        assert_eq!(32, row.len());
        row[..4].copy_from_slice(&[14, 121, 1, 3]);
      }

      {
        // row 1 of V-plane of the bottom-middle tile
        let tile_plane = &mut tile_states[7].rec.planes[2];
        let row = &mut tile_plane[1];
        assert_eq!(32, row.len());
        row[11..16].copy_from_slice(&[6, 5, 2, 11, 8]);
      }
    }

    // check that writes on tiles correctly affected the underlying frame

    let plane = &fs.rec.planes[0];
    let y = plane.cfg.yorigin + 12;
    let x = plane.cfg.xorigin + 35;
    let idx = y * plane.cfg.stride + x;
    assert_eq!(&[4, 42, 12, 18, 15, 31], &plane.data[idx..idx + 6]);

    let plane = &fs.rec.planes[1];
    let offset = (64, 32); // middle-right tile, chroma plane
    let y = plane.cfg.yorigin + offset.1 + 8;
    let x = plane.cfg.xorigin + offset.0;
    let idx = y * plane.cfg.stride + x;
    assert_eq!(&[14, 121, 1, 3], &plane.data[idx..idx + 4]);

    let plane = &fs.rec.planes[2];
    let offset = (32, 64); // bottom-middle tile, chroma plane
    let y = plane.cfg.yorigin + offset.1 + 1;
    let x = plane.cfg.xorigin + offset.0 + 11;
    let idx = y * plane.cfg.stride + x;
    assert_eq!(&[6, 5, 2, 11, 8], &plane.data[idx..idx + 5]);
  }

  #[test]
  fn test_tile_restoration_edges() {
    let (fi, mut fs, mut fb, frame_rate) = setup(64, 80);

    let ti = TilingInfo::from_target_tiles(
      fi.sb_size_log2(),
      fi.width,
      fi.height,
      frame_rate,
      2,
      2,
      false,
    );
    let iter = ti.tile_iter_mut(&mut fs, &mut fb);
    let mut tile_states = iter.map(|ctx| ctx.ts).collect::<Vec<_>>();

    assert_eq!(tile_states.len(), 2);

    {
      let trs = &mut tile_states[0].restoration;
      let units = &trs.planes[0].units;
      assert_eq!(units.x(), 0);
      assert_eq!(units.y(), 0);
      assert_eq!(units.cols(), 1);
      assert_eq!(units.rows(), 1);
    }

    {
      let trs = &mut tile_states[1].restoration;
      let units = &trs.planes[0].units;
      assert_eq!(units.x(), 0);
      assert_eq!(units.y(), 1);
      // no units, the tile is too small (less than 1/2 super-block)
      assert_eq!(units.cols() * units.rows(), 0);
    }
  }

  #[test]
  fn test_tile_restoration_write() {
    let (fi, mut fs, mut fb, frame_rate) = setup(256, 256);

    {
      // 2x2 tiles, each one containing 2×2 restoration units (1 super-block per restoration unit)
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        1,
        1,
        false,
      );
      let iter = ti.tile_iter_mut(&mut fs, &mut fb);
      let mut tile_states = iter.map(|ctx| ctx.ts).collect::<Vec<_>>();

      {
        // unit (1, 0) of Y-plane of the top-left tile
        let units = &mut tile_states[0].restoration.planes[0].units;
        units[0][1].filter =
          RestorationFilter::Wiener { coeffs: [[1, 2, 3], [4, 5, 6]] };
      }

      {
        // unit (0, 1) of U-plane of the bottom-right tile
        let units = &mut tile_states[3].restoration.planes[1].units;
        units[1][0].filter =
          RestorationFilter::Sgrproj { set: 42, xqd: [10, 20] };
      }

      {
        // unit (1, 1) of V-plane of the bottom-left tile
        let units = &mut tile_states[2].restoration.planes[2].units;
        units[1][1].filter =
          RestorationFilter::Sgrproj { set: 5, xqd: [1, 2] };
      }
    }

    // check that writes on tiles correctly affected the underlying restoration units

    let units = &mut fs.restoration.planes[0].units;
    assert_eq!(
      units[0][1].filter,
      RestorationFilter::Wiener { coeffs: [[1, 2, 3], [4, 5, 6]] }
    );

    let units = &mut fs.restoration.planes[1].units;
    assert_eq!(
      units[3][2].filter,
      RestorationFilter::Sgrproj { set: 42, xqd: [10, 20] }
    );

    let units = &mut fs.restoration.planes[2].units;
    assert_eq!(
      units[3][1].filter,
      RestorationFilter::Sgrproj { set: 5, xqd: [1, 2] }
    );
  }

  #[test]
  fn test_tile_motion_vectors_write() {
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    {
      // 4x4 tiles requested, will actually get 3x3 tiles
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        2,
        2,
        false,
      );
      let iter = ti.tile_iter_mut(&mut fs, &mut fb);
      let mut tile_states = iter.map(|ctx| ctx.ts).collect::<Vec<_>>();

      {
        // block (8, 5) of the top-left tile (of the first ref frame)
        let me_stats = &mut tile_states[0].me_stats[0];
        me_stats[5][8].mv = MotionVector { col: 42, row: 38 };
        println!("{:?}", me_stats[5][8].mv);
      }

      {
        // block (4, 2) of the middle-right tile (of ref frame 2)
        let me_stats = &mut tile_states[5].me_stats[2];
        me_stats[2][3].mv = MotionVector { col: 2, row: 14 };
      }
    }

    // check that writes on tiled views affected the underlying motion vectors

    let me_stats = &fs.frame_me_stats.read().unwrap()[0];
    assert_eq!(MotionVector { col: 42, row: 38 }, me_stats[5][8].mv);

    let me_stats = &fs.frame_me_stats.read().unwrap()[2];
    let mix = (128 >> MI_SIZE_LOG2) + 3;
    let miy = (64 >> MI_SIZE_LOG2) + 2;
    assert_eq!(MotionVector { col: 2, row: 14 }, me_stats[miy][mix].mv);
  }

  #[test]
  fn test_tile_blocks_write() {
    let (fi, mut fs, mut fb, frame_rate) = setup(160, 144);

    {
      // 4x4 tiles requested, will actually get 3x3 tiles
      let ti = TilingInfo::from_target_tiles(
        fi.sb_size_log2(),
        fi.width,
        fi.height,
        frame_rate,
        2,
        2,
        false,
      );
      let iter = ti.tile_iter_mut(&mut fs, &mut fb);
      let mut tbs = iter.map(|ctx| ctx.tb).collect::<Vec<_>>();

      {
        // top-left tile
        let tb = &mut tbs[0];
        // block (4, 3)
        tb[3][4].n4_w = 42;
        // block (8, 5)
        tb[5][8].segmentation_idx = 14;
      }

      {
        // middle-right tile
        let tb = &mut tbs[5];
        // block (0, 1)
        tb[1][0].n4_h = 11;
        // block (7, 5)
        tb[5][7].cdef_index = 3;
      }

      {
        // bottom-middle tile
        let tb = &mut tbs[7];
        // block (3, 2)
        tb[2][3].mode = PredictionMode::PAETH_PRED;
        // block (1, 1)
        tb[1][1].n4_w = 8;
      }
    }

    // check that writes on tiles correctly affected the underlying blocks

    assert_eq!(42, fb[3][4].n4_w);
    assert_eq!(14, fb[5][8].segmentation_idx);

    assert_eq!(11, fb[17][32].n4_h);
    assert_eq!(3, fb[21][39].cdef_index);

    assert_eq!(PredictionMode::PAETH_PRED, fb[34][19].mode);
    assert_eq!(8, fb[33][17].n4_w);
  }

  #[test]
  fn tile_log2_overflow() {
    assert_eq!(TilingInfo::tile_log2(1, usize::MAX), None);
  }

  #[test]
  fn from_target_tiles_422() {
    let sb_size_log2 = 6;
    let is_422_p = true;
    let frame_rate = 60.;
    let sb_size = 1 << sb_size_log2;

    for frame_height in (sb_size..4352).step_by(sb_size) {
      for tile_rows_log2 in
        0..=TilingInfo::tile_log2(1, frame_height >> sb_size_log2).unwrap()
      {
        for frame_width in (sb_size..7680).step_by(sb_size) {
          for tile_cols_log2 in
            0..=TilingInfo::tile_log2(1, frame_width >> sb_size_log2).unwrap()
          {
            let ti = TilingInfo::from_target_tiles(
              sb_size_log2,
              frame_width,
              frame_height,
              frame_rate,
              tile_cols_log2,
              tile_rows_log2,
              is_422_p,
            );
            assert_eq!(
              ti.tile_cols_log2,
              TilingInfo::tile_log2(1, ti.cols).unwrap()
            );
            assert_eq!(
              ti.tile_rows_log2,
              TilingInfo::tile_log2(1, ti.rows).unwrap()
            );
          }
        }
      }
    }
  }
}
