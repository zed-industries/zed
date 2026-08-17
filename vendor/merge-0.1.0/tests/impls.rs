// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

use merge::Merge;

fn test<T: std::fmt::Debug + Merge + PartialEq>(expected: T, mut left: T, right: T) {
    left.merge(right);
    assert_eq!(expected, left);
}

#[test]
fn test_option() {
    test(Some(1), Some(1), Some(2));
    test(Some(2), None, Some(2));
    test(None::<usize>, None, None);
}
