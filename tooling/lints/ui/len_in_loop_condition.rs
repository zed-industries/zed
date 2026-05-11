// Tests for the `len_in_loop_condition` lint.

#![allow(unused)]

use std::collections::BTreeMap;

fn main() {
    // --- Should warn ---

    // Range upper bound calls .len() on a Vec.
    let v = vec![1, 2, 3];
    for i in 0..v.len() {
        let _ = v[i];
    }

    // Range upper bound calls .len() on a slice.
    let s: &[u8] = &[1, 2, 3];
    for i in 0..s.len() {
        let _ = s[i];
    }

    // Inclusive range upper bound calls .len().
    let v = vec![10, 20];
    for i in 0..=v.len() {
        let _ = i;
    }

    // Range starting at non-zero.
    let v = vec![1, 2, 3, 4, 5];
    for i in 2..v.len() {
        let _ = v[i];
    }

    // While loop with .len() in condition.
    let v = vec![1, 2, 3];
    let mut i = 0;
    while i < v.len() {
        i += 1;
    }

    // While loop with .len() on the left side of a comparison.
    let v = vec![1, 2, 3];
    let mut i = 0;
    while v.len() > i {
        i += 1;
    }

    // BTreeMap .len() in a for-loop bound.
    let map: BTreeMap<i32, i32> = BTreeMap::new();
    for i in 0..map.len() {
        let _ = i;
    }

    // --- Should NOT warn ---

    // .len() hoisted into a local before the loop.
    let v = vec![1, 2, 3];
    let len = v.len();
    for i in 0..len {
        let _ = v[i];
    }

    // Iterator-based loop — no range with .len().
    let v = vec![1, 2, 3];
    for item in &v {
        let _ = item;
    }

    // Range with a numeric upper bound.
    for i in 0..10 {
        let _ = i;
    }

    // While loop without .len() in the condition.
    let mut i = 0;
    while i < 10 {
        i += 1;
    }

    // .len() used outside a loop — not flagged.
    let v = vec![1, 2, 3];
    let _ = v.len();

    // .len() in a loop body but not in the condition.
    let v = vec![1, 2, 3];
    for i in 0..3 {
        let _ = v.len();
    }
}
