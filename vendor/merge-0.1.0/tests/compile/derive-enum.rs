// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

use merge::Merge;

#[derive(Merge)]
enum E {
    V1,
    V2,
    V3,
}

fn main() {}
