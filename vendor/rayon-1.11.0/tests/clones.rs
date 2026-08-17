use rayon::prelude::*;
use std::fmt::Debug;

fn check<I>(iter: I)
where
    I: ParallelIterator<Item: Debug + PartialEq> + Clone,
{
    let a: Vec<_> = iter.clone().collect();
    let b: Vec<_> = iter.collect();
    assert_eq!(a, b);
}

fn check_count<I>(iter: I)
where
    I: ParallelIterator + Clone,
{
    assert_eq!(iter.clone().count(), iter.count());
}

#[test]
fn clone_binary_heap() {
    use std::collections::BinaryHeap;
    let heap: BinaryHeap<_> = (0..1000).collect();
    check(heap.par_iter());
    check(heap.into_par_iter());
}

#[test]
fn clone_btree_map() {
    use std::collections::BTreeMap;
    let map: BTreeMap<_, _> = (0..1000).enumerate().collect();
    check(map.par_iter());
}

#[test]
fn clone_btree_set() {
    use std::collections::BTreeSet;
    let set: BTreeSet<_> = (0..1000).collect();
    check(set.par_iter());
}

#[test]
fn clone_hash_map() {
    use std::collections::HashMap;
    let map: HashMap<_, _> = (0..1000).enumerate().collect();
    check(map.par_iter());
}

#[test]
fn clone_hash_set() {
    use std::collections::HashSet;
    let set: HashSet<_> = (0..1000).collect();
    check(set.par_iter());
}

#[test]
fn clone_linked_list() {
    use std::collections::LinkedList;
    let list: LinkedList<_> = (0..1000).collect();
    check(list.par_iter());
    check(list.into_par_iter());
}

#[test]
fn clone_vec_deque() {
    use std::collections::VecDeque;
    let deque: VecDeque<_> = (0..1000).collect();
    check(deque.par_iter());
    check(deque.into_par_iter());
}

#[test]
fn clone_option() {
    let option = Some(0);
    check(option.par_iter());
    check(option.into_par_iter());
}

#[test]
fn clone_result() {
    let result = Ok::<_, ()>(0);
    check(result.par_iter());
    check(result.into_par_iter());
}

#[test]
fn clone_range() {
    check((0..1000).into_par_iter());
}

#[test]
fn clone_range_inclusive() {
    check((0..=1000).into_par_iter());
}

#[test]
fn clone_str() {
    let s = include_str!("clones.rs");
    check(s.par_chars());
    check(s.par_lines());
    check(s.par_split('\n'));
    check(s.par_split_inclusive('\n'));
    check(s.par_split_terminator('\n'));
    check(s.par_split_whitespace());
    check(s.par_split_ascii_whitespace());
}

#[test]
fn clone_vec() {
    let v: Vec<_> = (0..1000).collect();
    check(v.par_iter());
    check(v.par_chunk_by(i32::eq));
    check(v.par_chunks(42));
    check(v.par_chunks_exact(42));
    check(v.par_rchunks(42));
    check(v.par_rchunks_exact(42));
    check(v.par_windows(42));
    check(v.par_split(|x| x % 3 == 0));
    check(v.par_split_inclusive(|x| x % 3 == 0));
    check(v.into_par_iter());
}

#[test]
fn clone_array() {
    let a = [0i32; 100];
    check(a.into_par_iter());
}

#[test]
fn clone_adaptors() {
    let v: Vec<_> = (0..1000).map(Some).collect();
    check(v.par_iter().by_exponential_blocks());
    check(v.par_iter().by_uniform_blocks(100));
    check(v.par_iter().chain(&v));
    check(v.par_iter().cloned());
    check(v.par_iter().copied());
    check(v.par_iter().enumerate());
    check(v.par_iter().filter(|_| true));
    check(v.par_iter().filter_map(|x| *x));
    check(v.par_iter().flat_map(|x| *x));
    check(v.par_iter().flat_map_iter(|x| *x));
    check(v.par_iter().flatten());
    check(v.par_iter().flatten_iter());
    check(v.par_iter().with_max_len(1).fold(|| 0, |x, _| x));
    check(v.par_iter().with_max_len(1).fold_with(0, |x, _| x));
    check(v.par_iter().with_max_len(1).fold_chunks(1, || 0, |x, _| x));
    check(
        v.par_iter()
            .with_max_len(1)
            .fold_chunks_with(1, 0, |x, _| x),
    );
    check(v.par_iter().with_max_len(1).try_fold(|| 0, |_, &x| x));
    check(v.par_iter().with_max_len(1).try_fold_with(0, |_, &x| x));
    check(v.par_iter().inspect(|_| ()));
    check(v.par_iter().update(|_| ()));
    check(v.par_iter().interleave(&v));
    check(v.par_iter().interleave_shortest(&v));
    check(v.par_iter().intersperse(&None));
    check(v.par_iter().chunks(3));
    check(v.par_iter().map(|x| x));
    check(v.par_iter().map_with(0, |_, x| x));
    check(v.par_iter().map_init(|| 0, |_, x| x));
    check(v.par_iter().panic_fuse());
    check(v.par_iter().positions(|_| true));
    check(v.par_iter().rev());
    check(v.par_iter().skip(42));
    check(v.par_iter().skip_any_while(|_| false));
    check(v.par_iter().take(42));
    check(v.par_iter().take_any_while(|_| true));
    check(v.par_iter().cloned().while_some());
    check(v.par_iter().with_max_len(1));
    check(v.par_iter().with_min_len(1));
    check(v.par_iter().zip(&v));
    check(v.par_iter().zip_eq(&v));
    check(v.par_iter().step_by(2));
}

#[test]
fn clone_counted_adaptors() {
    let v: Vec<_> = (0..1000).collect();
    check_count(v.par_iter().skip_any(42));
    check_count(v.par_iter().take_any(42));
}

#[test]
fn clone_empty() {
    check(rayon::iter::empty::<i32>());
}

#[test]
fn clone_once() {
    check(rayon::iter::once(10));
}

#[test]
fn clone_repeat() {
    let x: Option<i32> = None;
    check(rayon::iter::repeat(x).while_some());
    check(rayon::iter::repeat_n(x, 1000));
}

#[test]
fn clone_splitter() {
    check(rayon::iter::split(0..1000, |x| (x, None)));
}

#[test]
fn clone_multizip() {
    let v: &Vec<_> = &(0..1000).collect();
    check((v,).into_par_iter());
    check((v, v).into_par_iter());
    check((v, v, v).into_par_iter());
    check((v, v, v, v).into_par_iter());
    check((v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v, v, v, v, v).into_par_iter());
    check((v, v, v, v, v, v, v, v, v, v, v, v).into_par_iter());
}
