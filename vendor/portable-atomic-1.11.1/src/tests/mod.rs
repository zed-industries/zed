// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_imports
)]

#[macro_use]
pub(crate) mod helper;

#[allow(dead_code)]
#[path = "../../version.rs"]
mod version;

use test_helper as _;

use super::*;

test_atomic_bool_pub!();
test_atomic_ptr_pub!();

test_atomic_int_pub!(isize);
test_atomic_int_pub!(usize);
test_atomic_int_pub!(i8);
test_atomic_int_pub!(u8);
test_atomic_int_pub!(i16);
test_atomic_int_pub!(u16);
test_atomic_int_pub!(i32);
test_atomic_int_pub!(u32);
test_atomic_int_pub!(i64);
test_atomic_int_pub!(u64);
test_atomic_int_pub!(i128);
test_atomic_int_pub!(u128);

#[cfg(all(feature = "float", portable_atomic_unstable_f16))]
test_atomic_float_pub!(f16);
#[cfg(feature = "float")]
test_atomic_float_pub!(f32);
#[cfg(feature = "float")]
test_atomic_float_pub!(f64);
#[cfg(all(feature = "float", portable_atomic_unstable_f128))]
test_atomic_float_pub!(f128);

#[deny(improper_ctypes)]
extern "C" {
    fn _atomic_bool_ffi_safety(_: AtomicBool);
    fn _atomic_ptr_ffi_safety(_: AtomicPtr<u8>);
    fn _atomic_isize_ffi_safety(_: AtomicIsize);
    fn _atomic_usize_ffi_safety(_: AtomicUsize);
    fn _atomic_i8_ffi_safety(_: AtomicI8);
    fn _atomic_u8_ffi_safety(_: AtomicU8);
    fn _atomic_i16_ffi_safety(_: AtomicI16);
    fn _atomic_u16_ffi_safety(_: AtomicU16);
    fn _atomic_i32_ffi_safety(_: AtomicI32);
    fn _atomic_u32_ffi_safety(_: AtomicU32);
    fn _atomic_i64_ffi_safety(_: AtomicI64);
    fn _atomic_u64_ffi_safety(_: AtomicU64);
    // TODO: https://github.com/rust-lang/lang-team/issues/255
    // fn _atomic_i128_ffi_safety(_: AtomicI128);
    // fn _atomic_u128_ffi_safety(_: AtomicU128);
    #[cfg(all(feature = "float", portable_atomic_unstable_f16))]
    fn _atomic_f16_ffi_safety(_: AtomicF16);
    #[cfg(feature = "float")]
    fn _atomic_f32_ffi_safety(_: AtomicF32);
    #[cfg(feature = "float")]
    fn _atomic_f64_ffi_safety(_: AtomicF64);
    #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
    fn _atomic_f128_ffi_safety(_: AtomicF128);
}

#[test]
fn test_is_lock_free() {
    assert!(AtomicI8::is_always_lock_free());
    assert!(AtomicI8::is_lock_free());
    assert!(AtomicU8::is_always_lock_free());
    assert!(AtomicU8::is_lock_free());
    assert!(AtomicI16::is_always_lock_free());
    assert!(AtomicI16::is_lock_free());
    assert!(AtomicU16::is_always_lock_free());
    assert!(AtomicU16::is_lock_free());
    #[cfg(all(feature = "float", portable_atomic_unstable_f16))]
    assert!(AtomicF16::is_always_lock_free());
    #[cfg(all(feature = "float", portable_atomic_unstable_f16))]
    assert!(AtomicF16::is_lock_free());
    assert!(AtomicI32::is_always_lock_free());
    assert!(AtomicI32::is_lock_free());
    assert!(AtomicU32::is_always_lock_free());
    assert!(AtomicU32::is_lock_free());
    #[cfg(feature = "float")]
    assert!(AtomicF32::is_always_lock_free());
    #[cfg(feature = "float")]
    assert!(AtomicF32::is_lock_free());
    #[cfg(not(portable_atomic_no_cfg_target_has_atomic))]
    {
        if cfg!(any(
            target_has_atomic = "64",
            all(
                target_arch = "riscv32",
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
                any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
            ),
        )) {
            assert!(AtomicI64::is_always_lock_free());
            assert!(AtomicI64::is_lock_free());
            assert!(AtomicU64::is_always_lock_free());
            assert!(AtomicU64::is_lock_free());
            #[cfg(feature = "float")]
            assert!(AtomicF64::is_always_lock_free());
            #[cfg(feature = "float")]
            assert!(AtomicF64::is_lock_free());
        } else if cfg!(all(
            feature = "fallback",
            target_arch = "arm",
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            any(target_os = "linux", target_os = "android"),
            not(any(target_feature = "v6", portable_atomic_target_feature = "v6")),
            not(portable_atomic_no_outline_atomics),
            not(target_has_atomic = "64"),
            not(portable_atomic_test_detect_false),
        )) {
            assert!(!AtomicI64::is_always_lock_free());
            assert!(!AtomicU64::is_always_lock_free());
            #[cfg(feature = "float")]
            assert!(!AtomicF64::is_always_lock_free());
            assert!(AtomicI64::is_lock_free());
            assert!(AtomicU64::is_lock_free());
            #[cfg(feature = "float")]
            assert!(AtomicF64::is_lock_free());
        } else {
            assert!(!AtomicI64::is_always_lock_free());
            assert!(!AtomicU64::is_always_lock_free());
            #[cfg(feature = "float")]
            assert!(!AtomicF64::is_always_lock_free());
            #[cfg(not(target_arch = "riscv32"))]
            {
                assert!(!AtomicI64::is_lock_free());
                assert!(!AtomicU64::is_lock_free());
                #[cfg(feature = "float")]
                assert!(!AtomicF64::is_lock_free());
            }
            #[cfg(target_arch = "riscv32")]
            {
                // TODO(riscv): check detect.has_zacas
            }
        }
    }
    if cfg!(portable_atomic_no_asm) && cfg!(not(portable_atomic_unstable_asm)) {
        assert!(!AtomicI128::is_always_lock_free());
        assert!(!AtomicI128::is_lock_free());
        assert!(!AtomicU128::is_always_lock_free());
        assert!(!AtomicU128::is_lock_free());
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
        assert!(!AtomicF128::is_always_lock_free());
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
        assert!(!AtomicF128::is_lock_free());
    } else if cfg!(any(
        all(
            target_arch = "aarch64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
        ),
        all(
            target_arch = "arm64ec",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
        all(
            target_arch = "x86_64",
            any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        ),
        all(
            target_arch = "riscv64",
            any(target_feature = "zacas", portable_atomic_target_feature = "zacas"),
        ),
        all(
            target_arch = "powerpc64",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            portable_atomic_unstable_asm_experimental_arch,
            any(
                target_feature = "quadword-atomics",
                portable_atomic_target_feature = "quadword-atomics",
            ),
        ),
        all(
            target_arch = "s390x",
            not(all(
                any(miri, portable_atomic_sanitize_thread),
                not(portable_atomic_atomic_intrinsics),
            )),
            not(portable_atomic_no_asm),
        ),
    )) {
        assert!(AtomicI128::is_always_lock_free());
        assert!(AtomicI128::is_lock_free());
        assert!(AtomicU128::is_always_lock_free());
        assert!(AtomicU128::is_lock_free());
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
        assert!(AtomicF128::is_always_lock_free());
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
        assert!(AtomicF128::is_lock_free());
    } else {
        assert!(!AtomicI128::is_always_lock_free());
        assert!(!AtomicU128::is_always_lock_free());
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
        assert!(!AtomicF128::is_always_lock_free());
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "powerpc64",
            target_arch = "riscv64",
        )))]
        {
            assert!(!AtomicI128::is_lock_free());
            assert!(!AtomicU128::is_lock_free());
            #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
            assert!(!AtomicF128::is_lock_free());
        }
        #[cfg(target_arch = "x86_64")]
        {
            let has_cmpxchg16b = cfg!(all(
                feature = "fallback",
                not(portable_atomic_no_outline_atomics),
                not(any(target_env = "sgx", miri)),
                not(portable_atomic_test_detect_false),
            )) && std::is_x86_feature_detected!("cmpxchg16b");
            assert_eq!(AtomicI128::is_lock_free(), has_cmpxchg16b);
            assert_eq!(AtomicU128::is_lock_free(), has_cmpxchg16b);
            #[cfg(all(feature = "float", portable_atomic_unstable_f128))]
            assert_eq!(AtomicF128::is_lock_free(), has_cmpxchg16b);
        }
        #[cfg(target_arch = "powerpc64")]
        {
            // TODO(powerpc64): is_powerpc_feature_detected is unstable
        }
        #[cfg(target_arch = "riscv64")]
        {
            // TODO(riscv): check detect.has_zacas
        }
    }
}

// test version parsing code used in the build script.
#[test]
fn test_rustc_version() {
    use self::version::Version;

    // rustc 1.34 (rustup)
    let v = Version::parse(
        "rustc 1.34.2 (6c2484dc3 2019-05-13)
binary: rustc
commit-hash: 6c2484dc3c532c052f159264e970278d8b77cdc9
commit-date: 2019-05-13
host: x86_64-apple-darwin
release: 1.34.2
LLVM version: 8.0",
    )
    .unwrap();
    assert_eq!(v, Version::stable(34, 8));

    // rustc 1.50 (rustup)
    let v = Version::parse(
        "rustc 1.50.0 (cb75ad5db 2021-02-10)
binary: rustc
commit-hash: cb75ad5db02783e8b0222fee363c5f63f7e2cf5b
commit-date: 2021-02-10
host: aarch64-unknown-linux-gnu
release: 1.50.0",
    )
    .unwrap();
    assert_eq!(v, Version::stable(50, 0));

    // rustc 1.67 (rustup)
    let v = Version::parse(
        "rustc 1.67.0 (fc594f156 2023-01-24)
binary: rustc
commit-hash: fc594f15669680fa70d255faec3ca3fb507c3405
commit-date: 2023-01-24
host: aarch64-apple-darwin
release: 1.67.0
LLVM version: 15.0.6",
    )
    .unwrap();
    assert_eq!(v, Version::stable(67, 15));

    // rustc 1.68-beta (rustup)
    let v = Version::parse(
        "rustc 1.68.0-beta.2 (10b73bf73 2023-02-01)
binary: rustc
commit-hash: 10b73bf73a6b770cd92ad8ff538173bc3298411c
commit-date: 2023-02-01
host: aarch64-apple-darwin
release: 1.68.0-beta.2
LLVM version: 15.0.6",
    )
    .unwrap();
    // We do not distinguish between stable and beta because we are only
    // interested in whether unstable features are potentially available.
    assert_eq!(v, Version::stable(68, 15));

    // rustc nightly-2019-01-27 (rustup)
    let v = Version::parse(
        "rustc 1.33.0-nightly (20c2cba61 2019-01-26)
binary: rustc
commit-hash: 20c2cba61dc83e612d25ed496025171caa3db30f
commit-date: 2019-01-26
host: x86_64-apple-darwin
release: 1.33.0-nightly
LLVM version: 8.0",
    )
    .unwrap();
    assert_eq!(v.minor, 33);
    assert!(v.nightly);
    assert_eq!(v.llvm, 8);
    assert_eq!(v.commit_date().year, 2019);
    assert_eq!(v.commit_date().month, 1);
    assert_eq!(v.commit_date().day, 26);

    // rustc 1.69-nightly (rustup)
    let v = Version::parse(
        "rustc 1.69.0-nightly (bd39bbb4b 2023-02-07)
binary: rustc
commit-hash: bd39bbb4bb92df439bf6d85470e296cc6a47ffbd
commit-date: 2023-02-07
host: aarch64-apple-darwin
release: 1.69.0-nightly
LLVM version: 15.0.7",
    )
    .unwrap();
    assert_eq!(v.minor, 69);
    assert!(v.nightly);
    assert_eq!(v.llvm, 15);
    assert_eq!(v.commit_date().year, 2023);
    assert_eq!(v.commit_date().month, 2);
    assert_eq!(v.commit_date().day, 7);

    // clippy-driver 1.69-nightly (rustup)
    let v = Version::parse(
        "rustc 1.69.0-nightly (bd39bbb4b 2023-02-07)
binary: rustc
commit-hash: bd39bbb4bb92df439bf6d85470e296cc6a47ffbd
commit-date: 2023-02-07
host: aarch64-apple-darwin
release: 1.69.0-nightly
LLVM version: 15.0.7",
    )
    .unwrap();
    assert_eq!(v.minor, 69);
    assert!(v.nightly);
    assert_eq!(v.llvm, 15);
    assert_eq!(v.commit_date().year, 2023);
    assert_eq!(v.commit_date().month, 2);
    assert_eq!(v.commit_date().day, 7);

    // rustc 1.69-dev (from source: ./x.py build)
    let v = Version::parse(
        "rustc 1.69.0-dev
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-unknown-linux-gnu
release: 1.69.0-dev
LLVM version: 16.0.0",
    )
    .unwrap();
    assert_eq!(v.minor, 69);
    assert!(v.nightly);
    assert_eq!(v.llvm, 16);
    assert_eq!(v.commit_date().year, 0);
    assert_eq!(v.commit_date().month, 0);
    assert_eq!(v.commit_date().day, 0);

    // rustc 1.48 (debian 11: apt-get install cargo)
    let v = Version::parse(
        "rustc 1.48.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-unknown-linux-gnu
release: 1.48.0
LLVM version: 11.0",
    )
    .unwrap();
    assert_eq!(v, Version::stable(48, 11));

    // rustc 1.67 (fedora: dnf install cargo)
    let v = Version::parse(
        "rustc 1.67.0 (fc594f156 2023-01-24) (Fedora 1.67.0-2.fc37)
binary: rustc
commit-hash: fc594f15669680fa70d255faec3ca3fb507c3405
commit-date: 2023-01-24
host: aarch64-unknown-linux-gnu
release: 1.67.0
LLVM version: 15.0.7",
    )
    .unwrap();
    assert_eq!(v, Version::stable(67, 15));

    // rustc 1.64 (alpine: apk add cargo)
    let v = Version::parse(
        "rustc 1.64.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-alpine-linux-musl
release: 1.64.0
LLVM version: 15.0.3",
    )
    .unwrap();
    assert_eq!(v, Version::stable(64, 15));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    use std::fmt;

    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };
    use serde_test::{Token, assert_tokens};

    #[derive(Debug)]
    struct DebugPartialEq<T>(T);
    impl<T: fmt::Debug> PartialEq for DebugPartialEq<T> {
        fn eq(&self, other: &Self) -> bool {
            std::format!("{:?}", self) == std::format!("{:?}", other)
        }
    }
    impl<T: Serialize> Serialize for DebugPartialEq<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }
    impl<'de, T: Deserialize<'de>> Deserialize<'de> for DebugPartialEq<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(Self)
        }
    }

    macro_rules! t {
        ($atomic_type:ty, $value_type:ident $(as $token_value_type:ident)?, $token_type:ident) => {
            std::eprint!("test_serde {} ... ", stringify!($value_type));
            assert_tokens(&DebugPartialEq(<$atomic_type>::new($value_type::MAX)), &[
                Token::$token_type($value_type::MAX $(as $token_value_type)?),
            ]);
            assert_tokens(&DebugPartialEq(<$atomic_type>::new($value_type::MIN)), &[
                Token::$token_type($value_type::MIN $(as $token_value_type)?),
            ]);
            std::eprintln!("ok");
        };
    }

    assert_tokens(&DebugPartialEq(AtomicBool::new(true)), &[Token::Bool(true)]);
    assert_tokens(&DebugPartialEq(AtomicBool::new(false)), &[Token::Bool(false)]);
    t!(AtomicIsize, isize as i64, I64);
    t!(AtomicUsize, usize as u64, U64);
    t!(AtomicI8, i8, I8);
    t!(AtomicU8, u8, U8);
    t!(AtomicI16, i16, I16);
    t!(AtomicU16, u16, U16);
    t!(AtomicI32, i32, I32);
    t!(AtomicU32, u32, U32);
    t!(AtomicI64, i64, I64);
    t!(AtomicU64, u64, U64);
    t!(AtomicI128, i128, I128);
    t!(AtomicU128, u128, U128);
    // TODO(f16_and_f128): Test f16 & f128 once stabilized.
    #[cfg(feature = "float")]
    t!(AtomicF32, f32, F32);
    #[cfg(feature = "float")]
    // TODO: fixed in LLVM 18?
    #[cfg(not(target_arch = "mips"))] // LLVM 17 (nightly-2023-08-09) bug: assertion failed at core/src/num/diy_float.rs:78:9
    t!(AtomicF64, f64, F64);
}
