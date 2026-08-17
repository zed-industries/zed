// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use core::convert::TryFrom;

use tiny_skia_path::{IntRect, IntSize, Rect};

use crate::LengthU32;

/// A screen `IntRect`.
///
/// # Guarantees
///
/// - X and Y are in 0..=i32::MAX range.
/// - Width and height are in 1..=i32::MAX range.
/// - x+width and y+height does not overflow.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ScreenIntRect {
    x: u32,
    y: u32,
    width: LengthU32,
    height: LengthU32,
}

impl ScreenIntRect {
    /// Creates a new `ScreenIntRect`.
    pub fn from_xywh(x: u32, y: u32, width: u32, height: u32) -> Option<Self> {
        i32::try_from(x).ok()?;
        i32::try_from(y).ok()?;
        i32::try_from(width).ok()?;
        i32::try_from(height).ok()?;

        x.checked_add(width)?;
        y.checked_add(height)?;

        let width = LengthU32::new(width)?;
        let height = LengthU32::new(height)?;

        Some(ScreenIntRect {
            x,
            y,
            width,
            height,
        })
    }

    /// Creates a new `ScreenIntRect`.
    pub const fn from_xywh_safe(x: u32, y: u32, width: LengthU32, height: LengthU32) -> Self {
        ScreenIntRect {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns rect's X position.
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Returns rect's Y position.
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Returns rect's width.
    pub fn width(&self) -> u32 {
        self.width.get()
    }

    /// Returns rect's height.
    pub fn height(&self) -> u32 {
        self.height.get()
    }

    /// Returns rect's width.
    pub fn width_safe(&self) -> LengthU32 {
        self.width
    }

    /// Returns rect's left edge.
    pub fn left(&self) -> u32 {
        self.x
    }

    /// Returns rect's top edge.
    pub fn top(&self) -> u32 {
        self.y
    }

    /// Returns rect's right edge.
    ///
    /// The right edge is at least 1.
    pub fn right(&self) -> u32 {
        // No overflow is guaranteed by constructors.
        self.x + self.width.get()
    }

    /// Returns rect's bottom edge.
    ///
    /// The bottom edge is at least 1.
    pub fn bottom(&self) -> u32 {
        // No overflow is guaranteed by constructors.
        self.y + self.height.get()
    }

    /// Returns rect's size.
    pub fn size(&self) -> IntSize {
        IntSize::from_wh(self.width(), self.height()).unwrap()
    }

    /// Checks that the rect is completely includes `other` Rect.
    pub fn contains(&self, other: &Self) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.right() >= other.right()
            && self.bottom() >= other.bottom()
    }

    /// Converts into a `IntRect`.
    pub fn to_int_rect(&self) -> IntRect {
        // Everything is already checked by constructors.
        IntRect::from_xywh(
            self.x as i32,
            self.y as i32,
            self.width.get(),
            self.height.get(),
        )
        .unwrap()
    }

    /// Converts into a `Rect`.
    pub fn to_rect(&self) -> Rect {
        // Can't fail, because `ScreenIntRect` is always valid.
        // And u32 always fits into f32.
        Rect::from_ltrb(
            self.x as f32,
            self.y as f32,
            self.x as f32 + self.width.get() as f32,
            self.y as f32 + self.height.get() as f32,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod screen_int_rect_tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(ScreenIntRect::from_xywh(0, 0, 0, 0), None);
        assert_eq!(ScreenIntRect::from_xywh(0, 0, 1, 0), None);
        assert_eq!(ScreenIntRect::from_xywh(0, 0, 0, 1), None);

        assert_eq!(ScreenIntRect::from_xywh(0, 0, u32::MAX, u32::MAX), None);
        assert_eq!(ScreenIntRect::from_xywh(0, 0, 1, u32::MAX), None);
        assert_eq!(ScreenIntRect::from_xywh(0, 0, u32::MAX, 1), None);

        assert_eq!(ScreenIntRect::from_xywh(u32::MAX, 0, 1, 1), None);
        assert_eq!(ScreenIntRect::from_xywh(0, u32::MAX, 1, 1), None);

        assert_eq!(
            ScreenIntRect::from_xywh(u32::MAX, u32::MAX, u32::MAX, u32::MAX),
            None
        );

        let r = ScreenIntRect::from_xywh(1, 2, 3, 4).unwrap();
        assert_eq!(r.x(), 1);
        assert_eq!(r.y(), 2);
        assert_eq!(r.width(), 3);
        assert_eq!(r.height(), 4);
        assert_eq!(r.right(), 4);
        assert_eq!(r.bottom(), 6);
    }
}

pub trait IntSizeExt {
    /// Converts the current size into a `IntRect` at a provided position.
    fn to_screen_int_rect(&self, x: u32, y: u32) -> ScreenIntRect;
}

impl IntSizeExt for IntSize {
    fn to_screen_int_rect(&self, x: u32, y: u32) -> ScreenIntRect {
        ScreenIntRect::from_xywh(x, y, self.width(), self.height()).unwrap()
    }
}

pub trait IntRectExt {
    /// Converts into `ScreenIntRect`.
    ///
    /// # Checks
    ///
    /// - x >= 0
    /// - y >= 0
    fn to_screen_int_rect(&self) -> Option<ScreenIntRect>;
}

impl IntRectExt for IntRect {
    fn to_screen_int_rect(&self) -> Option<ScreenIntRect> {
        let x = u32::try_from(self.x()).ok()?;
        let y = u32::try_from(self.y()).ok()?;
        ScreenIntRect::from_xywh(x, y, self.width(), self.height())
    }
}
