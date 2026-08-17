// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::TxSize;
use super::TxType;

use super::HTX_TAB;
use super::VTX_TAB;

pub type TxfmShift = [i8; 3];
pub type TxfmShifts = [TxfmShift; 3];

// Shift so that the first shift is 4 - (bd - 8) to align with the initial
// design of daala_tx
// 8 bit 4x4 is an exception and only shifts by 3 in the first stage
const FWD_SHIFT_4X4: TxfmShifts = [[3, 0, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_8X8: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_16X16: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_32X32: TxfmShifts = [[4, -2, 0], [2, 0, 0], [0, 0, 2]];
const FWD_SHIFT_64X64: TxfmShifts = [[4, -1, -2], [2, 0, -1], [0, 0, 1]];
const FWD_SHIFT_4X8: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_8X4: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_8X16: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_16X8: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_16X32: TxfmShifts = [[4, -2, 0], [2, 0, 0], [0, 0, 2]];
const FWD_SHIFT_32X16: TxfmShifts = [[4, -2, 0], [2, 0, 0], [0, 0, 2]];
const FWD_SHIFT_32X64: TxfmShifts = [[4, -1, -2], [2, 0, -1], [0, 0, 1]];
const FWD_SHIFT_64X32: TxfmShifts = [[4, -1, -2], [2, 0, -1], [0, 0, 1]];
const FWD_SHIFT_4X16: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_16X4: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_8X32: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_32X8: TxfmShifts = [[4, -1, 0], [2, 0, 1], [0, 0, 3]];
const FWD_SHIFT_16X64: TxfmShifts = [[4, -2, 0], [2, 0, 0], [0, 0, 2]];
const FWD_SHIFT_64X16: TxfmShifts = [[4, -2, 0], [2, 0, 0], [0, 0, 2]];

const FWD_SHIFT_4X4_WHT: TxfmShift = [0, 0, 2];

pub const FWD_TXFM_SHIFT_LS: [TxfmShifts; TxSize::TX_SIZES_ALL] = [
  FWD_SHIFT_4X4,
  FWD_SHIFT_8X8,
  FWD_SHIFT_16X16,
  FWD_SHIFT_32X32,
  FWD_SHIFT_64X64,
  FWD_SHIFT_4X8,
  FWD_SHIFT_8X4,
  FWD_SHIFT_8X16,
  FWD_SHIFT_16X8,
  FWD_SHIFT_16X32,
  FWD_SHIFT_32X16,
  FWD_SHIFT_32X64,
  FWD_SHIFT_64X32,
  FWD_SHIFT_4X16,
  FWD_SHIFT_16X4,
  FWD_SHIFT_8X32,
  FWD_SHIFT_32X8,
  FWD_SHIFT_16X64,
  FWD_SHIFT_64X16,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxfmType {
  DCT4,
  DCT8,
  DCT16,
  DCT32,
  DCT64,
  ADST4,
  ADST8,
  ADST16,
  Identity4,
  Identity8,
  Identity16,
  Identity32,
  WHT4,
}

impl TxfmType {
  const TX_TYPES_1D: usize = 5;
  const AV1_TXFM_TYPE_LS: [[Option<TxfmType>; Self::TX_TYPES_1D]; 5] = [
    [
      Some(TxfmType::DCT4),
      Some(TxfmType::ADST4),
      Some(TxfmType::ADST4),
      Some(TxfmType::Identity4),
      Some(TxfmType::WHT4),
    ],
    [
      Some(TxfmType::DCT8),
      Some(TxfmType::ADST8),
      Some(TxfmType::ADST8),
      Some(TxfmType::Identity8),
      None,
    ],
    [
      Some(TxfmType::DCT16),
      Some(TxfmType::ADST16),
      Some(TxfmType::ADST16),
      Some(TxfmType::Identity16),
      None,
    ],
    [Some(TxfmType::DCT32), None, None, Some(TxfmType::Identity32), None],
    [Some(TxfmType::DCT64), None, None, None, None],
  ];
}

#[derive(Debug, Clone, Copy)]
pub struct Txfm2DFlipCfg {
  pub tx_size: TxSize,
  /// Flip upside down
  pub ud_flip: bool,
  /// Flip left to right
  pub lr_flip: bool,
  pub shift: TxfmShift,
  pub txfm_type_col: TxfmType,
  pub txfm_type_row: TxfmType,
}

impl Txfm2DFlipCfg {
  /// # Panics
  ///
  /// - If called with an invalid combination of `tx_size` and `tx_type`
  pub fn fwd(tx_type: TxType, tx_size: TxSize, bd: usize) -> Self {
    let tx_type_1d_col = VTX_TAB[tx_type as usize];
    let tx_type_1d_row = HTX_TAB[tx_type as usize];
    let txw_idx = tx_size.width_index();
    let txh_idx = tx_size.height_index();
    let txfm_type_col =
      TxfmType::AV1_TXFM_TYPE_LS[txh_idx][tx_type_1d_col as usize].unwrap();
    let txfm_type_row =
      TxfmType::AV1_TXFM_TYPE_LS[txw_idx][tx_type_1d_row as usize].unwrap();
    let (ud_flip, lr_flip) = Self::get_flip_cfg(tx_type);
    let shift = if tx_type == TxType::WHT_WHT {
      FWD_SHIFT_4X4_WHT
    } else {
      FWD_TXFM_SHIFT_LS[tx_size as usize][(bd - 8) / 2]
    };

    Txfm2DFlipCfg {
      tx_size,
      ud_flip,
      lr_flip,
      shift,
      txfm_type_col,
      txfm_type_row,
    }
  }

  /// Determine the flip config, returning `(ud_flip, lr_flip)`
  const fn get_flip_cfg(tx_type: TxType) -> (bool, bool) {
    use self::TxType::*;
    match tx_type {
      DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST | IDTX | V_DCT | H_DCT
      | V_ADST | H_ADST | WHT_WHT => (false, false),
      FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => (true, false),
      DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => (false, true),
      FLIPADST_FLIPADST => (true, true),
    }
  }
}

macro_rules! store_coeffs {
  ( $arr:expr, $( $x:expr ),* ) => {
      {
      let mut i: i32 = -1;
      $(
        i += 1;
        $arr[i as usize] = $x;
      )*
    }
  };
}

macro_rules! impl_1d_tx {
() => {
  impl_1d_tx! {allow(unused_attributes), }
};

($m:meta, $($s:ident),*) => {
  pub trait TxOperations: Copy {
    $($s)* fn zero() -> Self;

    $($s)* fn tx_mul<const SHIFT: i32>(self, mul: i32) -> Self;
    $($s)* fn rshift1(self) -> Self;
    $($s)* fn add(self, b: Self) -> Self;
    $($s)* fn sub(self, b: Self) -> Self;
    $($s)* fn add_avg(self, b: Self) -> Self;
    $($s)* fn sub_avg(self, b: Self) -> Self;

    $($s)* fn copy_fn(self) -> Self {
      self
    }
  }

  #[inline]
  fn get_func(t: TxfmType) -> TxfmFunc {
    use self::TxfmType::*;
    match t {
      DCT4 => daala_fdct4,
      DCT8 => daala_fdct8,
      DCT16 => daala_fdct16,
      DCT32 => daala_fdct32,
      DCT64 => daala_fdct64,
      ADST4 => daala_fdst_vii_4,
      ADST8 => daala_fdst8,
      ADST16 => daala_fdst16,
      Identity4 => fidentity,
      Identity8 => fidentity,
      Identity16 => fidentity,
      Identity32 => fidentity,
      WHT4 => fwht4,
    }
  }

  trait RotateKernelPi4<T: TxOperations> {
  const ADD: $($s)* fn(T, T) -> T;
  const SUB: $($s)* fn(T, T) -> T;

  #[$m]
  $($s)* fn kernel<const SHIFT0: i32, const SHIFT1: i32>(p0: T, p1: T, m: (i32, i32)) -> (T, T) {
    let t = Self::ADD(p1, p0);
    let (a, out0) = (p0.tx_mul::<SHIFT0>(m.0), t.tx_mul::<SHIFT1>(m.1));
    let out1 = Self::SUB(a, out0);
    (out0, out1)
  }
}

struct RotatePi4Add;
struct RotatePi4AddAvg;
struct RotatePi4Sub;
struct RotatePi4SubAvg;

impl<T: TxOperations> RotateKernelPi4<T> for RotatePi4Add {
  const ADD: $($s)* fn(T, T) -> T = T::add;
  const SUB: $($s)* fn(T, T) -> T = T::sub;
}

impl<T: TxOperations> RotateKernelPi4<T> for RotatePi4AddAvg {
  const ADD: $($s)* fn(T, T) -> T = T::add_avg;
  const SUB: $($s)* fn(T, T) -> T = T::sub;
}

impl<T: TxOperations> RotateKernelPi4<T> for RotatePi4Sub {
  const ADD: $($s)* fn(T, T) -> T = T::sub;
  const SUB: $($s)* fn(T, T) -> T = T::add;
}

impl<T: TxOperations> RotateKernelPi4<T> for RotatePi4SubAvg {
  const ADD: $($s)* fn(T, T) -> T = T::sub_avg;
  const SUB: $($s)* fn(T, T) -> T = T::add;
}

trait RotateKernel<T: TxOperations> {
  const ADD: $($s)* fn(T, T) -> T;
  const SUB: $($s)* fn(T, T) -> T;
  const SHIFT: $($s)* fn(T) -> T;

  #[$m]
  $($s)* fn half_kernel<const SHIFT0: i32, const SHIFT1: i32, const SHIFT2: i32>(
    p0: (T, T), p1: T, m: (i32, i32, i32),
  ) -> (T, T) {
    let t = Self::ADD(p1, p0.0);
    let (a, b, c) = (p0.1.tx_mul::<SHIFT0>(m.0), p1.tx_mul::<SHIFT1>(m.1), t.tx_mul::<SHIFT2>(m.2));
    let out0 = b.add(c);
    let shifted = Self::SHIFT(c);
    let out1 = Self::SUB(a, shifted);
    (out0, out1)
  }

  #[$m]
  $($s)* fn kernel<const SHIFT0: i32, const SHIFT1: i32, const SHIFT2: i32>(p0: T, p1: T, m: (i32, i32, i32)) -> (T, T) {
    Self::half_kernel::<SHIFT0, SHIFT1, SHIFT2>((p0, p0), p1, m)
  }
}

trait RotateKernelNeg<T: TxOperations> {
  const ADD: $($s)* fn(T, T) -> T;

  #[$m]
  $($s)* fn kernel<const SHIFT0: i32, const SHIFT1: i32, const SHIFT2: i32>(p0: T, p1: T, m: (i32, i32, i32)) -> (T, T) {
    let t = Self::ADD(p0, p1);
    let (a, b, c) = (p0.tx_mul::<SHIFT0>(m.0), p1.tx_mul::<SHIFT1>(m.1), t.tx_mul::<SHIFT2>(m.2));
    let out0 = b.sub(c);
    let out1 = c.sub(a);
    (out0, out1)
  }
}

struct RotateAdd;
struct RotateAddAvg;
struct RotateAddShift;
struct RotateSub;
struct RotateSubAvg;
struct RotateSubShift;
struct RotateNeg;
struct RotateNegAvg;

impl<T: TxOperations> RotateKernel<T> for RotateAdd {
  const ADD: $($s)* fn(T, T) -> T = T::add;
  const SUB: $($s)* fn(T, T) -> T = T::sub;
  const SHIFT: $($s)* fn(T) -> T = T::copy_fn;
}

impl<T: TxOperations> RotateKernel<T> for RotateAddAvg {
  const ADD: $($s)* fn(T, T) -> T = T::add_avg;
  const SUB: $($s)* fn(T, T) -> T = T::sub;
  const SHIFT: $($s)* fn(T) -> T = T::copy_fn;
}

impl<T: TxOperations> RotateKernel<T> for RotateAddShift {
  const ADD: $($s)* fn(T, T) -> T = T::add;
  const SUB: $($s)* fn(T, T) -> T = T::sub;
  const SHIFT: $($s)* fn(T) -> T = T::rshift1;
}

impl<T: TxOperations> RotateKernel<T> for RotateSub {
  const ADD: $($s)* fn(T, T) -> T = T::sub;
  const SUB: $($s)* fn(T, T) -> T = T::add;
  const SHIFT: $($s)* fn(T) -> T = T::copy_fn;
}

impl<T: TxOperations> RotateKernel<T> for RotateSubAvg {
  const ADD: $($s)* fn(T, T) -> T = T::sub_avg;
  const SUB: $($s)* fn(T, T) -> T = T::add;
  const SHIFT: $($s)* fn(T) -> T = T::copy_fn;
}

impl<T: TxOperations> RotateKernel<T> for RotateSubShift {
  const ADD: $($s)* fn(T, T) -> T = T::sub;
  const SUB: $($s)* fn(T, T) -> T = T::add;
  const SHIFT: $($s)* fn(T) -> T = T::rshift1;
}

impl<T: TxOperations> RotateKernelNeg<T> for RotateNeg {
  const ADD: $($s)* fn(T, T) -> T = T::sub;
}

impl<T: TxOperations> RotateKernelNeg<T> for RotateNegAvg {
  const ADD: $($s)* fn(T, T) -> T = T::sub_avg;
}

#[inline]
#[$m]
$($s)* fn butterfly_add<T: TxOperations>(p0: T, p1: T) -> ((T, T), T) {
  let p0 = p0.add(p1);
  let p0h = p0.rshift1();
  let p1h = p1.sub(p0h);
  ((p0h, p0), p1h)
}

#[inline]
#[$m]
$($s)* fn butterfly_sub<T: TxOperations>(p0: T, p1: T) -> ((T, T), T) {
  let p0 = p0.sub(p1);
  let p0h = p0.rshift1();
  let p1h = p1.add(p0h);
  ((p0h, p0), p1h)
}

#[inline]
#[$m]
$($s)* fn butterfly_neg<T: TxOperations>(p0: T, p1: T) -> (T, (T, T)) {
  let p1 = p0.sub(p1);
  let p1h = p1.rshift1();
  let p0h = p0.sub(p1h);
  (p0h, (p1h, p1))
}

#[inline]
#[$m]
$($s)* fn butterfly_add_asym<T: TxOperations>(p0: (T, T), p1h: T) -> (T, T) {
  let p1 = p1h.add(p0.0);
  let p0 = p0.1.sub(p1);
  (p0, p1)
}

#[inline]
#[$m]
$($s)* fn butterfly_sub_asym<T: TxOperations>(p0: (T, T), p1h: T) -> (T, T) {
  let p1 = p1h.sub(p0.0);
  let p0 = p0.1.add(p1);
  (p0, p1)
}

#[inline]
#[$m]
$($s)* fn butterfly_neg_asym<T: TxOperations>(p0h: T, p1: (T, T)) -> (T, T) {
  let p0 = p0h.add(p1.0);
  let p1 = p0.sub(p1.1);
  (p0, p1)
}

#[$m]
$($s)* fn daala_fdct_ii_2_asym<T: TxOperations>(p0h: T, p1: (T, T)) -> (T, T) {
  butterfly_neg_asym(p0h, p1)
}

#[$m]
$($s)* fn daala_fdst_iv_2_asym<T: TxOperations>(p0: (T, T), p1h: T) -> (T, T) {
  //   473/512 = (Sin[3*Pi/8] + Cos[3*Pi/8])/Sqrt[2] = 0.9238795325112867
  // 3135/4096 = (Sin[3*Pi/8] - Cos[3*Pi/8])*Sqrt[2] = 0.7653668647301795
  // 4433/8192 = Cos[3*Pi/8]*Sqrt[2]                 = 0.5411961001461971
  RotateAdd::half_kernel::<9, 12, 13>(p0, p1h, (473, 3135, 4433))
}

#[$m]
$($s)* fn daala_fdct_ii_4<T: TxOperations>(
  q0: T, q1: T, q2: T, q3: T, output: &mut [T],
) {
  // +/- Butterflies with asymmetric output.
  let (q0h, q3) = butterfly_neg(q0, q3);
  let (q1, q2h) = butterfly_add(q1, q2);

  // Embedded 2-point transforms with asymmetric input.
  let (q0, q1) = daala_fdct_ii_2_asym(q0h, q1);
  let (q3, q2) = daala_fdst_iv_2_asym(q3, q2h);

  store_coeffs!(output, q0, q1, q2, q3);
}

#[$m]
$($s)* fn daala_fdct4<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 4);
  let mut temp_out: [T; 4] = [T::zero(); 4];
  daala_fdct_ii_4(coeffs[0], coeffs[1], coeffs[2], coeffs[3], &mut temp_out);

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[2];
  coeffs[2] = temp_out[1];
  coeffs[3] = temp_out[3];
}

#[$m]
$($s)* fn daala_fdst_vii_4<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 4);

  let q0 = coeffs[0];
  let q1 = coeffs[1];
  let q2 = coeffs[2];
  let q3 = coeffs[3];
  let t0 = q1.add(q3);
  // t1 = (q0 + q1 - q3)/2
  let t1 = q1.add(q0.sub_avg(t0));
  let t2 = q0.sub(q1);
  let t3 = q2;
  let t4 = q0.add(q3);
  // 7021/16384 ~= 2*Sin[2*Pi/9]/3 ~= 0.428525073124360
  let t0 = t0.tx_mul::<14>(7021);
  // 37837/32768 ~= 4*Sin[3*Pi/9]/3 ~= 1.154700538379252
  let t1 = t1.tx_mul::<15>(37837);
  // 21513/32768 ~= 2*Sin[4*Pi/9]/3 ~= 0.656538502008139
  let t2 = t2.tx_mul::<15>(21513);
  // 37837/32768 ~= 4*Sin[3*Pi/9]/3 ~= 1.154700538379252
  let t3 = t3.tx_mul::<15>(37837);
  // 467/2048 ~= 2*Sin[1*Pi/9]/3 ~= 0.228013428883779
  let t4 = t4.tx_mul::<11>(467);
  let t3h = t3.rshift1();
  let u4 = t4.add(t3h);
  coeffs[0] = t0.add(u4);
  coeffs[1] = t1;
  coeffs[2] = t0.add(t2.sub(t3h));
  coeffs[3] = t2.add(t3.sub(u4));
}

#[$m]
$($s)* fn daala_fdct_ii_2<T: TxOperations>(p0: T, p1: T) -> (T, T) {
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4]  = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]            = 1.4142135623730951
  let (p1, p0) = RotatePi4SubAvg::kernel::<13, 13>(p1, p0, (11585, 11585));
  (p0, p1)
}

#[$m]
$($s)* fn daala_fdst_iv_2<T: TxOperations>(p0: T, p1: T) -> (T, T) {
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8]  = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8]  = 0.5411961001461971
  //  3135/4096 = 2*Cos[3*Pi/8]              = 0.7653668647301796
  RotateAddAvg::kernel::<13, 14, 12>(p0, p1, (10703, 8867, 3135))
}

#[$m]
$($s)* fn daala_fdct_ii_4_asym<T: TxOperations>(
  q0h: T, q1: (T, T), q2h: T, q3: (T, T), output: &mut [T],
) {
  // +/- Butterflies with asymmetric input.
  let (q0, q3) = butterfly_neg_asym(q0h, q3);
  let (q1, q2) = butterfly_sub_asym(q1, q2h);

  // Embedded 2-point orthonormal transforms.
  let (q0, q1) = daala_fdct_ii_2(q0, q1);
  let (q3, q2) = daala_fdst_iv_2(q3, q2);

  store_coeffs!(output, q0, q1, q2, q3);
}

#[$m]
$($s)* fn daala_fdst_iv_4_asym<T: TxOperations>(
  q0: (T, T), q1h: T, q2: (T, T), q3h: T, output: &mut [T],
) {
  // Stage 0
  //  9633/16384 = (Sin[7*Pi/16] + Cos[7*Pi/16])/2 = 0.5879378012096793
  //  12873/8192 = (Sin[7*Pi/16] - Cos[7*Pi/16])*2 = 1.5713899167742045
  // 12785/32768 = Cos[7*Pi/16]*2                  = 0.3901806440322565
  let (q0, q3) = RotateAddShift::half_kernel::<14, 13, 15>(
    q0,
    q3h,
    (9633, 12873, 12785),
  );
  // 11363/16384 = (Sin[5*Pi/16] + Cos[5*Pi/16])/2 = 0.6935199226610738
  // 18081/32768 = (Sin[5*Pi/16] - Cos[5*Pi/16])*2 = 0.5517987585658861
  //  4551/4096 = Cos[5*Pi/16]*2                  = 1.1111404660392044
  let (q2, q1) = RotateSubShift::half_kernel::<14, 15, 12>(
    q2,
    q1h,
    (11363, 18081, 4551),
  );

  // Stage 1
  let (q2, q3) = butterfly_sub_asym((q2.rshift1(), q2), q3);
  let (q0, q1) = butterfly_sub_asym((q0.rshift1(), q0), q1);

  // Stage 2
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (q2, q1) = RotatePi4AddAvg::kernel::<13, 13>(q2, q1, (11585, 11585));

  store_coeffs!(output, q0, q1, q2, q3);
}

#[$m]
$($s)* fn daala_fdct_ii_8<T: TxOperations>(
  r0: T, r1: T, r2: T, r3: T, r4: T, r5: T, r6: T, r7: T, output: &mut [T],
) {
  // +/- Butterflies with asymmetric output.
  let (r0h, r7) = butterfly_neg(r0, r7);
  let (r1, r6h) = butterfly_add(r1, r6);
  let (r2h, r5) = butterfly_neg(r2, r5);
  let (r3, r4h) = butterfly_add(r3, r4);

  // Embedded 4-point transforms with asymmetric input.
  daala_fdct_ii_4_asym(r0h, r1, r2h, r3, &mut output[0..4]);
  daala_fdst_iv_4_asym(r7, r6h, r5, r4h, &mut output[4..8]);
  output[4..8].reverse();
}

#[$m]
$($s)* fn daala_fdct8<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 8);
  let mut temp_out: [T; 8] = [T::zero(); 8];
  daala_fdct_ii_8(
    coeffs[0],
    coeffs[1],
    coeffs[2],
    coeffs[3],
    coeffs[4],
    coeffs[5],
    coeffs[6],
    coeffs[7],
    &mut temp_out,
  );

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[4];
  coeffs[2] = temp_out[2];
  coeffs[3] = temp_out[6];
  coeffs[4] = temp_out[1];
  coeffs[5] = temp_out[5];
  coeffs[6] = temp_out[3];
  coeffs[7] = temp_out[7];
}

#[$m]
$($s)* fn daala_fdst_iv_8<T: TxOperations>(
  r0: T, r1: T, r2: T, r3: T, r4: T, r5: T, r6: T, r7: T, output: &mut [T],
) {
  // Stage 0
  // 17911/16384 = Sin[15*Pi/32] + Cos[15*Pi/32] = 1.0932018670017576
  // 14699/16384 = Sin[15*Pi/32] - Cos[15*Pi/32] = 0.8971675863426363
  //    803/8192 = Cos[15*Pi/32]                 = 0.0980171403295606
  let (r0, r7) =
    RotateAdd::kernel::<14, 14, 13>(r0, r7, (17911, 14699, 803));
  // 20435/16384 = Sin[13*Pi/32] + Cos[13*Pi/32] = 1.24722501298667123
  // 21845/32768 = Sin[13*Pi/32] - Cos[13*Pi/32] = 0.66665565847774650
  //   1189/4096 = Cos[13*Pi/32]                 = 0.29028467725446233
  let (r6, r1) =
    RotateSub::kernel::<14, 15, 12>(r6, r1, (20435, 21845, 1189));
  // 22173/16384 = Sin[11*Pi/32] + Cos[11*Pi/32] = 1.3533180011743526
  //   3363/8192 = Sin[11*Pi/32] - Cos[11*Pi/32] = 0.4105245275223574
  // 15447/32768 = Cos[11*Pi/32]                 = 0.47139673682599764
  let (r2, r5) =
    RotateAdd::kernel::<14, 13, 15>(r2, r5, (22173, 3363, 15447));
  // 23059/16384 = Sin[9*Pi/32] + Cos[9*Pi/32] = 1.4074037375263826
  //  2271/16384 = Sin[9*Pi/32] - Cos[9*Pi/32] = 0.1386171691990915
  //   5197/8192 = Cos[9*Pi/32]                = 0.6343932841636455
  let (r4, r3) =
    RotateSub::kernel::<14, 14, 13>(r4, r3, (23059, 2271, 5197));

  // Stage 1
  let (r0, r3h) = butterfly_add(r0, r3);
  let (r2, r1h) = butterfly_sub(r2, r1);
  let (r5, r6h) = butterfly_add(r5, r6);
  let (r7, r4h) = butterfly_sub(r7, r4);

  // Stage 2
  let (r7, r6) = butterfly_add_asym(r7, r6h);
  let (r5, r3) = butterfly_add_asym(r5, r3h);
  let (r2, r4) = butterfly_add_asym(r2, r4h);
  let (r0, r1) = butterfly_sub_asym(r0, r1h);

  // Stage 3
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[3*Pi/8]             = 0.7653668647301796
  let (r3, r4) =
    RotateSubAvg::kernel::<13, 14, 12>(r3, r4, (10703, 8867, 3135));
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[3*Pi/8]             = 0.7653668647301796
  let (r2, r5) =
    RotateNegAvg::kernel::<13, 14, 12>(r2, r5, (10703, 8867, 3135));
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (r1, r6) = RotatePi4SubAvg::kernel::<13, 13>(r1, r6, (11585, 11585));

  store_coeffs!(output, r0, r1, r2, r3, r4, r5, r6, r7);
}

#[$m]
$($s)* fn daala_fdst8<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 8);
  let mut temp_out: [T; 8] = [T::zero(); 8];
  daala_fdst_iv_8(
    coeffs[0],
    coeffs[1],
    coeffs[2],
    coeffs[3],
    coeffs[4],
    coeffs[5],
    coeffs[6],
    coeffs[7],
    &mut temp_out,
  );

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[4];
  coeffs[2] = temp_out[2];
  coeffs[3] = temp_out[6];
  coeffs[4] = temp_out[1];
  coeffs[5] = temp_out[5];
  coeffs[6] = temp_out[3];
  coeffs[7] = temp_out[7];
}

#[$m]
$($s)* fn daala_fdst_iv_4<T: TxOperations>(
  q0: T, q1: T, q2: T, q3: T, output: &mut [T],
) {
  // Stage 0
  // 13623/16384 = (Sin[7*Pi/16] + Cos[7*Pi/16])/Sqrt[2] = 0.831469612302545
  //   4551/4096 = (Sin[7*Pi/16] - Cos[7*Pi/16])*Sqrt[2] = 1.111140466039204
  //  9041/32768 = Cos[7*Pi/16]*Sqrt[2]                  = 0.275899379282943
  let (q0, q3) =
    RotateAddShift::kernel::<14, 12, 11>(q0, q3, (13623, 4551, 565));
  // 16069/16384 = (Sin[5*Pi/16] + Cos[5*Pi/16])/Sqrt[2] = 0.9807852804032304
  // 12785/32768 = (Sin[5*Pi/16] - Cos[5*Pi/16])*Sqrt[2] = 0.3901806440322566
  //   1609/2048 = Cos[5*Pi/16]*Sqrt[2]                  = 0.7856949583871021
  let (q2, q1) =
    RotateSubShift::kernel::<14, 15, 11>(q2, q1, (16069, 12785, 1609));

  // Stage 1
  let (q2, q3) = butterfly_sub_asym((q2.rshift1(), q2), q3);
  let (q0, q1) = butterfly_sub_asym((q0.rshift1(), q0), q1);

  // Stage 2
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (q2, q1) = RotatePi4AddAvg::kernel::<13, 13>(q2, q1, (11585, 11585));

  store_coeffs!(output, q0, q1, q2, q3);
}


#[$m]
$($s)* fn daala_fdct_ii_8_asym<T: TxOperations>(
  r0h: T, r1: (T, T), r2h: T, r3: (T, T), r4h: T, r5: (T, T), r6h: T,
  r7: (T, T), output: &mut [T],
) {
  // +/- Butterflies with asymmetric input.
  let (r0, r7) = butterfly_neg_asym(r0h, r7);
  let (r1, r6) = butterfly_sub_asym(r1, r6h);
  let (r2, r5) = butterfly_neg_asym(r2h, r5);
  let (r3, r4) = butterfly_sub_asym(r3, r4h);

  // Embedded 4-point orthonormal transforms.
  daala_fdct_ii_4(r0, r1, r2, r3, &mut output[0..4]);
  daala_fdst_iv_4(r7, r6, r5, r4, &mut output[4..8]);
  output[4..8].reverse();
}

#[$m]
$($s)* fn daala_fdst_iv_8_asym<T: TxOperations>(
  r0: (T, T), r1h: T, r2: (T, T), r3h: T, r4: (T, T), r5h: T, r6: (T, T),
  r7h: T, output: &mut [T],
) {
  // Stage 0
  // 12665/16384 = (Sin[15*Pi/32] + Cos[15*Pi/32])/Sqrt[2] = 0.77301045336274
  //   5197/4096 = (Sin[15*Pi/32] - Cos[15*Pi/32])*Sqrt[2] = 1.26878656832729
  //  2271/16384 = Cos[15*Pi/32]*Sqrt[2]                   = 0.13861716919909
  let (r0, r7) =
    RotateAdd::half_kernel::<14, 12, 14>(r0, r7h, (12665, 5197, 2271));
  // 14449/16384 = Sin[13*Pi/32] + Cos[13*Pi/32])/Sqrt[2] = 0.881921264348355
  // 30893/32768 = Sin[13*Pi/32] - Cos[13*Pi/32])*Sqrt[2] = 0.942793473651995
  //   3363/8192 = Cos[13*Pi/32]*Sqrt[2]                  = 0.410524527522357
  let (r6, r1) =
    RotateSub::half_kernel::<14, 15, 13>(r6, r1h, (14449, 30893, 3363));
  // 15679/16384 = Sin[11*Pi/32] + Cos[11*Pi/32])/Sqrt[2] = 0.956940335732209
  //   1189/2048 = Sin[11*Pi/32] - Cos[11*Pi/32])*Sqrt[2] = 0.580569354508925
  //   5461/8192 = Cos[11*Pi/32]*Sqrt[2]                  = 0.666655658477747
  let (r2, r5) =
    RotateAdd::half_kernel::<14, 11, 13>(r2, r5h, (15679, 1189, 5461));
  // 16305/16384 = (Sin[9*Pi/32] + Cos[9*Pi/32])/Sqrt[2] = 0.9951847266721969
  //    803/4096 = (Sin[9*Pi/32] - Cos[9*Pi/32])*Sqrt[2] = 0.1960342806591213
  // 14699/16384 = Cos[9*Pi/32]*Sqrt[2]                  = 0.8971675863426364
  let (r4, r3) =
    RotateSub::half_kernel::<14, 12, 14>(r4, r3h, (16305, 803, 14699));

  // Stage 1
  let (r0, r3h) = butterfly_add(r0, r3);
  let (r2, r1h) = butterfly_sub(r2, r1);
  let (r5, r6h) = butterfly_add(r5, r6);
  let (r7, r4h) = butterfly_sub(r7, r4);

  // Stage 2
  let (r7, r6) = butterfly_add_asym(r7, r6h);
  let (r5, r3) = butterfly_add_asym(r5, r3h);
  let (r2, r4) = butterfly_add_asym(r2, r4h);
  let (r0, r1) = butterfly_sub_asym(r0, r1h);

  // Stage 3
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[3*Pi/8]             = 0.7653668647301796
  let (r3, r4) =
    RotateSubAvg::kernel::<9, 14, 12>(r3, r4, (669, 8867, 3135));
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[3*Pi/8]             = 0.7653668647301796
  let (r2, r5) =
    RotateNegAvg::kernel::<9, 14, 12>(r2, r5, (669, 8867, 3135));
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (r1, r6) = RotatePi4SubAvg::kernel::<12, 13>(r1, r6, (5793, 11585));

  store_coeffs!(output, r0, r1, r2, r3, r4, r5, r6, r7);
}

#[$m]
$($s)* fn daala_fdct_ii_16<T: TxOperations>(
  s0: T, s1: T, s2: T, s3: T, s4: T, s5: T, s6: T, s7: T, s8: T, s9: T, sa: T,
  sb: T, sc: T, sd: T, se: T, sf: T, output: &mut [T],
) {
  // +/- Butterflies with asymmetric output.
  let (s0h, sf) = butterfly_neg(s0, sf);
  let (s1, seh) = butterfly_add(s1, se);
  let (s2h, sd) = butterfly_neg(s2, sd);
  let (s3, sch) = butterfly_add(s3, sc);
  let (s4h, sb) = butterfly_neg(s4, sb);
  let (s5, sah) = butterfly_add(s5, sa);
  let (s6h, s9) = butterfly_neg(s6, s9);
  let (s7, s8h) = butterfly_add(s7, s8);

  // Embedded 8-point transforms with asymmetric input.
  daala_fdct_ii_8_asym(s0h, s1, s2h, s3, s4h, s5, s6h, s7, &mut output[0..8]);
  daala_fdst_iv_8_asym(sf, seh, sd, sch, sb, sah, s9, s8h, &mut output[8..16]);
  output[8..16].reverse();
}

#[$m]
$($s)* fn daala_fdct16<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 16);
  let mut temp_out: [T; 16] = [T::zero(); 16];
  daala_fdct_ii_16(
    coeffs[0],
    coeffs[1],
    coeffs[2],
    coeffs[3],
    coeffs[4],
    coeffs[5],
    coeffs[6],
    coeffs[7],
    coeffs[8],
    coeffs[9],
    coeffs[10],
    coeffs[11],
    coeffs[12],
    coeffs[13],
    coeffs[14],
    coeffs[15],
    &mut temp_out,
  );

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[8];
  coeffs[2] = temp_out[4];
  coeffs[3] = temp_out[12];
  coeffs[4] = temp_out[2];
  coeffs[5] = temp_out[10];
  coeffs[6] = temp_out[6];
  coeffs[7] = temp_out[14];
  coeffs[8] = temp_out[1];
  coeffs[9] = temp_out[9];
  coeffs[10] = temp_out[5];
  coeffs[11] = temp_out[13];
  coeffs[12] = temp_out[3];
  coeffs[13] = temp_out[11];
  coeffs[14] = temp_out[7];
  coeffs[15] = temp_out[15];
}

#[$m]
$($s)* fn daala_fdst_iv_16<T: TxOperations>(
  s0: T, s1: T, s2: T, s3: T, s4: T, s5: T, s6: T, s7: T, s8: T, s9: T, sa: T,
  sb: T, sc: T, sd: T, se: T, sf: T, output: &mut [T],
) {
  // Stage 0
  // 24279/32768 = (Sin[31*Pi/64] + Cos[31*Pi/64])/Sqrt[2] = 0.74095112535496
  //  11003/8192 = (Sin[31*Pi/64] - Cos[31*Pi/64])*Sqrt[2] = 1.34311790969404
  //  1137/16384 = Cos[31*Pi/64]*Sqrt[2]                   = 0.06939217050794
  let (s0, sf) =
    RotateAddShift::kernel::<15, 13, 14>(s0, sf, (24279, 11003, 1137));
  // 1645/2048 = (Sin[29*Pi/64] + Cos[29*Pi/64])/Sqrt[2] = 0.8032075314806449
  //   305/256 = (Sin[29*Pi/64] - Cos[29*Pi/64])*Sqrt[2] = 1.1913986089848667
  //  425/2048 = Cos[29*Pi/64]*Sqrt[2]                   = 0.2075082269882116
  let (se, s1) =
    RotateSubShift::kernel::<11, 8, 11>(se, s1, (1645, 305, 425));
  // 14053/32768 = (Sin[27*Pi/64] + Cos[27*Pi/64])/Sqrt[2] = 0.85772861000027
  //   8423/8192 = (Sin[27*Pi/64] - Cos[27*Pi/64])*Sqrt[2] = 1.02820548838644
  //   2815/8192 = Cos[27*Pi/64]*Sqrt[2]                   = 0.34362586580705
  let (s2, sd) =
    RotateAddShift::kernel::<14, 13, 13>(s2, sd, (14053, 8423, 2815));
  // 14811/16384 = (Sin[25*Pi/64] + Cos[25*Pi/64])/Sqrt[2] = 0.90398929312344
  //   7005/8192 = (Sin[25*Pi/64] - Cos[25*Pi/64])*Sqrt[2] = 0.85511018686056
  //   3903/8192 = Cos[25*Pi/64]*Sqrt[2]                   = 0.47643419969316
  let (sc, s3) =
    RotateSubShift::kernel::<14, 13, 13>(sc, s3, (14811, 7005, 3903));
  // 30853/32768 = (Sin[23*Pi/64] + Cos[23*Pi/64])/Sqrt[2] = 0.94154406518302
  // 11039/16384 = (Sin[23*Pi/64] - Cos[23*Pi/64])*Sqrt[2] = 0.67377970678444
  //  9907/16384 = Cos[23*Pi/64]*Sqrt[2]                   = 0.60465421179080
  let (s4, sb) =
    RotateAddShift::kernel::<15, 14, 14>(s4, sb, (30853, 11039, 9907));
  // 15893/16384 = (Sin[21*Pi/64] + Cos[21*Pi/64])/Sqrt[2] = 0.97003125319454
  //   3981/8192 = (Sin[21*Pi/64] - Cos[21*Pi/64])*Sqrt[2] = 0.89716758634264
  //   1489/2048 = Cos[21*Pi/64]*Sqrt[2]                   = 0.72705107329128
  let (sa, s5) =
    RotateSubShift::kernel::<14, 13, 11>(sa, s5, (15893, 3981, 1489));
  // 32413/32768 = (Sin[19*Pi/64] + Cos[19*Pi/64])/Sqrt[2] = 0.98917650996478
  //    601/2048 = (Sin[19*Pi/64] - Cos[19*Pi/64])*Sqrt[2] = 0.29346094891072
  // 13803/16384 = Cos[19*Pi/64]*Sqrt[2]                   = 0.84244603550942
  let (s6, s9) =
    RotateAddShift::kernel::<15, 11, 14>(s6, s9, (32413, 601, 13803));
  // 32729/32768 = (Sin[17*Pi/64] + Cos[17*Pi/64])/Sqrt[2] = 0.99879545620517
  //    201/2048 = (Sin[17*Pi/64] - Cos[17*Pi/64])*Sqrt[2] = 0.09813534865484
  //   1945/2048 = Cos[17*Pi/64]*Sqrt[2]                   = 0.94972778187775
  let (s8, s7) =
    RotateSubShift::kernel::<15, 11, 11>(s8, s7, (32729, 201, 1945));

  // Stage 1
  let (s0, s7) = butterfly_sub_asym((s0.rshift1(), s0), s7);
  let (s8, sf) = butterfly_sub_asym((s8.rshift1(), s8), sf);
  let (s4, s3) = butterfly_add_asym((s4.rshift1(), s4), s3);
  let (sc, sb) = butterfly_add_asym((sc.rshift1(), sc), sb);
  let (s2, s5) = butterfly_sub_asym((s2.rshift1(), s2), s5);
  let (sa, sd) = butterfly_sub_asym((sa.rshift1(), sa), sd);
  let (s6, s1) = butterfly_add_asym((s6.rshift1(), s6), s1);
  let (se, s9) = butterfly_add_asym((se.rshift1(), se), s9);

  // Stage 2
  let ((_s8h, s8), s4h) = butterfly_add(s8, s4);
  let ((_s7h, s7), sbh) = butterfly_add(s7, sb);
  let ((_sah, sa), s6h) = butterfly_sub(sa, s6);
  let ((_s5h, s5), s9h) = butterfly_sub(s5, s9);
  let (s0, s3h) = butterfly_add(s0, s3);
  let (sd, seh) = butterfly_add(sd, se);
  let (s2, s1h) = butterfly_sub(s2, s1);
  let (sf, sch) = butterfly_sub(sf, sc);

  // Stage 3
  //     301/256 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  //   1609/2048 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  // 12785/32768 = 2*Cos[7*Pi/16]              = 0.3901806440322565
  let (s8, s7) =
    RotateAddAvg::kernel::<8, 11, 15>(s8, s7, (301, 1609, 12785));
  // 11363/8192 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  // 9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //  4551/8192 = Cos[5*Pi/16]                = 0.5555702330196022
  let (s9, s6) =
    RotateAdd::kernel::<13, 15, 13>(s9h, s6h, (11363, 9041, 4551));
  //  5681/4096 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  // 9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //  4551/4096 = 2*Cos[5*Pi/16]              = 1.1111404660392044
  let (s5, sa) =
    RotateNegAvg::kernel::<12, 15, 12>(s5, sa, (5681, 9041, 4551));
  //   9633/8192 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  // 12873/16384 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  //  6393/32768 = Cos[7*Pi/16]                = 0.1950903220161283
  let (s4, sb) =
    RotateNeg::kernel::<13, 14, 15>(s4h, sbh, (9633, 12873, 6393));

  // Stage 4
  let (s2, sc) = butterfly_add_asym(s2, sch);
  let (s0, s1) = butterfly_sub_asym(s0, s1h);
  let (sf, se) = butterfly_add_asym(sf, seh);
  let (sd, s3) = butterfly_add_asym(sd, s3h);
  let (s7, s6) = butterfly_add_asym((s7.rshift1(), s7), s6);
  let (s8, s9) = butterfly_sub_asym((s8.rshift1(), s8), s9);
  let (sa, sb) = butterfly_sub_asym((sa.rshift1(), sa), sb);
  let (s5, s4) = butterfly_add_asym((s5.rshift1(), s5), s4);

  // Stage 5
  //    669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[7*Pi/8]             = 0.7653668647301796
  let (sc, s3) =
    RotateAddAvg::kernel::<9, 14, 12>(sc, s3, (669, 8867, 3135));
  //    669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3870398453221475
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/4096 = 2*Cos[3*Pi/8]             = 0.7653668647301796
  let (s2, sd) =
    RotateNegAvg::kernel::<9, 14, 12>(s2, sd, (669, 8867, 3135));
  //  5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (sa, s5) = RotatePi4AddAvg::kernel::<12, 13>(sa, s5, (5793, 11585));
  //  5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (s6, s9) = RotatePi4AddAvg::kernel::<12, 13>(s6, s9, (5793, 11585));
  //  5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  // 11585/8192 = 2*Cos[Pi/4]           = 1.4142135623730951
  let (se, s1) = RotatePi4AddAvg::kernel::<12, 13>(se, s1, (5793, 11585));

  store_coeffs!(
    output, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf
  );
}

#[$m]
$($s)* fn daala_fdst16<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 16);
  let mut temp_out: [T; 16] = [T::zero(); 16];
  daala_fdst_iv_16(
    coeffs[0],
    coeffs[1],
    coeffs[2],
    coeffs[3],
    coeffs[4],
    coeffs[5],
    coeffs[6],
    coeffs[7],
    coeffs[8],
    coeffs[9],
    coeffs[10],
    coeffs[11],
    coeffs[12],
    coeffs[13],
    coeffs[14],
    coeffs[15],
    &mut temp_out,
  );

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[8];
  coeffs[2] = temp_out[4];
  coeffs[3] = temp_out[12];
  coeffs[4] = temp_out[2];
  coeffs[5] = temp_out[10];
  coeffs[6] = temp_out[6];
  coeffs[7] = temp_out[14];
  coeffs[8] = temp_out[1];
  coeffs[9] = temp_out[9];
  coeffs[10] = temp_out[5];
  coeffs[11] = temp_out[13];
  coeffs[12] = temp_out[3];
  coeffs[13] = temp_out[11];
  coeffs[14] = temp_out[7];
  coeffs[15] = temp_out[15];
}

#[$m]
$($s)* fn daala_fdct_ii_16_asym<T: TxOperations>(
  s0h: T, s1: (T, T), s2h: T, s3: (T, T), s4h: T, s5: (T, T), s6h: T,
  s7: (T, T), s8h: T, s9: (T, T), sah: T, sb: (T, T), sch: T, sd: (T, T),
  seh: T, sf: (T, T), output: &mut [T],
) {
  // +/- Butterflies with asymmetric input.
  let (s0, sf) = butterfly_neg_asym(s0h, sf);
  let (s1, se) = butterfly_sub_asym(s1, seh);
  let (s2, sd) = butterfly_neg_asym(s2h, sd);
  let (s3, sc) = butterfly_sub_asym(s3, sch);
  let (s4, sb) = butterfly_neg_asym(s4h, sb);
  let (s5, sa) = butterfly_sub_asym(s5, sah);
  let (s6, s9) = butterfly_neg_asym(s6h, s9);
  let (s7, s8) = butterfly_sub_asym(s7, s8h);

  // Embedded 8-point orthonormal transforms.
  daala_fdct_ii_8(s0, s1, s2, s3, s4, s5, s6, s7, &mut output[0..8]);
  daala_fdst_iv_8(sf, se, sd, sc, sb, sa, s9, s8, &mut output[8..16]);
  output[8..16].reverse();
}

#[$m]
$($s)* fn daala_fdst_iv_16_asym<T: TxOperations>(
  s0: (T, T), s1h: T, s2: (T, T), s3h: T, s4: (T, T), s5h: T, s6: (T, T),
  s7h: T, s8: (T, T), s9h: T, sa: (T, T), sbh: T, sc: (T, T), sdh: T,
  se: (T, T), sfh: T, output: &mut [T],
) {
  // Stage 0
  //   1073/2048 = (Sin[31*Pi/64] + Cos[31*Pi/64])/2 = 0.5239315652662953
  // 62241/32768 = (Sin[31*Pi/64] - Cos[31*Pi/64])*2 = 1.8994555637555088
  //   201/16384 = Cos[31*Pi/64]*2                   = 0.0981353486548360
  let (s0, sf) =
    RotateAddShift::half_kernel::<11, 15, 11>(s0, sfh, (1073, 62241, 201));
  // 18611/32768 = (Sin[29*Pi/64] + Cos[29*Pi/64])/2 = 0.5679534922100714
  // 55211/32768 = (Sin[29*Pi/64] - Cos[29*Pi/64])*2 = 1.6848920710188384
  //    601/2048 = Cos[29*Pi/64]*2                   = 0.2934609489107235
  let (se, s1) = RotateSubShift::half_kernel::<15, 15, 11>(
    se,
    s1h,
    (18611, 55211, 601),
  );
  //  9937/16384 = (Sin[27*Pi/64] + Cos[27*Pi/64])/2 = 0.6065057165489039
  //   1489/1024 = (Sin[27*Pi/64] - Cos[27*Pi/64])*2 = 1.4541021465825602
  //   3981/8192 = Cos[27*Pi/64]*2                   = 0.4859603598065277
  let (s2, sd) =
    RotateAddShift::half_kernel::<14, 10, 13>(s2, sdh, (9937, 1489, 3981));
  // 10473/16384 = (Sin[25*Pi/64] + Cos[25*Pi/64])/2 = 0.6392169592876205
  // 39627/32768 = (Sin[25*Pi/64] - Cos[25*Pi/64])*2 = 1.2093084235816014
  // 11039/16384 = Cos[25*Pi/64]*2                   = 0.6737797067844401
  let (sc, s3) = RotateSubShift::half_kernel::<14, 15, 14>(
    sc,
    s3h,
    (10473, 39627, 11039),
  );
  // 2727/4096 = (Sin[23*Pi/64] + Cos[23*Pi/64])/2 = 0.6657721932768628
  // 3903/4096 = (Sin[23*Pi/64] - Cos[23*Pi/64])*2 = 0.9528683993863225
  // 7005/8192 = Cos[23*Pi/64]*2                   = 0.8551101868605642
  let (s4, sb) =
    RotateAddShift::half_kernel::<12, 12, 13>(s4, sbh, (2727, 3903, 7005));
  // 5619/8192 = (Sin[21*Pi/64] + Cos[21*Pi/64])/2 = 0.6859156770967569
  // 2815/4096 = (Sin[21*Pi/64] - Cos[21*Pi/64])*2 = 0.6872517316141069
  // 8423/8192 = Cos[21*Pi/64]*2                   = 1.0282054883864433
  let (sa, s5) =
    RotateSubShift::half_kernel::<13, 12, 13>(sa, s5h, (5619, 2815, 8423));
  //   2865/4096 = (Sin[19*Pi/64] + Cos[19*Pi/64])/2 = 0.6994534179865391
  // 13588/32768 = (Sin[19*Pi/64] - Cos[19*Pi/64])*2 = 0.4150164539764232
  //     305/256 = Cos[19*Pi/64]*2                   = 1.1913986089848667
  let (s6, s9) =
    RotateAddShift::half_kernel::<12, 15, 8>(s6, s9h, (2865, 13599, 305));
  // 23143/32768 = (Sin[17*Pi/64] + Cos[17*Pi/64])/2 = 0.7062550401009887
  //   1137/8192 = (Sin[17*Pi/64] - Cos[17*Pi/64])*2 = 0.1387843410158816
  //  11003/8192 = Cos[17*Pi/64]*2                   = 1.3431179096940367
  let (s8, s7) = RotateSubShift::half_kernel::<15, 13, 13>(
    s8,
    s7h,
    (23143, 1137, 11003),
  );

  // Stage 1
  let (s0, s7) = butterfly_sub_asym((s0.rshift1(), s0), s7);
  let (s8, sf) = butterfly_sub_asym((s8.rshift1(), s8), sf);
  let (s4, s3) = butterfly_add_asym((s4.rshift1(), s4), s3);
  let (sc, sb) = butterfly_add_asym((sc.rshift1(), sc), sb);
  let (s2, s5) = butterfly_sub_asym((s2.rshift1(), s2), s5);
  let (sa, sd) = butterfly_sub_asym((sa.rshift1(), sa), sd);
  let (s6, s1) = butterfly_add_asym((s6.rshift1(), s6), s1);
  let (se, s9) = butterfly_add_asym((se.rshift1(), se), s9);

  // Stage 2
  let ((_s8h, s8), s4h) = butterfly_add(s8, s4);
  let ((_s7h, s7), sbh) = butterfly_add(s7, sb);
  let ((_sah, sa), s6h) = butterfly_sub(sa, s6);
  let ((_s5h, s5), s9h) = butterfly_sub(s5, s9);
  let (s0, s3h) = butterfly_add(s0, s3);
  let (sd, seh) = butterfly_add(sd, se);
  let (s2, s1h) = butterfly_sub(s2, s1);
  let (sf, sch) = butterfly_sub(sf, sc);

  // Stage 3
  //   9633/8192 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  // 12873/16384 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  //  6393/32768 = Cos[7*Pi/16]                = 0.1950903220161283
  let (s8, s7) =
    RotateAdd::kernel::<13, 14, 15>(s8, s7, (9633, 12873, 6393));
  // 22725/16384 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  //  9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //   4551/8192 = Cos[5*Pi/16]                = 0.5555702330196022
  let (s9, s6) =
    RotateAdd::kernel::<14, 15, 13>(s9h, s6h, (22725, 9041, 4551));
  //  11363/8192 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  //  9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //   4551/8192 = Cos[5*Pi/16]                = 0.5555702330196022
  let (s5, sa) =
    RotateNeg::kernel::<13, 15, 13>(s5, sa, (11363, 9041, 4551));
  //  9633/32768 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  // 12873/16384 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  //  6393/32768 = Cos[7*Pi/16]                = 0.1950903220161283
  let (s4, sb) =
    RotateNeg::kernel::<13, 14, 15>(s4h, sbh, (9633, 12873, 6393));

  // Stage 4
  let (s2, sc) = butterfly_add_asym(s2, sch);
  let (s0, s1) = butterfly_sub_asym(s0, s1h);
  let (sf, se) = butterfly_add_asym(sf, seh);
  let (sd, s3) = butterfly_add_asym(sd, s3h);
  let (s7, s6) = butterfly_add_asym((s7.rshift1(), s7), s6);
  let (s8, s9) = butterfly_sub_asym((s8.rshift1(), s8), s9);
  let (sa, sb) = butterfly_sub_asym((sa.rshift1(), sa), sb);
  let (s5, s4) = butterfly_add_asym((s5.rshift1(), s5), s4);

  // Stage 5
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (sc, s3) =
    RotateAdd::kernel::<13, 14, 13>(sc, s3, (10703, 8867, 3135));
  // 10703/8192 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3870398453221475
  // 8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //  3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (s2, sd) =
    RotateNeg::kernel::<13, 14, 13>(s2, sd, (10703, 8867, 3135));
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //  5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (sa, s5) = RotatePi4Add::kernel::<13, 13>(sa, s5, (11585, 5793));
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //  5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (s6, s9) = RotatePi4Add::kernel::<13, 13>(s6, s9, (11585, 5793));
  // 11585/8192 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //  5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (se, s1) = RotatePi4Add::kernel::<13, 13>(se, s1, (11585, 5793));

  store_coeffs!(
    output, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, sa, sb, sc, sd, se, sf
  );
}

#[$m]
$($s)* fn daala_fdct_ii_32<T: TxOperations>(
  t0: T, t1: T, t2: T, t3: T, t4: T, t5: T, t6: T, t7: T, t8: T, t9: T, ta: T,
  tb: T, tc: T, td: T, te: T, tf: T, tg: T, th: T, ti: T, tj: T, tk: T, tl: T,
  tm: T, tn: T, to: T, tp: T, tq: T, tr: T, ts: T, tt: T, tu: T, tv: T,
  output: &mut [T],
) {
  // +/- Butterflies with asymmetric output.
  let (t0h, tv) = butterfly_neg(t0, tv);
  let (t1, tuh) = butterfly_add(t1, tu);
  let (t2h, tt) = butterfly_neg(t2, tt);
  let (t3, tsh) = butterfly_add(t3, ts);
  let (t4h, tr) = butterfly_neg(t4, tr);
  let (t5, tqh) = butterfly_add(t5, tq);
  let (t6h, tp) = butterfly_neg(t6, tp);
  let (t7, toh) = butterfly_add(t7, to);
  let (t8h, tn) = butterfly_neg(t8, tn);
  let (t9, tmh) = butterfly_add(t9, tm);
  let (tah, tl) = butterfly_neg(ta, tl);
  let (tb, tkh) = butterfly_add(tb, tk);
  let (tch, tj) = butterfly_neg(tc, tj);
  let (td, tih) = butterfly_add(td, ti);
  let (teh, th) = butterfly_neg(te, th);
  let (tf, tgh) = butterfly_add(tf, tg);

  // Embedded 16-point transforms with asymmetric input.
  daala_fdct_ii_16_asym(
    t0h,
    t1,
    t2h,
    t3,
    t4h,
    t5,
    t6h,
    t7,
    t8h,
    t9,
    tah,
    tb,
    tch,
    td,
    teh,
    tf,
    &mut output[0..16],
  );
  daala_fdst_iv_16_asym(
    tv,
    tuh,
    tt,
    tsh,
    tr,
    tqh,
    tp,
    toh,
    tn,
    tmh,
    tl,
    tkh,
    tj,
    tih,
    th,
    tgh,
    &mut output[16..32],
  );
  output[16..32].reverse();
}

#[$m]
$($s)* fn daala_fdct32<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 32);
  let mut temp_out: [T; 32] = [T::zero(); 32];
  daala_fdct_ii_32(
    coeffs[0],
    coeffs[1],
    coeffs[2],
    coeffs[3],
    coeffs[4],
    coeffs[5],
    coeffs[6],
    coeffs[7],
    coeffs[8],
    coeffs[9],
    coeffs[10],
    coeffs[11],
    coeffs[12],
    coeffs[13],
    coeffs[14],
    coeffs[15],
    coeffs[16],
    coeffs[17],
    coeffs[18],
    coeffs[19],
    coeffs[20],
    coeffs[21],
    coeffs[22],
    coeffs[23],
    coeffs[24],
    coeffs[25],
    coeffs[26],
    coeffs[27],
    coeffs[28],
    coeffs[29],
    coeffs[30],
    coeffs[31],
    &mut temp_out,
  );

  coeffs[0] = temp_out[0];
  coeffs[1] = temp_out[16];
  coeffs[2] = temp_out[8];
  coeffs[3] = temp_out[24];
  coeffs[4] = temp_out[4];
  coeffs[5] = temp_out[20];
  coeffs[6] = temp_out[12];
  coeffs[7] = temp_out[28];
  coeffs[8] = temp_out[2];
  coeffs[9] = temp_out[18];
  coeffs[10] = temp_out[10];
  coeffs[11] = temp_out[26];
  coeffs[12] = temp_out[6];
  coeffs[13] = temp_out[22];
  coeffs[14] = temp_out[14];
  coeffs[15] = temp_out[30];
  coeffs[16] = temp_out[1];
  coeffs[17] = temp_out[17];
  coeffs[18] = temp_out[9];
  coeffs[19] = temp_out[25];
  coeffs[20] = temp_out[5];
  coeffs[21] = temp_out[21];
  coeffs[22] = temp_out[13];
  coeffs[23] = temp_out[29];
  coeffs[24] = temp_out[3];
  coeffs[25] = temp_out[19];
  coeffs[26] = temp_out[11];
  coeffs[27] = temp_out[27];
  coeffs[28] = temp_out[7];
  coeffs[29] = temp_out[23];
  coeffs[30] = temp_out[15];
  coeffs[31] = temp_out[31];
}

#[$m]
$($s)* fn daala_fdct_ii_32_asym<T: TxOperations>(
  t0h: T, t1: (T, T), t2h: T, t3: (T, T), t4h: T, t5: (T, T), t6h: T,
  t7: (T, T), t8h: T, t9: (T, T), tah: T, tb: (T, T), tch: T, td: (T, T),
  teh: T, tf: (T, T), tgh: T, th: (T, T), tih: T, tj: (T, T), tkh: T,
  tl: (T, T), tmh: T, tn: (T, T), toh: T, tp: (T, T), tqh: T, tr: (T, T),
  tsh: T, tt: (T, T), tuh: T, tv: (T, T), output: &mut [T],
) {
  // +/- Butterflies with asymmetric input.
  let (t0, tv) = butterfly_neg_asym(t0h, tv);
  let (t1, tu) = butterfly_sub_asym(t1, tuh);
  let (t2, tt) = butterfly_neg_asym(t2h, tt);
  let (t3, ts) = butterfly_sub_asym(t3, tsh);
  let (t4, tr) = butterfly_neg_asym(t4h, tr);
  let (t5, tq) = butterfly_sub_asym(t5, tqh);
  let (t6, tp) = butterfly_neg_asym(t6h, tp);
  let (t7, to) = butterfly_sub_asym(t7, toh);
  let (t8, tn) = butterfly_neg_asym(t8h, tn);
  let (t9, tm) = butterfly_sub_asym(t9, tmh);
  let (ta, tl) = butterfly_neg_asym(tah, tl);
  let (tb, tk) = butterfly_sub_asym(tb, tkh);
  let (tc, tj) = butterfly_neg_asym(tch, tj);
  let (td, ti) = butterfly_sub_asym(td, tih);
  let (te, th) = butterfly_neg_asym(teh, th);
  let (tf, tg) = butterfly_sub_asym(tf, tgh);

  // Embedded 16-point orthonormal transforms.
  daala_fdct_ii_16(
    t0,
    t1,
    t2,
    t3,
    t4,
    t5,
    t6,
    t7,
    t8,
    t9,
    ta,
    tb,
    tc,
    td,
    te,
    tf,
    &mut output[0..16],
  );
  daala_fdst_iv_16(
    tv,
    tu,
    tt,
    ts,
    tr,
    tq,
    tp,
    to,
    tn,
    tm,
    tl,
    tk,
    tj,
    ti,
    th,
    tg,
    &mut output[16..32],
  );
  output[16..32].reverse();
}

#[$m]
$($s)* fn daala_fdst_iv_32_asym<T: TxOperations>(
  t0: (T, T), t1h: T, t2: (T, T), t3h: T, t4: (T, T), t5h: T, t6: (T, T),
  t7h: T, t8: (T, T), t9h: T, ta: (T, T), tbh: T, tc: (T, T), tdh: T,
  te: (T, T), tfh: T, tg: (T, T), thh: T, ti: (T, T), tjh: T, tk: (T, T),
  tlh: T, tm: (T, T), tnh: T, to: (T, T), tph: T, tq: (T, T), trh: T,
  ts: (T, T), tth: T, tu: (T, T), tvh: T, output: &mut [T],
) {
  // Stage 0
  //   5933/8192 = (Sin[63*Pi/128] + Cos[63*Pi/128])/Sqrt[2] = 0.72424708295147
  // 22595/16384 = (Sin[63*Pi/128] - Cos[63*Pi/128])*Sqrt[2] = 1.37908108947413
  //  1137/32768 = Cos[63*Pi/128]*Sqrt[2]                    = 0.03470653821440
  let (t0, tv) =
    RotateAdd::half_kernel::<13, 14, 15>(t0, tvh, (5933, 22595, 1137));
  //   6203/8192 = (Sin[61*Pi/128] + Cos[61*Pi/128])/Sqrt[2] = 0.75720884650648
  // 21403/16384 = (Sin[61*Pi/128] - Cos[61*Pi/128])*Sqrt[2] = 1.30634568590755
  //  3409/32768 = Cos[61*Pi/128]*Sqrt[2]                    = 0.10403600355271
  let (tu, t1) =
    RotateSub::half_kernel::<13, 14, 15>(tu, t1h, (6203, 21403, 3409));
  // 25833/32768 = (Sin[59*Pi/128] + Cos[59*Pi/128])/Sqrt[2] = 0.78834642762661
  //     315/256 = (Sin[59*Pi/128] - Cos[59*Pi/128])*Sqrt[2] = 1.23046318116125
  //  5673/32768 = Cos[59*Pi/128]*Sqrt[2]                    = 0.17311483704598
  let (t2, tt) =
    RotateAdd::half_kernel::<15, 8, 15>(t2, tth, (25833, 315, 5673));
  // 26791/32768 = (Sin[57*Pi/128] + Cos[57*Pi/128])/Sqrt[2] = 0.81758481315158
  //   4717/4096 = (Sin[57*Pi/128] - Cos[57*Pi/128])*Sqrt[2] = 1.15161638283569
  //  7923/32768 = Cos[57*Pi/128]*Sqrt[2]                    = 0.24177662173374
  let (ts, t3) =
    RotateSub::half_kernel::<15, 12, 15>(ts, t3h, (26791, 4717, 7923));
  //   6921/8192 = (Sin[55*Pi/128] + Cos[55*Pi/128])/Sqrt[2] = 0.84485356524971
  // 17531/16384 = (Sin[55*Pi/128] - Cos[55*Pi/128])*Sqrt[2] = 1.06999523977419
  // 10153/32768 = Cos[55*Pi/128]*Sqrt[2]                    = 0.30985594536261
  let (t4, tr) =
    RotateAdd::half_kernel::<13, 14, 15>(t4, trh, (6921, 17531, 10153));
  // 28511/32768 = (Sin[53*Pi/128] + Cos[53*Pi/128])/Sqrt[2] = 0.87008699110871
  // 32303/32768 = (Sin[53*Pi/128] - Cos[53*Pi/128])*Sqrt[2] = 0.98579638445957
  //   1545/4096 = Cos[53*Pi/128]*Sqrt[2]                    = 0.37718879887893
  let (tq, t5) =
    RotateSub::half_kernel::<15, 15, 12>(tq, t5h, (28511, 32303, 1545));
  // 29269/32768 = (Sin[51*Pi/128] + Cos[51*Pi/128])/Sqrt[2] = 0.89322430119552
  // 14733/16384 = (Sin[51*Pi/128] - Cos[51*Pi/128])*Sqrt[2] = 0.89922265930921
  //   1817/4096 = Cos[51*Pi/128]*Sqrt[2]                    = 0.44361297154091
  let (t6, tp) =
    RotateAdd::half_kernel::<15, 14, 12>(t6, tph, (29269, 14733, 1817));
  // 29957/32768 = (Sin[49*Pi/128] + Cos[49*Pi/128])/Sqrt[2] = 0.91420975570353
  // 13279/16384 = (Sin[49*Pi/128] - Cos[49*Pi/128])*Sqrt[2] = 0.81048262800998
  //  8339/16384 = Cos[49*Pi/128]*Sqrt[2]                    = 0.50896844169854
  let (to, t7) =
    RotateSub::half_kernel::<15, 14, 14>(to, t7h, (29957, 13279, 8339));
  //   7643/8192 = (Sin[47*Pi/128] + Cos[47*Pi/128])/Sqrt[2] = 0.93299279883474
  // 11793/16384 = (Sin[47*Pi/128] - Cos[47*Pi/128])*Sqrt[2] = 0.71979007306998
  // 18779/32768 = Cos[47*Pi/128]*Sqrt[2]                    = 0.57309776229975
  let (t8, tn) =
    RotateAdd::half_kernel::<13, 14, 15>(t8, tnh, (7643, 11793, 18779));
  // 15557/16384 = (Sin[45*Pi/128] + Cos[45*Pi/128])/Sqrt[2] = 0.94952818059304
  // 20557/32768 = (Sin[45*Pi/128] - Cos[45*Pi/128])*Sqrt[2] = 0.62736348079778
  // 20835/32768 = Cos[45*Pi/128]*Sqrt[2]                    = 0.63584644019415
  let (tm, t9) =
    RotateSub::half_kernel::<14, 15, 15>(tm, t9h, (15557, 20557, 20835));
  // 31581/32768 = (Sin[43*Pi/128] + Cos[43*Pi/128])/Sqrt[2] = 0.96377606579544
  // 17479/32768 = (Sin[43*Pi/128] - Cos[43*Pi/128])*Sqrt[2] = 0.53342551494980
  // 22841/32768 = Cos[43*Pi/128]*Sqrt[2]                    = 0.69706330832054
  let (ta, tl) =
    RotateAdd::half_kernel::<15, 15, 15>(ta, tlh, (31581, 17479, 22841));
  //   7993/8192 = (Sin[41*Pi/128] + Cos[41*Pi/128])/Sqrt[2] = 0.97570213003853
  // 14359/32768 = (Sin[41*Pi/128] - Cos[41*Pi/128])*Sqrt[2] = 0.43820248031374
  //   3099/4096 = Cos[41*Pi/128]*Sqrt[2]                    = 0.75660088988166
  let (tk, tb) =
    RotateSub::half_kernel::<13, 15, 12>(tk, tbh, (7993, 14359, 3099));
  // 16143/16384 = (Sin[39*Pi/128] + Cos[39*Pi/128])/Sqrt[2] = 0.98527764238894
  //   2801/8192 = (Sin[39*Pi/128] - Cos[39*Pi/128])*Sqrt[2] = 0.34192377752060
  // 26683/32768 = Cos[39*Pi/128]*Sqrt[2]                    = 0.81431575362864
  let (tc, tj) =
    RotateAdd::half_kernel::<14, 13, 15>(tc, tjh, (16143, 2801, 26683));
  // 16261/16384 = (Sin[37*Pi/128] + Cos[37*Pi/128])/Sqrt[2] = 0.99247953459871
  //  4011/16384 = (Sin[37*Pi/128] - Cos[37*Pi/128])*Sqrt[2] = 0.24482135039843
  // 14255/16384 = Cos[37*Pi/128]*Sqrt[2]                    = 0.87006885939949
  let (ti, td) =
    RotateSub::half_kernel::<14, 14, 14>(ti, tdh, (16261, 4011, 14255));
  // 32679/32768 = (Sin[35*Pi/128] + Cos[35*Pi/128])/Sqrt[2] = 0.99729045667869
  //  4821/32768 = (Sin[35*Pi/128] - Cos[35*Pi/128])*Sqrt[2] = 0.14712912719933
  // 30269/32768 = Cos[35*Pi/128]*Sqrt[2]                    = 0.92372589307902
  let (te, th) =
    RotateAdd::half_kernel::<15, 15, 15>(te, thh, (32679, 4821, 30269));
  // 16379/16384 = (Sin[33*Pi/128] + Cos[33*Pi/128])/Sqrt[2] = 0.99969881869620
  //    201/4096 = (Sin[33*Pi/128] - Cos[33*Pi/128])*Sqrt[2] = 0.04908245704582
  // 15977/16384 = Cos[33*Pi/128]*Sqrt[2]                    = 0.97515759017329
  let (tg, tf) =
    RotateSub::half_kernel::<14, 12, 14>(tg, tfh, (16379, 201, 15977));

  // Stage 1
  let (t0, tfh) = butterfly_add(t0, tf);
  let (tv, tgh) = butterfly_sub(tv, tg);
  let (th, tuh) = butterfly_add(th, tu);
  let (te, t1h) = butterfly_sub(te, t1);
  let (t2, tdh) = butterfly_add(t2, td);
  let (tt, tih) = butterfly_sub(tt, ti);
  let (tj, tsh) = butterfly_add(tj, ts);
  let (tc, t3h) = butterfly_sub(tc, t3);
  let (t4, tbh) = butterfly_add(t4, tb);
  let (tr, tkh) = butterfly_sub(tr, tk);
  let (tl, tqh) = butterfly_add(tl, tq);
  let (ta, t5h) = butterfly_sub(ta, t5);
  let (t6, t9h) = butterfly_add(t6, t9);
  let (tp, tmh) = butterfly_sub(tp, tm);
  let (tn, toh) = butterfly_add(tn, to);
  let (t8, t7h) = butterfly_sub(t8, t7);

  // Stage 2
  let (t0, t7) = butterfly_sub_asym(t0, t7h);
  let (tv, to) = butterfly_add_asym(tv, toh);
  let (tp, tu) = butterfly_sub_asym(tp, tuh);
  let (t6, t1) = butterfly_add_asym(t6, t1h);
  let (t2, t5) = butterfly_sub_asym(t2, t5h);
  let (tt, tq) = butterfly_add_asym(tt, tqh);
  let (tr, ts) = butterfly_sub_asym(tr, tsh);
  let (t4, t3) = butterfly_add_asym(t4, t3h);
  let (t8, tg) = butterfly_add_asym(t8, tgh);
  let (te, tm) = butterfly_sub_asym(te, tmh);
  let (tn, tf) = butterfly_add_asym(tn, tfh);
  let (th, t9) = butterfly_sub_asym(th, t9h);
  let (ta, ti) = butterfly_add_asym(ta, tih);
  let (tc, tk) = butterfly_sub_asym(tc, tkh);
  let (tl, td) = butterfly_add_asym(tl, tdh);
  let (tj, tb) = butterfly_sub_asym(tj, tbh);

  // Stage 3
  // 17911/16384 = Sin[15*Pi/32] + Cos[15*Pi/32] = 1.0932018670017576
  // 14699/16384 = Sin[15*Pi/32] - Cos[15*Pi/32] = 0.8971675863426363
  //    803/8192 = Cos[15*Pi/32]                 = 0.0980171403295606
  let (tf, tg) =
    RotateSub::kernel::<14, 14, 13>(tf, tg, (17911, 14699, 803));
  //  10217/8192 = Sin[13*Pi/32] + Cos[13*Pi/32] = 1.2472250129866712
  //   5461/8192 = Sin[13*Pi/32] - Cos[13*Pi/32] = 0.6666556584777465
  //   1189/4096 = Cos[13*Pi/32]                 = 0.2902846772544623
  let (th, te) =
    RotateAdd::kernel::<13, 13, 12>(th, te, (10217, 5461, 1189));
  //   5543/4096 = Sin[11*Pi/32] + Cos[11*Pi/32] = 1.3533180011743526
  //   3363/8192 = Sin[11*Pi/32] - Cos[11*Pi/32] = 0.4105245275223574
  //  7723/16384 = Cos[11*Pi/32]                 = 0.4713967368259976
  let (ti, td) =
    RotateAdd::kernel::<12, 13, 14>(ti, td, (5543, 3363, 7723));
  //  11529/8192 = Sin[9*Pi/32] + Cos[9*Pi/32] = 1.4074037375263826
  //  2271/16384 = Sin[9*Pi/32] - Cos[9*Pi/32] = 0.1386171691990915
  //   5197/8192 = Cos[9*Pi/32]                = 0.6343932841636455
  let (tc, tj) =
    RotateSub::kernel::<13, 14, 13>(tc, tj, (11529, 2271, 5197));
  //  11529/8192 = Sin[9*Pi/32] + Cos[9*Pi/32] = 1.4074037375263826
  //  2271/16384 = Sin[9*Pi/32] - Cos[9*Pi/32] = 0.1386171691990915
  //   5197/8192 = Cos[9*Pi/32]                = 0.6343932841636455
  let (tb, tk) =
    RotateNeg::kernel::<13, 14, 13>(tb, tk, (11529, 2271, 5197));
  //   5543/4096 = Sin[11*Pi/32] + Cos[11*Pi/32] = 1.3533180011743526
  //   3363/8192 = Sin[11*Pi/32] - Cos[11*Pi/32] = 0.4105245275223574
  //  7723/16384 = Cos[11*Pi/32]                 = 0.4713967368259976
  let (ta, tl) =
    RotateNeg::kernel::<12, 13, 14>(ta, tl, (5543, 3363, 7723));
  //  10217/8192 = Sin[13*Pi/32] + Cos[13*Pi/32] = 1.2472250129866712
  //   5461/8192 = Sin[13*Pi/32] - Cos[13*Pi/32] = 0.6666556584777465
  //   1189/4096 = Cos[13*Pi/32]                 = 0.2902846772544623
  let (t9, tm) =
    RotateNeg::kernel::<13, 13, 12>(t9, tm, (10217, 5461, 1189));
  // 17911/16384 = Sin[15*Pi/32] + Cos[15*Pi/32] = 1.0932018670017576
  // 14699/16384 = Sin[15*Pi/32] - Cos[15*Pi/32] = 0.8971675863426363
  //    803/8192 = Cos[15*Pi/32]                 = 0.0980171403295606
  let (t8, tn) =
    RotateNeg::kernel::<14, 14, 13>(t8, tn, (17911, 14699, 803));

  // Stage 4
  let (t3, t0h) = butterfly_sub(t3, t0);
  let (ts, tvh) = butterfly_add(ts, tv);
  let (tu, tth) = butterfly_sub(tu, tt);
  let (t1, t2h) = butterfly_add(t1, t2);
  let ((_toh, to), t4h) = butterfly_add(to, t4);
  let ((_tqh, tq), t6h) = butterfly_sub(tq, t6);
  let ((_t7h, t7), trh) = butterfly_add(t7, tr);
  let ((_t5h, t5), tph) = butterfly_sub(t5, tp);
  let (tb, t8h) = butterfly_sub(tb, t8);
  let (tk, tnh) = butterfly_add(tk, tn);
  let (tm, tlh) = butterfly_sub(tm, tl);
  let (t9, tah) = butterfly_add(t9, ta);
  let (tf, tch) = butterfly_sub(tf, tc);
  let (tg, tjh) = butterfly_add(tg, tj);
  let (ti, thh) = butterfly_sub(ti, th);
  let (td, teh) = butterfly_add(td, te);

  // Stage 5
  //     301/256 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  //   1609/2048 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  //  6393/32768 = Cos[7*Pi/16]                = 0.1950903220161283
  let (to, t7) = RotateAdd::kernel::<8, 11, 15>(to, t7, (301, 1609, 6393));
  //  11363/8192 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  //  9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //   4551/8192 = Cos[5*Pi/16]                = 0.5555702330196022
  let (tph, t6h) =
    RotateAdd::kernel::<13, 15, 13>(tph, t6h, (11363, 9041, 4551));
  //   5681/4096 = Sin[5*Pi/16] + Cos[5*Pi/16] = 1.3870398453221475
  //  9041/32768 = Sin[5*Pi/16] - Cos[5*Pi/16] = 0.2758993792829431
  //   4551/8192 = Cos[5*Pi/16]                = 0.5555702330196022
  let (t5, tq) =
    RotateNeg::kernel::<12, 15, 13>(t5, tq, (5681, 9041, 4551));
  //   9633/8192 = Sin[7*Pi/16] + Cos[7*Pi/16] = 1.1758756024193586
  // 12873/16384 = Sin[7*Pi/16] - Cos[7*Pi/16] = 0.7856949583871022
  //  6393/32768 = Cos[7*Pi/16]                = 0.1950903220161283
  let (t4h, trh) =
    RotateNeg::kernel::<13, 14, 15>(t4h, trh, (9633, 12873, 6393));

  // Stage 6
  let (t1, t0) = butterfly_add_asym(t1, t0h);
  let (tu, tv) = butterfly_sub_asym(tu, tvh);
  let (ts, t2) = butterfly_sub_asym(ts, t2h);
  let (t3, tt) = butterfly_sub_asym(t3, tth);
  let (t5, t4) = butterfly_add_asym((t5.rshift1(), t5), t4h);
  let (tq, tr) = butterfly_sub_asym((tq.rshift1(), tq), trh);
  let (t7, t6) = butterfly_add_asym((t7.rshift1(), t7), t6h);
  let (to, tp) = butterfly_sub_asym((to.rshift1(), to), tph);
  let (t9, t8) = butterfly_add_asym(t9, t8h);
  let (tm, tn) = butterfly_sub_asym(tm, tnh);
  let (tk, ta) = butterfly_sub_asym(tk, tah);
  let (tb, tl) = butterfly_sub_asym(tb, tlh);
  let (ti, tc) = butterfly_add_asym(ti, tch);
  let (td, tj) = butterfly_add_asym(td, tjh);
  let (tf, te) = butterfly_add_asym(tf, teh);
  let (tg, th) = butterfly_sub_asym(tg, thh);

  // Stage 7
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (t2, tt) = RotateNeg::kernel::<9, 14, 13>(t2, tt, (669, 8867, 3135));
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (ts, t3) = RotateAdd::kernel::<9, 14, 13>(ts, t3, (669, 8867, 3135));
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (ta, tl) = RotateNeg::kernel::<9, 14, 13>(ta, tl, (669, 8867, 3135));
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (tk, tb) = RotateAdd::kernel::<9, 14, 13>(tk, tb, (669, 8867, 3135));
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (tc, tj) = RotateAdd::kernel::<9, 14, 13>(tc, tj, (669, 8867, 3135));
  //     669/512 = Sin[3*Pi/8] + Cos[3*Pi/8] = 1.3065629648763766
  //  8867/16384 = Sin[3*Pi/8] - Cos[3*Pi/8] = 0.5411961001461969
  //   3135/8192 = Cos[3*Pi/8]               = 0.3826834323650898
  let (ti, td) = RotateNeg::kernel::<9, 14, 13>(ti, td, (669, 8867, 3135));
  //   5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //   5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (tu, t1) = RotatePi4Add::kernel::<12, 13>(tu, t1, (5793, 5793));
  //   5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //   5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (tq, t5) = RotatePi4Add::kernel::<12, 13>(tq, t5, (5793, 5793));
  //   5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //   5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (tp, t6) = RotatePi4Sub::kernel::<12, 13>(tp, t6, (5793, 5793));
  //   5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //   5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (tm, t9) = RotatePi4Add::kernel::<12, 13>(tm, t9, (5793, 5793));
  //   5793/4096 = Sin[Pi/4] + Cos[Pi/4] = 1.4142135623730951
  //   5793/8192 = Cos[Pi/4]             = 0.7071067811865475
  let (te, th) = RotatePi4Add::kernel::<12, 13>(te, th, (5793, 5793));

  store_coeffs!(
    output, t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, ta, tb, tc, td, te, tf,
    tg, th, ti, tj, tk, tl, tm, tn, to, tp, tq, tr, ts, tt, tu, tv
  );
}

#[allow(clippy::identity_op)]
#[$m]
$($s)* fn daala_fdct64<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 64);
  // Use arrays to avoid ridiculous variable names
  let mut asym: [(T, T); 32] = [(T::zero(), T::zero()); 32];
  let mut half: [T; 32] = [T::zero(); 32];
  // +/- Butterflies with asymmetric output.
  {
    #[$m]
    #[inline]
    $($s)* fn butterfly_pair<T: TxOperations>(
      half: &mut [T; 32], asym: &mut [(T, T); 32], input: &[T], i: usize
    ) {
      let j = i * 2;
      let (ah, c) = butterfly_neg(input[j], input[63 - j]);
      let (b, dh) = butterfly_add(input[j + 1], input[63 - j - 1]);
      half[i] = ah;
      half[31 - i] = dh;
      asym[i] = b;
      asym[31 - i] = c;
    }
    butterfly_pair(&mut half, &mut asym, coeffs, 0);
    butterfly_pair(&mut half, &mut asym, coeffs, 1);
    butterfly_pair(&mut half, &mut asym, coeffs, 2);
    butterfly_pair(&mut half, &mut asym, coeffs, 3);
    butterfly_pair(&mut half, &mut asym, coeffs, 4);
    butterfly_pair(&mut half, &mut asym, coeffs, 5);
    butterfly_pair(&mut half, &mut asym, coeffs, 6);
    butterfly_pair(&mut half, &mut asym, coeffs, 7);
    butterfly_pair(&mut half, &mut asym, coeffs, 8);
    butterfly_pair(&mut half, &mut asym, coeffs, 9);
    butterfly_pair(&mut half, &mut asym, coeffs, 10);
    butterfly_pair(&mut half, &mut asym, coeffs, 11);
    butterfly_pair(&mut half, &mut asym, coeffs, 12);
    butterfly_pair(&mut half, &mut asym, coeffs, 13);
    butterfly_pair(&mut half, &mut asym, coeffs, 14);
    butterfly_pair(&mut half, &mut asym, coeffs, 15);
  }

  let mut temp_out: [T; 64] = [T::zero(); 64];
  // Embedded 2-point transforms with asymmetric input.
  daala_fdct_ii_32_asym(
    half[0],
    asym[0],
    half[1],
    asym[1],
    half[2],
    asym[2],
    half[3],
    asym[3],
    half[4],
    asym[4],
    half[5],
    asym[5],
    half[6],
    asym[6],
    half[7],
    asym[7],
    half[8],
    asym[8],
    half[9],
    asym[9],
    half[10],
    asym[10],
    half[11],
    asym[11],
    half[12],
    asym[12],
    half[13],
    asym[13],
    half[14],
    asym[14],
    half[15],
    asym[15],
    &mut temp_out[0..32],
  );
  daala_fdst_iv_32_asym(
    asym[31],
    half[31],
    asym[30],
    half[30],
    asym[29],
    half[29],
    asym[28],
    half[28],
    asym[27],
    half[27],
    asym[26],
    half[26],
    asym[25],
    half[25],
    asym[24],
    half[24],
    asym[23],
    half[23],
    asym[22],
    half[22],
    asym[21],
    half[21],
    asym[20],
    half[20],
    asym[19],
    half[19],
    asym[18],
    half[18],
    asym[17],
    half[17],
    asym[16],
    half[16],
    &mut temp_out[32..64],
  );
  temp_out[32..64].reverse();

  // Store a reordered version of output in temp_out
  #[$m]
  #[inline]
  $($s)* fn reorder_4<T: TxOperations>(
    output: &mut [T], i: usize, tmp: [T; 64], j: usize
  ) {
    output[0 + i * 4] = tmp[0 + j];
    output[1 + i * 4] = tmp[32 + j];
    output[2 + i * 4] = tmp[16 + j];
    output[3 + i * 4] = tmp[48 + j];
  }
  reorder_4(coeffs, 0, temp_out, 0);
  reorder_4(coeffs, 1, temp_out, 8);
  reorder_4(coeffs, 2, temp_out, 4);
  reorder_4(coeffs, 3, temp_out, 12);
  reorder_4(coeffs, 4, temp_out, 2);
  reorder_4(coeffs, 5, temp_out, 10);
  reorder_4(coeffs, 6, temp_out, 6);
  reorder_4(coeffs, 7, temp_out, 14);

  reorder_4(coeffs, 8, temp_out, 1);
  reorder_4(coeffs, 9, temp_out, 9);
  reorder_4(coeffs, 10, temp_out, 5);
  reorder_4(coeffs, 11, temp_out, 13);
  reorder_4(coeffs, 12, temp_out, 3);
  reorder_4(coeffs, 13, temp_out, 11);
  reorder_4(coeffs, 14, temp_out, 7);
  reorder_4(coeffs, 15, temp_out, 15);
}

#[$m]
$($s)* fn fidentity<T: TxOperations>(_coeffs: &mut [T]) {}

#[$m]
$($s)* fn fwht4<T: TxOperations>(coeffs: &mut [T]) {
  assert!(coeffs.len() >= 4);
  let x0 = coeffs[0];
  let x1 = coeffs[1];
  let x2 = coeffs[2];
  let x3 = coeffs[3];

  let s0 = x0.add(x1);
  let s1 = x3.sub(x2);
  let s2 = s0.sub_avg(s1);

  let q1 = s2.sub(x2);
  let q0 = s0.sub(q1);
  let q3 = s2.sub(x1);
  let q2 = s1.add(q3);

  store_coeffs!(coeffs, q0, q1, q2, q3);
}

}

}
