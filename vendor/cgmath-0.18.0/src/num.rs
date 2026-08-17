// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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

use approx;

use std::fmt;
use std::ops::*;

use num_traits::{Float, Num, NumCast};

/// Base numeric types with partial ordering
pub trait BaseNum:
    Copy
    + Clone
    + fmt::Debug
    + Num
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
}

impl<T> BaseNum for T where
    T: Copy
        + Clone
        + fmt::Debug
        + Num
        + NumCast
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
{
}

/// Base floating point types
pub trait BaseFloat:
    BaseNum
    + Float
    + approx::AbsDiffEq<Epsilon = Self>
    + approx::RelativeEq<Epsilon = Self>
    + approx::UlpsEq<Epsilon = Self>
{
}

impl<T> BaseFloat for T where
    T: BaseNum
        + Float
        + approx::AbsDiffEq<Epsilon = Self>
        + approx::RelativeEq<Epsilon = Self>
        + approx::UlpsEq<Epsilon = Self>
{
}
