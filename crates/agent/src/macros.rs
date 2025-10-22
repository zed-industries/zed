macro_rules! _simd {
    (
        8 => $_8:expr,
        16 => $_16:expr,
        32 => $_32:expr,
        64 => $_64:expr $(,)?
    ) => {{
        #[cfg(not(any(
            target_feature = "avx2",
            target_feature = "avx512f",
            target_feature = "neon",
            target_feature = "sse2"
        )))]
        let rslt = $_8;

        #[cfg(all(
            target_feature = "neon",
            not(any(target_feature = "avx2", target_feature = "avx512f"))
        ))]
        let rslt = $_16;

        #[cfg(all(
            target_feature = "sse2",
            not(any(
                target_feature = "avx2",
                target_feature = "avx512f",
                target_feature = "neon"
            ))
        ))]
        let rslt = $_16;

        #[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
        let rslt = $_32;

        #[cfg(target_feature = "avx512f")]
        let rslt = $_64;

        rslt
    }};
}

macro_rules! _simd_slice {
    (
        ($(($immutables_ident:ident, $immutables_chunk:expr)),* $(,)?),
        ($(($mutables_ident:ident, $mutables_chunk:expr)),* $(,)?),
        $_chunk_pat:pat => $_chunk_expr:expr,
        $_rest_pat:pat => $_rest_expr:expr $(,)?
    ) => {{
        const fn calc(simd: usize, chunk: usize) -> usize {
            (simd / chunk) * chunk
        }
        let (($($immutables_ident,)*), ($($mutables_ident,)*)) = _simd! {
          8 => (
              ($($immutables_ident.as_chunks::<{calc(8, $immutables_chunk)}>(),)*),
              ($($mutables_ident.as_chunks_mut::<{calc(8, $mutables_chunk)}>(),)*)
          ),
          16 => (
              ($($immutables_ident.as_chunks::<{calc(16, $immutables_chunk)}>(),)*),
              ($($mutables_ident.as_chunks_mut::<{calc(16, $mutables_chunk)}>(),)*)
          ),
          32 => (
              ($($immutables_ident.as_chunks::<{calc(32, $immutables_chunk)}>(),)*),
              ($($mutables_ident.as_chunks_mut::<{calc(32, $mutables_chunk)}>(),)*)
          ),
          64 => (
              ($($immutables_ident.as_chunks::<{calc(64, $immutables_chunk)}>(),)*),
              ($($mutables_ident.as_chunks_mut::<{calc(64, $mutables_chunk)}>(),)*)
          )
        };
        let $_chunk_pat = (
            ($({ $immutables_ident.0 },)*),
            ($({ $mutables_ident.0 },)*)
        );
        $_chunk_expr
        let $_rest_pat = (
            ($({ $immutables_ident.1 },)*),
            ($({ $mutables_ident.1 },)*)
        );
        $_rest_expr
    }};
}
