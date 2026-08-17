#[macro_use]
extern crate criterion;

use criterion::{Criterion, Bencher, black_box};
use std::{
    ops::DerefMut,
    sync::Arc,
};

trait Mutex<T>: Send + Sync + 'static {
    type Guard<'a>: DerefMut<Target = T> where Self: 'a;
    fn new(x: T) -> Self;
    fn lock(&self) -> Self::Guard<'_>;
}

impl<T: Send + 'static> Mutex<T> for spin::mutex::SpinMutex<T> {
    type Guard<'a> = spin::mutex::SpinMutexGuard<'a, T> where Self: 'a;
    fn new(x: T) -> Self { spin::mutex::SpinMutex::new(x) }
    fn lock(&self) -> Self::Guard<'_> { self.lock() }
}

impl<T: Send + 'static> Mutex<T> for spin::mutex::TicketMutex<T> {
    type Guard<'a> = spin::mutex::TicketMutexGuard<'a, T> where Self: 'a;
    fn new(x: T) -> Self { spin::mutex::TicketMutex::new(x) }
    fn lock(&self) -> Self::Guard<'_> { self.lock() }
}

impl<T: Send + 'static> Mutex<T> for std::sync::Mutex<T> {
    type Guard<'a> = std::sync::MutexGuard<'a, T> where Self: 'a;
    fn new(x: T) -> Self { std::sync::Mutex::new(x) }
    fn lock(&self) -> Self::Guard<'_> { self.lock().unwrap() }
}

fn gen_create<M: Mutex<u32>>(b: &mut Bencher) {
    b.iter(|| {
        let n = black_box(42);
        M::new(n)
    });
}

fn gen_lock_unlock<M: Mutex<u32>>(b: &mut Bencher) {
    let m = M::new(0);
    b.iter(|| {
        let mut m = m.lock();
        *m = m.wrapping_add(1);
        drop(m);
    });
}

fn gen_lock_unlock_read_contention<M: Mutex<u32>>(b: &mut Bencher) {
    let m = Arc::new(M::new(0));
    let thread = std::thread::spawn({
        let m = m.clone();
        move || {
            while Arc::strong_count(&m) > 1 {
                for _ in 0..1000 {
                    black_box(*m.lock());
                }
            }
        }
    });
    b.iter(|| {
        let mut m = m.lock();
        *m = m.wrapping_add(1);
        drop(m);
    });
    drop(m);
    thread.join().unwrap();
}

fn gen_lock_unlock_write_contention<M: Mutex<u32>>(b: &mut Bencher) {
    let m = Arc::new(M::new(0));
    let thread = std::thread::spawn({
        let m = m.clone();
        move || {
            while Arc::strong_count(&m) > 1 {
                for _ in 0..1000 {
                    let mut m = m.lock();
                    *m = m.wrapping_add(1);
                    drop(m);
                }
            }
        }
    });
    b.iter(|| {
        let mut m = m.lock();
        *m = m.wrapping_add(1);
        drop(m);
    });
    drop(m);
    thread.join().unwrap();
}

fn create(b: &mut Criterion) {
    b.bench_function("create-spin-spinmutex", |b| gen_create::<spin::mutex::SpinMutex<u32>>(b));
    b.bench_function("create-spin-ticketmutex", |b| gen_create::<spin::mutex::TicketMutex<u32>>(b));
    b.bench_function("create-std", |b| gen_create::<std::sync::Mutex<u32>>(b));
}

fn lock_unlock(b: &mut Criterion) {
    b.bench_function("lock_unlock-spin-spinmutex", |b| gen_lock_unlock::<spin::mutex::SpinMutex<u32>>(b));
    b.bench_function("lock_unlock-spin-ticketmutex", |b| gen_lock_unlock::<spin::mutex::TicketMutex<u32>>(b));
    b.bench_function("lock_unlock-std", |b| gen_lock_unlock::<std::sync::Mutex<u32>>(b));
}

fn lock_unlock_read_contention(b: &mut Criterion) {
    b.bench_function("lock_unlock_read_contention-spin-spinmutex", |b| gen_lock_unlock_read_contention::<spin::mutex::SpinMutex<u32>>(b));
    b.bench_function("lock_unlock_read_contention-spin-ticketmutex", |b| gen_lock_unlock_read_contention::<spin::mutex::TicketMutex<u32>>(b));
    b.bench_function("lock_unlock_read_contention-std", |b| gen_lock_unlock_read_contention::<std::sync::Mutex<u32>>(b));
}

fn lock_unlock_write_contention(b: &mut Criterion) {
    b.bench_function("lock_unlock_write_contention-spin-spinmutex", |b| gen_lock_unlock_write_contention::<spin::mutex::SpinMutex<u32>>(b));
    b.bench_function("lock_unlock_write_contention-spin-ticketmutex", |b| gen_lock_unlock_write_contention::<spin::mutex::TicketMutex<u32>>(b));
    b.bench_function("lock_unlock_write_contention-std", |b| gen_lock_unlock_write_contention::<std::sync::Mutex<u32>>(b));
}

criterion_group!(
    mutex,
    create,
    lock_unlock,
    lock_unlock_read_contention,
    lock_unlock_write_contention,
);

criterion_main!(mutex);
