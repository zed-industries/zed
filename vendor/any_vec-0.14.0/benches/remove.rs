mod utils;

use std::time::{Duration, Instant};
use criterion::{criterion_group, criterion_main, Criterion};
use any_vec::AnyVec;
use crate::utils::bench_custom;

const SIZE: usize = 10000;

type Element = String;
static VALUE: Element = String::new();

trait Impl{
    fn index(i:usize) -> usize;

    fn vec_remove() -> Duration {
        let mut vec = Vec::new();
        for _ in 0..SIZE{
            vec.push(VALUE.clone());
        }

        let start = Instant::now();
            for i in (0..SIZE).rev(){
                vec.remove(Self::index(i));
            }
        start.elapsed()
    }

    fn any_vec_remove() -> Duration {
        let mut any_vec: AnyVec = AnyVec::new::<Element>();
        for _ in 0..SIZE{
            any_vec.downcast_mut::<Element>().unwrap()
                .push(VALUE.clone());
        }

        let start = Instant::now();
            for i in (0..SIZE).rev(){
                any_vec.remove(Self::index(i));
            }
        start.elapsed()
    }

    fn any_vec_typed_remove() -> Duration {
        let mut any_vec: AnyVec = AnyVec::new::<Element>();
        for _ in 0..SIZE{
            any_vec.downcast_mut::<Element>().unwrap()
                .push(VALUE.clone());
        }

        let start = Instant::now();
            for i in (0..SIZE).rev(){
                any_vec.downcast_mut::<Element>().unwrap()
                    .remove(Self::index(i));
            }
        start.elapsed()
    }
}

struct Front;
impl Impl for Front {
    #[inline]
    fn index(_: usize) -> usize { 0 }
}

struct Back;
impl Impl for Back {
    #[inline]
    fn index(i: usize) -> usize { i }
}

pub fn bench_remove(c: &mut Criterion) {
    bench_custom(c, "Vec remove front", Front::vec_remove);
    bench_custom(c, "AnyVec remove front", Front::any_vec_remove);
    bench_custom(c, "AnyVecTyped remove front", Front::any_vec_typed_remove);

    bench_custom(c, "Vec remove back", Back::vec_remove);
    bench_custom(c, "AnyVec remove back", Back::any_vec_remove);
    bench_custom(c, "AnyVecTyped remove back", Back::any_vec_typed_remove);
}

criterion_group!(benches, bench_remove);
criterion_main!(benches);