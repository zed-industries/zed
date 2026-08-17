// Copyright 2025 FastLabs Developers
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

use std::fmt::Display;

#[stacksafe::stacksafe]
fn sum(nums: &[u64]) -> u64 {
    if let Some((head, tail)) = nums.split_first() {
        head + sum(tail)
    } else {
        0
    }
}

#[stacksafe::stacksafe]
fn dyn_ret<T, U>(b: bool, x: T, y: U) -> Box<dyn Display>
where
    T: Display + 'static,
    U: Display + 'static,
{
    if b { Box::new(x) } else { Box::new(y) }
}

#[stacksafe::stacksafe]
fn impl_ret<T>(b: bool, x: T, y: T) -> impl Display
where T: Display {
    if b { Box::new(x) } else { Box::new(y) }
}

#[stacksafe::stacksafe]
fn no_ret(x: &mut u32) {
    *x *= 10;
}

#[stacksafe::stacksafe]
fn mut_arg(mut x: u32) -> u32 {
    x *= 10;
    x
}

#[test]
fn test_sum() {
    let n = 10_000_000;
    let v: Vec<u64> = (0..n).collect();
    assert_eq!(sum(&v), 49999995000000);
}

#[test]
fn test_dyn_ret() {
    assert_eq!("10", format!("{}", dyn_ret(true, 10, "20")));
    assert_eq!("20", format!("{}", dyn_ret(false, 10, "20")));
}

#[test]
fn test_impl_ret() {
    assert_eq!("10", format!("{}", impl_ret(true, 10, 20)));
    assert_eq!("20", format!("{}", impl_ret(false, 10, 20)));
}

#[test]
fn test_mut_arg() {
    assert_eq!(100, mut_arg(10));
}

#[test]
fn test_no_ret() {
    let mut x = 42;
    no_ret(&mut x);
    assert_eq!(x, 420);
}
