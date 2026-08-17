#[cfg(asm_neon)]
pub use neon::*;
#[cfg(not(any(asm_x86_64, asm_neon)))]
pub use rust::*;
#[cfg(asm_x86_64)]
pub use x86::*;

#[cfg(not(any(asm_x86_64, asm_neon)))]
mod rust {
    use arg_enum_proc_macro::ArgEnum;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, ArgEnum, Default)]
    #[allow(clippy::upper_case_acronyms)]

    pub enum CpuFeatureLevel {
        #[default]
        RUST,
    }

    impl CpuFeatureLevel {
        #[cfg(test)]
        #[allow(unused)]
        #[inline]
        pub const fn all() -> &'static [Self] {
            use CpuFeatureLevel::*;
            &[RUST]
        }
    }
}

#[cfg(asm_x86_64)]
#[macro_use]
mod x86 {
    use std::{env, str::FromStr};

    use arg_enum_proc_macro::ArgEnum;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, ArgEnum)]
    #[allow(clippy::upper_case_acronyms)]
    pub enum CpuFeatureLevel {
        RUST,
        SSE2,
        SSSE3,
        #[arg_enum(alias = "sse4.1")]
        SSE4_1,
        AVX2,
        AVX512,
        #[arg_enum(alias = "avx512vpclmulqdq")]
        AVX512ICL,
    }

    impl CpuFeatureLevel {
        #[cfg(test)]
        pub const fn all() -> &'static [Self] {
            &[
                CpuFeatureLevel::RUST,
                CpuFeatureLevel::SSE2,
                CpuFeatureLevel::SSSE3,
                CpuFeatureLevel::SSE4_1,
                CpuFeatureLevel::AVX2,
                CpuFeatureLevel::AVX512,
                CpuFeatureLevel::AVX512ICL,
            ]
        }

        #[inline]
        pub const fn len() -> usize {
            CpuFeatureLevel::AVX512ICL as usize + 1
        }

        #[inline]
        pub const fn as_index(self) -> usize {
            self as usize
        }
    }

    impl Default for CpuFeatureLevel {
        #[inline]
        fn default() -> CpuFeatureLevel {
            fn avx512_detected() -> bool {
                is_x86_feature_detected!("avx512bw")
                    && is_x86_feature_detected!("avx512cd")
                    && is_x86_feature_detected!("avx512dq")
                    && is_x86_feature_detected!("avx512f")
                    && is_x86_feature_detected!("avx512vl")
            }
            #[allow(deprecated)] // Until MSRV >= 1.69.0
            fn avx512icl_detected() -> bool {
                // Per dav1d, these are the flags needed.
                avx512_detected()
                    && is_x86_feature_detected!("avx512vnni")
                    && is_x86_feature_detected!("avx512ifma")
                    && is_x86_feature_detected!("avx512vbmi")
                    && is_x86_feature_detected!("avx512vbmi2")
                    && is_x86_feature_detected!("avx512vpopcntdq")
                    && is_x86_feature_detected!("avx512bitalg")
                    && is_x86_feature_detected!("avx512gfni")
                    && is_x86_feature_detected!("avx512vaes")
                    && is_x86_feature_detected!("avx512vpclmulqdq")
            }

            let detected: CpuFeatureLevel = if avx512icl_detected() {
                CpuFeatureLevel::AVX512ICL
            } else if avx512_detected() {
                CpuFeatureLevel::AVX512
            } else if is_x86_feature_detected!("avx2") {
                CpuFeatureLevel::AVX2
            } else if is_x86_feature_detected!("sse4.1") {
                CpuFeatureLevel::SSE4_1
            } else if is_x86_feature_detected!("ssse3") {
                CpuFeatureLevel::SSSE3
            } else if is_x86_feature_detected!("sse2") {
                CpuFeatureLevel::SSE2
            } else {
                CpuFeatureLevel::RUST
            };
            let manual: CpuFeatureLevel = match env::var("CPU_TARGET") {
                Ok(feature) => CpuFeatureLevel::from_str(&feature).unwrap_or(detected),
                Err(_e) => detected,
            };
            if manual > detected {
                detected
            } else {
                manual
            }
        }
    }

    // Create a static lookup table for CPUFeatureLevel enums
    // Note: keys are CpuFeatureLevels without any prefix (no CpuFeatureLevel::)
    macro_rules! cpu_function_lookup_table {
        // version for default visibility
        ($name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
            static $name: [$type; crate::cpu::CpuFeatureLevel::len()] = {
            use crate::cpu::CpuFeatureLevel;
            #[allow(unused_mut)]
            let mut out: [$type; CpuFeatureLevel::len()] = [$empty; CpuFeatureLevel::len()];

            // Can't use out[0][.] == $empty in static as of rust 1.40
            #[allow(unused_mut)]
            let mut set: [bool; CpuFeatureLevel::len()] = [false; CpuFeatureLevel::len()];

            #[allow(unused_imports)]
            use CpuFeatureLevel::*;
            $(
                out[$key as usize] = $value;
                set[$key as usize] = true;
            )*
            cpu_function_lookup_table!(waterfall_cpu_features(out, set, [SSE2, SSSE3, SSE4_1, AVX2, AVX512, AVX512ICL]));
            out
            };
        };
        ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
            $pub cpu_function_lookup_table!($name: [$type], default: $empty, [$(($key, $value)),*]);
        };

        // Fill empty output functions with the existent functions they support.
        // cpus should be in order of lowest cpu level to highest
        // Used like an internal function
        // Put in here to avoid adding more public macros
        (waterfall_cpu_features($out:ident, $set:ident, [$($cpu:ident),*])) => {
            // Use an array to emulate if statements (not supported in static as of
            // rust 1.40). Setting best[0] (false) and best[1] (true) is equivalent to
            // doing nothing and overriding our value respectively.
            #[allow(unused_assignments)]
            let mut best = [$out[0], $out[0]];
            $(
            // If the current entry has a function, update out best function.
            best[$set[$cpu as usize] as usize] = $out[$cpu as usize];
            // Update our current entry. Does nothing if it already had a function.
            $out[$cpu as usize] = best[1];
            )*
        };

        // use $name_$key as our values
        ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
            pastey::item!{
                cpu_function_lookup_table!(
                    $pub, $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
                );
            }
        };

        // version for default visibility
        ($name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
            pastey::item!{
                cpu_function_lookup_table!(
                    $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
                );
            }
        };
    }
}

#[cfg(asm_neon)]
#[macro_use]
mod neon {
    use std::{env, str::FromStr};

    use arg_enum_proc_macro::ArgEnum;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, ArgEnum)]
    #[allow(clippy::upper_case_acronyms)]
    pub enum CpuFeatureLevel {
        RUST,
        NEON,
    }

    impl CpuFeatureLevel {
        #[cfg(test)]
        #[inline]
        pub const fn all() -> &'static [Self] {
            use CpuFeatureLevel::*;
            &[RUST, NEON]
        }

        #[inline]
        pub const fn len() -> usize {
            CpuFeatureLevel::NEON as usize + 1
        }

        #[inline]
        pub fn as_index(self) -> usize {
            self as usize
        }
    }

    impl Default for CpuFeatureLevel {
        #[inline]
        fn default() -> CpuFeatureLevel {
            let detected = CpuFeatureLevel::NEON;
            let manual: CpuFeatureLevel = match env::var("CPU_TARGET") {
                Ok(feature) => CpuFeatureLevel::from_str(&feature).unwrap_or(detected),
                Err(_e) => detected,
            };
            if manual > detected {
                detected
            } else {
                manual
            }
        }
    }

    // Create a static lookup table for CPUFeatureLevel enums
    // Note: keys are CpuFeatureLevels without any prefix (no CpuFeatureLevel::)
    macro_rules! cpu_function_lookup_table {
        // version for default visibility
        ($name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
            static $name: [$type; crate::cpu::CpuFeatureLevel::len()] = {
            use crate::cpu::CpuFeatureLevel;
            #[allow(unused_mut)]
            let mut out: [$type; CpuFeatureLevel::len()] = [$empty; CpuFeatureLevel::len()];

            // Can't use out[0][.] == $empty in static as of rust 1.40
            #[allow(unused_mut)]
            let mut set: [bool; CpuFeatureLevel::len()] = [false; CpuFeatureLevel::len()];

            #[allow(unused_imports)]
            use CpuFeatureLevel::*;
            $(
                out[$key as usize] = $value;
                set[$key as usize] = true;
            )*
            cpu_function_lookup_table!(waterfall_cpu_features(out, set, [NEON]));
            out
            };
        };

        ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$(($key:ident, $value:expr)),*]) => {
            $pub cpu_function_lookup_table!($name: [$type], default: $empty, [$(($key, $value)),*]);
        };
        // Fill empty output functions with the existent functions they support.
        // cpus should be in order of lowest cpu level to highest
        // Used like an internal function
        // Put in here to avoid adding more public macros
        (waterfall_cpu_features($out:ident, $set:ident, [$($cpu:ident),*])) => {
            // Use an array to emulate if statements (not supported in static as of
            // rust 1.40). Setting best[0] (false) and best[1] (true) is equivalent to
            // doing nothing and overriding our value respectively.
            #[allow(unused_assignments)]
            let mut best = [$out[0], $out[0]];
            $(
            // If the current entry has a function, update out best function.
            best[$set[$cpu as usize] as usize] = $out[$cpu as usize];
            // Update our current entry. Does nothing if it already had a function.
            $out[$cpu as usize] = best[1];
            )*
        };


        // use $name_$key as our values
        ($pub:vis, $name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
            pastey::item!{
            cpu_function_lookup_table!(
                $pub, $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
            );
            }
        };

        // version for default visibility
        ($name:ident: [$type:ty], default: $empty:expr, [$($key:ident),*]) => {
            pastey::item!{
            cpu_function_lookup_table!(
                $name: [$type], default: $empty, [$(($key, [<$name _$key>])),*]
            );
            }
        };
    }
}
