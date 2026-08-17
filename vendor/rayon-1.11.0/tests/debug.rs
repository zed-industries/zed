use rayon::prelude::*;
use std::fmt::Debug;

fn check<I>(iter: I)
where
    I: ParallelIterator + Debug,
{
    println!("{iter:?}");
}

#[test]
fn debug_binary_heap() {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<_> = (0..10).collect();
    check(heap.par_iter());
    check(heap.par_drain());
    check(heap.into_par_iter());
}

#[test]
fn debug_btree_map() {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<_, _> = (0..10).enumerate().collect();
    check(map.par_iter());
    check(map.par_iter_mut());
    check(map.into_par_iter());
}

#[test]
fn debug_btree_set() {
    use std::collections::BTreeSet;
    let set: BTreeSet<_> = (0..10).collect();
    check(set.par_iter());
    check(set.into_par_iter());
}

#[test]
fn debug_hash_map() {
    use std::collections::HashMap;
    let mut map: HashMap<_, _> = (0..10).enumerate().collect();
    check(map.par_iter());
    check(map.par_iter_mut());
    check(map.par_drain());
    check(map.into_par_iter());
}

#[test]
fn debug_hash_set() {
    use std::collections::HashSet;
    let mut set: HashSet<_> = (0..10).collect();
    check(set.par_iter());
    check(set.par_drain());
    check(set.into_par_iter());
}

#[test]
fn debug_linked_list() {
    use std::collections::LinkedList;
    let mut list: LinkedList<_> = (0..10).collect();
    check(list.par_iter());
    check(list.par_iter_mut());
    check(list.into_par_iter());
}

#[test]
fn debug_vec_deque() {
    use std::collections::VecDeque;
    let mut deque: VecDeque<_> = (0..10).collect();
    check(deque.par_iter());
    check(deque.par_iter_mut());
    check(deque.par_drain(..));
    check(deque.into_par_iter());
}

#[test]
fn debug_option() {
    let mut option = Some(0);
    check(option.par_iter());
    check(option.par_iter_mut());
    check(option.into_par_iter());
}

#[test]
fn debug_result() {
    let mut result = Ok::<_, ()>(0);
    check(result.par_iter());
    check(result.par_iter_mut());
    check(result.into_par_iter());
}

#[test]
fn debug_range() {
    check((0..10).into_par_iter());
}

#[test]
fn debug_range_inclusive() {
    check((0..=10).into_par_iter());
}

#[test]
fn debug_str() {
    let s = "a b c d\ne f g";
    check(s.par_chars());
    check(s.par_lines());
    check(s.par_split('\n'));
    check(s.par_split_inclusive('\n'));
    check(s.par_split_terminator('\n'));
    check(s.par_split_whitespace());
    check(s.par_split_ascii_whitespace());
}

#[test]
fn debug_string() {
    let mut s = "a b c d\ne f g".to_string();
    s.par_drain(..);
}

#[test]
fn debug_vec() {
    let mut v: Vec<_> = (0..10).collect();
    check(v.par_iter());
    check(v.par_iter_mut());
    check(v.par_chunk_by(i32::eq));
    check(v.par_chunk_by_mut(i32::eq));
    check(v.par_chunks(42));
    check(v.par_chunks_exact(42));
    check(v.par_chunks_mut(42));
    check(v.par_chunks_exact_mut(42));
    check(v.par_rchunks(42));
    check(v.par_rchunks_exact(42));
    check(v.par_rchunks_mut(42));
    check(v.par_rchunks_exact_mut(42));
    check(v.par_windows(42));
    check(v.par_split(|x| x % 3 == 0));
    check(v.par_split_inclusive(|x| x % 3 == 0));
    check(v.par_split_mut(|x| x % 3 == 0));
    check(v.par_split_inclusive_mut(|x| x % 3 == 0));
    check(v.par_drain(..));
    check(v.into_par_iter());
}

#[test]
fn debug_array() {
    let a = [0i32; 10];
    check(a.into_par_iter());
}

#[test]
fn debug_adaptors() {
    let v: Vec<_> = (0..10).collect();
    check(v.par_iter().by_exponential_blocks());
    check(v.par_iter().by_uniform_blocks(5));
    check(v.par_iter().chain(&v));
    check(v.par_iter().cloned());
    check(v.par_iter().copied());
    check(v.par_iter().enumerate());
    check(v.par_iter().filter(|_| true));
    check(v.par_iter().filter_map(Some));
    check(v.par_iter().flat_map(Some));
    check(v.par_iter().flat_map_iter(Some));
    check(v.par_iter().map(Some).flatten());
    check(v.par_iter().map(Some).flatten_iter());
    check(v.par_iter().fold(|| 0, |x, _| x));
    check(v.par_iter().fold_with(0, |x, _| x));
    check(v.par_iter().fold_chunks(3, || 0, |x, _| x));
    check(v.par_iter().fold_chunks_with(3, 0, |x, _| x));
    check(v.par_iter().try_fold(|| 0, |x, _| Some(x)));
    check(v.par_iter().try_fold_with(0, |x, _| Some(x)));
    check(v.par_iter().inspect(|_| ()));
    check(v.par_iter().update(|_| ()));
    check(v.par_iter().interleave(&v));
    check(v.par_iter().interleave_shortest(&v));
    check(v.par_iter().intersperse(&-1));
    check(v.par_iter().chunks(3));
    check(v.par_iter().map(|x| x));
    check(v.par_iter().map_with(0, |_, x| x));
    check(v.par_iter().map_init(|| 0, |_, x| x));
    check(v.par_iter().panic_fuse());
    check(v.par_iter().positions(|_| true));
    check(v.par_iter().rev());
    check(v.par_iter().skip(1));
    check(v.par_iter().skip_any(1));
    check(v.par_iter().skip_any_while(|_| false));
    check(v.par_iter().take(1));
    check(v.par_iter().take_any(1));
    check(v.par_iter().take_any_while(|_| true));
    check(v.par_iter().map(Some).while_some());
    check(v.par_iter().with_max_len(1));
    check(v.par_iter().with_min_len(1));
    check(v.par_iter().zip(&v));
    check(v.par_iter().zip_eq(&v));
    check(v.par_iter().step_by(2));
}

#[test]
fn debug_empty() {
    check(rayon::iter::empty::<i32>());
}

#[test]
fn debug_once() {
    check(rayon::iter::once(10));
}

#[test]
fn debug_repeat() {
    let x: Option<i32> = None;
    check(rayon::iter::repeat(x));
    check(rayon::iter::repeat_n(x, 10));
}

#[test]
fn debug_splitter() {
    check(rayon::iter::split(0..10, |x| (x, None)));
}

#[test]
fn debug_multizip() {
    let v: &Vec<_> = &(0..10).collect();
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
