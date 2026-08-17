use std::{
    marker::PhantomData,
    ops,
    ops::{Index, IndexMut},
    slice,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use arrayvec::ArrayVec;
use v_frame::{frame::Frame, pixel::Pixel, plane::Plane};

const MV_IN_USE_BITS: usize = 14;
pub const MV_UPP: i32 = 1 << MV_IN_USE_BITS;
pub const MV_LOW: i32 = -(1 << MV_IN_USE_BITS);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MotionVector {
    pub row: i16,
    pub col: i16,
}

impl MotionVector {
    pub const fn quantize_to_fullpel(self) -> Self {
        Self {
            row: (self.row / 8) * 8,
            col: (self.col / 8) * 8,
        }
    }
}

impl ops::Mul<i16> for MotionVector {
    type Output = MotionVector;

    fn mul(self, rhs: i16) -> MotionVector {
        MotionVector {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl ops::Mul<u16> for MotionVector {
    type Output = MotionVector;

    fn mul(self, rhs: u16) -> MotionVector {
        MotionVector {
            row: self.row * rhs as i16,
            col: self.col * rhs as i16,
        }
    }
}

impl ops::Shr<u8> for MotionVector {
    type Output = MotionVector;

    fn shr(self, rhs: u8) -> MotionVector {
        MotionVector {
            row: self.row >> rhs,
            col: self.col >> rhs,
        }
    }
}

impl ops::Shl<u8> for MotionVector {
    type Output = MotionVector;

    fn shl(self, rhs: u8) -> MotionVector {
        MotionVector {
            row: self.row << rhs,
            col: self.col << rhs,
        }
    }
}

impl ops::Add<MotionVector> for MotionVector {
    type Output = MotionVector;

    fn add(self, rhs: MotionVector) -> MotionVector {
        MotionVector {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MEStats {
    pub mv: MotionVector,
    /// SAD value on the scale of a 128x128 block
    pub normalized_sad: u32,
}

#[derive(Debug, Clone)]
pub struct FrameMEStats {
    stats: Box<[MEStats]>,
    pub cols: usize,
    pub rows: usize,
}

pub const REF_FRAMES_LOG2: usize = 3;
pub const REF_FRAMES: usize = 1 << REF_FRAMES_LOG2;
pub type RefMEStats = Arc<RwLock<[FrameMEStats; REF_FRAMES]>>;
pub type ReadGuardMEStats<'a> = RwLockReadGuard<'a, [FrameMEStats; REF_FRAMES]>;
pub type WriteGuardMEStats<'a> = RwLockWriteGuard<'a, [FrameMEStats; REF_FRAMES]>;

impl FrameMEStats {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            // dynamic allocation: once per frame
            stats: vec![MEStats::default(); cols * rows].into_boxed_slice(),
            cols,
            rows,
        }
    }

    pub fn new_arc_array(cols: usize, rows: usize) -> RefMEStats {
        Arc::new(RwLock::new([
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
            FrameMEStats::new(cols, rows),
        ]))
    }
}

impl Index<usize> for FrameMEStats {
    type Output = [MEStats];

    fn index(&self, index: usize) -> &Self::Output {
        &self.stats[index * self.cols..(index + 1) * self.cols]
    }
}

impl IndexMut<usize> for FrameMEStats {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stats[index * self.cols..(index + 1) * self.cols]
    }
}

/// Tiled view of `FrameMEStats`
#[derive(Debug)]
pub struct TileMEStats<'a> {
    data: *const MEStats,
    // expressed in mi blocks
    // private to guarantee borrowing rules
    x: usize,
    y: usize,
    cols: usize,
    rows: usize,
    /// number of cols in the underlying `FrameMEStats`
    stride: usize,
    phantom: PhantomData<&'a MotionVector>,
}

/// Mutable tiled view of `FrameMEStats`
#[derive(Debug)]
pub struct TileMEStatsMut<'a> {
    data: *mut MEStats,
    // expressed in mi blocks
    // private to guarantee borrowing rules
    x: usize,
    y: usize,
    cols: usize,
    rows: usize,
    /// number of cols in the underlying `FrameMEStats`
    stride: usize,
    phantom: PhantomData<&'a mut MotionVector>,
}

// common impl for TileMotionVectors and TileMotionVectorsMut
macro_rules! tile_me_stats_common {
  // $name: TileMEStats or TileMEStatsMut
  // $opt_mut: nothing or mut
  ($name:ident $(,$opt_mut:tt)?) => {
    impl<'a> $name<'a> {

      /// # Panics
      ///
      /// - If the requested dimensions are larger than the frame MV size
      #[allow(dead_code)]
      pub fn new(
        frame_mvs: &'a $($opt_mut)? FrameMEStats,
        x: usize,
        y: usize,
        cols: usize,
        rows: usize,
      ) -> Self {
        assert!(x + cols <= frame_mvs.cols);
        assert!(y + rows <= frame_mvs.rows);
        Self {
          data: & $($opt_mut)? frame_mvs[y][x],
          x,
          y,
          cols,
          rows,
          stride: frame_mvs.cols,
          phantom: PhantomData,
        }
      }

      #[allow(dead_code)]
      pub const fn x(&self) -> usize {
        self.x
      }

      #[allow(dead_code)]
      pub const fn y(&self) -> usize {
        self.y
      }

      #[allow(dead_code)]
      pub const fn cols(&self) -> usize {
        self.cols
      }

      #[allow(dead_code)]
      pub const fn rows(&self) -> usize {
        self.rows
      }
    }

    unsafe impl Send for $name<'_> {}
    unsafe impl Sync for $name<'_> {}

    impl Index<usize> for $name<'_> {
      type Output = [MEStats];

      fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows);
        // SAFETY: The above assert ensures we do not access OOB data.
        unsafe {
          let ptr = self.data.add(index * self.stride);
          slice::from_raw_parts(ptr, self.cols)
        }
      }
    }
  }
}

tile_me_stats_common!(TileMEStats);
tile_me_stats_common!(TileMEStatsMut, mut);

impl TileMEStatsMut<'_> {
    pub const fn as_const(&self) -> TileMEStats<'_> {
        TileMEStats {
            data: self.data,
            x: self.x,
            y: self.y,
            cols: self.cols,
            rows: self.rows,
            stride: self.stride,
            phantom: PhantomData,
        }
    }
}

impl IndexMut<usize> for TileMEStatsMut<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.rows);
        // SAFETY: The above assert ensures we do not access OOB data.
        unsafe {
            let ptr = self.data.add(index * self.stride);
            slice::from_raw_parts_mut(ptr, self.cols)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum MVSamplingMode {
    INIT,
    CORNER { right: bool, bottom: bool },
}

#[derive(Debug, Clone)]
pub struct ReferenceFrame<T: Pixel> {
    pub frame: Arc<Frame<T>>,
    pub input_hres: Arc<Plane<T>>,
    pub input_qres: Arc<Plane<T>>,
    pub frame_me_stats: RefMEStats,
}

#[derive(Debug, Clone, Default)]
pub struct ReferenceFramesSet<T: Pixel> {
    pub frames: [Option<Arc<ReferenceFrame<T>>>; REF_FRAMES],
}

impl<T: Pixel> ReferenceFramesSet<T> {
    pub fn new() -> Self {
        Self {
            frames: Default::default(),
            // deblock: Default::default()
        }
    }
}

pub struct MotionEstimationSubsets {
    pub min_sad: u32,
    pub median: Option<MotionVector>,
    pub subset_b: ArrayVec<MotionVector, 5>,
    pub subset_c: ArrayVec<MotionVector, 5>,
}

impl MotionEstimationSubsets {
    pub fn all_mvs(&self) -> ArrayVec<MotionVector, 11> {
        let mut all = ArrayVec::new();
        if let Some(median) = self.median {
            all.push(median);
        }

        all.extend(self.subset_b.iter().copied());
        all.extend(self.subset_c.iter().copied());

        all
    }
}
