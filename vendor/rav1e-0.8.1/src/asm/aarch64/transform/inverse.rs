// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::tiling::PlaneRegionMut;
use crate::transform::inverse::*;
use crate::transform::*;
use crate::{Pixel, PixelType};

use crate::asm::shared::transform::inverse::*;

pub fn inverse_transform_add<T: Pixel>(
  input: &[T::Coeff], output: &mut PlaneRegionMut<'_, T>, eob: u16,
  tx_size: TxSize, tx_type: TxType, bd: usize, cpu: CpuFeatureLevel,
) {
  match T::type_enum() {
    PixelType::U8 => {
      if let Some(func) = INV_TXFM_FNS[cpu.as_index()][tx_size][tx_type] {
        return call_inverse_func(
          func,
          input,
          output,
          eob,
          tx_size.width(),
          tx_size.height(),
          bd,
        );
      }
    }
    PixelType::U16 if bd == 10 => {
      if let Some(func) = INV_TXFM_HBD_FNS[cpu.as_index()][tx_size][tx_type] {
        return call_inverse_hbd_func(
          func,
          input,
          output,
          eob,
          tx_size.width(),
          tx_size.height(),
          bd,
        );
      }
    }
    PixelType::U16 => {}
  };

  rust::inverse_transform_add(input, output, eob, tx_size, tx_type, bd, cpu);
}

macro_rules! decl_itx_fns {
  // Takes a 2d list of tx types for W and H
  ([$([$(($ENUM:expr, $TYPE1:ident, $TYPE2:ident)),*]),*], $W:expr, $H:expr,
   $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      // For each tx type, declare an function for the current WxH
      $(
        $(
          extern {
            // Note: type1 and type2 are flipped
            fn [<rav1e_inv_txfm_add_ $TYPE2 _$TYPE1 _$W x $H _8bpc_$OPT_LOWER>](
              dst: *mut u8, dst_stride: libc::ptrdiff_t, coeff: *mut i16,
              eob: i32
            );
          }
        )*
      )*
      // Create a lookup table for the tx types declared above
      const [<INV_TXFM_FNS_$W _$H _$OPT_UPPER>]: [Option<InvTxfmFunc>; TX_TYPES_PLUS_LL] = {
        let mut out: [Option<InvTxfmFunc>; TX_TYPES_PLUS_LL] = [None; TX_TYPES_PLUS_LL];
        $(
          $(
            out[$ENUM as usize] = Some([<rav1e_inv_txfm_add_$TYPE2 _$TYPE1 _$W x $H _8bpc_$OPT_LOWER>]);
          )*
        )*
        out
      };
    }
  };
}

macro_rules! decl_itx_hbd_fns {
  // Takes a 2d list of tx types for W and H
  ([$([$(($ENUM:expr, $TYPE1:ident, $TYPE2:ident)),*]),*], $W:expr, $H:expr,
   $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      // For each tx type, declare an function for the current WxH
      $(
        $(
          extern {
            // Note: type1 and type2 are flipped
            fn [<rav1e_inv_txfm_add_ $TYPE2 _$TYPE1 _$W x $H _16bpc_$OPT_LOWER>](
              dst: *mut u16, dst_stride: libc::ptrdiff_t, coeff: *mut i16,
              eob: i32, bitdepth_max: i32,
            );
          }
        )*
      )*
      // Create a lookup table for the tx types declared above
      const [<INV_TXFM_HBD_FNS_$W _$H _$OPT_UPPER>]: [Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL] = {
        let mut out: [Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL] = [None; TX_TYPES_PLUS_LL];
        $(
          $(
            out[$ENUM as usize] = Some([<rav1e_inv_txfm_add_$TYPE2 _$TYPE1 _$W x $H _16bpc_$OPT_LOWER>]);
          )*
        )*
        out
      };
    }
  };
}

macro_rules! create_wxh_tables {
  // Create a lookup table for each cpu feature
  ([$([$(($W:expr, $H:expr)),*]),*], $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      const [<INV_TXFM_FNS_$OPT_UPPER>]: [[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] = {
        let mut out: [[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] =
          [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL];
        // For each dimension, add an entry to the table
        $(
          $(
            out[TxSize::[<TX_ $W X $H>] as usize] = [<INV_TXFM_FNS_$W _$H _$OPT_UPPER>];
          )*
        )*
        out
      };
    }
  };

  // Loop through cpu features
  ($DIMS:tt, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      create_wxh_tables!($DIMS, $OPT_LOWER, $OPT_UPPER);
    )*
  };
}

macro_rules! create_wxh_hbd_tables {
  // Create a lookup table for each cpu feature
  ([$([$(($W:expr, $H:expr)),*]),*], $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      const [<INV_TXFM_HBD_FNS_$OPT_UPPER>]: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] = {
        let mut out: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] =
          [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL];
        // For each dimension, add an entry to the table
        $(
          $(
            out[TxSize::[<TX_ $W X $H>] as usize] = [<INV_TXFM_HBD_FNS_$W _$H _$OPT_UPPER>];
          )*
        )*
        out
      };
    }
  };

  // Loop through cpu features
  ($DIMS:tt, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      create_wxh_hbd_tables!($DIMS, $OPT_LOWER, $OPT_UPPER);
    )*
  };
}

macro_rules! impl_itx_fns {
  ($TYPES:tt, $W:expr, $H:expr, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      decl_itx_fns!($TYPES, $W, $H, $OPT_LOWER, $OPT_UPPER);
    )*
  };

  // Loop over a list of dimensions
  ($TYPES_VALID:tt, [$(($W:expr, $H:expr)),*], $OPT:tt) => {
    $(
      impl_itx_fns!($TYPES_VALID, $W, $H, $OPT);
    )*
  };

  ($TYPES64:tt, $DIMS64:tt, $TYPES32:tt, $DIMS32:tt, $TYPES16:tt, $DIMS16:tt,
   $TYPES84:tt, $DIMS84:tt, $TYPES4:tt, $DIMS4:tt, $OPT:tt) => {
    // Make 2d list of tx types for each set of dimensions. Each set of
    //   dimensions uses a superset of the previous set of tx types.
    impl_itx_fns!([$TYPES64], $DIMS64, $OPT);
    impl_itx_fns!([$TYPES64, $TYPES32], $DIMS32, $OPT);
    impl_itx_fns!([$TYPES64, $TYPES32, $TYPES16], $DIMS16, $OPT);
    impl_itx_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84], $DIMS84, $OPT
    );
    impl_itx_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84, $TYPES4], $DIMS4, $OPT
    );

    // Pool all of the dimensions together to create a table for each cpu
    // feature level.
    create_wxh_tables!(
      [$DIMS64, $DIMS32, $DIMS16, $DIMS84, $DIMS4], $OPT
    );
  };
}

impl_itx_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16 and 4x4)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8)],
  // 4x4
  [(TxType::WHT_WHT, wht, wht)],
  [(4, 4)],
  [(neon, NEON)]
);

cpu_function_lookup_table!(
  INV_TXFM_FNS: [[[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL]],
  default: [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  [NEON]
);

macro_rules! impl_itx_hbd_fns {

  ($TYPES:tt, $W:expr, $H:expr, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      decl_itx_hbd_fns!($TYPES, $W, $H, $OPT_LOWER, $OPT_UPPER);
    )*
  };

  // Loop over a list of dimensions
  ($TYPES_VALID:tt, [$(($W:expr, $H:expr)),*], $OPT:tt) => {
    $(
      impl_itx_hbd_fns!($TYPES_VALID, $W, $H, $OPT);
    )*
  };

  ($TYPES64:tt, $DIMS64:tt, $TYPES32:tt, $DIMS32:tt, $TYPES16:tt, $DIMS16:tt,
   $TYPES84:tt, $DIMS84:tt, $TYPES4:tt, $DIMS4:tt, $OPT:tt) => {
    // Make 2d list of tx types for each set of dimensions. Each set of
    //   dimensions uses a superset of the previous set of tx types.
    impl_itx_hbd_fns!([$TYPES64], $DIMS64, $OPT);
    impl_itx_hbd_fns!([$TYPES64, $TYPES32], $DIMS32, $OPT);
    impl_itx_hbd_fns!([$TYPES64, $TYPES32, $TYPES16], $DIMS16, $OPT);
    impl_itx_hbd_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84], $DIMS84, $OPT
    );
    impl_itx_hbd_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84, $TYPES4], $DIMS4, $OPT
    );

    // Pool all of the dimensions together to create a table for each cpu
    // feature level.
    create_wxh_hbd_tables!(
      [$DIMS64, $DIMS32, $DIMS16, $DIMS84, $DIMS4], $OPT
    );
  };
}

impl_itx_hbd_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16 and 4x4)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8)],
  // 4x4
  [(TxType::WHT_WHT, wht, wht)],
  [(4, 4)],
  [(neon, NEON)]
);

cpu_function_lookup_table!(
  INV_TXFM_HBD_FNS: [[[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL]],
  default: [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  [NEON]
);
