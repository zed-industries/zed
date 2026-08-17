use super::{IndexedParallelIterator, ParallelIterator};

mod consumer;
use self::consumer::CollectConsumer;
use self::consumer::CollectResult;
use super::unzip::unzip_indexed;

mod test;

/// Collects the results of the exact iterator into the specified vector.
///
/// This is called by `IndexedParallelIterator::collect_into_vec`.
pub(super) fn collect_into_vec<I, T>(pi: I, v: &mut Vec<T>)
where
    I: IndexedParallelIterator<Item = T>,
    T: Send,
{
    v.truncate(0); // clear any old data
    let len = pi.len();
    collect_with_consumer(v, len, |consumer| pi.drive(consumer));
}

/// Collects the results of the iterator into the specified vector.
///
/// Technically, this only works for `IndexedParallelIterator`, but we're faking a
/// bit of specialization here until Rust can do that natively.  Callers are
/// using `opt_len` to find the length before calling this, and only exact
/// iterators will return anything but `None` there.
///
/// Since the type system doesn't understand that contract, we have to allow
/// *any* `ParallelIterator` here, and `CollectConsumer` has to also implement
/// `UnindexedConsumer`.  That implementation panics `unreachable!` in case
/// there's a bug where we actually do try to use this unindexed.
pub(super) fn special_extend<I, T>(pi: I, len: usize, v: &mut Vec<T>)
where
    I: ParallelIterator<Item = T>,
    T: Send,
{
    collect_with_consumer(v, len, |consumer| pi.drive_unindexed(consumer));
}

/// Unzips the results of the exact iterator into the specified vectors.
///
/// This is called by `IndexedParallelIterator::unzip_into_vecs`.
pub(super) fn unzip_into_vecs<I, A, B>(pi: I, left: &mut Vec<A>, right: &mut Vec<B>)
where
    I: IndexedParallelIterator<Item = (A, B)>,
    A: Send,
    B: Send,
{
    // clear any old data
    left.truncate(0);
    right.truncate(0);

    let len = pi.len();
    collect_with_consumer(right, len, |right_consumer| {
        let mut right_result = None;
        collect_with_consumer(left, len, |left_consumer| {
            let (left_r, right_r) = unzip_indexed(pi, left_consumer, right_consumer);
            right_result = Some(right_r);
            left_r
        });
        right_result.unwrap()
    });
}

/// Create a consumer on the slice of memory we are collecting into.
///
/// The consumer needs to be used inside the scope function, and the
/// complete collect result passed back.
///
/// This method will verify the collect result, and panic if the slice
/// was not fully written into. Otherwise, in the successful case,
/// the vector is complete with the collected result.
fn collect_with_consumer<T, F>(vec: &mut Vec<T>, len: usize, scope_fn: F)
where
    T: Send,
    F: FnOnce(CollectConsumer<'_, T>) -> CollectResult<'_, T>,
{
    // Reserve space for `len` more elements in the vector,
    vec.reserve(len);

    // Create the consumer and run the callback for collection.
    let result = scope_fn(CollectConsumer::appender(vec, len));

    // The `CollectResult` represents a contiguous part of the slice, that has
    // been written to. On unwind here, the `CollectResult` will be dropped. If
    // some producers on the way did not produce enough elements, partial
    // `CollectResult`s may have been dropped without being reduced to the final
    // result, and we will see that as the length coming up short.
    //
    // Here, we assert that added length is fully initialized. This is checked
    // by the following assert, which verifies if a complete `CollectResult`
    // was produced; if the length is correct, it is necessarily covering the
    // target slice. Since we know that the consumer cannot have escaped from
    // `drive` (by parametricity, essentially), we know that any stores that
    // will happen, have happened. Unless some code is buggy, that means we
    // should have seen `len` total writes.
    let actual_writes = result.len();
    assert!(
        actual_writes == len,
        "expected {len} total writes, but got {actual_writes}"
    );

    // Release the result's mutable borrow and "proxy ownership"
    // of the elements, before the vector takes it over.
    result.release_ownership();

    let new_len = vec.len() + len;

    unsafe {
        vec.set_len(new_len);
    }
}
