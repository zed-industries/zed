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

//! Macro instantiation tests

#[macro_use]
extern crate approx;

#[test]
fn test_abs_diff_eq() {
    let _: bool = abs_diff_eq!(1.0, 1.0);
    let _: bool = abs_diff_eq!(1.0, 1.0, epsilon = 1.0);
}

#[test]
fn test_abs_diff_eq_trailing_commas() {
    let _: bool = abs_diff_eq!(1.0, 1.0,);
    let _: bool = abs_diff_eq!(1.0, 1.0, epsilon = 1.0,);
}

#[test]
fn test_abs_diff_ne() {
    let _: bool = abs_diff_ne!(1.0, 1.0);
    let _: bool = abs_diff_ne!(1.0, 1.0, epsilon = 1.0);
}

#[test]
fn test_abs_diff_ne_trailing_commas() {
    let _: bool = abs_diff_ne!(1.0, 1.0,);
    let _: bool = abs_diff_ne!(1.0, 1.0, epsilon = 1.0,);
}

#[test]
fn test_relative_eq() {
    let _: bool = relative_eq!(1.0, 1.0);
    let _: bool = relative_eq!(1.0, 1.0, epsilon = 1.0);
    let _: bool = relative_eq!(1.0, 1.0, max_relative = 1.0);
    let _: bool = relative_eq!(1.0, 1.0, epsilon = 1.0, max_relative = 1.0);
}

#[test]
fn test_relative_eq_trailing_commas() {
    let _: bool = relative_eq!(1.0, 1.0,);
    let _: bool = relative_eq!(1.0, 1.0, epsilon = 1.0, max_relative = 1.0,);
}

#[test]
fn test_relative_ne() {
    let _: bool = relative_ne!(1.0, 1.0);
    let _: bool = relative_ne!(1.0, 1.0, epsilon = 1.0);
    let _: bool = relative_ne!(1.0, 1.0, max_relative = 1.0);
    let _: bool = relative_ne!(1.0, 1.0, epsilon = 1.0, max_relative = 1.0);
}

#[test]
fn test_relative_ne_trailing_commas() {
    let _: bool = relative_ne!(1.0, 1.0,);
    let _: bool = relative_ne!(1.0, 1.0, epsilon = 1.0, max_relative = 1.0,);
}

#[test]
fn test_ulps_eq() {
    let _: bool = ulps_eq!(1.0, 1.0);
    let _: bool = ulps_eq!(1.0, 1.0, epsilon = 1.0);
    let _: bool = ulps_eq!(1.0, 1.0, max_ulps = 1);
    let _: bool = ulps_eq!(1.0, 1.0, epsilon = 1.0, max_ulps = 1);
}

#[test]
fn test_ulps_eq_trailing_commas() {
    let _: bool = ulps_eq!(1.0, 1.0,);
    let _: bool = ulps_eq!(1.0, 1.0, epsilon = 1.0, max_ulps = 1,);
}

#[test]
fn test_ulps_ne() {
    let _: bool = ulps_ne!(1.0, 1.0);
    let _: bool = ulps_ne!(1.0, 1.0, epsilon = 1.0);
    let _: bool = ulps_ne!(1.0, 1.0, max_ulps = 1);
    let _: bool = ulps_ne!(1.0, 1.0, epsilon = 1.0, max_ulps = 1);
}

#[test]
fn test_ulps_ne_trailing_commas() {
    let _: bool = ulps_ne!(1.0, 1.0,);
    let _: bool = ulps_ne!(1.0, 1.0, epsilon = 1.0, max_ulps = 1,);
}
