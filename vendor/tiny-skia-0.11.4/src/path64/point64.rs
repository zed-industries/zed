// Copyright 2012 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SearchAxis {
    X,
    Y,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Point64 {
    pub x: f64,
    pub y: f64,
}

impl Point64 {
    pub fn from_xy(x: f64, y: f64) -> Self {
        Point64 { x, y }
    }

    pub fn from_point(p: Point) -> Self {
        Point64 {
            x: f64::from(p.x),
            y: f64::from(p.y),
        }
    }

    pub fn zero() -> Self {
        Point64 { x: 0.0, y: 0.0 }
    }

    pub fn to_point(&self) -> Point {
        Point::from_xy(self.x as f32, self.y as f32)
    }

    pub fn axis_coord(&self, axis: SearchAxis) -> f64 {
        match axis {
            SearchAxis::X => self.x,
            SearchAxis::Y => self.y,
        }
    }
}
