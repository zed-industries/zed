///
/// Threading support functions and statics

#[cfg(feature="threading")]
use std::cmp::min;
#[cfg(feature="threading")]
use std::str::FromStr;
#[cfg(feature="threading")]
use once_cell::sync::Lazy;

#[cfg(feature="threading")]
pub use thread_tree::ThreadTree as ThreadPool;
#[cfg(feature="threading")]
pub use thread_tree::ThreadTreeCtx as ThreadPoolCtx;

use crate::kernel::GemmKernel;
use crate::util::RangeChunk;

/// Dummy threadpool
#[cfg(not(feature="threading"))]
pub(crate) struct ThreadPool;

#[cfg(not(feature="threading"))]
pub(crate) type ThreadPoolCtx<'a> = &'a ();

#[cfg(not(feature="threading"))]
impl ThreadPool {
    /// Get top dummy thread pool context
    pub(crate) fn top(&self) -> ThreadPoolCtx<'_> { &() }
}

pub(crate) fn get_thread_pool<'a>() -> (usize, ThreadPoolCtx<'a>) {
    let reg = &*REGISTRY;
    (reg.nthreads, reg.thread_pool().top())
}

struct Registry {
    nthreads: usize,
    #[cfg(feature="threading")]
    thread_pool: Box<ThreadPool>,
}

impl Registry {
    fn thread_pool(&self) -> &ThreadPool {
        #[cfg(feature="threading")]
        return &*REGISTRY.thread_pool;
        #[cfg(not(feature="threading"))]
        return &ThreadPool;
    }
}

#[cfg(not(feature="threading"))]
const REGISTRY: &'static Registry = &Registry { nthreads: 1 };

#[cfg(feature="threading")]
/// Maximum (usefully) supported threads at the moment
const MAX_THREADS: usize = 4;

#[cfg(feature="threading")]
static REGISTRY: Lazy<Registry> = Lazy::new(|| {
    let var = ::std::env::var("MATMUL_NUM_THREADS").ok();
    let threads = match var {
        Some(s) if !s.is_empty() => {
            if let Ok(nt) = usize::from_str(&s) {
                nt
            } else {
                eprintln!("Failed to parse MATMUL_NUM_THREADS");
                1
            }
        }
        _otherwise => num_cpus::get_physical(),
    };

    // Ensure threads in 1 <= threads <= MAX_THREADS
    let threads = 1.max(threads).min(MAX_THREADS);

    let tp = if threads <= 1 {
        Box::new(ThreadPool::new_level0())
    } else if threads <= 3 {
        ThreadPool::new_with_level(1)
    } else {
        ThreadPool::new_with_level(2)
    };

    Registry {
        nthreads: threads,
        thread_pool: tp,
    }
});

/// Describe how many threads we use in each loop
#[derive(Copy, Clone)]
pub(crate) struct LoopThreadConfig {
    /// Loop 3 threads
    pub(crate) loop3: u8,
    /// Loop 2 threads
    pub(crate) loop2: u8,
}

impl LoopThreadConfig {
    /// Decide how many threads to use in each loop
    pub(crate) fn new<K>(m: usize, k: usize, n: usize, max_threads: usize) -> Self
        where K: GemmKernel
    {
        let default_config = LoopThreadConfig { loop3: 1, loop2: 1 };

        #[cfg(not(feature="threading"))]
        {
            let _ = (m, k, n, max_threads); // used
            return default_config;
        }

        #[cfg(feature="threading")]
        {
            if max_threads == 1 {
                return default_config;
            }

            Self::new_impl(m, k, n, max_threads, K::mc())
        }
    }

    #[cfg(feature="threading")]
    fn new_impl(m: usize, k: usize, n: usize, max_threads: usize, kmc: usize) -> Self {
        // use a heuristic to try not to use too many threads for smaller matrices
        let size_factor = m * k + k * n;
        let thread_factor = 1 << 14;
        // pure guesswork in terms of what the default should be
        let arch_factor = if cfg!(target_arch="arm") {
            20
        } else {
            1
        };

        // At the moment only a configuration of 1, 2, or 4 threads is supported.
        //
        // Prefer to split Loop 3 if only 2 threads are available, (because it was better in a
        // square matrix benchmark).

        let matrix_max_threads = size_factor / (thread_factor / arch_factor);
        let mut max_threads = max_threads.min(matrix_max_threads);

        let loop3 = if max_threads >= 2 && m >= 3 * (kmc / 2) {
            max_threads /= 2;
            2
        } else {
            1
        };
        let loop2 = if max_threads >= 2 { 2 } else { 1 };

        LoopThreadConfig {
            loop3,
            loop2,
        }
    }

    /// Number of packing buffers for A
    #[inline(always)]
    pub(crate) fn num_pack_a(&self) -> usize { self.loop3 as usize }
}


impl RangeChunk {
    /// "Builder" method to create a RangeChunkParallel
    pub(crate) fn parallel(self, nthreads: u8, pool: ThreadPoolCtx) -> RangeChunkParallel<fn()> {
        fn nop() {}

        RangeChunkParallel {
            nthreads,
            pool,
            range: self,
            thread_local: nop,
        }
    }
}

/// Intermediate struct for building the parallel execution of a range chunk.
pub(crate) struct RangeChunkParallel<'a, G> {
    range: RangeChunk,
    nthreads: u8,
    pool: ThreadPoolCtx<'a>,
    thread_local: G,
}

impl<'a, G> RangeChunkParallel<'a, G> {
    #[cfg(feature="threading")]
    /// Set thread local setup function - called once per thread to setup thread local data.
    pub(crate) fn thread_local<G2, R>(self, func: G2) -> RangeChunkParallel<'a, G2>
        where G2: Fn(usize, usize) -> R + Sync
    {
        RangeChunkParallel {
            nthreads: self.nthreads,
            pool: self.pool,
            thread_local: func,
            range: self.range,
        }
    }

    #[cfg(not(feature="threading"))]
    /// Set thread local setup function - called once per thread to setup thread local data.
    pub(crate) fn thread_local<G2, R>(self, func: G2) -> RangeChunkParallel<'a, G2>
        where G2: FnOnce(usize, usize) -> R + Sync
    {
        RangeChunkParallel {
            nthreads: self.nthreads,
            pool: self.pool,
            thread_local: func,
            range: self.range,
        }
    }
}

#[cfg(not(feature="threading"))]
impl<G, R> RangeChunkParallel<'_, G>
    where G: FnOnce(usize, usize) -> R + Sync,
{
    pub(crate) fn for_each<F>(self, for_each: F)
        where F: Fn(ThreadPoolCtx<'_>, &mut R, usize, usize) + Sync,
    {
        let mut local = (self.thread_local)(0, 1);
        for (ln, chunk_size) in self.range {
            for_each(self.pool, &mut local, ln, chunk_size)
        }
    }
}


#[cfg(feature="threading")]
impl<G, R> RangeChunkParallel<'_, G>
    where G: Fn(usize, usize) -> R + Sync,
{
    /// Execute loop iterations (parallel if enabled) using the given closure.
    ///
    /// The closure gets the following arguments for each iteration:
    ///
    /// - Thread pool context (used for child threads)
    /// - Mutable reference to thread local data
    /// - index of chunk (like RangeChunk)
    /// - size of chunk (like RangeChunk)
    pub(crate) fn for_each<F>(self, for_each: F)
        where F: Fn(ThreadPoolCtx<'_>, &mut R, usize, usize) + Sync,
    {
        fn inner<F, G, R>(range: RangeChunk, index: usize, nthreads: usize, pool: ThreadPoolCtx<'_>,
                          thread_local: G, for_each: F)
            where G: Fn(usize, usize) -> R + Sync,
                  F: Fn(ThreadPoolCtx<'_>, &mut R, usize, usize) + Sync
        {
            let mut local = thread_local(index, nthreads);
            for (ln, chunk_size) in range.part(index, nthreads) {
                for_each(pool, &mut local, ln, chunk_size)
            }
        }

        debug_assert!(self.nthreads <= 4, "this method does not support nthreads > 4, got {}",
                      self.nthreads);
        let pool = self.pool;
        let range = self.range;
        let for_each = &for_each;
        let local = &self.thread_local;
        let nthreads = min(self.nthreads as usize, 4);
        let f = move |ctx: ThreadPoolCtx<'_>, i| inner(range, i, nthreads, ctx, local, for_each);
        if nthreads >= 4 {
            pool.join4(&f);
        } else if nthreads >= 3 {
            pool.join3l(&f);
        } else if nthreads >= 2 {
            pool.join(|ctx| f(ctx, 0), |ctx| f(ctx, 1));
        } else {
            f(pool, 0)
        }
    }

}

