// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use serde::{Deserialize, Serialize};
use zvariant::{OwnedValue, Type, Value};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, OwnedValue, Type, Value,
)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const INVALID: Rect = Rect {
        x: -1,
        y: -1,
        width: -1,
        height: -1,
    };
}

impl From<accesskit::Rect> for Rect {
    fn from(value: accesskit::Rect) -> Rect {
        Rect {
            x: value.x0 as i32,
            y: value.y0 as i32,
            width: value.width() as i32,
            height: value.height() as i32,
        }
    }
}
