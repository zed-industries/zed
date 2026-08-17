// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

#![cfg(feature = "derive")]

use merge::Merge;

fn test<T: std::fmt::Debug + Merge + PartialEq>(expected: T, mut left: T, right: T) {
    left.merge(right);
    assert_eq!(expected, left);
}

#[test]
fn test_bool_overwrite_false() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::bool::overwrite_false)] bool);

    test(S(false), S(false), S(false));
    test(S(true), S(false), S(true));
    test(S(true), S(true), S(false));
    test(S(true), S(true), S(true));
}

#[test]
fn test_bool_overwrite_true() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::bool::overwrite_true)] bool);

    test(S(false), S(false), S(false));
    test(S(false), S(false), S(true));
    test(S(false), S(true), S(false));
    test(S(true), S(true), S(true));
}

#[cfg(feature = "num")]
#[test]
fn test_num_saturating_add() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::num::saturating_add)] u8);

    test(S(0), S(0), S(0));
    test(S(1), S(0), S(1));
    test(S(255), S(255), S(10));
    test(S(40), S(30), S(10));
}

#[cfg(feature = "num")]
#[test]
fn test_num_overwrite_zero() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::num::overwrite_zero)] u8);

    test(S(0), S(0), S(0));
    test(S(1), S(0), S(1));
    test(S(255), S(255), S(10));
}

#[test]
fn test_ord_max() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::ord::max)] u8);

    test(S(2), S(1), S(2));
    test(S(2), S(2), S(1));
    test(S(2), S(2), S(2));
    test(S(2), S(2), S(0));
    test(S(2), S(0), S(2));
    test(S(33), S(33), S(11));
}

#[test]
fn test_ord_min() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::ord::min)] u8);

    test(S(1), S(1), S(2));
    test(S(1), S(2), S(1));
    test(S(2), S(2), S(2));
    test(S(0), S(2), S(0));
    test(S(0), S(0), S(2));
    test(S(11), S(33), S(11));
}

#[cfg(feature = "std")]
#[test]
fn test_vec_overwrite_empty() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::vec::overwrite_empty)] Vec<u8>);

    test(S(vec![]), S(vec![]), S(vec![]));
    test(S(vec![1]), S(vec![]), S(vec![1]));
    test(S(vec![0]), S(vec![0]), S(vec![1]));
    test(S(vec![255]), S(vec![255]), S(vec![10]));
}

#[cfg(feature = "std")]
#[test]
fn test_vec_append() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::vec::append)] Vec<u8>);

    test(S(vec![]), S(vec![]), S(vec![]));
    test(S(vec![1]), S(vec![]), S(vec![1]));
    test(S(vec![0, 1]), S(vec![0]), S(vec![1]));
    test(S(vec![255, 10]), S(vec![255]), S(vec![10]));
    test(S(vec![0, 1, 2, 3, 4]), S(vec![0, 1, 2]), S(vec![3, 4]));
    test(S(vec![3, 4, 0, 1, 2]), S(vec![3, 4]), S(vec![0, 1, 2]));
}

#[cfg(feature = "std")]
#[test]
fn test_vec_prepend() {
    #[derive(Debug, Merge, PartialEq)]
    struct S(#[merge(strategy = merge::vec::prepend)] Vec<u8>);

    test(S(vec![]), S(vec![]), S(vec![]));
    test(S(vec![1]), S(vec![]), S(vec![1]));
    test(S(vec![1, 0]), S(vec![0]), S(vec![1]));
    test(S(vec![10, 255]), S(vec![255]), S(vec![10]));
    test(S(vec![3, 4, 0, 1, 2]), S(vec![0, 1, 2]), S(vec![3, 4]));
    test(S(vec![0, 1, 2, 3, 4]), S(vec![3, 4]), S(vec![0, 1, 2]));
}
