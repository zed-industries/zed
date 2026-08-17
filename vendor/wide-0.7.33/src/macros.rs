macro_rules! int_uint_consts {
  ($type:ty, $lanes:expr, $simd:ty, $bits:expr) => {
    // ensure the size of the SIMD type is the same as the size of the array and number of bits is OK
    const _: () = assert!(
      core::mem::size_of::<$simd>() == core::mem::size_of::<[$type; $lanes]>()
        && core::mem::size_of::<$simd>() * 8 == $bits as usize
    );

    impl $simd {
      pub const ONE: $simd = <$simd>::new([1; $lanes]);
      pub const ZERO: $simd = <$simd>::new([0; $lanes]);
      pub const MAX: $simd = <$simd>::new([<$type>::MAX; $lanes]);
      pub const MIN: $simd = <$simd>::new([<$type>::MIN; $lanes]);

      /// The number of lanes in this SIMD vector.
      pub const LANES: u16 = $lanes;

      /// The size of this SIMD vector in bits.
      pub const BITS: u16 = $bits;
    }
  };
}
