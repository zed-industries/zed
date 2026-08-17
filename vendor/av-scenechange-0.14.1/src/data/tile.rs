use std::iter::FusedIterator;

use v_frame::{
    frame::Frame,
    math::Fixed,
    pixel::Pixel,
    plane::{Plane, PlaneOffset},
};

use crate::data::{
    block::BlockOffset,
    frame::{FrameState, MAX_PLANES},
    motion::{FrameMEStats, TileMEStatsMut, WriteGuardMEStats},
    plane::{PlaneBlockOffset, PlaneRegion, Rect},
    superblock::{PlaneSuperBlockOffset, SuperBlockOffset, MI_SIZE, MI_SIZE_LOG2, SB_SIZE_LOG2},
};

pub const MAX_TILE_WIDTH: usize = 4096;
pub const MAX_TILE_AREA: usize = 4096 * 2304;
pub const MAX_TILE_COLS: usize = 64;
pub const MAX_TILE_ROWS: usize = 64;
pub const MAX_TILE_RATE: f64 = 4096f64 * 2176f64 * 60f64 * 1.1;

/// Tiled view of a frame
#[derive(Debug)]
pub struct Tile<'a, T: Pixel> {
    pub planes: [PlaneRegion<'a, T>; MAX_PLANES],
}

// common impl for Tile and TileMut
macro_rules! tile_common {
  // $name: Tile or TileMut
  // $pr_type: PlaneRegion or PlaneRegionMut
  // $iter: iter or iter_mut
  //opt_mut: nothing or mut
  ($name:ident, $pr_type:ident, $iter:ident $(,$opt_mut:tt)?) => {
    impl<'a, T: Pixel> $name<'a, T> {

      pub fn new(
        frame: &'a $($opt_mut)? Frame<T>,
        luma_rect: TileRect,
      ) -> Self {
        let mut planes_iter = frame.planes.$iter();
        Self {
          planes: [
            {
              let plane = planes_iter.next().unwrap();
              $pr_type::new(plane, luma_rect.into())
            },
            {
              let plane = planes_iter.next().unwrap();
              let rect = luma_rect.decimated(plane.cfg.xdec, plane.cfg.ydec);
              $pr_type::new(plane, rect.into())
            },
            {
              let plane = planes_iter.next().unwrap();
              let rect = luma_rect.decimated(plane.cfg.xdec, plane.cfg.ydec);
              $pr_type::new(plane, rect.into())
            },
          ],
        }
      }
    }
  }
}

tile_common!(Tile, PlaneRegion, iter);

/// Rectangle of a tile, in pixels
///
/// This is similar to Rect, but with unsigned (x, y) for convenience.
#[derive(Debug, Clone, Copy)]
pub struct TileRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl TileRect {
    pub const fn decimated(self, xdec: usize, ydec: usize) -> Self {
        Self {
            x: self.x >> xdec,
            y: self.y >> ydec,
            width: self.width >> xdec,
            height: self.height >> ydec,
        }
    }

    pub const fn to_frame_plane_offset(self, tile_po: PlaneOffset) -> PlaneOffset {
        PlaneOffset {
            x: self.x as isize + tile_po.x,
            y: self.y as isize + tile_po.y,
        }
    }
}

impl From<TileRect> for Rect {
    fn from(tile_rect: TileRect) -> Rect {
        Rect {
            x: tile_rect.x as isize,
            y: tile_rect.y as isize,
            width: tile_rect.width,
            height: tile_rect.height,
        }
    }
}

/// Tiled view of `FrameState`
///
/// Contrary to `PlaneRegionMut` and `TileMut`, there is no const version:
///  - in practice, we don't need it;
///  - it would require to instantiate a const version of every of its inner
///    tiled views recursively.
///
/// # `TileState` fields
///
/// The way the `FrameState` fields are mapped depend on how they are accessed
/// tile-wise and frame-wise.
///
/// Some fields (like `qc`) are only used during tile-encoding, so they are only
/// stored in `TileState`.
///
/// Some other fields (like `input` or `segmentation`) are not written
/// tile-wise, so they just reference the matching field in `FrameState`.
///
/// Some others (like `rec`) are written tile-wise, but must be accessible
/// frame-wise once the tile views vanish (e.g. for deblocking).
#[derive(Debug)]
pub struct TileStateMut<'a, T: Pixel> {
    pub sbo: PlaneSuperBlockOffset,
    pub sb_width: usize,
    pub sb_height: usize,
    pub mi_width: usize,
    pub mi_height: usize,
    pub width: usize,
    pub height: usize,
    pub input_tile: Tile<'a, T>, // the current tile
    pub input_hres: &'a Plane<T>,
    pub input_qres: &'a Plane<T>,
    pub me_stats: Vec<TileMEStatsMut<'a>>,
}

impl<'a, T: Pixel> TileStateMut<'a, T> {
    pub fn new(
        fs: &'a mut FrameState<T>,
        sbo: PlaneSuperBlockOffset,
        width: usize,
        height: usize,
        frame_me_stats: &'a mut [FrameMEStats],
    ) -> Self {
        debug_assert!(
            width % MI_SIZE == 0,
            "Tile width must be a multiple of MI_SIZE"
        );
        debug_assert!(
            height % MI_SIZE == 0,
            "Tile width must be a multiple of MI_SIZE"
        );

        let sb_rounded_width = width.align_power_of_two(SB_SIZE_LOG2);
        let sb_rounded_height = height.align_power_of_two(SB_SIZE_LOG2);

        let luma_rect = TileRect {
            x: sbo.0.x << SB_SIZE_LOG2,
            y: sbo.0.y << SB_SIZE_LOG2,
            width: sb_rounded_width,
            height: sb_rounded_height,
        };
        let sb_width = width.align_power_of_two_and_shift(SB_SIZE_LOG2);
        let sb_height = height.align_power_of_two_and_shift(SB_SIZE_LOG2);

        Self {
            sbo,
            sb_width,
            sb_height,
            mi_width: width >> MI_SIZE_LOG2,
            mi_height: height >> MI_SIZE_LOG2,
            width,
            height,
            input_tile: Tile::new(&fs.input, luma_rect),
            input_hres: &fs.input_hres,
            input_qres: &fs.input_qres,
            me_stats: frame_me_stats
                .iter_mut()
                .map(|fmvs| {
                    TileMEStatsMut::new(
                        fmvs,
                        sbo.0.x << (SB_SIZE_LOG2 - MI_SIZE_LOG2),
                        sbo.0.y << (SB_SIZE_LOG2 - MI_SIZE_LOG2),
                        width >> MI_SIZE_LOG2,
                        height >> MI_SIZE_LOG2,
                    )
                })
                .collect(),
        }
    }

    pub fn to_frame_block_offset(&self, tile_bo: TileBlockOffset) -> PlaneBlockOffset {
        let bx = self.sbo.0.x << (SB_SIZE_LOG2 - MI_SIZE_LOG2);
        let by = self.sbo.0.y << (SB_SIZE_LOG2 - MI_SIZE_LOG2);
        PlaneBlockOffset(BlockOffset {
            x: bx + tile_bo.0.x,
            y: by + tile_bo.0.y,
        })
    }
}

/// Absolute offset in blocks inside a tile, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TileBlockOffset(pub BlockOffset);

impl TileBlockOffset {
    /// Convert to plane offset without decimation.
    pub const fn to_luma_plane_offset(self) -> PlaneOffset {
        self.0.to_luma_plane_offset()
    }

    pub fn with_offset(self, col_offset: isize, row_offset: isize) -> TileBlockOffset {
        Self(self.0.with_offset(col_offset, row_offset))
    }
}

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
}

impl TilingInfo {
    /// # Panics
    ///
    /// Panics if the resulting tile sizes would be too large.
    pub fn from_target_tiles(
        frame_width: usize,
        frame_height: usize,
        frame_rate: f64,
        tile_cols_log2: usize,
        tile_rows_log2: usize,
        is_422_p: bool,
    ) -> Self {
        // <https://aomediacodec.github.io/av1-spec/#tile-info-syntax>

        // Frame::new() aligns to the next multiple of 8
        let frame_width = frame_width.align_power_of_two(3);
        let frame_height = frame_height.align_power_of_two(3);
        let frame_width_sb = frame_width.align_power_of_two_and_shift(SB_SIZE_LOG2);
        let frame_height_sb = frame_height.align_power_of_two_and_shift(SB_SIZE_LOG2);
        let sb_cols = frame_width.align_power_of_two_and_shift(SB_SIZE_LOG2);
        let sb_rows = frame_height.align_power_of_two_and_shift(SB_SIZE_LOG2);

        // these are bitstream-defined values and must not be changed
        let max_tile_width_sb = MAX_TILE_WIDTH >> SB_SIZE_LOG2;
        let max_tile_area_sb = MAX_TILE_AREA >> (2 * SB_SIZE_LOG2);
        let min_tile_cols_log2 = Self::tile_log2(max_tile_width_sb, sb_cols).unwrap();
        let max_tile_cols_log2 = Self::tile_log2(1, sb_cols.min(MAX_TILE_COLS)).unwrap();
        let max_tile_rows_log2 = Self::tile_log2(1, sb_rows.min(MAX_TILE_ROWS)).unwrap();
        let min_tiles_log2 =
            min_tile_cols_log2.max(Self::tile_log2(max_tile_area_sb, sb_cols * sb_rows).unwrap());

        // Implements restriction in Annex A of the spec.
        // Unlike the other restrictions, this one does not change
        // the header coding of the tile rows/cols.
        let min_tiles_ratelimit_log2 = min_tiles_log2.max(
            ((frame_width * frame_height) as f64 * frame_rate / MAX_TILE_RATE)
                .ceil()
                .log2()
                .ceil() as usize,
        );

        let tile_cols_log2 = tile_cols_log2.clamp(min_tile_cols_log2, max_tile_cols_log2);
        let tile_width_sb_pre = sb_cols.align_power_of_two_and_shift(tile_cols_log2);

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
        let min_tile_rows_ratelimit_log2 = min_tiles_ratelimit_log2.saturating_sub(tile_cols_log2);
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
        }
    }

    /// Return the smallest value for `k` such that `blkSize << k` is greater
    /// than or equal to `target`.
    ///
    /// <https://aomediacodec.github.io/av1-spec/#tile-size-calculation-function>
    pub fn tile_log2(blk_size: usize, target: usize) -> Option<usize> {
        let mut k = 0;
        while (blk_size.checked_shl(k)?) < target {
            k += 1;
        }
        Some(k as usize)
    }

    /// Split frame-level structures into tiles
    ///
    /// Provide mutable tiled views of frame-level structures.
    pub fn tile_iter_mut<'a, T: Pixel>(
        &self,
        fs: &'a mut FrameState<T>,
    ) -> TileContextIterMut<'a, T> {
        let afs = fs as *mut _;
        let frame_me_stats = fs.frame_me_stats.write().expect("poisoned lock");
        TileContextIterMut {
            ti: *self,
            fs: afs,
            next: 0,
            frame_me_stats,
        }
    }
}

/// Iterator over tiled views
pub struct TileContextIterMut<'a, T: Pixel> {
    ti: TilingInfo,
    fs: *mut FrameState<T>,
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
                    let x = sbo.0.x << SB_SIZE_LOG2;
                    let y = sbo.0.y << SB_SIZE_LOG2;
                    let tile_width = self.ti.tile_width_sb << SB_SIZE_LOG2;
                    let tile_height = self.ti.tile_height_sb << SB_SIZE_LOG2;
                    let width = tile_width.min(self.ti.frame_width - x);
                    let height = tile_height.min(self.ti.frame_height - y);
                    TileStateMut::new(fs, sbo, width, height, frame_me_stats)
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

impl<T: Pixel> ExactSizeIterator for TileContextIterMut<'_, T> {
}
impl<T: Pixel> FusedIterator for TileContextIterMut<'_, T> {
}

/// Container for all tiled views
pub struct TileContextMut<'a, T: Pixel> {
    pub ts: TileStateMut<'a, T>,
}
