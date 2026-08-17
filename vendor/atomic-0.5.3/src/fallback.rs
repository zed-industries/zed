// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::cmp;
use core::hint;
use core::mem;
use core::num::Wrapping;
use core::ops;
use core::ptr;
use core::slice;
use core::sync::atomic::{AtomicUsize, Ordering};

// We use an AtomicUsize instead of an AtomicBool because it performs better
// on architectures that don't have byte-sized atomics.
//
// We give each spinlock its own cache line to avoid false sharing.
#[repr(align(64))]
struct SpinLock(AtomicUsize);

impl SpinLock {
    fn lock(&self) {
        while self
            .0
            .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.0.load(Ordering::Relaxed) != 0 {
                hint::spin_loop();
            }
        }
    }

    fn unlock(&self) {
        self.0.store(0, Ordering::Release);
    }
}

// A big array of spinlocks which we use to guard atomic accesses. A spinlock is
// chosen based on a hash of the address of the atomic object, which helps to
// reduce contention compared to a single global lock.
macro_rules! array {
    (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
        => {array!(@as_expr [$($body)*])};
    (@accum (1, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)*))};
    (@accum (2, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (4, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (8, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (16, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (32, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (64, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};

    (@as_expr $e:expr) => {$e};

    [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
}
static SPINLOCKS: [SpinLock; 64] = array![SpinLock(AtomicUsize::new(0)); 64];

// Spinlock pointer hashing function from compiler-rt
#[inline]
fn lock_for_addr(addr: usize) -> &'static SpinLock {
    // Disregard the lowest 4 bits.  We want all values that may be part of the
    // same memory operation to hash to the same value and therefore use the same
    // lock.
    let mut hash = addr >> 4;
    // Use the next bits as the basis for the hash
    let low = hash & (SPINLOCKS.len() - 1);
    // Now use the high(er) set of bits to perturb the hash, so that we don't
    // get collisions from atomic fields in a single object
    hash >>= 16;
    hash ^= low;
    // Return a pointer to the lock to use
    &SPINLOCKS[hash & (SPINLOCKS.len() - 1)]
}

#[inline]
fn lock(addr: usize) -> LockGuard {
    let lock = lock_for_addr(addr);
    lock.lock();
    LockGuard(lock)
}

struct LockGuard(&'static SpinLock);
impl Drop for LockGuard {
    #[inline]
    fn drop(&mut self) {
        self.0.unlock();
    }
}

#[inline]
pub unsafe fn atomic_load<T>(dst: *mut T) -> T {
    let _l = lock(dst as usize);
    ptr::read(dst)
}

#[inline]
pub unsafe fn atomic_store<T>(dst: *mut T, val: T) {
    let _l = lock(dst as usize);
    ptr::write(dst, val);
}

#[inline]
pub unsafe fn atomic_swap<T>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    ptr::replace(dst, val)
}

#[inline]
pub unsafe fn atomic_compare_exchange<T>(dst: *mut T, current: T, new: T) -> Result<T, T> {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    // compare_exchange compares with memcmp instead of Eq
    let a = slice::from_raw_parts(&result as *const _ as *const u8, mem::size_of_val(&result));
    let b = slice::from_raw_parts(
        &current as *const _ as *const u8,
        mem::size_of_val(&current),
    );
    if a == b {
        ptr::write(dst, new);
        Ok(result)
    } else {
        Err(result)
    }
}

#[inline]
pub unsafe fn atomic_add<T: Copy>(dst: *mut T, val: T) -> T
where
    Wrapping<T>: ops::Add<Output = Wrapping<T>>,
{
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, (Wrapping(result) + Wrapping(val)).0);
    result
}

#[inline]
pub unsafe fn atomic_sub<T: Copy>(dst: *mut T, val: T) -> T
where
    Wrapping<T>: ops::Sub<Output = Wrapping<T>>,
{
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, (Wrapping(result) - Wrapping(val)).0);
    result
}

#[inline]
pub unsafe fn atomic_and<T: Copy + ops::BitAnd<Output = T>>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, result & val);
    result
}

#[inline]
pub unsafe fn atomic_or<T: Copy + ops::BitOr<Output = T>>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, result | val);
    result
}

#[inline]
pub unsafe fn atomic_xor<T: Copy + ops::BitXor<Output = T>>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, result ^ val);
    result
}

#[inline]
pub unsafe fn atomic_min<T: Copy + cmp::Ord>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, cmp::min(result, val));
    result
}

#[inline]
pub unsafe fn atomic_max<T: Copy + cmp::Ord>(dst: *mut T, val: T) -> T {
    let _l = lock(dst as usize);
    let result = ptr::read(dst);
    ptr::write(dst, cmp::max(result, val));
    result
}
