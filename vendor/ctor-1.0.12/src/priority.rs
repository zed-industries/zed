//! Utility macro for converting priority values to literals.

/// Make a macro that zero-pads priority values to 3 digits.
macro_rules! __make_priority_literal {
    ($dollar:tt $(
        ($prefix:literal $( $priority:tt )*)
    )*) => {
        /// Convert a priority value to a literal if 0 < N < 1000, otherwise pass through the value.
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __priority_to_literal {
            $(
                $(
                    ($dollar n:path, $dollar a:tt = $priority ) => {
                        $n!($a, (concat!(stringify!($prefix), stringify!($priority))));
                    };
                )*
            )*

            ($n:path, $a:tt=$dollar priority:tt) => {
                $n!($a, ($dollar priority));
            };
        }
    };
}

#[cfg(target_os = "aix")]
pub(crate) mod aix {
    macro_rules! __make_aix_priority_literal {
        ( $dollar:tt $($priority:tt $d0:tt $d1:tt $d2:tt)* ) => {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! __priority_to_literal {
                $(
                    ($dollar n:path, $dollar a:tt = $priority ) => {
                        $n!($a, (concat!("80000", $d0, $d1, $d2)));
                    };
                )*

                ($n:path, $a:tt=$dollar priority:tt) => {
                    $n!($a, ($dollar priority));
                };
            }
        }
    }

    pub(crate) use __make_aix_priority_literal;
}

#[cfg(not(target_os = "aix"))]
__make_priority_literal! { $
    (00 0 1 2 3 4 5 6 7 8 9)
    (0 10 11 12 13 14 15 16 17 18 19)
    (0 20 21 22 23 24 25 26 27 28 29)
    (0 30 31 32 33 34 35 36 37 38 39)
    (0 40 41 42 43 44 45 46 47 48 49)
    (0 50 51 52 53 54 55 56 57 58 59)
    (0 60 61 62 63 64 65 66 67 68 69)
    (0 70 71 72 73 74 75 76 77 78 79)
    (0 80 81 82 83 84 85 86 87 88 89)
    (0 90 91 92 93 94 95 96 97 98 99)
}
