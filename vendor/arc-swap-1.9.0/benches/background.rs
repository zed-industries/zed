use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use arc_swap::{ArcSwap, ArcSwapOption, Cache};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_utils::thread;
use once_cell::sync::Lazy;

// Mostly a leftover from earlier times, but it still allows one to tweak the number of ops per one
// iteration of the benchmark easily, so it's left in here.
const ITERS: usize = 1;

macro_rules! method {
    ($c: expr, $name:ident) => {{
        let mut g = $c.benchmark_group(&format!("{}_{}", NAME, stringify!($name)));
        noise(&mut g, "r1", 1, 0, 0, $name);
        noise(&mut g, "r3", 3, 0, 0, $name);
        noise(&mut g, "l1", 0, 1, 0, $name);
        noise(&mut g, "l3", 0, 3, 0, $name);
        noise(&mut g, "rw", 1, 0, 1, $name);
        noise(&mut g, "lw", 0, 1, 1, $name);
        noise(&mut g, "w2", 0, 0, 2, $name);
        g.bench_function("uncontended", |b| b.iter($name));
        g.finish();
    }};
}

macro_rules! noise {
    () => {
        use criterion::measurement::Measurement;
        use criterion::BenchmarkGroup;

        use super::{thread, Arc, AtomicBool, Ordering, ITERS};

        fn noise<M: Measurement, F: Fn()>(
            g: &mut BenchmarkGroup<M>,
            name: &str,
            readers: usize,
            leasers: usize,
            writers: usize,
            f: F,
        ) {
            let flag = Arc::new(AtomicBool::new(true));
            thread::scope(|s| {
                for _ in 0..readers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            read();
                        }
                    });
                }
                for _ in 0..leasers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            lease();
                        }
                    });
                }
                for _ in 0..writers {
                    s.spawn(|_| {
                        while flag.load(Ordering::Relaxed) {
                            write();
                        }
                    });
                }
                g.bench_function(name, |b| b.iter(&f));
                flag.store(false, Ordering::Relaxed);
            })
            .unwrap();
        }
    };
}

macro_rules! strategy {
    ($name: ident, $type: ty) => {
        mod $name {
            use super::*;

            static A: Lazy<$type> = Lazy::new(|| <$type>::from_pointee(0));
            const NAME: &str = stringify!($name);

            fn lease() {
                for _ in 0..ITERS {
                    black_box(**A.load());
                }
            }

            // Leases kind of degrade in performance if there are multiple on the same thread.
            fn four_leases() {
                for _ in 0..ITERS {
                    let l1 = A.load();
                    let l2 = A.load();
                    let l3 = A.load();
                    let l4 = A.load();
                    black_box((**l1, **l2, **l3, **l4));
                }
            }

            fn read() {
                for _ in 0..ITERS {
                    black_box(A.load_full());
                }
            }

            fn write() {
                for _ in 0..ITERS {
                    black_box(A.store(Arc::new(0)));
                }
            }

            noise!();

            pub fn run_all(c: &mut Criterion) {
                method!(c, read);
                method!(c, write);
                method!(c, lease);
                method!(c, four_leases);
            }
        }
    };
}

strategy!(arc_swap_b, ArcSwap::<usize>);

mod arc_swap_option {
    use super::{black_box, ArcSwapOption, Criterion, Lazy};

    static A: Lazy<ArcSwapOption<usize>> = Lazy::new(|| ArcSwapOption::from(None));
    const NAME: &str = "arc_swap_option";

    fn lease() {
        for _ in 0..ITERS {
            black_box(A.load().as_ref().map(|l| **l).unwrap_or(0));
        }
    }

    fn read() {
        for _ in 0..ITERS {
            black_box(A.load_full().map(|a| -> usize { *a }).unwrap_or(0));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(A.store(Some(Arc::new(0))));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
        method!(c, lease);
    }
}

mod arc_swap_cached {
    use super::{black_box, ArcSwap, Cache, Criterion, Lazy};

    static A: Lazy<ArcSwap<usize>> = Lazy::new(|| ArcSwap::from_pointee(0));
    const NAME: &str = "arc_swap_cached";

    fn read() {
        let mut cache = Cache::from(&A as &ArcSwap<usize>);
        for _ in 0..ITERS {
            black_box(Arc::clone(cache.load()));
        }
    }

    fn lease() {
        for _ in 0..ITERS {
            black_box(**A.load());
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(A.store(Arc::new(0)));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
    }
}

mod mutex {
    use super::{black_box, Criterion, Lazy, Mutex};

    static M: Lazy<Mutex<Arc<usize>>> = Lazy::new(|| Mutex::new(Arc::new(0)));
    const NAME: &str = "mutex";

    fn lease() {
        for _ in 0..ITERS {
            black_box(**M.lock().unwrap());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            black_box(Arc::clone(&*M.lock().unwrap()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(*M.lock().unwrap() = Arc::new(42));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
    }
}

mod parking_mutex {
    use parking_lot::Mutex as ParkingMutex;

    use super::{black_box, Criterion, Lazy};

    static M: Lazy<ParkingMutex<Arc<usize>>> = Lazy::new(|| ParkingMutex::new(Arc::new(0)));
    const NAME: &str = "parking_mutex";

    fn lease() {
        for _ in 0..ITERS {
            black_box(**M.lock());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            black_box(Arc::clone(&*M.lock()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(*M.lock() = Arc::new(42));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
    }
}

mod rwlock {
    use std::sync::RwLock;

    use super::{black_box, Criterion, Lazy};

    static L: Lazy<RwLock<Arc<usize>>> = Lazy::new(|| RwLock::new(Arc::new(0)));
    const NAME: &str = "rwlock";

    fn lease() {
        for _ in 0..ITERS {
            black_box(**L.read().unwrap());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            black_box(Arc::clone(&*L.read().unwrap()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(*L.write().unwrap() = Arc::new(42));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
    }
}

mod parking_rwlock {
    use parking_lot::RwLock;

    use super::{black_box, Criterion, Lazy};

    static L: Lazy<RwLock<Arc<usize>>> = Lazy::new(|| RwLock::new(Arc::new(0)));
    const NAME: &str = "parking_rwlock";

    fn lease() {
        for _ in 0..ITERS {
            black_box(**L.read());
        }
    }

    fn read() {
        for _ in 0..ITERS {
            black_box(Arc::clone(&*L.read()));
        }
    }

    fn write() {
        for _ in 0..ITERS {
            black_box(*L.write() = Arc::new(42));
        }
    }

    noise!();

    pub fn run_all(c: &mut Criterion) {
        method!(c, read);
        method!(c, write);
    }
}

criterion_group!(
    benches,
    arc_swap_b::run_all,
    arc_swap_option::run_all,
    arc_swap_cached::run_all,
    mutex::run_all,
    parking_mutex::run_all,
    rwlock::run_all,
    parking_rwlock::run_all,
);
criterion_main!(benches);
