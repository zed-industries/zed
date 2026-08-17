use crate::error::Error;

use vsimd::ascii::AsciiCase;

vsimd::dispatch!(
    name        = {parse_simple},
    signature   = {pub unsafe fn(src: *const u8, dst: *mut u8) -> Result<(), Error>},
    fallback    = {crate::parse::parse_simple_fallback},
    simd        = {crate::parse::parse_simple_simd},
    targets     = {"avx2", "ssse3", "sse2", "neon", "simd128"},
    fastest     = {"avx2", "neon", "simd128"},
);

vsimd::dispatch!(
    name        = {parse_hyphenated},
    signature   = {pub unsafe fn(src: *const u8, dst: *mut u8) -> Result<(), Error>},
    fallback    = {crate::parse::parse_hyphenated_fallback},
    simd        = {crate::parse::parse_hyphenated_simd},
    targets     = {"avx2", "sse4.1", "neon", "simd128"},
    fastest     = {"avx2", "neon", "simd128"},
);

vsimd::dispatch!(
    name        = {format_simple},
    signature   = {pub unsafe fn(src: *const u8, dst: *mut u8, case: AsciiCase) -> ()},
    fallback    = {crate::format::format_simple_fallback},
    simd        = {crate::format::format_simple_simd},
    targets     = {"avx2", "ssse3", "sse2", "neon", "simd128"},
    fastest     = {"avx2", "neon", "simd128"},
);

vsimd::dispatch!(
    name        = {format_hyphenated},
    signature   = {pub unsafe fn(src: *const u8, dst: *mut u8, case: AsciiCase) -> ()},
    fallback    = {crate::format::format_hyphenated_fallback},
    simd        = {crate::format::format_hyphenated_simd},
    targets     = {"avx2", "sse4.1", "neon", "simd128"},
    fastest     = {"avx2", "neon", "simd128"},
);
