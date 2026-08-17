use fixedbitset::*;

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::*;
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

extern crate alloc;

const BITS: usize = core::mem::size_of::<Block>() * 8;

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn it_works() {
    const N: usize = 50;
    let mut fb = FixedBitSet::with_capacity(N);

    for i in 0..(N + 10) {
        assert_eq!(fb.contains(i), false);
    }

    fb.insert(10);
    fb.set(11, false);
    fb.set(12, false);
    fb.set(12, true);
    fb.set(N - 1, true);

    assert!(fb.contains(10));
    assert!(!fb.contains(11));
    assert!(fb.contains(12));
    assert!(fb.contains(N - 1));
    for i in 0..N {
        let contain = i == 10 || i == 12 || i == N - 1;
        assert_eq!(contain, fb[i]);
    }

    fb.clear();
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn with_blocks() {
    let fb = FixedBitSet::with_capacity_and_blocks(50, vec![8, 0]);
    assert!(fb.contains(3));

    let ones: Vec<_> = fb.ones().collect();
    assert_eq!(ones.len(), 1);

    let ones: Vec<_> = fb.ones().rev().collect();
    assert_eq!(ones.len(), 1);

    let ones: Vec<_> = fb.ones().rev().alternate().collect();
    assert_eq!(ones.len(), 1);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn with_blocks_too_small() {
    let mut fb = FixedBitSet::with_capacity_and_blocks(500, vec![8, 0]);
    fb.insert(400);
    assert!(fb.contains(400));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn with_blocks_too_big() {
    let fb = FixedBitSet::with_capacity_and_blocks(1, vec![8]);

    // since capacity is 1, 3 shouldn't be set here
    assert!(!fb.contains(3));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn with_blocks_too_big_range_check() {
    let fb = FixedBitSet::with_capacity_and_blocks(1, vec![0xff]);

    // since capacity is 1, only 0 should be set
    assert!(fb.contains(0));
    for i in 1..0xff {
        assert!(!fb.contains(i));
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn grow() {
    let mut fb = FixedBitSet::with_capacity(48);
    for i in 0..fb.len() {
        fb.set(i, true);
    }

    let old_len = fb.len();
    fb.grow(72);
    for j in 0..fb.len() {
        assert_eq!(fb.contains(j), j < old_len);
    }
    fb.set(64, true);
    assert!(fb.contains(64));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn grow_and_insert() {
    let mut fb = FixedBitSet::default();
    for i in 0..100 {
        if i % 3 == 0 {
            fb.grow_and_insert(i);
        }
    }

    assert_eq!(fb.count_ones(..), 34);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn test_toggle() {
    let mut fb = FixedBitSet::with_capacity(16);
    fb.toggle(1);
    fb.put(2);
    fb.toggle(2);
    fb.put(3);
    assert!(fb.contains(1));
    assert!(!fb.contains(2));
    assert!(fb.contains(3));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn copy_bit() {
    let mut fb = FixedBitSet::with_capacity(48);
    for i in 0..fb.len() {
        fb.set(i, true);
    }
    fb.set(42, false);
    fb.copy_bit(42, 2);
    assert!(!fb.contains(42));
    assert!(!fb.contains(2));
    assert!(fb.contains(1));
    fb.copy_bit(1, 42);
    assert!(fb.contains(42));
    fb.copy_bit(1024, 42);
    assert!(!fb[42]);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn count_ones() {
    let mut fb = FixedBitSet::with_capacity(100);
    fb.set(11, true);
    fb.set(12, true);
    fb.set(7, true);
    fb.set(35, true);
    fb.set(40, true);
    fb.set(77, true);
    fb.set(95, true);
    fb.set(50, true);
    fb.set(99, true);
    assert_eq!(fb.count_ones(..7), 0);
    assert_eq!(fb.count_ones(..8), 1);
    assert_eq!(fb.count_ones(..11), 1);
    assert_eq!(fb.count_ones(..12), 2);
    assert_eq!(fb.count_ones(..13), 3);
    assert_eq!(fb.count_ones(..35), 3);
    assert_eq!(fb.count_ones(..36), 4);
    assert_eq!(fb.count_ones(..40), 4);
    assert_eq!(fb.count_ones(..41), 5);
    assert_eq!(fb.count_ones(50..), 4);
    assert_eq!(fb.count_ones(70..95), 1);
    assert_eq!(fb.count_ones(70..96), 2);
    assert_eq!(fb.count_ones(70..99), 2);
    assert_eq!(fb.count_ones(..), 9);
    assert_eq!(fb.count_ones(0..100), 9);
    assert_eq!(fb.count_ones(0..0), 0);
    assert_eq!(fb.count_ones(100..100), 0);
    assert_eq!(fb.count_ones(7..), 9);
    assert_eq!(fb.count_ones(8..), 8);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn count_zeroes() {
    let mut fb = FixedBitSet::with_capacity(100);
    fb.set(11, true);
    fb.set(12, true);
    fb.set(7, true);
    fb.set(35, true);
    fb.set(40, true);
    fb.set(77, true);
    fb.set(95, true);
    fb.set(50, true);
    fb.set(99, true);
    assert_eq!(fb.count_zeroes(..7), 7);
    assert_eq!(fb.count_zeroes(..8), 7);
    assert_eq!(fb.count_zeroes(..11), 10);
    assert_eq!(fb.count_zeroes(..12), 10);
    assert_eq!(fb.count_zeroes(..13), 10);
    assert_eq!(fb.count_zeroes(..35), 32);
    assert_eq!(fb.count_zeroes(..36), 32);
    assert_eq!(fb.count_zeroes(..40), 36);
    assert_eq!(fb.count_zeroes(..41), 36);
    assert_eq!(fb.count_zeroes(50..), 46);
    assert_eq!(fb.count_zeroes(70..95), 24);
    assert_eq!(fb.count_zeroes(70..96), 24);
    assert_eq!(fb.count_zeroes(70..99), 27);
    assert_eq!(fb.count_zeroes(..), 91);
    assert_eq!(fb.count_zeroes(0..100), 91);
    assert_eq!(fb.count_zeroes(0..0), 0);
    assert_eq!(fb.count_zeroes(100..100), 0);
    assert_eq!(fb.count_zeroes(7..), 84);
    assert_eq!(fb.count_zeroes(8..), 84);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn minimum() {
    let mut fb = FixedBitSet::with_capacity(100);
    assert_eq!(fb.minimum(), None);
    fb.set(95, true);
    assert_eq!(fb.minimum(), Some(95));
    fb.set(77, true);
    assert_eq!(fb.minimum(), Some(77));
    fb.set(12, true);
    assert_eq!(fb.minimum(), Some(12));
    fb.set(40, true);
    assert_eq!(fb.minimum(), Some(12));
    fb.set(35, true);
    assert_eq!(fb.minimum(), Some(12));
    fb.set(11, true);
    assert_eq!(fb.minimum(), Some(11));
    fb.set(7, true);
    assert_eq!(fb.minimum(), Some(7));
    fb.set(50, true);
    assert_eq!(fb.minimum(), Some(7));
    fb.set(99, true);
    assert_eq!(fb.minimum(), Some(7));
    fb.clear();
    assert_eq!(fb.minimum(), None);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn maximum() {
    let mut fb = FixedBitSet::with_capacity(100);
    assert_eq!(fb.maximum(), None);
    fb.set(11, true);
    assert_eq!(fb.maximum(), Some(11));
    fb.set(12, true);
    assert_eq!(fb.maximum(), Some(12));
    fb.set(7, true);
    assert_eq!(fb.maximum(), Some(12));
    fb.set(40, true);
    assert_eq!(fb.maximum(), Some(40));
    fb.set(35, true);
    assert_eq!(fb.maximum(), Some(40));
    fb.set(95, true);
    assert_eq!(fb.maximum(), Some(95));
    fb.set(50, true);
    assert_eq!(fb.maximum(), Some(95));
    fb.set(77, true);
    assert_eq!(fb.maximum(), Some(95));
    fb.set(99, true);
    assert_eq!(fb.maximum(), Some(99));
    fb.clear();
    assert_eq!(fb.maximum(), None);
}

/* Helper for testing double ended iterator */
#[cfg(test)]
struct Alternating<I> {
    iter: I,
    front: bool,
}

#[cfg(test)]
impl<I: Iterator + DoubleEndedIterator> Iterator for Alternating<I> {
    type Item = I::Item;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn next(&mut self) -> Option<Self::Item> {
        if self.front {
            self.front = false;
            self.iter.next()
        } else {
            self.front = true;
            self.iter.next_back()
        }
    }
}
#[cfg(test)]
trait AlternatingExt: Iterator + DoubleEndedIterator + Sized {
    fn alternate(self) -> Alternating<Self> {
        Alternating {
            iter: self,
            front: true,
        }
    }
}

#[cfg(test)]
impl<I: Iterator + DoubleEndedIterator> AlternatingExt for I {}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn ones() {
    let mut fb = FixedBitSet::with_capacity(100);
    fb.set(11, true);
    fb.set(12, true);
    fb.set(7, true);
    fb.set(35, true);
    fb.set(40, true);
    fb.set(77, true);
    fb.set(95, true);
    fb.set(50, true);
    fb.set(99, true);

    let ones: Vec<_> = fb.ones().collect();
    let ones_rev: Vec<_> = fb.ones().rev().collect();
    let ones_alternating: Vec<_> = fb.ones().alternate().collect();

    let mut known_result = vec![7, 11, 12, 35, 40, 50, 77, 95, 99];

    assert_eq!(known_result, ones);
    known_result.reverse();
    assert_eq!(known_result, ones_rev);
    let known_result: Vec<_> = known_result.into_iter().rev().alternate().collect();
    assert_eq!(known_result, ones_alternating);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn into_ones() {
    fn create() -> FixedBitSet {
        let mut fb = FixedBitSet::with_capacity(100);
        fb.set(11, true);
        fb.set(12, true);
        fb.set(7, true);
        fb.set(35, true);
        fb.set(40, true);
        fb.set(77, true);
        fb.set(95, true);
        fb.set(50, true);
        fb.set(99, true);
        fb
    }

    let ones: Vec<_> = create().into_ones().collect();
    let ones_rev: Vec<_> = create().into_ones().rev().collect();
    let ones_alternating: Vec<_> = create().into_ones().alternate().collect();

    let mut known_result = vec![7, 11, 12, 35, 40, 50, 77, 95, 99];

    assert_eq!(known_result, ones);
    known_result.reverse();
    assert_eq!(known_result, ones_rev);
    let known_result: Vec<_> = known_result.into_iter().rev().alternate().collect();
    assert_eq!(known_result, ones_alternating);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn size_hint() {
    let iters = if cfg!(miri) { 250 } else { 1000 };
    for s in 0..iters {
        let mut bitset = FixedBitSet::with_capacity(s);
        bitset.insert_range(..);
        let mut t = s;
        let mut iter = bitset.ones().rev();
        loop {
            match iter.next() {
                None => break,
                Some(_) => {
                    t -= 1;
                    assert!(iter.size_hint().1.unwrap() >= t);
                    // factor two, because we have first block and last block
                    assert!(iter.size_hint().1.unwrap() <= t + 2 * BITS);
                }
            }
        }
        assert_eq!(t, 0);
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn size_hint_alternate() {
    let iters = if cfg!(miri) { 250 } else { 1000 };
    for s in 0..iters {
        let mut bitset = FixedBitSet::with_capacity(s);
        bitset.insert_range(..);
        let mut t = s;
        extern crate std;
        let mut iter = bitset.ones().alternate();
        loop {
            match iter.next() {
                None => break,
                Some(_) => {
                    t -= 1;
                    assert!(iter.size_hint().1.unwrap() >= t);
                    assert!(iter.size_hint().1.unwrap() <= t + 3 * BITS);
                }
            }
        }
        assert_eq!(t, 0);
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn iter_ones_range() {
    fn test_range(from: usize, to: usize, capa: usize) {
        assert!(to <= capa);
        let mut fb = FixedBitSet::with_capacity(capa);
        for i in from..to {
            fb.insert(i);
        }
        let ones: Vec<_> = fb.ones().collect();
        let expected: Vec<_> = (from..to).collect();
        let ones_rev: Vec<_> = fb.ones().rev().collect();
        let expected_rev: Vec<_> = (from..to).rev().collect();
        let ones_rev_alt: Vec<_> = fb.ones().rev().alternate().collect();
        let expected_rev_alt: Vec<_> = (from..to).rev().alternate().collect();
        assert_eq!(expected, ones);
        assert_eq!(expected_rev, ones_rev);
        assert_eq!(expected_rev_alt, ones_rev_alt);
    }

    for i in 0..100 {
        test_range(i, 100, 100);
        test_range(0, i, 100);
    }
}

#[should_panic]
#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn count_ones_oob() {
    let fb = FixedBitSet::with_capacity(100);
    fb.count_ones(90..101);
}

#[should_panic]
#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn count_ones_negative_range() {
    let fb = FixedBitSet::with_capacity(100);
    fb.count_ones(90..80);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn count_ones_panic() {
    let iters = if cfg!(miri) { 48 } else { 128 };
    for i in 1..iters {
        let fb = FixedBitSet::with_capacity(i);
        for j in 0..fb.len() + 1 {
            for k in j..fb.len() + 1 {
                assert_eq!(fb.count_ones(j..k), 0);
            }
        }
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn default() {
    let fb = FixedBitSet::default();
    assert_eq!(fb.len(), 0);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn insert_range() {
    let mut fb = FixedBitSet::with_capacity(97);
    fb.insert_range(..3);
    fb.insert_range(9..32);
    fb.insert_range(37..81);
    fb.insert_range(90..);
    for i in 0..97 {
        assert_eq!(
            fb.contains(i),
            i < 3 || 9 <= i && i < 32 || 37 <= i && i < 81 || 90 <= i
        );
    }
    assert!(!fb.contains(97));
    assert!(!fb.contains(127));
    assert!(!fb.contains(128));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn contains_all_in_range() {
    let mut fb = FixedBitSet::with_capacity(48);
    fb.insert_range(..);

    fb.remove_range(..32);
    fb.remove_range(37..);

    assert!(fb.contains_all_in_range(32..37));
    assert!(fb.contains_all_in_range(32..35));
    assert!(!fb.contains_all_in_range(32..));
    assert!(!fb.contains_all_in_range(..37));
    assert!(!fb.contains_all_in_range(..));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn contains_any_in_range() {
    let mut fb = FixedBitSet::with_capacity(48);
    fb.insert_range(..);

    fb.remove_range(..32);
    fb.remove_range(37..);

    assert!(!fb.contains_any_in_range(..32));
    assert!(fb.contains_any_in_range(32..37));
    assert!(fb.contains_any_in_range(32..35));
    assert!(fb.contains_any_in_range(32..));
    assert!(fb.contains_any_in_range(..37));
    assert!(!fb.contains_any_in_range(37..));
    assert!(fb.contains_any_in_range(..));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn remove_range() {
    let mut fb = FixedBitSet::with_capacity(48);
    fb.insert_range(..);

    fb.remove_range(..32);
    fb.remove_range(37..);

    for i in 0..48 {
        assert_eq!(fb.contains(i), 32 <= i && i < 37);
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn set_range() {
    let mut fb = FixedBitSet::with_capacity(48);
    fb.insert_range(..);

    fb.set_range(..32, false);
    fb.set_range(37.., false);
    fb.set_range(5..9, true);
    fb.set_range(40..40, true);

    for i in 0..48 {
        assert_eq!(fb.contains(i), 5 <= i && i < 9 || 32 <= i && i < 37);
    }
    assert!(!fb.contains(48));
    assert!(!fb.contains(64));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn toggle_range() {
    let mut fb = FixedBitSet::with_capacity(40);
    fb.insert_range(..10);
    fb.insert_range(34..38);

    fb.toggle_range(5..12);
    fb.toggle_range(30..);

    for i in 0..40 {
        assert_eq!(
            fb.contains(i),
            i < 5 || 10 <= i && i < 12 || 30 <= i && i < 34 || 38 <= i
        );
    }
    assert!(!fb.contains(40));
    assert!(!fb.contains(64));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitand_equal_lengths() {
    let len = 109;
    let a_end = 59;
    let b_start = 23;
    let mut a = FixedBitSet::with_capacity(len);
    let mut b = FixedBitSet::with_capacity(len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a & &b;
    for i in 0..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..len {
        assert!(!ab.contains(i));
    }
    assert_eq!(a.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitand_first_smaller() {
    let a_len = 113;
    let b_len = 137;
    let len = core::cmp::min(a_len, b_len);
    let a_end = 97;
    let b_start = 89;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a & &b;
    for i in 0..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..len {
        assert!(!ab.contains(i));
    }
    assert_eq!(a.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitand_first_larger() {
    let a_len = 173;
    let b_len = 137;
    let len = core::cmp::min(a_len, b_len);
    let a_end = 107;
    let b_start = 43;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a & &b;
    for i in 0..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..len {
        assert!(!ab.contains(i));
    }
    assert_eq!(b.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn intersection() {
    let len = 109;
    let a_end = 59;
    let b_start = 23;
    let mut a = FixedBitSet::with_capacity(len);
    let mut b = FixedBitSet::with_capacity(len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let count = a.intersection_count(&b);
    let iterator_count = a.intersection(&b).count();
    let mut ab = a.intersection(&b).collect::<FixedBitSet>();

    for i in 0..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..len {
        assert!(!ab.contains(i));
    }

    a.intersect_with(&b);
    // intersection + collect produces the same results but with a shorter length.
    ab.grow(a.len());
    assert_eq!(
        ab, a,
        "intersection and intersect_with produce the same results"
    );
    assert_eq!(
        ab.count_ones(..),
        count,
        "intersection and intersection_count produce the same results"
    );
    assert_eq!(
        count, iterator_count,
        "intersection and intersection_count produce the same results"
    );
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn union() {
    let a_len = 173;
    let b_len = 137;
    let a_start = 139;
    let b_end = 107;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(a_start.., true);
    b.set_range(..b_end, true);
    let count = a.union_count(&b);
    let iterator_count = a.union(&b).count();
    let ab = a.union(&b).collect::<FixedBitSet>();
    for i in a_start..a_len {
        assert!(ab.contains(i));
    }
    for i in 0..b_end {
        assert!(ab.contains(i));
    }
    for i in b_end..a_start {
        assert!(!ab.contains(i));
    }

    a.union_with(&b);
    assert_eq!(ab, a, "union and union_with produce the same results");
    assert_eq!(
        count,
        ab.count_ones(..),
        "union and union_count produce the same results"
    );
    assert_eq!(
        count, iterator_count,
        "union and union_count produce the same results"
    );
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn difference() {
    let a_len = 83;
    let b_len = 151;
    let a_start = 0;
    let a_end = 79;
    let b_start = 53;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(a_start..a_end, true);
    b.set_range(b_start..b_len, true);
    let count = a.difference_count(&b);
    let iterator_count = a.difference(&b).count();
    let mut a_diff_b = a.difference(&b).collect::<FixedBitSet>();
    for i in a_start..b_start {
        assert!(a_diff_b.contains(i));
    }
    for i in b_start..b_len {
        assert!(!a_diff_b.contains(i));
    }

    a.difference_with(&b);
    // difference + collect produces the same results but with a shorter length.
    a_diff_b.grow(a.len());
    assert_eq!(
        a_diff_b, a,
        "difference and difference_with produce the same results"
    );
    assert_eq!(
        a_diff_b.count_ones(..),
        count,
        "difference and difference_count produce the same results"
    );
    assert_eq!(
        count, iterator_count,
        "intersection and intersection_count produce the same results"
    );
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn symmetric_difference() {
    let a_len = 83;
    let b_len = 151;
    let a_start = 47;
    let a_end = 79;
    let b_start = 53;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(a_start..a_end, true);
    b.set_range(b_start..b_len, true);
    let count = a.symmetric_difference_count(&b);
    let iterator_count = a.symmetric_difference(&b).count();
    let a_sym_diff_b = a.symmetric_difference(&b).collect::<FixedBitSet>();
    for i in 0..a_start {
        assert!(!a_sym_diff_b.contains(i));
    }
    for i in a_start..b_start {
        assert!(a_sym_diff_b.contains(i));
    }
    for i in b_start..a_end {
        assert!(!a_sym_diff_b.contains(i));
    }
    for i in a_end..b_len {
        assert!(a_sym_diff_b.contains(i));
    }

    a.symmetric_difference_with(&b);
    assert_eq!(
        a_sym_diff_b, a,
        "symmetric_difference and _with produce the same results"
    );
    assert_eq!(
        a_sym_diff_b.count_ones(..),
        count,
        "symmetric_difference and _count produce the same results"
    );
    assert_eq!(
        count, iterator_count,
        "symmetric_difference and _count produce the same results"
    );
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitor_equal_lengths() {
    let len = 109;
    let a_start = 17;
    let a_end = 23;
    let b_start = 19;
    let b_end = 59;
    let mut a = FixedBitSet::with_capacity(len);
    let mut b = FixedBitSet::with_capacity(len);
    a.set_range(a_start..a_end, true);
    b.set_range(b_start..b_end, true);
    let ab = &a | &b;
    for i in 0..a_start {
        assert!(!ab.contains(i));
    }
    for i in a_start..b_end {
        assert!(ab.contains(i));
    }
    for i in b_end..len {
        assert!(!ab.contains(i));
    }
    assert_eq!(ab.len(), len);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitor_first_smaller() {
    let a_len = 113;
    let b_len = 137;
    let a_end = 89;
    let b_start = 97;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a | &b;
    for i in 0..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..b_len {
        assert!(ab.contains(i));
    }
    assert_eq!(b_len, ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitor_first_larger() {
    let a_len = 173;
    let b_len = 137;
    let a_start = 139;
    let b_end = 107;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(a_start.., true);
    b.set_range(..b_end, true);
    let ab = &a | &b;
    for i in a_start..a_len {
        assert!(ab.contains(i));
    }
    for i in 0..b_end {
        assert!(ab.contains(i));
    }
    for i in b_end..a_start {
        assert!(!ab.contains(i));
    }
    assert_eq!(a_len, ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitxor_equal_lengths() {
    let len = 109;
    let a_end = 59;
    let b_start = 23;
    let mut a = FixedBitSet::with_capacity(len);
    let mut b = FixedBitSet::with_capacity(len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a ^ &b;
    for i in 0..b_start {
        assert!(ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(!ab.contains(i));
    }
    for i in a_end..len {
        assert!(ab.contains(i));
    }
    assert_eq!(a.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitxor_first_smaller() {
    let a_len = 113;
    let b_len = 137;
    let len = core::cmp::max(a_len, b_len);
    let a_end = 97;
    let b_start = 89;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a ^ &b;
    for i in 0..b_start {
        assert!(ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(!ab.contains(i));
    }
    for i in a_end..len {
        assert!(ab.contains(i));
    }
    assert_eq!(b.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitxor_first_larger() {
    let a_len = 173;
    let b_len = 137;
    let len = core::cmp::max(a_len, b_len);
    let a_end = 107;
    let b_start = 43;
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a ^ &b;
    for i in 0..b_start {
        assert!(ab.contains(i));
    }
    for i in b_start..a_end {
        assert!(!ab.contains(i));
    }
    for i in a_end..b_len {
        assert!(ab.contains(i));
    }
    for i in b_len..len {
        assert!(!ab.contains(i));
    }
    assert_eq!(a.len(), ab.len());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitand_assign_shorter() {
    let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let a_and_b: Vec<usize> = vec![2, 7, 31, 32];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a &= b;
    let res = a.ones().collect::<Vec<usize>>();

    assert!(res == a_and_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitand_assign_longer() {
    let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let a_and_b: Vec<usize> = vec![2, 7, 31, 32];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a &= b;
    let res = a.ones().collect::<Vec<usize>>();
    assert!(res == a_and_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitor_assign_shorter() {
    let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let a_or_b: Vec<usize> = vec![2, 3, 7, 8, 11, 19, 23, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a |= b;
    let res = a.ones().collect::<Vec<usize>>();
    assert!(res == a_or_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitor_assign_longer() {
    let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let a_or_b: Vec<usize> = vec![2, 3, 7, 8, 11, 19, 23, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a |= b;
    let res = a.ones().collect::<Vec<usize>>();
    assert_eq!(res, a_or_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitxor_assign_shorter() {
    let a_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let b_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let a_xor_b: Vec<usize> = vec![3, 8, 11, 19, 23, 37, 41, 43, 47, 71, 73, 101];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a ^= b;
    let res = a.ones().collect::<Vec<usize>>();
    assert!(res == a_xor_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn bitxor_assign_longer() {
    let a_ones: Vec<usize> = vec![2, 7, 8, 11, 23, 31, 32];
    let b_ones: Vec<usize> = vec![2, 3, 7, 19, 31, 32, 37, 41, 43, 47, 71, 73, 101];
    let a_xor_b: Vec<usize> = vec![3, 8, 11, 19, 23, 37, 41, 43, 47, 71, 73, 101];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    a ^= b;
    let res = a.ones().collect::<Vec<usize>>();
    assert!(res == a_xor_b);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn op_assign_ref() {
    let mut a = FixedBitSet::with_capacity(8);
    let b = FixedBitSet::with_capacity(8);

    //check that all assign type operators work on references
    a &= &b;
    a |= &b;
    a ^= &b;
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn subset_superset_shorter() {
    let a_ones: Vec<usize> = vec![7, 31, 32, 63];
    let b_ones: Vec<usize> = vec![2, 7, 19, 31, 32, 37, 41, 43, 47, 63, 73, 101];
    let mut a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let b = b_ones.iter().cloned().collect::<FixedBitSet>();
    assert!(a.is_subset(&b) && b.is_superset(&a));
    a.insert(14);
    assert!(!a.is_subset(&b) && !b.is_superset(&a));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn subset_superset_longer() {
    let a_len = 153;
    let b_len = 75;
    let a_ones: Vec<usize> = vec![7, 31, 32, 63];
    let b_ones: Vec<usize> = vec![2, 7, 19, 31, 32, 37, 41, 43, 47, 63, 73];
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.extend(a_ones.iter().cloned());
    b.extend(b_ones.iter().cloned());
    assert!(a.is_subset(&b) && b.is_superset(&a));
    a.insert(100);
    assert!(!a.is_subset(&b) && !b.is_superset(&a));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn is_disjoint_first_shorter() {
    let a_len = 75;
    let b_len = 153;
    let a_ones: Vec<usize> = vec![2, 19, 32, 37, 41, 43, 47, 73];
    let b_ones: Vec<usize> = vec![7, 23, 31, 63, 124];
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.extend(a_ones.iter().cloned());
    b.extend(b_ones.iter().cloned());
    assert!(a.is_disjoint(&b));
    a.insert(63);
    assert!(!a.is_disjoint(&b));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn is_disjoint_first_longer() {
    let a_ones: Vec<usize> = vec![2, 19, 32, 37, 41, 43, 47, 73, 101];
    let b_ones: Vec<usize> = vec![7, 23, 31, 63];
    let a = a_ones.iter().cloned().collect::<FixedBitSet>();
    let mut b = b_ones.iter().cloned().collect::<FixedBitSet>();
    assert!(a.is_disjoint(&b));
    b.insert(2);
    assert!(!a.is_disjoint(&b));
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn extend_on_empty() {
    let items: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 167];
    let mut fbs = FixedBitSet::with_capacity(0);
    fbs.extend(items.iter().cloned());
    let ones = fbs.ones().collect::<Vec<usize>>();
    assert!(ones == items);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn extend() {
    let items: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 167];
    let mut fbs = FixedBitSet::with_capacity(168);
    let new: Vec<usize> = vec![7, 37, 67, 137];
    for i in &new {
        fbs.put(*i);
    }

    fbs.extend(items.iter().cloned());

    let ones = fbs.ones().collect::<Vec<usize>>();
    let expected = {
        let mut tmp = items.clone();
        tmp.extend(new);
        tmp.sort();
        tmp.dedup();
        tmp
    };

    assert_eq!(ones, expected);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn from_iterator() {
    let items: Vec<usize> = vec![0, 2, 4, 6, 8];
    let fb = items.iter().cloned().collect::<FixedBitSet>();
    for i in items {
        assert!(fb.contains(i));
    }
    for i in vec![1, 3, 5, 7] {
        assert!(!fb.contains(i));
    }
    assert_eq!(fb.len(), 9);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn from_iterator_ones() {
    let len = 257;
    let mut fb = FixedBitSet::with_capacity(len);
    for i in (0..len).filter(|i| i % 7 == 0) {
        fb.put(i);
    }
    fb.put(len - 1);
    let dup = fb.ones().collect::<FixedBitSet>();

    assert_eq!(fb.len(), dup.len());
    assert_eq!(
        fb.ones().collect::<Vec<usize>>(),
        dup.ones().collect::<Vec<usize>>()
    );
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn zeroes() {
    let len = 232;
    let mut fb = FixedBitSet::with_capacity(len);
    for i in (0..len).filter(|i| i % 7 == 0) {
        fb.insert(i);
    }
    let zeroes = fb.zeroes().collect::<Vec<usize>>();

    assert_eq!(
        zeroes,
        vec![
            1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25, 26, 27,
            29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 48, 50, 51, 52, 53,
            54, 55, 57, 58, 59, 60, 61, 62, 64, 65, 66, 67, 68, 69, 71, 72, 73, 74, 75, 76, 78, 79,
            80, 81, 82, 83, 85, 86, 87, 88, 89, 90, 92, 93, 94, 95, 96, 97, 99, 100, 101, 102, 103,
            104, 106, 107, 108, 109, 110, 111, 113, 114, 115, 116, 117, 118, 120, 121, 122, 123,
            124, 125, 127, 128, 129, 130, 131, 132, 134, 135, 136, 137, 138, 139, 141, 142, 143,
            144, 145, 146, 148, 149, 150, 151, 152, 153, 155, 156, 157, 158, 159, 160, 162, 163,
            164, 165, 166, 167, 169, 170, 171, 172, 173, 174, 176, 177, 178, 179, 180, 181, 183,
            184, 185, 186, 187, 188, 190, 191, 192, 193, 194, 195, 197, 198, 199, 200, 201, 202,
            204, 205, 206, 207, 208, 209, 211, 212, 213, 214, 215, 216, 218, 219, 220, 221, 222,
            223, 225, 226, 227, 228, 229, 230
        ]
    );
}

#[cfg(feature = "std")]
#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn binary_trait() {
    let items: Vec<usize> = vec![1, 5, 7, 10, 14, 15];
    let fb = items.iter().cloned().collect::<FixedBitSet>();

    assert_eq!(alloc::format!("{:b}", fb), "0100010100100011");
    assert_eq!(alloc::format!("{:#b}", fb), "0b0100010100100011");
}

#[cfg(feature = "std")]
#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn display_trait() {
    let len = 8;
    let mut fb = FixedBitSet::with_capacity(len);

    fb.put(4);
    fb.put(2);

    assert_eq!(alloc::format!("{}", fb), "00101000");
    assert_eq!(alloc::format!("{:#}", fb), "0b00101000");
}

// TODO: Rewite this test to be platform agnostic.
#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
#[cfg(all(feature = "serde", target_pointer_width = "64"))]
fn test_serialize() {
    let mut fb = FixedBitSet::with_capacity(10);
    fb.put(2);
    fb.put(3);
    fb.put(6);
    fb.put(8);
    let serialized = serde_json::to_string(&fb).unwrap();
    assert_eq!(r#"{"length":10,"data":[76,1,0,0,0,0,0,0]}"#, serialized);
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn test_is_clear() {
    let mut fb = FixedBitSet::with_capacity(0);
    assert!(fb.is_clear());

    fb.grow(1);
    assert!(fb.is_clear());

    fb.put(0);
    assert!(!fb.is_clear());

    fb.grow(42);
    fb.clear();
    assert!(fb.is_clear());

    fb.put(17);
    fb.put(19);
    assert!(!fb.is_clear());
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn test_is_full() {
    let mut fb = FixedBitSet::with_capacity(0);
    assert!(fb.is_full());

    fb.grow(1);
    assert!(!fb.is_full());

    fb.put(0);
    assert!(fb.is_full());

    fb.grow(42);
    fb.clear();
    assert!(!fb.is_full());

    fb.put(17);
    fb.put(19);
    assert!(!fb.is_full());

    fb.insert_range(..);
    assert!(fb.is_full());
}

#[test]
fn clone() {
    let mut fb = FixedBitSet::with_capacity(10000);
    fb.set(11, true);
    fb.set(12, true);
    fb.set(7, true);
    fb.set(35, true);
    fb.set(40, true);
    fb.set(77, true);
    fb.set(95, true);
    fb.set(50, true);
    fb.set(99, true);

    let fb_clone = fb.clone();
    let mut fb_clone_from_smaller = FixedBitSet::with_capacity(1000000);
    let mut fb_clone_from_same = FixedBitSet::with_capacity(10000);
    let mut fb_clone_from_bigger = FixedBitSet::with_capacity(100);
    fb_clone_from_smaller.clone_from(&fb);
    fb_clone_from_same.clone_from(&fb);
    fb_clone_from_bigger.clone_from(&fb);

    assert_eq!(&fb, &fb_clone);
    assert_eq!(&fb, &fb_clone_from_smaller);
    assert_eq!(&fb, &fb_clone_from_same);
    assert_eq!(&fb, &fb_clone_from_bigger);
}
