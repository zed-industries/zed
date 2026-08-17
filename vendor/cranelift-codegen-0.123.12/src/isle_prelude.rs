//! Shared ISLE prelude implementation for optimization (mid-end) and
//! lowering (backend) ISLE environments.

/// Helper macro to define methods in `prelude.isle` within `impl Context for
/// ...` for each backend. These methods are shared amongst all backends.
#[macro_export]
#[doc(hidden)]
macro_rules! isle_common_prelude_methods {
    () => {
        isle_numerics_methods!();

        /// We don't have a way of making a `()` value in isle directly.
        #[inline]
        fn unit(&mut self) -> Unit {
            ()
        }

        #[inline]
        fn checked_add_with_type(&mut self, ty: Type, a: u64, b: u64) -> Option<u64> {
            let c = a.checked_add(b)?;
            let ty_mask = self.ty_mask(ty);
            if (c & !ty_mask) == 0 { Some(c) } else { None }
        }

        #[inline]
        fn add_overflows_with_type(&mut self, ty: Type, a: u64, b: u64) -> bool {
            self.checked_add_with_type(ty, a, b).is_none()
        }

        #[inline]
        fn imm64_sdiv(&mut self, ty: Type, x: Imm64, y: Imm64) -> Option<Imm64> {
            // Sign extend `x` and `y`.
            let shift = u32::checked_sub(64, ty.bits()).unwrap_or(0);
            let x = (x.bits() << shift) >> shift;
            let y = (y.bits() << shift) >> shift;

            // NB: We can't rely on `checked_div` to detect `ty::MIN / -1`
            // (which overflows and should trap) because we are working with
            // `i64` values here, and `i32::MIN != i64::MIN`, for
            // example. Therefore, we have to explicitly check for this case
            // ourselves.
            let min = ((self.ty_smin(ty) as i64) << shift) >> shift;
            if x == min && y == -1 {
                return None;
            }

            let ty_mask = self.ty_mask(ty) as i64;
            let result = x.checked_div(y)? & ty_mask;
            Some(Imm64::new(result))
        }

        #[inline]
        fn imm64_shl(&mut self, ty: Type, x: Imm64, y: Imm64) -> Imm64 {
            // Mask off any excess shift bits.
            let shift_mask = (ty.bits() - 1) as u64;
            let y = (y.bits() as u64) & shift_mask;

            // Mask the result to `ty` bits.
            let ty_mask = self.ty_mask(ty) as i64;
            Imm64::new((x.bits() << y) & ty_mask)
        }

        #[inline]
        fn imm64_ushr(&mut self, ty: Type, x: Imm64, y: Imm64) -> Imm64 {
            let ty_mask = self.ty_mask(ty);
            let x = (x.bits() as u64) & ty_mask;

            // Mask off any excess shift bits.
            let shift_mask = (ty.bits() - 1) as u64;
            let y = (y.bits() as u64) & shift_mask;

            // NB: No need to mask off high bits because they are already zero.
            Imm64::new((x >> y) as i64)
        }

        #[inline]
        fn imm64_sshr(&mut self, ty: Type, x: Imm64, y: Imm64) -> Imm64 {
            // Sign extend `x` from `ty.bits()`-width to the full 64 bits.
            let shift = u32::checked_sub(64, ty.bits()).unwrap_or(0);
            let x = (x.bits() << shift) >> shift;

            // Mask off any excess shift bits.
            let shift_mask = (ty.bits() - 1) as i64;
            let y = y.bits() & shift_mask;

            // Mask off sign bits that aren't part of `ty`.
            let ty_mask = self.ty_mask(ty) as i64;
            Imm64::new((x >> y) & ty_mask)
        }

        #[inline]
        fn i64_sextend_u64(&mut self, ty: Type, x: u64) -> i64 {
            let shift_amt = std::cmp::max(0, 64 - ty.bits());
            ((x as i64) << shift_amt) >> shift_amt
        }

        #[inline]
        fn i64_sextend_imm64(&mut self, ty: Type, x: Imm64) -> i64 {
            x.sign_extend_from_width(ty.bits()).bits()
        }

        #[inline]
        fn u64_uextend_imm64(&mut self, ty: Type, x: Imm64) -> u64 {
            (x.bits() as u64) & self.ty_mask(ty)
        }

        #[inline]
        fn imm64_icmp(&mut self, ty: Type, cc: &IntCC, x: Imm64, y: Imm64) -> Imm64 {
            let ux = self.u64_uextend_imm64(ty, x);
            let uy = self.u64_uextend_imm64(ty, y);
            let sx = self.i64_sextend_imm64(ty, x);
            let sy = self.i64_sextend_imm64(ty, y);
            let result = match cc {
                IntCC::Equal => ux == uy,
                IntCC::NotEqual => ux != uy,
                IntCC::UnsignedGreaterThanOrEqual => ux >= uy,
                IntCC::UnsignedGreaterThan => ux > uy,
                IntCC::UnsignedLessThanOrEqual => ux <= uy,
                IntCC::UnsignedLessThan => ux < uy,
                IntCC::SignedGreaterThanOrEqual => sx >= sy,
                IntCC::SignedGreaterThan => sx > sy,
                IntCC::SignedLessThanOrEqual => sx <= sy,
                IntCC::SignedLessThan => sx < sy,
            };
            Imm64::new(result.into())
        }

        #[inline]
        fn ty_bits(&mut self, ty: Type) -> u8 {
            use std::convert::TryInto;
            ty.bits().try_into().unwrap()
        }

        #[inline]
        fn ty_bits_u16(&mut self, ty: Type) -> u16 {
            ty.bits() as u16
        }

        #[inline]
        fn ty_bits_u64(&mut self, ty: Type) -> u64 {
            ty.bits() as u64
        }

        #[inline]
        fn ty_bytes(&mut self, ty: Type) -> u16 {
            u16::try_from(ty.bytes()).unwrap()
        }

        #[inline]
        fn ty_mask(&mut self, ty: Type) -> u64 {
            let ty_bits = ty.bits();
            debug_assert_ne!(ty_bits, 0);
            let shift = 64_u64
                .checked_sub(ty_bits.into())
                .expect("unimplemented for > 64 bits");
            u64::MAX >> shift
        }

        #[inline]
        fn ty_lane_mask(&mut self, ty: Type) -> u64 {
            let ty_lane_count = ty.lane_count();
            debug_assert_ne!(ty_lane_count, 0);
            let shift = 64_u64
                .checked_sub(ty_lane_count.into())
                .expect("unimplemented for > 64 bits");
            u64::MAX >> shift
        }

        #[inline]
        fn ty_lane_count(&mut self, ty: Type) -> u64 {
            ty.lane_count() as u64
        }

        #[inline]
        fn ty_umin(&mut self, _ty: Type) -> u64 {
            0
        }

        #[inline]
        fn ty_umax(&mut self, ty: Type) -> u64 {
            self.ty_mask(ty)
        }

        #[inline]
        fn ty_smin(&mut self, ty: Type) -> u64 {
            let ty_bits = ty.bits();
            debug_assert_ne!(ty_bits, 0);
            let shift = 64_u64
                .checked_sub(ty_bits.into())
                .expect("unimplemented for > 64 bits");
            (i64::MIN as u64) >> shift
        }

        #[inline]
        fn ty_smax(&mut self, ty: Type) -> u64 {
            let ty_bits = ty.bits();
            debug_assert_ne!(ty_bits, 0);
            let shift = 64_u64
                .checked_sub(ty_bits.into())
                .expect("unimplemented for > 64 bits");
            (i64::MAX as u64) >> shift
        }

        fn fits_in_16(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() <= 16 && !ty.is_dynamic_vector() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn fits_in_32(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() <= 32 && !ty.is_dynamic_vector() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn lane_fits_in_32(&mut self, ty: Type) -> Option<Type> {
            if !ty.is_vector() && !ty.is_dynamic_vector() {
                None
            } else if ty.lane_type().bits() <= 32 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn fits_in_64(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() <= 64 && !ty.is_dynamic_vector() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_int_ref_scalar_64(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() <= 64 && !ty.is_float() && !ty.is_vector() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_int_ref_scalar_64_extract(&mut self, ty: Type) -> Option<Type> {
            self.ty_int_ref_scalar_64(ty)
        }

        #[inline]
        fn ty_16(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 16 { Some(ty) } else { None }
        }

        #[inline]
        fn ty_32(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 32 { Some(ty) } else { None }
        }

        #[inline]
        fn ty_64(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 64 { Some(ty) } else { None }
        }

        #[inline]
        fn ty_128(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 128 { Some(ty) } else { None }
        }

        #[inline]
        fn ty_32_or_64(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 32 || ty.bits() == 64 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_8_or_16(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 8 || ty.bits() == 16 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_16_or_32(&mut self, ty: Type) -> Option<Type> {
            if ty.bits() == 16 || ty.bits() == 32 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn int_fits_in_32(&mut self, ty: Type) -> Option<Type> {
            match ty {
                I8 | I16 | I32 => Some(ty),
                _ => None,
            }
        }

        #[inline]
        fn ty_int_ref_64(&mut self, ty: Type) -> Option<Type> {
            match ty {
                I64 => Some(ty),
                _ => None,
            }
        }

        #[inline]
        fn ty_int_ref_16_to_64(&mut self, ty: Type) -> Option<Type> {
            match ty {
                I16 | I32 | I64 => Some(ty),
                _ => None,
            }
        }

        #[inline]
        fn ty_int(&mut self, ty: Type) -> Option<Type> {
            ty.is_int().then(|| ty)
        }

        #[inline]
        fn ty_scalar(&mut self, ty: Type) -> Option<Type> {
            if ty.lane_count() == 1 { Some(ty) } else { None }
        }

        #[inline]
        fn ty_scalar_float(&mut self, ty: Type) -> Option<Type> {
            if ty.is_float() { Some(ty) } else { None }
        }

        #[inline]
        fn ty_float_or_vec(&mut self, ty: Type) -> Option<Type> {
            if ty.is_float() || ty.is_vector() {
                Some(ty)
            } else {
                None
            }
        }

        fn ty_vector_float(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.lane_type().is_float() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vector_not_float(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && !ty.lane_type().is_float() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vec64_ctor(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.bits() == 64 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vec64(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.bits() == 64 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vec128(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.bits() == 128 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_dyn_vec64(&mut self, ty: Type) -> Option<Type> {
            if ty.is_dynamic_vector() && dynamic_to_fixed(ty).bits() == 64 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_dyn_vec128(&mut self, ty: Type) -> Option<Type> {
            if ty.is_dynamic_vector() && dynamic_to_fixed(ty).bits() == 128 {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vec64_int(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.bits() == 64 && ty.lane_type().is_int() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_vec128_int(&mut self, ty: Type) -> Option<Type> {
            if ty.is_vector() && ty.bits() == 128 && ty.lane_type().is_int() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_addr64(&mut self, ty: Type) -> Option<Type> {
            match ty {
                I64 => Some(ty),
                _ => None,
            }
        }

        #[inline]
        fn u64_from_imm64(&mut self, imm: Imm64) -> u64 {
            imm.bits() as u64
        }

        #[inline]
        fn imm64_power_of_two(&mut self, x: Imm64) -> Option<u64> {
            let x = i64::from(x);
            let x = u64::try_from(x).ok()?;
            if x.is_power_of_two() {
                Some(x.trailing_zeros().into())
            } else {
                None
            }
        }

        #[inline]
        fn u64_from_bool(&mut self, b: bool) -> u64 {
            if b { u64::MAX } else { 0 }
        }

        #[inline]
        fn multi_lane(&mut self, ty: Type) -> Option<(u32, u32)> {
            if ty.lane_count() > 1 {
                Some((ty.lane_bits(), ty.lane_count()))
            } else {
                None
            }
        }

        #[inline]
        fn dynamic_lane(&mut self, ty: Type) -> Option<(u32, u32)> {
            if ty.is_dynamic_vector() {
                Some((ty.lane_bits(), ty.min_lane_count()))
            } else {
                None
            }
        }

        #[inline]
        fn ty_dyn64_int(&mut self, ty: Type) -> Option<Type> {
            if ty.is_dynamic_vector() && ty.min_bits() == 64 && ty.lane_type().is_int() {
                Some(ty)
            } else {
                None
            }
        }

        #[inline]
        fn ty_dyn128_int(&mut self, ty: Type) -> Option<Type> {
            if ty.is_dynamic_vector() && ty.min_bits() == 128 && ty.lane_type().is_int() {
                Some(ty)
            } else {
                None
            }
        }

        fn u16_from_ieee16(&mut self, val: Ieee16) -> u16 {
            val.bits()
        }

        fn u32_from_ieee32(&mut self, val: Ieee32) -> u32 {
            val.bits()
        }

        fn u64_from_ieee64(&mut self, val: Ieee64) -> u64 {
            val.bits()
        }

        fn u8_from_uimm8(&mut self, val: Uimm8) -> u8 {
            val
        }

        fn not_vec32x2(&mut self, ty: Type) -> Option<Type> {
            if ty.lane_bits() == 32 && ty.lane_count() == 2 {
                None
            } else {
                Some(ty)
            }
        }

        fn not_i64x2(&mut self, ty: Type) -> Option<()> {
            if ty == I64X2 { None } else { Some(()) }
        }

        fn trap_code_division_by_zero(&mut self) -> TrapCode {
            TrapCode::INTEGER_DIVISION_BY_ZERO
        }

        fn trap_code_integer_overflow(&mut self) -> TrapCode {
            TrapCode::INTEGER_OVERFLOW
        }

        fn trap_code_bad_conversion_to_integer(&mut self) -> TrapCode {
            TrapCode::BAD_CONVERSION_TO_INTEGER
        }

        fn nonzero_u64_from_imm64(&mut self, val: Imm64) -> Option<u64> {
            match val.bits() {
                0 => None,
                n => Some(n as u64),
            }
        }

        #[inline]
        fn u32_nonnegative(&mut self, x: u32) -> Option<u32> {
            if (x as i32) >= 0 { Some(x) } else { None }
        }

        #[inline]
        fn imm64(&mut self, x: u64) -> Imm64 {
            Imm64::new(x as i64)
        }

        #[inline]
        fn imm64_masked(&mut self, ty: Type, x: u64) -> Imm64 {
            Imm64::new((x & self.ty_mask(ty)) as i64)
        }

        #[inline]
        fn offset32(&mut self, x: Offset32) -> i32 {
            x.into()
        }

        #[inline]
        fn lane_type(&mut self, ty: Type) -> Type {
            ty.lane_type()
        }

        #[inline]
        fn ty_half_lanes(&mut self, ty: Type) -> Option<Type> {
            if ty.lane_count() == 1 {
                None
            } else {
                ty.lane_type().by(ty.lane_count() / 2)
            }
        }

        #[inline]
        fn ty_half_width(&mut self, ty: Type) -> Option<Type> {
            ty.half_width()
        }

        #[inline]
        fn ty_equal(&mut self, lhs: Type, rhs: Type) -> bool {
            lhs == rhs
        }

        #[inline]
        fn offset32_to_i32(&mut self, offset: Offset32) -> i32 {
            offset.into()
        }

        #[inline]
        fn i32_to_offset32(&mut self, offset: i32) -> Offset32 {
            Offset32::new(offset)
        }

        #[inline]
        fn mem_flags_trusted(&mut self) -> MemFlags {
            MemFlags::trusted()
        }

        #[inline]
        fn little_or_native_endian(&mut self, flags: MemFlags) -> Option<MemFlags> {
            match flags.explicit_endianness() {
                Some(crate::ir::Endianness::Little) | None => Some(flags),
                Some(crate::ir::Endianness::Big) => None,
            }
        }

        #[inline]
        fn intcc_unsigned(&mut self, x: &IntCC) -> IntCC {
            x.unsigned()
        }

        #[inline]
        fn signed_cond_code(&mut self, cc: &IntCC) -> Option<IntCC> {
            match cc {
                IntCC::Equal
                | IntCC::UnsignedGreaterThanOrEqual
                | IntCC::UnsignedGreaterThan
                | IntCC::UnsignedLessThanOrEqual
                | IntCC::UnsignedLessThan
                | IntCC::NotEqual => None,
                IntCC::SignedGreaterThanOrEqual
                | IntCC::SignedGreaterThan
                | IntCC::SignedLessThanOrEqual
                | IntCC::SignedLessThan => Some(*cc),
            }
        }

        #[inline]
        fn intcc_swap_args(&mut self, cc: &IntCC) -> IntCC {
            cc.swap_args()
        }

        #[inline]
        fn intcc_complement(&mut self, cc: &IntCC) -> IntCC {
            cc.complement()
        }

        #[inline]
        fn intcc_without_eq(&mut self, x: &IntCC) -> IntCC {
            x.without_equal()
        }

        #[inline]
        fn floatcc_swap_args(&mut self, cc: &FloatCC) -> FloatCC {
            cc.swap_args()
        }

        #[inline]
        fn floatcc_complement(&mut self, cc: &FloatCC) -> FloatCC {
            cc.complement()
        }

        fn floatcc_unordered(&mut self, cc: &FloatCC) -> bool {
            match *cc {
                FloatCC::Unordered
                | FloatCC::UnorderedOrEqual
                | FloatCC::UnorderedOrLessThan
                | FloatCC::UnorderedOrLessThanOrEqual
                | FloatCC::UnorderedOrGreaterThan
                | FloatCC::UnorderedOrGreaterThanOrEqual => true,
                _ => false,
            }
        }

        #[inline]
        fn unpack_value_array_2(&mut self, arr: &ValueArray2) -> (Value, Value) {
            let [a, b] = *arr;
            (a, b)
        }

        #[inline]
        fn pack_value_array_2(&mut self, a: Value, b: Value) -> ValueArray2 {
            [a, b]
        }

        #[inline]
        fn unpack_value_array_3(&mut self, arr: &ValueArray3) -> (Value, Value, Value) {
            let [a, b, c] = *arr;
            (a, b, c)
        }

        #[inline]
        fn pack_value_array_3(&mut self, a: Value, b: Value, c: Value) -> ValueArray3 {
            [a, b, c]
        }

        #[inline]
        fn unpack_block_array_2(&mut self, arr: &BlockArray2) -> (BlockCall, BlockCall) {
            let [a, b] = *arr;
            (a, b)
        }

        #[inline]
        fn pack_block_array_2(&mut self, a: BlockCall, b: BlockCall) -> BlockArray2 {
            [a, b]
        }

        fn u128_replicated_u64(&mut self, val: u128) -> Option<u64> {
            let low64 = val as u64 as u128;
            if (low64 | (low64 << 64)) == val {
                Some(low64 as u64)
            } else {
                None
            }
        }

        fn u64_replicated_u32(&mut self, val: u64) -> Option<u64> {
            let low32 = val as u32 as u64;
            if (low32 | (low32 << 32)) == val {
                Some(low32)
            } else {
                None
            }
        }

        fn u32_replicated_u16(&mut self, val: u64) -> Option<u64> {
            let val = val as u32;
            let low16 = val as u16 as u32;
            if (low16 | (low16 << 16)) == val {
                Some(low16.into())
            } else {
                None
            }
        }

        fn u16_replicated_u8(&mut self, val: u64) -> Option<u8> {
            let val = val as u16;
            let low8 = val as u8 as u16;
            if (low8 | (low8 << 8)) == val {
                Some(low8 as u8)
            } else {
                None
            }
        }

        fn u128_low_bits(&mut self, val: u128) -> u64 {
            val as u64
        }

        fn u128_high_bits(&mut self, val: u128) -> u64 {
            (val >> 64) as u64
        }

        fn f16_min(&mut self, a: Ieee16, b: Ieee16) -> Option<Ieee16> {
            a.minimum(b).non_nan()
        }

        fn f16_max(&mut self, a: Ieee16, b: Ieee16) -> Option<Ieee16> {
            a.maximum(b).non_nan()
        }

        fn f16_neg(&mut self, n: Ieee16) -> Ieee16 {
            -n
        }

        fn f16_abs(&mut self, n: Ieee16) -> Ieee16 {
            n.abs()
        }

        fn f16_copysign(&mut self, a: Ieee16, b: Ieee16) -> Ieee16 {
            a.copysign(b)
        }

        fn f32_add(&mut self, lhs: Ieee32, rhs: Ieee32) -> Option<Ieee32> {
            (lhs + rhs).non_nan()
        }

        fn f32_sub(&mut self, lhs: Ieee32, rhs: Ieee32) -> Option<Ieee32> {
            (lhs - rhs).non_nan()
        }

        fn f32_mul(&mut self, lhs: Ieee32, rhs: Ieee32) -> Option<Ieee32> {
            (lhs * rhs).non_nan()
        }

        fn f32_div(&mut self, lhs: Ieee32, rhs: Ieee32) -> Option<Ieee32> {
            (lhs / rhs).non_nan()
        }

        fn f32_sqrt(&mut self, n: Ieee32) -> Option<Ieee32> {
            n.sqrt().non_nan()
        }

        fn f32_ceil(&mut self, n: Ieee32) -> Option<Ieee32> {
            n.ceil().non_nan()
        }

        fn f32_floor(&mut self, n: Ieee32) -> Option<Ieee32> {
            n.floor().non_nan()
        }

        fn f32_trunc(&mut self, n: Ieee32) -> Option<Ieee32> {
            n.trunc().non_nan()
        }

        fn f32_nearest(&mut self, n: Ieee32) -> Option<Ieee32> {
            n.round_ties_even().non_nan()
        }

        fn f32_min(&mut self, a: Ieee32, b: Ieee32) -> Option<Ieee32> {
            a.minimum(b).non_nan()
        }

        fn f32_max(&mut self, a: Ieee32, b: Ieee32) -> Option<Ieee32> {
            a.maximum(b).non_nan()
        }

        fn f32_neg(&mut self, n: Ieee32) -> Ieee32 {
            -n
        }

        fn f32_abs(&mut self, n: Ieee32) -> Ieee32 {
            n.abs()
        }

        fn f32_copysign(&mut self, a: Ieee32, b: Ieee32) -> Ieee32 {
            a.copysign(b)
        }

        fn f64_add(&mut self, lhs: Ieee64, rhs: Ieee64) -> Option<Ieee64> {
            (lhs + rhs).non_nan()
        }

        fn f64_sub(&mut self, lhs: Ieee64, rhs: Ieee64) -> Option<Ieee64> {
            (lhs - rhs).non_nan()
        }

        fn f64_mul(&mut self, lhs: Ieee64, rhs: Ieee64) -> Option<Ieee64> {
            (lhs * rhs).non_nan()
        }

        fn f64_div(&mut self, lhs: Ieee64, rhs: Ieee64) -> Option<Ieee64> {
            (lhs / rhs).non_nan()
        }

        fn f64_sqrt(&mut self, n: Ieee64) -> Option<Ieee64> {
            n.sqrt().non_nan()
        }

        fn f64_ceil(&mut self, n: Ieee64) -> Option<Ieee64> {
            n.ceil().non_nan()
        }

        fn f64_floor(&mut self, n: Ieee64) -> Option<Ieee64> {
            n.floor().non_nan()
        }

        fn f64_trunc(&mut self, n: Ieee64) -> Option<Ieee64> {
            n.trunc().non_nan()
        }

        fn f64_nearest(&mut self, n: Ieee64) -> Option<Ieee64> {
            n.round_ties_even().non_nan()
        }

        fn f64_min(&mut self, a: Ieee64, b: Ieee64) -> Option<Ieee64> {
            a.minimum(b).non_nan()
        }

        fn f64_max(&mut self, a: Ieee64, b: Ieee64) -> Option<Ieee64> {
            a.maximum(b).non_nan()
        }

        fn f64_neg(&mut self, n: Ieee64) -> Ieee64 {
            -n
        }

        fn f64_abs(&mut self, n: Ieee64) -> Ieee64 {
            n.abs()
        }

        fn f64_copysign(&mut self, a: Ieee64, b: Ieee64) -> Ieee64 {
            a.copysign(b)
        }

        fn f128_min(&mut self, a: Ieee128, b: Ieee128) -> Option<Ieee128> {
            a.minimum(b).non_nan()
        }

        fn f128_max(&mut self, a: Ieee128, b: Ieee128) -> Option<Ieee128> {
            a.maximum(b).non_nan()
        }

        fn f128_neg(&mut self, n: Ieee128) -> Ieee128 {
            -n
        }

        fn f128_abs(&mut self, n: Ieee128) -> Ieee128 {
            n.abs()
        }

        fn f128_copysign(&mut self, a: Ieee128, b: Ieee128) -> Ieee128 {
            a.copysign(b)
        }

        #[inline]
        fn def_inst(&mut self, val: Value) -> Option<Inst> {
            self.dfg().value_def(val).inst()
        }
    };
}
