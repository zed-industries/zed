#[cfg(asm_x86_64)]
mod simd_x86;

use std::mem::{transmute, MaybeUninit};

use aligned::{Aligned, A64};
#[cfg(not(asm_x86_64))]
use rust::*;
#[cfg(asm_x86_64)]
use simd_x86::*;
use v_frame::{
    frame::Frame,
    pixel::Pixel,
    plane::{Plane, PlaneOffset},
};

use super::importance::IMPORTANCE_BLOCK_SIZE;
use crate::{
    cpu::CpuFeatureLevel,
    data::{
        block::{BlockSize, TxSize, MAX_TX_SIZE},
        plane::{Area, AsRegion, PlaneRegion, PlaneRegionMut, Rect},
        prediction::PredictionVariant,
        satd::get_satd,
        slice_assume_init_mut,
        superblock::MI_SIZE_LOG2,
        tile::TileRect,
    },
};

pub const BLOCK_TO_PLANE_SHIFT: usize = MI_SIZE_LOG2;

mod rust {
    use v_frame::pixel::Pixel;

    use super::IntraEdge;
    use crate::{
        cpu::CpuFeatureLevel,
        data::{block::TxSize, plane::PlaneRegionMut, prediction::PredictionVariant},
    };

    #[cfg_attr(
        all(asm_x86_64, any(target_feature = "ssse3", target_feature = "avx2")),
        cold
    )]
    pub(super) fn dispatch_predict_dc_intra<T: Pixel>(
        variant: PredictionVariant,
        dst: &mut PlaneRegionMut<'_, T>,
        tx_size: TxSize,
        bit_depth: usize,
        edge_buf: &IntraEdge<T>,
        _cpu: CpuFeatureLevel,
    ) {
        let width = tx_size.width();
        let height = tx_size.height();

        // left pixels are ordered from bottom to top and right-aligned
        let (left, _top_left, above) = edge_buf.as_slices();

        let above_slice = above;
        let left_slice = &left[left.len().saturating_sub(height)..];

        (match variant {
            PredictionVariant::NONE => pred_dc_128,
            PredictionVariant::LEFT => pred_dc_left,
            PredictionVariant::TOP => pred_dc_top,
            PredictionVariant::BOTH => pred_dc,
        })(dst, above_slice, left_slice, width, height, bit_depth)
    }

    fn pred_dc<T: Pixel>(
        output: &mut PlaneRegionMut<'_, T>,
        above: &[T],
        left: &[T],
        width: usize,
        height: usize,
        _bit_depth: usize,
    ) {
        let edges = left[..height].iter().chain(above[..width].iter());
        let len = (width + height) as u32;
        let avg = (edges.fold(0u32, |acc, &v| {
            let v: u32 = v.into();
            v + acc
        }) + (len >> 1))
            / len;
        let avg = T::cast_from(avg);

        for line in output.rows_iter_mut().take(height) {
            line[..width].fill(avg);
        }
    }

    fn pred_dc_128<T: Pixel>(
        output: &mut PlaneRegionMut<'_, T>,
        _above: &[T],
        _left: &[T],
        width: usize,
        height: usize,
        bit_depth: usize,
    ) {
        let v = T::cast_from(128u32 << (bit_depth - 8));
        for line in output.rows_iter_mut().take(height) {
            line[..width].fill(v);
        }
    }

    fn pred_dc_left<T: Pixel>(
        output: &mut PlaneRegionMut<'_, T>,
        _above: &[T],
        left: &[T],
        width: usize,
        height: usize,
        _bit_depth: usize,
    ) {
        let sum = left[..].iter().fold(0u32, |acc, &v| {
            let v: u32 = v.into();
            v + acc
        });
        let avg = T::cast_from((sum + (height >> 1) as u32) / height as u32);
        for line in output.rows_iter_mut().take(height) {
            line[..width].fill(avg);
        }
    }

    fn pred_dc_top<T: Pixel>(
        output: &mut PlaneRegionMut<'_, T>,
        above: &[T],
        _left: &[T],
        width: usize,
        height: usize,
        _bit_depth: usize,
    ) {
        let sum = above[..width].iter().fold(0u32, |acc, &v| {
            let v: u32 = v.into();
            v + acc
        });
        let avg = T::cast_from((sum + (width >> 1) as u32) / width as u32);
        for line in output.rows_iter_mut().take(height) {
            line[..width].fill(avg);
        }
    }
}

pub(crate) fn estimate_intra_costs<T: Pixel>(
    temp_plane: &mut Plane<T>,
    frame: &Frame<T>,
    bit_depth: usize,
    cpu_feature_level: CpuFeatureLevel,
) -> Box<[u32]> {
    let plane = &frame.planes[0];
    let plane_after_prediction = temp_plane;

    let bsize = BlockSize::from_width_and_height(IMPORTANCE_BLOCK_SIZE, IMPORTANCE_BLOCK_SIZE);
    let tx_size = bsize.tx_size();

    let h_in_imp_b = plane.cfg.height / IMPORTANCE_BLOCK_SIZE;
    let w_in_imp_b = plane.cfg.width / IMPORTANCE_BLOCK_SIZE;
    let mut intra_costs = Vec::with_capacity(h_in_imp_b * w_in_imp_b);

    for y in 0..h_in_imp_b {
        for x in 0..w_in_imp_b {
            let plane_org = plane.region(Area::Rect(Rect {
                x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
            }));

            // For scene detection, we are only going to support DC_PRED
            // for simplicity and speed purposes.
            let mut edge_buf = Aligned([MaybeUninit::uninit(); 4 * MAX_TX_SIZE + 1]);
            let edge_buf = get_intra_edges(
                &mut edge_buf,
                &plane.as_region(),
                PlaneOffset {
                    x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                    y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                },
                bit_depth,
            );

            let mut plane_after_prediction_region =
                plane_after_prediction.region_mut(Area::Rect(Rect {
                    x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                    y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                    width: IMPORTANCE_BLOCK_SIZE,
                    height: IMPORTANCE_BLOCK_SIZE,
                }));

            predict_dc_intra(
                TileRect {
                    x: x * IMPORTANCE_BLOCK_SIZE,
                    y: y * IMPORTANCE_BLOCK_SIZE,
                    width: IMPORTANCE_BLOCK_SIZE,
                    height: IMPORTANCE_BLOCK_SIZE,
                },
                &mut plane_after_prediction_region,
                tx_size,
                bit_depth,
                &edge_buf,
                cpu_feature_level,
            );

            let plane_after_prediction_region = plane_after_prediction.region(Area::Rect(Rect {
                x: (x * IMPORTANCE_BLOCK_SIZE) as isize,
                y: (y * IMPORTANCE_BLOCK_SIZE) as isize,
                width: IMPORTANCE_BLOCK_SIZE,
                height: IMPORTANCE_BLOCK_SIZE,
            }));

            let intra_cost = get_satd(
                &plane_org,
                &plane_after_prediction_region,
                bsize.width(),
                bsize.height(),
                bit_depth,
                cpu_feature_level,
            );

            intra_costs.push(intra_cost);
        }
    }

    intra_costs.into_boxed_slice()
}

pub fn get_intra_edges<'a, T: Pixel>(
    edge_buf: &'a mut IntraEdgeBuffer<T>,
    dst: &PlaneRegion<'_, T>,
    po: PlaneOffset,
    bit_depth: usize,
) -> IntraEdge<'a, T> {
    let tx_size = TxSize::TX_8X8;
    let mut init_left: usize = 0;
    let mut init_above: usize = 0;

    let base = 128u16 << (bit_depth - 8);

    {
        // left pixels are ordered from bottom to top and right-aligned
        let (left, not_left) = edge_buf.split_at_mut(2 * MAX_TX_SIZE);
        let (top_left, above) = not_left.split_at_mut(1);

        let x = po.x as usize;
        let y = po.y as usize;

        let needs_left = x != 0;
        let needs_top = y != 0;

        let rect_w = dst
            .rect()
            .width
            .min(dst.plane_cfg.width - dst.rect().x as usize);
        let rect_h = dst
            .rect()
            .height
            .min(dst.plane_cfg.height - dst.rect().y as usize);

        // Needs left
        if needs_left {
            let txh = if y + tx_size.height() > rect_h {
                rect_h - y
            } else {
                tx_size.height()
            };
            if x != 0 {
                for i in 0..txh {
                    debug_assert!(y + i < rect_h);
                    left[2 * MAX_TX_SIZE - 1 - i].write(dst[y + i][x - 1]);
                }
                if txh < tx_size.height() {
                    let val = dst[y + txh - 1][x - 1];
                    for i in txh..tx_size.height() {
                        left[2 * MAX_TX_SIZE - 1 - i].write(val);
                    }
                }
            } else {
                let val = if y != 0 {
                    dst[y - 1][0]
                } else {
                    T::cast_from(base + 1)
                };
                for v in left[2 * MAX_TX_SIZE - tx_size.height()..].iter_mut() {
                    v.write(val);
                }
            }
            init_left += tx_size.height();
        }

        // Needs top
        if needs_top {
            let txw = if x + tx_size.width() > rect_w {
                rect_w - x
            } else {
                tx_size.width()
            };
            if y != 0 {
                above[..txw].copy_from_slice(
                    // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
                    unsafe { transmute::<&[T], &[MaybeUninit<T>]>(&dst[y - 1][x..x + txw]) },
                );
                if txw < tx_size.width() {
                    let val = dst[y - 1][x + txw - 1];
                    for v in &mut above[txw..tx_size.width()] {
                        v.write(val);
                    }
                }
            } else {
                let val = if x != 0 {
                    dst[0][x - 1]
                } else {
                    T::cast_from(base - 1)
                };
                for v in &mut above[..tx_size.width()] {
                    v.write(val);
                }
            }
            init_above += tx_size.width();
        }

        top_left[0].write(T::cast_from(base));
    }
    IntraEdge::new(edge_buf, init_left, init_above)
}

pub fn predict_dc_intra<T: Pixel>(
    tile_rect: TileRect,
    dst: &mut PlaneRegionMut<'_, T>,
    tx_size: TxSize,
    bit_depth: usize,
    edge_buf: &IntraEdge<T>,
    cpu: CpuFeatureLevel,
) {
    let &Rect {
        x: frame_x,
        y: frame_y,
        ..
    } = dst.rect();
    debug_assert!(frame_x >= 0 && frame_y >= 0);
    // x and y are expressed relative to the tile
    let x = frame_x as usize - tile_rect.x;
    let y = frame_y as usize - tile_rect.y;

    let variant = PredictionVariant::new(x, y);

    dispatch_predict_dc_intra::<T>(variant, dst, tx_size, bit_depth, edge_buf, cpu);
}

type IntraEdgeBuffer<T> = Aligned<A64, [MaybeUninit<T>; 4 * MAX_TX_SIZE + 1]>;

pub struct IntraEdge<'a, T: Pixel>(&'a [T], &'a [T], &'a [T]);

impl<'a, T: Pixel> IntraEdge<'a, T> {
    fn new(edge_buf: &'a mut IntraEdgeBuffer<T>, init_left: usize, init_above: usize) -> Self {
        // SAFETY: Initialized in `get_intra_edges`.
        let left = unsafe {
            let begin_left = 2 * MAX_TX_SIZE - init_left;
            let end_above = 2 * MAX_TX_SIZE + 1 + init_above;
            slice_assume_init_mut(&mut edge_buf[begin_left..end_above])
        };
        let (left, top_left) = left.split_at(init_left);
        let (top_left, above) = top_left.split_at(1);
        Self(left, top_left, above)
    }

    pub const fn as_slices(&self) -> (&'a [T], &'a [T], &'a [T]) {
        (self.0, self.1, self.2)
    }

    #[allow(dead_code)]
    pub const fn top_left_ptr(&self) -> *const T {
        self.1.as_ptr()
    }
}
