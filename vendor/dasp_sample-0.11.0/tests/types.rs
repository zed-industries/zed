/// Expands to a unique module with a variety of tests for the given sample newtype.
///
/// Tests include basic operations and over/underflow checks.
macro_rules! test_type {
    ($T:ident, $mod_name:ident) => {
        mod $mod_name {
            #[test]
            fn ops() {
                use dasp_sample::types::$mod_name::$T;
                assert_eq!(
                    $T::new(8).unwrap() + $T::new(12).unwrap(),
                    $T::new(20).unwrap()
                );
                assert_eq!(
                    $T::new(12).unwrap() - $T::new(4).unwrap(),
                    $T::new(8).unwrap()
                );
                assert_eq!(
                    $T::new(2).unwrap() * $T::new(2).unwrap(),
                    $T::new(4).unwrap()
                );
                assert_eq!(
                    $T::new(3).unwrap() * $T::new(3).unwrap(),
                    $T::new(9).unwrap()
                );
                assert_eq!(
                    $T::new(5).unwrap() * $T::new(10).unwrap(),
                    $T::new(50).unwrap()
                );
                assert_eq!(
                    $T::new(16).unwrap() / $T::new(8).unwrap(),
                    $T::new(2).unwrap()
                );
                assert_eq!(
                    $T::new(8).unwrap() % $T::new(3).unwrap(),
                    $T::new(2).unwrap()
                );
            }

            #[cfg(debug_assertions)]
            #[test]
            #[should_panic]
            fn add_panic_debug() {
                use dasp_sample::types::$mod_name::{self, $T};
                let _ = $mod_name::MAX + $T::new(1).unwrap();
            }

            #[cfg(debug_assertions)]
            #[test]
            #[should_panic]
            fn sub_panic_debug() {
                use dasp_sample::types::$mod_name::{self, $T};
                let _ = $mod_name::MIN - $T::new(1).unwrap();
            }

            #[cfg(debug_assertions)]
            #[test]
            #[should_panic]
            fn mul_panic_debug() {
                use dasp_sample::types::$mod_name::{self, $T};
                let _ = $mod_name::MAX * $T::new(2).unwrap();
            }

            #[cfg(not(debug_assertions))]
            #[test]
            fn release_wrapping() {
                use dasp_sample::types::$mod_name::{self, $T};
                assert_eq!($mod_name::MIN - $T::new(1).unwrap(), $mod_name::MAX);
                assert_eq!($mod_name::MAX + $T::new(1).unwrap(), $mod_name::MIN);
            }
        }
    };
}

test_type!(I11, i11);
test_type!(U11, u11);
test_type!(I20, i20);
test_type!(U20, u20);
test_type!(I24, i24);
test_type!(U24, u24);
test_type!(I48, i48);
test_type!(U48, u48);
