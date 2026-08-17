use std::ops::Range;

/// Trait for processing the edit-scripts computed with [`diff`](crate::diff)
pub trait Sink: Sized {
    type Out;

    /// This method is called whenever a diff [`algorithm`](crate::Algorithm)
    /// finds a change between the two processed input files.
    /// A change is a continuous subsequence of [tokens](crate::intern::Token) `before` that needs
    /// to be replaced by a different continuous subsequence of tokens `after` to construct the second file from the first.
    ///
    /// These token subsequences are passed to this function in **strictly monotonically increasing order**.
    /// That means that for two subsequent calls `process_change(before1, after1)` and `process_change(before2, after2)`
    /// the following always holds:
    ///
    /// ``` no_compile
    /// assert!(before1.end < before2.start);
    /// assert!(after1.end < after2.start);
    /// ```
    ///
    /// # Parameters
    /// - **`before`** - the **position** of the removed token subsequence in the original file.
    /// - **`after`** - the **position** of the inserted token subsequence in the destination file.
    ///
    /// # Notes
    ////
    /// A `Sink` has no function to indicate that a section of a file remains unchanged.
    /// However due to the monotonically increasing calls, implementations can easily determine
    /// which subsequences remain unchanged by saving `before.end`/`after.end`.
    /// The range between `before.start`/`after.end` and the previous `before.end`/`after.end`
    /// is always unchanged.
    fn process_change(&mut self, before: Range<u32>, after: Range<u32>);

    /// This function is called after all calls to `process_change` are complete
    /// to obtain the final diff result
    fn finish(self) -> Self::Out;

    /// Utility method that constructs a [`Counter`] that tracks the total number
    /// of inserted and removed tokens in the changes passed to [`process_change`](crate::Sink::process_change).
    fn with_counter(self) -> Counter<Self> {
        Counter::new(self)
    }
}

impl<T: FnMut(Range<u32>, Range<u32>)> Sink for T {
    type Out = ();

    fn process_change(&mut self, before: Range<u32>, after: Range<u32>) {
        self(before, after)
    }

    fn finish(self) -> Self::Out {}
}

impl Sink for () {
    type Out = ();
    fn process_change(&mut self, _before: Range<u32>, _after: Range<u32>) {}
    fn finish(self) -> Self::Out {}
}

/// A [`Sink`] which wraps a different sink
/// and counts the number of `removed` and `inserted` [tokens](crate::intern::Token).
pub struct Counter<T> {
    /// Total number of recorded inserted [`tokens`](crate::intern::Token).
    /// Computed by summing the lengths of the `after` subsequences pass to [`process_change`](crate::Sink::process_change).
    pub removals: u32,
    /// Total number of recorded inserted [`tokens`](crate::intern::Token).
    /// Computed by summing the lengths of the `after` subsequences pass to [`process_change`](crate::Sink::process_change).
    pub insertions: u32,
    /// The [`Sink`] for which the counter records [`tokens`](crate::intern::Token).
    /// All calls to [`process_change`](crate::Sink::process_change) are forwarded to the `sink` by the counter.
    /// After [`finish`](crate::Sink::finish) is called, this field contains the output returned by the [`finish`](crate::Sink::finish)
    /// method of the wrapped [`Sink`].
    pub wrapped: T,
}

impl<S: Sink> Counter<S> {
    pub fn new(sink: S) -> Self {
        Self {
            insertions: 0,
            removals: 0,
            wrapped: sink,
        }
    }
}

impl<S: Sink> Sink for Counter<S> {
    type Out = Counter<S::Out>;
    fn process_change(&mut self, before: Range<u32>, after: Range<u32>) {
        self.removals += before.end - before.start;
        self.insertions += after.end - after.start;
        self.wrapped.process_change(before, after)
    }

    fn finish(self) -> Self::Out {
        Counter {
            removals: self.removals,
            insertions: self.insertions,
            wrapped: self.wrapped.finish(),
        }
    }
}

impl<T> Counter<T> {
    pub fn total(&self) -> usize {
        self.insertions as usize + self.removals as usize
    }
}

impl Default for Counter<()> {
    fn default() -> Self {
        Counter::new(())
    }
}
