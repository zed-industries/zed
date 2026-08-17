#![allow(nonstandard_style)]

use core::ffi::c_void;

use cty::{c_char, c_int, c_long, c_ulonglong};

/// The maximum number of bytes which may be used as an argument to a function
/// in the `_small` family ([`mi_malloc_small`], [`mi_zalloc_small`], etc).
pub const MI_SMALL_SIZE_MAX: usize = 128 * core::mem::size_of::<*mut c_void>();

extern "C" {
    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory,
    /// otherwise returns the same as [`mi_malloc(count *
    /// size)`](crate::mi_malloc).
    /// Equivalent to [`mi_calloc`], but returns uninitialized (and not zeroed)
    /// bytes.
    pub fn mi_mallocn(count: usize, size: usize) -> *mut c_void;

    /// Re-allocate memory to `count` elements of `size` bytes.
    ///
    /// The realloc equivalent of the [`mi_mallocn`] interface. Returns `null`
    /// if `count * size` overflows or on out-of-memory, otherwise returns the
    /// same as [`mi_realloc(p, count * size)`](crate::mi_realloc).
    pub fn mi_reallocn(p: *mut c_void, count: usize, size: usize) -> *mut c_void;

    /// Try to re-allocate memory to `newsize` bytes _in place_.
    ///
    /// Returns null on out-of-memory or if the memory could not be expanded in
    /// place. On success, returns the same pointer as `p`.
    ///
    /// If `newsize` is larger than the original `size` allocated for `p`, the
    /// bytes after `size` are uninitialized.
    ///
    /// If null is returned, the original pointer is not freed.
    ///
    /// Note: Conceptually, this is a realloc-like which returns null if it
    /// would be forced to reallocate memory and copy. In practice it's
    /// equivalent testing against [`mi_usable_size`](crate::mi_usable_size).
    pub fn mi_expand(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// This differs from [`mi_realloc`](crate::mi_realloc) in that on failure,
    /// `p` is freed.
    pub fn mi_reallocf(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Allocate and duplicate a nul-terminated C string.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strdup(s: *const c_char) -> *mut c_char;

    /// Allocate and duplicate a nul-terminated C string, up to `n` bytes.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strndup(s: *const c_char, n: usize) -> *mut c_char;

    /// Resolve a file path name, producing a `C` string which can be passed to
    /// [`mi_free`](crate::mi_free).
    ///
    /// `resolved_name` should be null, but can also point to a buffer of at
    /// least `PATH_MAX` bytes.
    ///
    /// If successful, returns a pointer to the resolved absolute file name, or
    /// `null` on failure (with `errno` set to the error code).
    ///
    /// If `resolved_name` was `null`, the returned result should be freed with
    /// [`mi_free`](crate::mi_free).
    ///
    /// This can rarely be useful in FFI code, but is mostly included for
    /// completeness.
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;

    /// Allocate `size * count` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory or if
    /// `size * count` overflows.
    ///
    /// Returns a unique pointer if called with `size * count` 0.
    pub fn mi_calloc_aligned(count: usize, size: usize, alignment: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`.
    ///
    /// Note that the resulting pointer itself is not aligned by the alignment,
    /// but after `offset` bytes it will be. This can be useful for allocating
    /// data with an inline header, where the data has a specific alignment
    /// requirement.
    ///
    /// Specifically, if `p` is the returned pointer `p.add(offset)` is aligned
    /// to `alignment`.
    pub fn mi_malloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`,
    /// zero-initialized.
    ///
    /// This is a [`mi_zalloc`](crate::mi_zalloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_zalloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` of bytes aligned by `alignment` and place the address of the
    /// allocated memory to `ptr`.
    ///
    /// Returns zero on success, invalid argument for invalid alignment, or out-of-memory.
    pub fn mi_posix_memalign(ptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;

    /// Allocate `size` bytes aligned by `alignment` with alignment as the first
    /// parameter.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    pub fn mi_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// Allocate `size * count` bytes aligned by `alignment` at a specified
    /// `offset`, zero-initialized.
    ///
    /// This is a [`calloc`](crate::mi_calloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_calloc_aligned_at(
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes aligned by `alignment` at a
    /// specified `offset`.
    ///
    /// This is a [`realloc`](crate::mi_realloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Zero initialized [re-allocation](crate::mi_realloc).
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`](crate::mi_calloc),
    /// [`mi_zalloc`](crate::mi_zalloc),
    /// [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), ...
    pub fn mi_rezalloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Zero initialized [re-allocation](crate::mi_realloc), following `calloc`
    /// paramater conventions.
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`](crate::mi_calloc),
    /// [`mi_zalloc`](crate::mi_zalloc),
    /// [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), ...
    pub fn mi_recalloc(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;

    /// Aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;

    /// Offset-aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Offset-aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned_at(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Allocate an object of no more than [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_malloc`](crate::mi_malloc) checks if `size` is
    /// small and calls this if
    /// so at runtime, so its' only worth using if you know for certain.
    pub fn mi_malloc_small(size: usize) -> *mut c_void;

    /// Allocate an zero-initialized object of no more than
    /// [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_zalloc`](crate::mi_zalloc) checks if `size` is
    /// small and calls this if so at runtime, so its' only worth using if you
    /// know for certain.
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;

    /// Return the available bytes in a memory block.
    ///
    /// The returned size can be used to call `mi_expand` successfully.
    pub fn mi_usable_size(p: *const c_void) -> usize;

    /// Return the used allocation size.
    ///
    /// Returns the size `n` that will be allocated, where `n >= size`.
    ///
    /// Generally, `mi_usable_size(mi_malloc(size)) == mi_good_size(size)`. This
    /// can be used to reduce internal wasted space when allocating buffers for
    /// example.
    ///
    /// See [`mi_usable_size`](crate::mi_usable_size).
    pub fn mi_good_size(size: usize) -> usize;

    /// Eagerly free memory.
    ///
    /// If `force` is true, aggressively return memory to the OS (can be
    /// expensive!)
    ///
    /// Regular code should not have to call this function. It can be beneficial
    /// in very narrow circumstances; in particular, when a long running thread
    /// allocates a lot of blocks that are freed by other threads it may improve
    /// resource usage by calling this every once in a while.
    pub fn mi_collect(force: bool);

    /// Checked free: If `p` came from mimalloc's heap (as decided by
    /// [`mi_is_in_heap_region`]), this is [`mi_free(p)`](crate::mi_free), but
    /// otherwise it is a no-op.
    pub fn mi_cfree(p: *mut c_void);

    /// Returns true if this is a pointer into a memory region that has been
    /// reserved by the mimalloc heap.
    ///
    /// This function is described by the mimalloc documentation as "relatively
    /// fast".
    ///
    /// See also [`mi_heap_check_owned`], which is (much) slower and slightly
    /// more precise, but only concerns a single `mi_heap`.
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;

    /// Layout-aware deallocation: Like [`mi_free`](crate::mi_free), but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function, and you are not required
    /// to use this to deallocate memory from an aligned allocation function.
    pub fn mi_free_size_aligned(p: *mut c_void, size: usize, alignment: usize);

    /// Size-aware deallocation: Like [`mi_free`](crate::mi_free), but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_size(p: *mut c_void, size: usize);

    /// Alignment-aware deallocation: Like [`mi_free`](crate::mi_free), but
    /// accepts the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_aligned(p: *mut c_void, alignment: usize);

    /// Print the main statistics.
    ///
    /// Ignores the passed in argument, and outputs to the registered output
    /// function or stderr by default.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print(_: *mut c_void);

    /// Print the main statistics.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Reset statistics.
    ///
    /// Note: This function is thread safe.
    pub fn mi_stats_reset();

    /// Merge thread local statistics with the main statistics and reset.
    ///
    /// Note: This function is thread safe.
    pub fn mi_stats_merge();

    /// Return the mimalloc version number.
    ///
    /// For example version 1.6.3 would return the number `163`.
    pub fn mi_version() -> c_int;

    /// Initialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically.
    pub fn mi_thread_init();

    /// Initialize the process.
    ///
    /// Should not be used on most systems, as it's called by thread_init or the
    /// process loader.
    pub fn mi_process_init();

    /// Return process information (time and memory usage). All parameters are
    /// optional (nullable) out-params:
    ///
    /// | Parameter        | Description |
    /// | :-               | :- |
    /// | `elapsed_msecs`  | Elapsed wall-clock time of the process in milli-seconds. |
    /// | `user_msecs`     | User time in milli-seconds (as the sum over all threads). |
    /// | `system_msecs`   | System time in milli-seconds. |
    /// | `current_rss`    | Current working set size (touched pages). |
    /// | `peak_rss`       | Peak working set size (touched pages). |
    /// | `current_commit` | Current committed memory (backed by the page file). |
    /// | `peak_commit`    | Peak committed memory (backed by the page file). |
    /// | `page_faults`    | Count of hard page faults. |
    ///
    /// The `current_rss` is precise on Windows and MacOSX; other systems
    /// estimate this using `current_commit`. The `commit` is precise on Windows
    /// but estimated on other systems as the amount of read/write accessible
    /// memory reserved by mimalloc.
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );

    /// Uninitialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically. Ensures that any memory that is not freed yet (but will
    /// be freed by other threads in the future) is properly handled.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_done();

    /// Print out heap statistics for this thread.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter
    ///
    /// Most detailed when using a debug build.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Register an output function.
    ///
    /// - `out` The output function, use `None` to output to stderr.
    /// - `arg` Argument that will be passed on to the output function.
    ///
    /// The `out` function is called to output any information from mimalloc,
    /// like verbose or warning messages.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);

    /// Register a deferred free function.
    ///
    /// - `deferred_free` Address of a deferred free-ing function or `None` to
    ///   unregister.
    /// - `arg` Argument that will be passed on to the deferred free function.
    ///
    /// Some runtime systems use deferred free-ing, for example when using
    /// reference counting to limit the worst case free time.
    ///
    /// Such systems can register (re-entrant) deferred free function to free
    /// more memory on demand.
    ///
    /// - When the `force` parameter is `true` all possible memory should be
    ///   freed.
    ///
    /// - The per-thread `heartbeat` parameter is monotonically increasing and
    ///   guaranteed to be deterministic if the program allocates
    ///   deterministically.
    ///
    /// - The `deferred_free` function is guaranteed to be called
    ///   deterministically after some number of allocations (regardless of
    ///   freeing or available free memory).
    ///
    /// At most one `deferred_free` function can be active.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_deferred_free(out: mi_deferred_free_fun, arg: *mut c_void);

    /// Register an error callback function.
    ///
    /// The `errfun` function is called on an error in mimalloc after emitting
    /// an error message (through the output function).
    ///
    /// It as always legal to just return from the `errfun` function in which
    /// case allocation functions generally return null or ignore the condition.
    ///
    /// The default function only calls abort() when compiled in secure mode
    /// with an `EFAULT` error. The possible error codes are:
    ///
    /// - `EAGAIN` (11): Double free was detected (only in debug and secure
    ///   mode).
    /// - `EFAULT` (14): Corrupted free list or meta-data was detected (only in
    ///   debug and secure mode).
    /// - `ENOMEM` (12): Not enough memory available to satisfy the request.
    /// - `EOVERFLOW` (75): Too large a request, for example in `mi_calloc`, the
    ///   `count` and `size` parameters are too large.
    /// - `EINVAL` (22): Trying to free or re-allocate an invalid pointer.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_error(out: mi_error_fun, arg: *mut c_void);
}

/// An output callback. Must be thread-safe.
///
/// See [`mi_stats_print_out`], [`mi_thread_stats_print_out`], [`mi_register_output`]
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;

/// Type of deferred free functions. Must be thread-safe.
///
/// - `force`: If true, all outstanding items should be freed.
/// - `heartbeat` A monotonically increasing count.
/// - `arg` Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_deferred_free`]
pub type mi_deferred_free_fun =
    Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;

/// Type of error callback functions. Must be thread-safe.
///
/// - `err`: Error code (see [`mi_register_error`] for a list).
/// - `arg`: Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_error`]
pub type mi_error_fun = Option<unsafe extern "C" fn(code: c_int, arg: *mut c_void)>;

/// Runtime options. All options are false by default.
pub type mi_option_t = c_int;

#[cfg(feature = "arena")]
/// Arena Id
pub type mi_arena_id_t = c_int;

// Note: mimalloc doc website seems to have the order of show_stats and
// show_errors reversed as of 1.6.3, however what I have here is correct:
// https://github.com/microsoft/mimalloc/issues/266#issuecomment-653822341

/// Print error messages to `stderr`.
pub const mi_option_show_errors: mi_option_t = 0;

/// Print statistics to `stderr` when the program is done.
pub const mi_option_show_stats: mi_option_t = 1;

/// Print verbose messages to `stderr`.
pub const mi_option_verbose: mi_option_t = 2;

/// ### The following options are experimental
///
/// Option (experimental) Use large OS pages (2MiB in size) if possible.
///
/// Use large OS pages (2MiB) when available; for some workloads this can
/// significantly improve performance. Use mi_option_verbose to check if
/// the large OS pages are enabled -- usually one needs to explicitly allow
/// large OS pages (as on Windows and Linux). However, sometimes the OS is
/// very slow to reserve contiguous physical memory for large OS pages so
/// use with care on systems that can have fragmented memory (for that
/// reason, we generally recommend to use mi_option_reserve_huge_os_pages
/// instead whenever possible).
pub const mi_option_large_os_pages: mi_option_t = 6;

/// Option (experimental) The number of huge OS pages (1GiB in size) to reserve at the start of the program.
///
/// This reserves the huge pages at startup and sometimes this can give a large (latency) performance
/// improvement on big workloads. Usually it is better to not use MIMALLOC_LARGE_OS_PAGES in
/// combination with this setting. Just like large OS pages, use with care as reserving contiguous
/// physical memory can take a long time when memory is fragmented (but reserving the huge pages is
/// done at startup only once). Note that we usually need to explicitly enable huge OS pages (as on
/// Windows and Linux)). With huge OS pages, it may be beneficial to set the setting
/// mi_option_eager_commit_delay=N (N is 1 by default) to delay the initial N segments (of 4MiB) of
/// a thread to not allocate in the huge OS pages; this prevents threads that are short lived and
/// allocate just a little to take up space in the huge OS page area (which cannot be reset).
pub const mi_option_reserve_huge_os_pages: mi_option_t = 7;

/// Option (experimental) Reserve huge OS pages at node N.
///
/// The huge pages are usually allocated evenly among NUMA nodes.
/// You can use mi_option_reserve_huge_os_pages_at=N where `N` is the numa node (starting at 0) to allocate all
/// the huge pages at a specific numa node instead.
pub const mi_option_reserve_huge_os_pages_at: mi_option_t = 8;

/// Option (experimental) Reserve specified amount of OS memory at startup, e.g. "1g" or "512m".
pub const mi_option_reserve_os_memory: mi_option_t = 9;

/// Option (experimental) the first N segments per thread are not eagerly committed (=1).
pub const mi_option_eager_commit_delay: mi_option_t = 14;

/// Option (experimental) Pretend there are at most N NUMA nodes; Use 0 to use the actual detected NUMA nodes at runtime.
pub const mi_option_use_numa_nodes: mi_option_t = 16;

/// Option (experimental) If set to 1, do not use OS memory for allocation (but only pre-reserved arenas)
pub const mi_option_limit_os_alloc: mi_option_t = 17;

/// Option (experimental) OS tag to assign to mimalloc'd memory
pub const mi_option_os_tag: mi_option_t = 18;

/// Option (experimental)
pub const mi_option_max_errors: mi_option_t = 19;

/// Option (experimental)
pub const mi_option_max_warnings: mi_option_t = 20;

#[cfg(not(feature = "v3"))]
/// Option (experimental)
pub const mi_option_max_segment_reclaim: mi_option_t = 21;

/// Last option.
#[cfg(not(feature = "v3"))]
pub const _mi_option_last: mi_option_t = 37;
#[cfg(feature = "v3")]
pub const _mi_option_last: mi_option_t = 43;

extern "C" {
    // Note: mi_option_{enable,disable} aren't exposed because they're redundant
    // and because of https://github.com/microsoft/mimalloc/issues/266.

    /// Returns true if the provided option is enabled.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;

    /// Enable or disable the given option.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], enables or disables the option. If it has,
    /// this function does nothing.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);

    /// Returns the value of the provided option.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed, see the mimalloc documentation for
    /// details: https://microsoft.github.io/mimalloc/group__options.html
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_get(option: mi_option_t) -> c_long;

    /// Set the option to the given value.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed,
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set(option: mi_option_t, value: c_long);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], sets the option to the given value. If it
    /// has, this function does nothing.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
}

/// First-class heaps that can be destroyed in one go.
///
/// Note: The pointers allocated out of a heap can be be freed using
/// [`mi_free`](crate::mi_free) -- there is no `mi_heap_free`.
///
/// # Example
///
/// ```
/// use libmimalloc_sys as mi;
/// unsafe {
///     let h = mi::mi_heap_new();
///     assert!(!h.is_null());
///     let p = mi::mi_heap_malloc(h, 50);
///     assert!(!p.is_null());
///
///     // use p...
///     mi::mi_free(p);
///
///     // Clean up the heap. Note that pointers allocated from `h`
///     // are *not* invalided by `mi_heap_delete`. You would have
///     // to use (the very dangerous) `mi_heap_destroy` for that
///     // behavior
///     mi::mi_heap_delete(h);
/// }
/// ```
pub enum mi_heap_t {}

/// An area of heap space contains blocks of a single size.
///
/// The bytes in freed blocks are `committed - used`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mi_heap_area_t {
    /// Start of the area containing heap blocks.
    pub blocks: *mut c_void,
    /// Bytes reserved for this area.
    pub reserved: usize,
    /// Current committed bytes of this area.
    pub committed: usize,
    /// Bytes in use by allocated blocks.
    pub used: usize,
    /// Size in bytes of one block.
    pub block_size: usize,
    /// Size in bytes of a full block including padding and metadata.
    pub full_block_size: usize,
    /// Heap tag associated with this area
    pub heap_tag: i32,
}

/// Visitor function passed to [`mi_heap_visit_blocks`]
///
/// Should return `true` to continue, and `false` to stop visiting (i.e. break)
///
/// This function is always first called for every `area` with `block` as a null
/// pointer. If `visit_all_blocks` was `true`, the function is then called for
/// every allocated block in that area.
pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: usize,
        arg: *mut c_void,
    ) -> bool,
>;

extern "C" {
    /// Create a new heap that can be used for allocation.
    pub fn mi_heap_new() -> *mut mi_heap_t;

    /// Delete a previously allocated heap.
    ///
    /// This will release resources and migrate any still allocated blocks in
    /// this heap (efficienty) to the default heap.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_delete(heap: *mut mi_heap_t);

    /// Destroy a heap, freeing all its still allocated blocks.
    ///
    /// Use with care as this will free all blocks still allocated in the heap.
    /// However, this can be a very efficient way to free all heap memory in one
    /// go.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);

    /// Set the default heap to use for [`mi_malloc`](crate::mi_malloc) et al.
    ///
    /// Returns the previous default heap.
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;

    /// Get the default heap that is used for [`mi_malloc`](crate::mi_malloc) et al.
    pub fn mi_heap_get_default() -> *mut mi_heap_t;

    /// Get the backing heap.
    ///
    /// The _backing_ heap is the initial default heap for a thread and always
    /// available for allocations. It cannot be destroyed or deleted except by
    /// exiting the thread.
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;

    /// Release outstanding resources in a specific heap.
    ///
    /// See also [`mi_collect`].
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);

    /// Equivalent to [`mi_malloc`](crate::mi_malloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_zalloc`](crate::mi_zalloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_calloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_mallocn`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_malloc_small`], but allocates out of the specific
    /// heap instead of the default.
    ///
    /// `size` must be smaller or equal to [`MI_SMALL_SIZE_MAX`].
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_realloc`](crate::mi_realloc), but allocates out of
    /// the specific heap instead of the default.
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_reallocn`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_reallocf`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_strdup`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;

    /// Equivalent to [`mi_strndup`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: usize) -> *mut c_char;

    /// Equivalent to [`mi_realpath`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;

    /// Equivalent to [`mi_malloc_aligned`](crate::mi_malloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_malloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned`], but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned`](crate::mi_realloc_aligned), but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_recalloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned`], but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Does a heap contain a pointer to a previously allocated block?
    ///
    /// `p` must be a pointer to a previously allocated block (in any heap) -- it cannot be some
    /// random pointer!
    ///
    /// Returns `true` if the block pointed to by `p` is in the `heap`.
    ///
    /// See [`mi_heap_check_owned`].
    pub fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Check safely if any pointer is part of a heap.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// given heap or any other mimalloc heap. Returns `true` if `p` points to a
    /// block in the given heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`mi_heap_contains_block`], [`mi_heap_get_default`], and
    /// [`mi_is_in_heap_region`]
    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Check safely if any pointer is part of the default heap of this thread.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// default heap for this thread, or any other mimalloc heap. Returns `true`
    /// if `p` points to a block in the default heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`mi_heap_contains_block`], [`mi_heap_get_default`]
    pub fn mi_check_owned(p: *const c_void) -> bool;

    /// Visit all areas and blocks in `heap`.
    ///
    /// If `visit_all_blocks` is false, the `visitor` is only called once for
    /// every heap area. If it's true, the `visitor` is also called for every
    /// allocated block inside every area (with `!block.is_null()`). Return
    /// `false` from the `visitor` to return early.
    ///
    /// `arg` is an extra argument passed into the `visitor`.
    ///
    /// Returns `true` if all areas and blocks were visited.
    ///
    /// Passing a `None` visitor is allowed, and is a no-op.
    pub fn mi_heap_visit_blocks(
        heap: *const mi_heap_t,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;

    #[cfg(feature = "arena")]
    /// Create a heap that only allocates in the specified arena
    pub fn mi_heap_new_in_arena(arena_id: mi_arena_id_t) -> *mut mi_heap_t;

    #[cfg(feature = "arena")]
    /// Reserve OS memory for use by mimalloc. Reserved areas are used
    /// before allocating from the OS again. By reserving a large area upfront,
    /// allocation can be more efficient, and can be better managed on systems
    /// without `mmap`/`VirtualAlloc` (like WASM for example).
    ///
    /// - `size` The size to reserve.
    /// - `commit` Commit the memory upfront.
    /// - `allow_large` Allow large OS pages (2MiB) to be used?
    /// - `exclusive` Only allow allocations if specifically for this arena.
    /// - `arena_id` Pointer who's value will be set to the new arena_id if successful.
    ///
    /// Returns 0 if successful, and an error code otherwise (e.g. `ENOMEM`)
    pub fn mi_reserve_os_memory_ex(
        size: usize,
        commit: bool,
        allow_large: bool,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> c_int;

    #[cfg(feature = "arena")]
    /// Manage a particular memory area for use by mimalloc.
    /// This is just like `mi_reserve_os_memory_ex` except that the area should already be
    /// allocated in some manner and available for use my mimalloc.
    ///
    /// # Safety
    /// mimalloc will likely segfault when allocating from the arena if the arena `start` & `size`
    /// aren't aligned with mimalloc's `MI_SEGMENT_ALIGN` (e.g. 32MB on x86_64 machines).
    ///
    /// - `start` Start of the memory area
    /// - `size` The size of the memory area. Must be large than `MI_ARENA_BLOCK_SIZE` (e.g. 64MB
    ///          on x86_64 machines).
    /// - `commit` Set true if the memory range is already commited.
    /// - `is_large` Set true if the memory range consists of large files, or if the memory should
    ///              not be decommitted or protected (like rdma etc.).
    /// - `is_zero` Set true if the memory range consists only of zeros.
    /// - `numa_node` Possible associated numa node or `-1`.
    /// - `exclusive` Only allow allocations if specifically for this arena.
    /// - `arena_id` Pointer who's value will be set to the new arena_id if successful.
    ///
    /// Returns `true` if arena was successfully allocated
    pub fn mi_manage_os_memory_ex(
        start: *const c_void,
        size: usize,
        is_committed: bool,
        is_large: bool,
        is_zero: bool,
        numa_node: c_int,
        exclusive: bool,
        arena_id: *mut mi_arena_id_t,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_usable_size() {
        let ptr = unsafe { mi_malloc(32) } as *mut u8;
        let usable_size = unsafe { mi_usable_size(ptr as *mut c_void) };
        assert!(
            usable_size >= 32,
            "usable_size should at least equal to the allocated size"
        );
    }

    #[test]
    fn runtime_stable_option() {
        unsafe {
            assert_eq!(mi_option_get(mi_option_show_errors), 0);
            mi_option_set(mi_option_show_errors, 1);
            assert_eq!(mi_option_get(mi_option_show_errors), 1);

            assert_eq!(mi_option_get(mi_option_show_stats), 0);
            mi_option_set(mi_option_show_stats, 1);
            assert_eq!(mi_option_get(mi_option_show_stats), 1);

            assert_eq!(mi_option_get(mi_option_verbose), 0);
            mi_option_set(mi_option_verbose, 1);
            assert_eq!(mi_option_get(mi_option_verbose), 1);
        }
    }
}
