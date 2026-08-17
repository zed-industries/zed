// Copyright 2015 Brendan Zabarauskas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Approximate equality of using the absolute difference.
#[macro_export]
macro_rules! abs_diff_eq {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::AbsDiff::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::AbsDiff::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
}

/// Approximate inequality of using the absolute difference.
#[macro_export]
macro_rules! abs_diff_ne {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::AbsDiff::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::AbsDiff::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
}

/// Approximate equality using both the absolute difference and relative based comparisons.
#[macro_export]
macro_rules! relative_eq {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::Relative::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::Relative::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
}

/// Approximate inequality using both the absolute difference and relative based comparisons.
#[macro_export]
macro_rules! relative_ne {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::Relative::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::Relative::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
}

/// Approximate equality using both the absolute difference and ULPs (Units in Last Place).
#[macro_export]
macro_rules! ulps_eq {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::Ulps::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::Ulps::default()$(.$opt($val))*.eq(&$lhs, &$rhs)
    };
}

/// Approximate inequality using both the absolute difference and ULPs (Units in Last Place).
#[macro_export]
macro_rules! ulps_ne {
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*) => {
        $crate::Ulps::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
    ($lhs:expr, $rhs:expr $(, $opt:ident = $val:expr)*,) => {
        $crate::Ulps::default()$(.$opt($val))*.ne(&$lhs, &$rhs)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_approx {
    ($eq:ident, $given:expr, $expected:expr) => {{
        match (&($given), &($expected)) {
            (given, expected) => assert!(
                $eq!(*given, *expected),
"assert_{}!({}, {})

    left  = {:?}
    right = {:?}

",
                stringify!($eq),
                stringify!($given),
                stringify!($expected),
                given, expected,
            ),
        }
    }};
    ($eq:ident, $given:expr, $expected:expr, $($opt:ident = $val:expr),+) => {{
        match (&($given), &($expected)) {
            (given, expected) => assert!(
                $eq!(*given, *expected, $($opt = $val),+),
"assert_{}!({}, {}, {})

    left  = {:?}
    right = {:?}

",
                stringify!($eq),
                stringify!($given),
                stringify!($expected),
                stringify!($($opt = $val),+),
                given, expected,
            ),
        }
    }};
}

/// An assertion that delegates to [`abs_diff_eq!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_abs_diff_eq {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(abs_diff_eq, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(abs_diff_eq, $given, $expected $(, $opt = $val)*)
    };
}

/// An assertion that delegates to [`abs_diff_ne!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_abs_diff_ne {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(abs_diff_ne, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(abs_diff_ne, $given, $expected $(, $opt = $val)*)
    };
}

/// An assertion that delegates to [`relative_eq!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_relative_eq {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(relative_eq, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(relative_eq, $given, $expected $(, $opt = $val)*)
    };
}

/// An assertion that delegates to [`relative_ne!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_relative_ne {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(relative_ne, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(relative_ne, $given, $expected $(, $opt = $val)*)
    };
}

/// An assertion that delegates to [`ulps_eq!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_ulps_eq {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(ulps_eq, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(ulps_eq, $given, $expected $(, $opt = $val)*)
    };
}

/// An assertion that delegates to [`ulps_ne!`], and panics with a helpful error on failure.
#[macro_export(local_inner_macros)]
macro_rules! assert_ulps_ne {
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*) => {
        __assert_approx!(ulps_ne, $given, $expected $(, $opt = $val)*)
    };
    ($given:expr, $expected:expr $(, $opt:ident = $val:expr)*,) => {
        __assert_approx!(ulps_ne, $given, $expected $(, $opt = $val)*)
    };
}
