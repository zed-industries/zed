use super::BinaryReader;
use crate::{Result, VisitOperator, VisitSimdOperator};

impl<'a> BinaryReader<'a> {
    pub(super) fn visit_0xfd_operator<T>(
        &mut self,
        pos: usize,
        visitor: &mut T,
    ) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitSimdOperator<'a>,
    {
        let code = self.read_var_u32()?;
        Ok(match code {
            0x00 => visitor.visit_v128_load(self.read_memarg(4)?),
            0x01 => visitor.visit_v128_load8x8_s(self.read_memarg(3)?),
            0x02 => visitor.visit_v128_load8x8_u(self.read_memarg(3)?),
            0x03 => visitor.visit_v128_load16x4_s(self.read_memarg(3)?),
            0x04 => visitor.visit_v128_load16x4_u(self.read_memarg(3)?),
            0x05 => visitor.visit_v128_load32x2_s(self.read_memarg(3)?),
            0x06 => visitor.visit_v128_load32x2_u(self.read_memarg(3)?),
            0x07 => visitor.visit_v128_load8_splat(self.read_memarg(0)?),
            0x08 => visitor.visit_v128_load16_splat(self.read_memarg(1)?),
            0x09 => visitor.visit_v128_load32_splat(self.read_memarg(2)?),
            0x0a => visitor.visit_v128_load64_splat(self.read_memarg(3)?),

            0x0b => visitor.visit_v128_store(self.read_memarg(4)?),
            0x0c => visitor.visit_v128_const(self.read_v128()?),
            0x0d => {
                let mut lanes: [u8; 16] = [0; 16];
                for lane in &mut lanes {
                    *lane = self.read_lane_index(32)?
                }
                visitor.visit_i8x16_shuffle(lanes)
            }

            0x0e => visitor.visit_i8x16_swizzle(),
            0x0f => visitor.visit_i8x16_splat(),
            0x10 => visitor.visit_i16x8_splat(),
            0x11 => visitor.visit_i32x4_splat(),
            0x12 => visitor.visit_i64x2_splat(),
            0x13 => visitor.visit_f32x4_splat(),
            0x14 => visitor.visit_f64x2_splat(),

            0x15 => visitor.visit_i8x16_extract_lane_s(self.read_lane_index(16)?),
            0x16 => visitor.visit_i8x16_extract_lane_u(self.read_lane_index(16)?),
            0x17 => visitor.visit_i8x16_replace_lane(self.read_lane_index(16)?),
            0x18 => visitor.visit_i16x8_extract_lane_s(self.read_lane_index(8)?),
            0x19 => visitor.visit_i16x8_extract_lane_u(self.read_lane_index(8)?),
            0x1a => visitor.visit_i16x8_replace_lane(self.read_lane_index(8)?),
            0x1b => visitor.visit_i32x4_extract_lane(self.read_lane_index(4)?),

            0x1c => visitor.visit_i32x4_replace_lane(self.read_lane_index(4)?),
            0x1d => visitor.visit_i64x2_extract_lane(self.read_lane_index(2)?),
            0x1e => visitor.visit_i64x2_replace_lane(self.read_lane_index(2)?),
            0x1f => visitor.visit_f32x4_extract_lane(self.read_lane_index(4)?),
            0x20 => visitor.visit_f32x4_replace_lane(self.read_lane_index(4)?),
            0x21 => visitor.visit_f64x2_extract_lane(self.read_lane_index(2)?),
            0x22 => visitor.visit_f64x2_replace_lane(self.read_lane_index(2)?),

            0x23 => visitor.visit_i8x16_eq(),
            0x24 => visitor.visit_i8x16_ne(),
            0x25 => visitor.visit_i8x16_lt_s(),
            0x26 => visitor.visit_i8x16_lt_u(),
            0x27 => visitor.visit_i8x16_gt_s(),
            0x28 => visitor.visit_i8x16_gt_u(),
            0x29 => visitor.visit_i8x16_le_s(),
            0x2a => visitor.visit_i8x16_le_u(),
            0x2b => visitor.visit_i8x16_ge_s(),
            0x2c => visitor.visit_i8x16_ge_u(),
            0x2d => visitor.visit_i16x8_eq(),
            0x2e => visitor.visit_i16x8_ne(),
            0x2f => visitor.visit_i16x8_lt_s(),
            0x30 => visitor.visit_i16x8_lt_u(),
            0x31 => visitor.visit_i16x8_gt_s(),
            0x32 => visitor.visit_i16x8_gt_u(),
            0x33 => visitor.visit_i16x8_le_s(),
            0x34 => visitor.visit_i16x8_le_u(),
            0x35 => visitor.visit_i16x8_ge_s(),
            0x36 => visitor.visit_i16x8_ge_u(),
            0x37 => visitor.visit_i32x4_eq(),
            0x38 => visitor.visit_i32x4_ne(),
            0x39 => visitor.visit_i32x4_lt_s(),
            0x3a => visitor.visit_i32x4_lt_u(),
            0x3b => visitor.visit_i32x4_gt_s(),
            0x3c => visitor.visit_i32x4_gt_u(),
            0x3d => visitor.visit_i32x4_le_s(),
            0x3e => visitor.visit_i32x4_le_u(),
            0x3f => visitor.visit_i32x4_ge_s(),
            0x40 => visitor.visit_i32x4_ge_u(),
            0x41 => visitor.visit_f32x4_eq(),
            0x42 => visitor.visit_f32x4_ne(),
            0x43 => visitor.visit_f32x4_lt(),
            0x44 => visitor.visit_f32x4_gt(),
            0x45 => visitor.visit_f32x4_le(),
            0x46 => visitor.visit_f32x4_ge(),
            0x47 => visitor.visit_f64x2_eq(),
            0x48 => visitor.visit_f64x2_ne(),
            0x49 => visitor.visit_f64x2_lt(),
            0x4a => visitor.visit_f64x2_gt(),
            0x4b => visitor.visit_f64x2_le(),
            0x4c => visitor.visit_f64x2_ge(),
            0x4d => visitor.visit_v128_not(),
            0x4e => visitor.visit_v128_and(),
            0x4f => visitor.visit_v128_andnot(),
            0x50 => visitor.visit_v128_or(),
            0x51 => visitor.visit_v128_xor(),
            0x52 => visitor.visit_v128_bitselect(),
            0x53 => visitor.visit_v128_any_true(),

            0x54 => {
                let memarg = self.read_memarg(0)?;
                let lane = self.read_lane_index(16)?;
                visitor.visit_v128_load8_lane(memarg, lane)
            }
            0x55 => {
                let memarg = self.read_memarg(1)?;
                let lane = self.read_lane_index(8)?;
                visitor.visit_v128_load16_lane(memarg, lane)
            }
            0x56 => {
                let memarg = self.read_memarg(2)?;
                let lane = self.read_lane_index(4)?;
                visitor.visit_v128_load32_lane(memarg, lane)
            }
            0x57 => {
                let memarg = self.read_memarg(3)?;
                let lane = self.read_lane_index(2)?;
                visitor.visit_v128_load64_lane(memarg, lane)
            }
            0x58 => {
                let memarg = self.read_memarg(0)?;
                let lane = self.read_lane_index(16)?;
                visitor.visit_v128_store8_lane(memarg, lane)
            }
            0x59 => {
                let memarg = self.read_memarg(1)?;
                let lane = self.read_lane_index(8)?;
                visitor.visit_v128_store16_lane(memarg, lane)
            }
            0x5a => {
                let memarg = self.read_memarg(2)?;
                let lane = self.read_lane_index(4)?;
                visitor.visit_v128_store32_lane(memarg, lane)
            }
            0x5b => {
                let memarg = self.read_memarg(3)?;
                let lane = self.read_lane_index(2)?;
                visitor.visit_v128_store64_lane(memarg, lane)
            }

            0x5c => visitor.visit_v128_load32_zero(self.read_memarg(2)?),
            0x5d => visitor.visit_v128_load64_zero(self.read_memarg(3)?),
            0x5e => visitor.visit_f32x4_demote_f64x2_zero(),
            0x5f => visitor.visit_f64x2_promote_low_f32x4(),
            0x60 => visitor.visit_i8x16_abs(),
            0x61 => visitor.visit_i8x16_neg(),
            0x62 => visitor.visit_i8x16_popcnt(),
            0x63 => visitor.visit_i8x16_all_true(),
            0x64 => visitor.visit_i8x16_bitmask(),
            0x65 => visitor.visit_i8x16_narrow_i16x8_s(),
            0x66 => visitor.visit_i8x16_narrow_i16x8_u(),
            0x67 => visitor.visit_f32x4_ceil(),
            0x68 => visitor.visit_f32x4_floor(),
            0x69 => visitor.visit_f32x4_trunc(),
            0x6a => visitor.visit_f32x4_nearest(),
            0x6b => visitor.visit_i8x16_shl(),
            0x6c => visitor.visit_i8x16_shr_s(),
            0x6d => visitor.visit_i8x16_shr_u(),
            0x6e => visitor.visit_i8x16_add(),
            0x6f => visitor.visit_i8x16_add_sat_s(),
            0x70 => visitor.visit_i8x16_add_sat_u(),
            0x71 => visitor.visit_i8x16_sub(),
            0x72 => visitor.visit_i8x16_sub_sat_s(),
            0x73 => visitor.visit_i8x16_sub_sat_u(),
            0x74 => visitor.visit_f64x2_ceil(),
            0x75 => visitor.visit_f64x2_floor(),
            0x76 => visitor.visit_i8x16_min_s(),
            0x77 => visitor.visit_i8x16_min_u(),
            0x78 => visitor.visit_i8x16_max_s(),
            0x79 => visitor.visit_i8x16_max_u(),
            0x7a => visitor.visit_f64x2_trunc(),
            0x7b => visitor.visit_i8x16_avgr_u(),
            0x7c => visitor.visit_i16x8_extadd_pairwise_i8x16_s(),
            0x7d => visitor.visit_i16x8_extadd_pairwise_i8x16_u(),
            0x7e => visitor.visit_i32x4_extadd_pairwise_i16x8_s(),
            0x7f => visitor.visit_i32x4_extadd_pairwise_i16x8_u(),
            0x80 => visitor.visit_i16x8_abs(),
            0x81 => visitor.visit_i16x8_neg(),
            0x82 => visitor.visit_i16x8_q15mulr_sat_s(),
            0x83 => visitor.visit_i16x8_all_true(),
            0x84 => visitor.visit_i16x8_bitmask(),
            0x85 => visitor.visit_i16x8_narrow_i32x4_s(),
            0x86 => visitor.visit_i16x8_narrow_i32x4_u(),
            0x87 => visitor.visit_i16x8_extend_low_i8x16_s(),
            0x88 => visitor.visit_i16x8_extend_high_i8x16_s(),
            0x89 => visitor.visit_i16x8_extend_low_i8x16_u(),
            0x8a => visitor.visit_i16x8_extend_high_i8x16_u(),
            0x8b => visitor.visit_i16x8_shl(),
            0x8c => visitor.visit_i16x8_shr_s(),
            0x8d => visitor.visit_i16x8_shr_u(),
            0x8e => visitor.visit_i16x8_add(),
            0x8f => visitor.visit_i16x8_add_sat_s(),
            0x90 => visitor.visit_i16x8_add_sat_u(),
            0x91 => visitor.visit_i16x8_sub(),
            0x92 => visitor.visit_i16x8_sub_sat_s(),
            0x93 => visitor.visit_i16x8_sub_sat_u(),
            0x94 => visitor.visit_f64x2_nearest(),
            0x95 => visitor.visit_i16x8_mul(),
            0x96 => visitor.visit_i16x8_min_s(),
            0x97 => visitor.visit_i16x8_min_u(),
            0x98 => visitor.visit_i16x8_max_s(),
            0x99 => visitor.visit_i16x8_max_u(),
            0x9b => visitor.visit_i16x8_avgr_u(),
            0x9c => visitor.visit_i16x8_extmul_low_i8x16_s(),
            0x9d => visitor.visit_i16x8_extmul_high_i8x16_s(),
            0x9e => visitor.visit_i16x8_extmul_low_i8x16_u(),
            0x9f => visitor.visit_i16x8_extmul_high_i8x16_u(),
            0xa0 => visitor.visit_i32x4_abs(),
            0xa1 => visitor.visit_i32x4_neg(),
            0xa3 => visitor.visit_i32x4_all_true(),
            0xa4 => visitor.visit_i32x4_bitmask(),
            0xa7 => visitor.visit_i32x4_extend_low_i16x8_s(),
            0xa8 => visitor.visit_i32x4_extend_high_i16x8_s(),
            0xa9 => visitor.visit_i32x4_extend_low_i16x8_u(),
            0xaa => visitor.visit_i32x4_extend_high_i16x8_u(),
            0xab => visitor.visit_i32x4_shl(),
            0xac => visitor.visit_i32x4_shr_s(),
            0xad => visitor.visit_i32x4_shr_u(),
            0xae => visitor.visit_i32x4_add(),
            0xb1 => visitor.visit_i32x4_sub(),
            0xb5 => visitor.visit_i32x4_mul(),
            0xb6 => visitor.visit_i32x4_min_s(),
            0xb7 => visitor.visit_i32x4_min_u(),
            0xb8 => visitor.visit_i32x4_max_s(),
            0xb9 => visitor.visit_i32x4_max_u(),
            0xba => visitor.visit_i32x4_dot_i16x8_s(),
            0xbc => visitor.visit_i32x4_extmul_low_i16x8_s(),
            0xbd => visitor.visit_i32x4_extmul_high_i16x8_s(),
            0xbe => visitor.visit_i32x4_extmul_low_i16x8_u(),
            0xbf => visitor.visit_i32x4_extmul_high_i16x8_u(),
            0xc0 => visitor.visit_i64x2_abs(),
            0xc1 => visitor.visit_i64x2_neg(),
            0xc3 => visitor.visit_i64x2_all_true(),
            0xc4 => visitor.visit_i64x2_bitmask(),
            0xc7 => visitor.visit_i64x2_extend_low_i32x4_s(),
            0xc8 => visitor.visit_i64x2_extend_high_i32x4_s(),
            0xc9 => visitor.visit_i64x2_extend_low_i32x4_u(),
            0xca => visitor.visit_i64x2_extend_high_i32x4_u(),
            0xcb => visitor.visit_i64x2_shl(),
            0xcc => visitor.visit_i64x2_shr_s(),
            0xcd => visitor.visit_i64x2_shr_u(),
            0xce => visitor.visit_i64x2_add(),
            0xd1 => visitor.visit_i64x2_sub(),
            0xd5 => visitor.visit_i64x2_mul(),
            0xd6 => visitor.visit_i64x2_eq(),
            0xd7 => visitor.visit_i64x2_ne(),
            0xd8 => visitor.visit_i64x2_lt_s(),
            0xd9 => visitor.visit_i64x2_gt_s(),
            0xda => visitor.visit_i64x2_le_s(),
            0xdb => visitor.visit_i64x2_ge_s(),
            0xdc => visitor.visit_i64x2_extmul_low_i32x4_s(),
            0xdd => visitor.visit_i64x2_extmul_high_i32x4_s(),
            0xde => visitor.visit_i64x2_extmul_low_i32x4_u(),
            0xdf => visitor.visit_i64x2_extmul_high_i32x4_u(),
            0xe0 => visitor.visit_f32x4_abs(),
            0xe1 => visitor.visit_f32x4_neg(),
            0xe3 => visitor.visit_f32x4_sqrt(),
            0xe4 => visitor.visit_f32x4_add(),
            0xe5 => visitor.visit_f32x4_sub(),
            0xe6 => visitor.visit_f32x4_mul(),
            0xe7 => visitor.visit_f32x4_div(),
            0xe8 => visitor.visit_f32x4_min(),
            0xe9 => visitor.visit_f32x4_max(),
            0xea => visitor.visit_f32x4_pmin(),
            0xeb => visitor.visit_f32x4_pmax(),
            0xec => visitor.visit_f64x2_abs(),
            0xed => visitor.visit_f64x2_neg(),
            0xef => visitor.visit_f64x2_sqrt(),
            0xf0 => visitor.visit_f64x2_add(),
            0xf1 => visitor.visit_f64x2_sub(),
            0xf2 => visitor.visit_f64x2_mul(),
            0xf3 => visitor.visit_f64x2_div(),
            0xf4 => visitor.visit_f64x2_min(),
            0xf5 => visitor.visit_f64x2_max(),
            0xf6 => visitor.visit_f64x2_pmin(),
            0xf7 => visitor.visit_f64x2_pmax(),
            0xf8 => visitor.visit_i32x4_trunc_sat_f32x4_s(),
            0xf9 => visitor.visit_i32x4_trunc_sat_f32x4_u(),
            0xfa => visitor.visit_f32x4_convert_i32x4_s(),
            0xfb => visitor.visit_f32x4_convert_i32x4_u(),
            0xfc => visitor.visit_i32x4_trunc_sat_f64x2_s_zero(),
            0xfd => visitor.visit_i32x4_trunc_sat_f64x2_u_zero(),
            0xfe => visitor.visit_f64x2_convert_low_i32x4_s(),
            0xff => visitor.visit_f64x2_convert_low_i32x4_u(),
            0x100 => visitor.visit_i8x16_relaxed_swizzle(),
            0x101 => visitor.visit_i32x4_relaxed_trunc_f32x4_s(),
            0x102 => visitor.visit_i32x4_relaxed_trunc_f32x4_u(),
            0x103 => visitor.visit_i32x4_relaxed_trunc_f64x2_s_zero(),
            0x104 => visitor.visit_i32x4_relaxed_trunc_f64x2_u_zero(),
            0x105 => visitor.visit_f32x4_relaxed_madd(),
            0x106 => visitor.visit_f32x4_relaxed_nmadd(),
            0x107 => visitor.visit_f64x2_relaxed_madd(),
            0x108 => visitor.visit_f64x2_relaxed_nmadd(),
            0x109 => visitor.visit_i8x16_relaxed_laneselect(),
            0x10a => visitor.visit_i16x8_relaxed_laneselect(),
            0x10b => visitor.visit_i32x4_relaxed_laneselect(),
            0x10c => visitor.visit_i64x2_relaxed_laneselect(),
            0x10d => visitor.visit_f32x4_relaxed_min(),
            0x10e => visitor.visit_f32x4_relaxed_max(),
            0x10f => visitor.visit_f64x2_relaxed_min(),
            0x110 => visitor.visit_f64x2_relaxed_max(),
            0x111 => visitor.visit_i16x8_relaxed_q15mulr_s(),
            0x112 => visitor.visit_i16x8_relaxed_dot_i8x16_i7x16_s(),
            0x113 => visitor.visit_i32x4_relaxed_dot_i8x16_i7x16_add_s(),

            _ => bail!(pos, "unknown 0xfd subopcode: 0x{code:x}"),
        })
    }
}
