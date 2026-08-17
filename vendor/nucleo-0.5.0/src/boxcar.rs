//! Adapted from the `boxcar` crate at <https://github.com/ibraheemdev/boxcar/blob/master/src/raw.rs>
//! under MIT licenes:
//!
//! Copyright (c) 2022 Ibraheem Ahmed
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::{ptr, slice};

use crate::{Item, Utf32String};

const BUCKETS: u32 = u32::BITS - SKIP_BUCKET;
const MAX_ENTRIES: u32 = u32::MAX - SKIP;

/// A lock-free, append-only vector.
pub(crate) struct Vec<T> {
    /// a counter used to retrieve a unique index to push to.
    ///
    /// this value may be more than the true length as it will
    /// be incremented before values are actually stored.
    inflight: AtomicU64,
    /// buckets of length 32, 64 .. 2^31
    buckets: [Bucket<T>; BUCKETS as usize],
    /// the number of matcher columns in this vector, its absolutely critical that
    /// this remains constant and after initilaziaton (safety invariant) since
    /// it is used to calculate the Entry layout
    columns: u32,
}

impl<T> Vec<T> {
    /// Constructs a new, empty `Vec<T>` with the specified capacity and matcher columns.
    pub fn with_capacity(capacity: u32, columns: u32) -> Vec<T> {
        assert_ne!(columns, 0, "there must be atleast one matcher column");
        let init = match capacity {
            0 => 0,
            // initialize enough buckets for `capacity` elements
            n => Location::of(n).bucket,
        };

        let mut buckets = [ptr::null_mut(); BUCKETS as usize];

        for (i, bucket) in buckets[..=init as usize].iter_mut().enumerate() {
            let len = Location::bucket_len(i as u32);
            *bucket = unsafe { Bucket::alloc(len, columns) };
        }

        Vec {
            buckets: buckets.map(Bucket::new),
            inflight: AtomicU64::new(0),
            columns,
        }
    }
    pub fn columns(&self) -> u32 {
        self.columns
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub fn count(&self) -> u32 {
        self.inflight
            .load(Ordering::Acquire)
            .min(MAX_ENTRIES as u64) as u32
    }

    // Returns a reference to the element at the given index.
    //
    // # Safety
    //
    // Entry at `index` must be initialized.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: u32) -> Item<'_, T> {
        let location = Location::of(index);

        unsafe {
            let entries = self
                .buckets
                .get_unchecked(location.bucket as usize)
                .entries
                .load(Ordering::Relaxed);
            debug_assert!(!entries.is_null());
            let entry = Bucket::<T>::get(entries, location.entry, self.columns);
            // this looks odd but is necessary to ensure cross
            // thread synchronization (essentially acting as a memory barrier)
            // since the caller must only guarantee that he has observed active on any thread
            // but the current thread might still have an old value cached (although unlikely)
            let _ = (*entry).active.load(Ordering::Acquire);
            Entry::read(entry, self.columns)
        }
    }

    /// Returns a reference to the element at the given index.
    pub fn get(&self, index: u32) -> Option<Item<'_, T>> {
        let location = Location::of(index);

        unsafe {
            // safety: `location.bucket` is always in bounds
            let entries = self
                .buckets
                .get_unchecked(location.bucket as usize)
                .entries
                .load(Ordering::Relaxed);

            // bucket is uninitialized
            if entries.is_null() {
                return None;
            }

            // safety: `location.entry` is always in bounds for it's bucket
            let entry = Bucket::<T>::get(entries, location.entry, self.columns);

            // safety: the entry is active
            (*entry)
                .active
                .load(Ordering::Acquire)
                .then(|| Entry::read(entry, self.columns))
        }
    }

    /// Appends an element to the back of the vector.
    pub fn push(&self, value: T, fill_columns: impl FnOnce(&T, &mut [Utf32String])) -> u32 {
        let index = self.inflight.fetch_add(1, Ordering::Release);
        // the inflight counter is a `u64` to catch overflows of the vector'scapacity
        let index: u32 = index.try_into().expect("overflowed maximum capacity");
        let location = Location::of(index);

        // eagerly allocate the next bucket if we are close to the end of this one
        if index == (location.bucket_len - (location.bucket_len >> 3)) {
            if let Some(next_bucket) = self.buckets.get(location.bucket as usize + 1) {
                Vec::get_or_alloc(next_bucket, location.bucket_len << 1, self.columns);
            }
        }

        // safety: `location.bucket` is always in bounds
        let bucket = unsafe { self.buckets.get_unchecked(location.bucket as usize) };
        let mut entries = bucket.entries.load(Ordering::Acquire);

        // the bucket has not been allocated yet
        if entries.is_null() {
            entries = Vec::get_or_alloc(bucket, location.bucket_len, self.columns);
        }

        unsafe {
            // safety: `location.entry` is always in bounds for it's bucket
            let entry = Bucket::get(entries, location.entry, self.columns);

            // safety: we have unique access to this entry.
            //
            // 1. it is impossible for another thread to attempt a `push`
            // to this location as we retrieved it from `inflight.fetch_add`
            //
            // 2. any thread trying to `get` this entry will see `active == false`,
            // and will not try to access it
            for col in Entry::matcher_cols_raw(entry, self.columns) {
                col.get().write(MaybeUninit::new(Utf32String::default()))
            }
            fill_columns(&value, Entry::matcher_cols_mut(entry, self.columns));
            (*entry).slot.get().write(MaybeUninit::new(value));
            // let other threads know that this entry is active
            (*entry).active.store(true, Ordering::Release);
        }

        index
    }

    /// race to initialize a bucket
    fn get_or_alloc(bucket: &Bucket<T>, len: u32, cols: u32) -> *mut Entry<T> {
        let entries = unsafe { Bucket::alloc(len, cols) };
        match bucket.entries.compare_exchange(
            ptr::null_mut(),
            entries,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            Ok(_) => entries,
            Err(found) => unsafe {
                Bucket::dealloc(entries, len, cols);
                found
            },
        }
    }

    /// Returns an iterator over the vector starting at `start`
    /// the iterator is deterministically sized and will not grow
    /// as more elements are pushed
    pub unsafe fn snapshot(&self, start: u32) -> Iter<'_, T> {
        let end = self
            .inflight
            .load(Ordering::Acquire)
            .min(MAX_ENTRIES as u64) as u32;
        assert!(start <= end, "index {start} is out of bounds!");
        Iter {
            location: Location::of(start),
            vec: self,
            idx: start,
            end,
        }
    }

    /// Returns an iterator over the vector starting at `start`
    /// the iterator is deterministically sized and will not grow
    /// as more elements are pushed
    pub unsafe fn par_snapshot(&self, start: u32) -> ParIter<'_, T> {
        let end = self
            .inflight
            .load(Ordering::Acquire)
            .min(MAX_ENTRIES as u64) as u32;
        assert!(start <= end, "index {start} is out of bounds!");

        ParIter {
            start,
            end,
            vec: self,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        for (i, bucket) in self.buckets.iter_mut().enumerate() {
            let entries = *bucket.entries.get_mut();

            if entries.is_null() {
                break;
            }

            let len = Location::bucket_len(i as u32);
            // safety: in drop
            unsafe { Bucket::dealloc(entries, len, self.columns) }
        }
    }
}
type SnapshotItem<'v, T> = (u32, Option<Item<'v, T>>);

pub struct Iter<'v, T> {
    location: Location,
    idx: u32,
    end: u32,
    vec: &'v Vec<T>,
}
impl<T> Iter<'_, T> {
    pub fn end(&self) -> u32 {
        self.end
    }
}

impl<'v, T> Iterator for Iter<'v, T> {
    type Item = SnapshotItem<'v, T>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            (self.end - self.idx) as usize,
            Some((self.end - self.idx) as usize),
        )
    }

    fn next(&mut self) -> Option<SnapshotItem<'v, T>> {
        if self.end == self.idx {
            return None;
        }
        debug_assert!(self.idx < self.end, "huh {} {}", self.idx, self.end);
        debug_assert!(self.end as u64 <= self.vec.inflight.load(Ordering::Relaxed));

        loop {
            let entries = unsafe {
                self.vec
                    .buckets
                    .get_unchecked(self.location.bucket as usize)
                    .entries
                    .load(Ordering::Relaxed)
            };
            debug_assert!(self.location.bucket < BUCKETS);

            if self.location.entry < self.location.bucket_len {
                if entries.is_null() {
                    // we still want to yield these
                    let index = self.idx;
                    self.location.entry += 1;
                    self.idx += 1;
                    return Some((index, None));
                }
                // safety: bounds and null checked above
                let entry = unsafe { Bucket::get(entries, self.location.entry, self.vec.columns) };
                let index = self.idx;
                self.location.entry += 1;
                self.idx += 1;

                let entry = unsafe {
                    (*entry)
                        .active
                        .load(Ordering::Acquire)
                        .then(|| Entry::read(entry, self.vec.columns))
                };
                return Some((index, entry));
            }

            self.location.entry = 0;
            self.location.bucket += 1;

            if self.location.bucket < BUCKETS {
                self.location.bucket_len = Location::bucket_len(self.location.bucket);
            }
        }
    }
}
impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct ParIter<'v, T> {
    end: u32,
    start: u32,
    vec: &'v Vec<T>,
}
impl<'v, T> ParIter<'v, T> {
    pub fn end(&self) -> u32 {
        self.end
    }
}

impl<'v, T: Send + Sync> rayon::iter::ParallelIterator for ParIter<'v, T> {
    type Item = SnapshotItem<'v, T>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        rayon::iter::plumbing::bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some((self.end - self.start) as usize)
    }
}

impl<T: Send + Sync> rayon::iter::IndexedParallelIterator for ParIter<'_, T> {
    fn len(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        rayon::iter::plumbing::bridge(self, consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: rayon::iter::plumbing::ProducerCallback<Self::Item>,
    {
        callback.callback(ParIterProducer {
            start: self.start,
            end: self.end,
            vec: self.vec,
        })
    }
}

struct ParIterProducer<'v, T: Send> {
    start: u32,
    end: u32,
    vec: &'v Vec<T>,
}

impl<'v, T: 'v + Send + Sync> rayon::iter::plumbing::Producer for ParIterProducer<'v, T> {
    type Item = SnapshotItem<'v, T>;
    type IntoIter = Iter<'v, T>;

    fn into_iter(self) -> Self::IntoIter {
        debug_assert!(self.start <= self.end);
        Iter {
            location: Location::of(self.start),
            idx: self.start,
            end: self.end,
            vec: self.vec,
        }
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        assert!(index <= (self.end - self.start) as usize);
        let index = index as u32;
        (
            ParIterProducer {
                start: self.start,
                end: self.start + index,
                vec: self.vec,
            },
            ParIterProducer {
                start: self.start + index,
                end: self.end,
                vec: self.vec,
            },
        )
    }
}

struct Bucket<T> {
    entries: AtomicPtr<Entry<T>>,
}

impl<T> Bucket<T> {
    fn layout(len: u32, layout: Layout) -> Layout {
        Layout::from_size_align(layout.size() * len as usize, layout.align())
            .expect("exceeded maximum allocation size")
    }

    unsafe fn alloc(len: u32, cols: u32) -> *mut Entry<T> {
        let layout = Entry::<T>::layout(cols);
        let arr_layout = Self::layout(len, layout);
        let entries = std::alloc::alloc(arr_layout);
        if entries.is_null() {
            std::alloc::handle_alloc_error(arr_layout)
        }

        for i in 0..len {
            let active = entries.add(i as usize * layout.size()) as *mut AtomicBool;
            active.write(AtomicBool::new(false))
        }
        entries as *mut Entry<T>
    }

    unsafe fn dealloc(entries: *mut Entry<T>, len: u32, cols: u32) {
        let layout = Entry::<T>::layout(cols);
        let arr_layout = Self::layout(len, layout);
        for i in 0..len {
            let entry = Bucket::get(entries, i, cols);
            if *(*entry).active.get_mut() {
                ptr::drop_in_place((*(*entry).slot.get()).as_mut_ptr());
                for matcher_col in Entry::matcher_cols_raw(entry, cols) {
                    ptr::drop_in_place((*matcher_col.get()).as_mut_ptr());
                }
            }
        }
        std::alloc::dealloc(entries as *mut u8, arr_layout)
    }

    unsafe fn get(entries: *mut Entry<T>, idx: u32, cols: u32) -> *mut Entry<T> {
        let layout = Entry::<T>::layout(cols);
        let ptr = entries as *mut u8;
        ptr.add(layout.size() * idx as usize) as *mut Entry<T>
    }

    fn new(entries: *mut Entry<T>) -> Bucket<T> {
        Bucket {
            entries: AtomicPtr::new(entries),
        }
    }
}

#[repr(C)]
struct Entry<T> {
    active: AtomicBool,
    slot: UnsafeCell<MaybeUninit<T>>,
    tail: [UnsafeCell<MaybeUninit<Utf32String>>; 0],
}

impl<T> Entry<T> {
    fn layout(cols: u32) -> Layout {
        let head = Layout::new::<Self>();
        let tail = Layout::array::<Utf32String>(cols as usize).expect("invalid memory layout");
        head.extend(tail)
            .expect("invalid memory layout")
            .0
            .pad_to_align()
    }

    unsafe fn matcher_cols_raw<'a>(
        ptr: *mut Entry<T>,
        cols: u32,
    ) -> &'a [UnsafeCell<MaybeUninit<Utf32String>>] {
        // this whole thing looks weird. The reason we do this is that
        // we must make sure the pointer retains its provenance which may (or may not?)
        // be lost if we used tail.as_ptr()
        let tail = std::ptr::addr_of!((*ptr).tail) as *const u8;
        let offset = tail.offset_from(ptr as *mut u8) as usize;
        let ptr = (ptr as *mut u8).add(offset) as *mut _;
        slice::from_raw_parts(ptr, cols as usize)
    }

    unsafe fn matcher_cols_mut<'a>(ptr: *mut Entry<T>, cols: u32) -> &'a mut [Utf32String] {
        // this whole thing looks weird. The reason we do this is that
        // we must make sure the pointer retains its provenance which may (or may not?)
        // be lost if we used tail.as_ptr()
        let tail = std::ptr::addr_of!((*ptr).tail) as *const u8;
        let offset = tail.offset_from(ptr as *mut u8) as usize;
        let ptr = (ptr as *mut u8).add(offset) as *mut _;
        slice::from_raw_parts_mut(ptr, cols as usize)
    }
    // # Safety
    //
    // Value must be initialized.
    unsafe fn read<'a>(ptr: *mut Entry<T>, cols: u32) -> Item<'a, T> {
        // this whole thing looks weird. The reason we do this is that
        // we must make sure the pointer retains its provenance which may (or may not?)
        // be lost if we used tail.as_ptr()
        let data = (*(*ptr).slot.get()).assume_init_ref();
        let tail = std::ptr::addr_of!((*ptr).tail) as *const u8;
        let offset = tail.offset_from(ptr as *mut u8) as usize;
        let ptr = (ptr as *mut u8).add(offset) as *mut _;
        let matcher_columns = slice::from_raw_parts(ptr, cols as usize);
        Item {
            data,
            matcher_columns,
        }
    }
}

#[derive(Debug)]
struct Location {
    // the index of the bucket
    bucket: u32,
    // the length of `bucket`
    bucket_len: u32,
    // the index of the entry in `bucket`
    entry: u32,
}

// skip the shorter buckets to avoid unnecessary allocations.
// this also reduces the maximum capacity of a vector.
const SKIP: u32 = 32;
const SKIP_BUCKET: u32 = (u32::BITS - SKIP.leading_zeros()) - 1;

impl Location {
    fn of(index: u32) -> Location {
        let skipped = index.checked_add(SKIP).expect("exceeded maximum length");
        let bucket = u32::BITS - skipped.leading_zeros();
        let bucket = bucket - (SKIP_BUCKET + 1);
        let bucket_len = Location::bucket_len(bucket);
        let entry = skipped ^ bucket_len;

        Location {
            bucket,
            bucket_len,
            entry,
        }
    }

    fn bucket_len(bucket: u32) -> u32 {
        1 << (bucket + SKIP_BUCKET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location() {
        assert_eq!(Location::bucket_len(0), 32);
        for i in 0..32 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 32);
            assert_eq!(loc.bucket, 0);
            assert_eq!(loc.entry, i);
        }

        assert_eq!(Location::bucket_len(1), 64);
        for i in 33..96 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 64);
            assert_eq!(loc.bucket, 1);
            assert_eq!(loc.entry, i - 32);
        }

        assert_eq!(Location::bucket_len(2), 128);
        for i in 96..224 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 128);
            assert_eq!(loc.bucket, 2);
            assert_eq!(loc.entry, i - 96);
        }

        let max = Location::of(MAX_ENTRIES);
        assert_eq!(max.bucket, BUCKETS - 1);
        assert_eq!(max.bucket_len, 1 << 31);
        assert_eq!(max.entry, (1 << 31) - 1);
    }
}
