// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

use merge::Merge;

#[derive(Merge)]
struct S {
    #[merge(ignore)]
    field1: Option<u8>,
}

fn main() {}
