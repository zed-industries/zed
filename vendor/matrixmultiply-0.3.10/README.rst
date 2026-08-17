matrixmultiply
==============

General matrix multiplication for f32, f64, and complex matrices. Operates on
matrices with general layout (they can use arbitrary row and column stride).

Please read the `API documentation here`__

__ https://docs.rs/matrixmultiply/


We presently provide a few good microkernels, portable and for x86-64 and
AArch64 NEON, and only one operation: the general matrix-matrix multiplication
(“gemm”).

This crate was inspired by the macro/microkernel approach to matrix
multiplication that is used by the BLIS_ project.

.. _BLIS: https://github.com/flame/blis

|crates|_

.. |crates| image:: https://img.shields.io/crates/v/matrixmultiply.svg
.. _crates: https://crates.io/crates/matrixmultiply

Development Goals
-----------------

- Code clarity and maintainability
- Portability and stable Rust
- Performance: provide target-specific microkernels when it is beneficial
- Testing: Test diverse inputs and test and benchmark all microkernels
- Small code footprint and fast compilation
- We are not reimplementing BLAS.

Benchmarks
----------

- ``cargo bench`` is useful for special cases and small matrices
- The best gemm and threading benchmark is is ``examples/benchmarks.rs`` which supports custom sizes,
  some configuration, and csv output.
  Use the script ``benches/benchloop.py`` to run benchmarks over parameter ranges.

Blog Posts About This Crate
---------------------------

+ `gemm: a rabbit hole`__

__ https://bluss.github.io/rust/2016/03/28/a-gemmed-rabbit-hole/

Recent Changes
--------------

- 0.3.10

  - sgemm: Reduce unnecessary AVX register permutations by `@SongXiaoXi <https://github.com/SongXiaoXi>`_ `#88 <https://github.com/bluss/matrixmultiply/pull/88>`_
  - Exclude alignment for MaskBuffer for i686-win7-windows-msvc by `@drewkett <https://github.com/drewkett>`_ `#87 <https://github.com/bluss/matrixmultiply/pull/87>`_
  - Fix cross builder CI and fix a few warnings by `@bluss <https://github.com/bluss>`_ `#89 <https://github.com/bluss/matrixmultiply/pull/89>`_

- 0.3.9

  - Fix debug assertion for alignment on s390x, involving alignment of the mask buffer;
    the change lowered the static alignment request to (16) in thread local storage
    on all platforms (except macos was unchanged).
  - Test with cargo-careful

- 0.3.8

  - Lower alignment requirement for thread local storage value on macos, since
    it's was not respected and caused a debug assertion. (Previous issue #55)

- 0.3.7

  - Rename a directory, avoiding spaces in filenames, to be compatible with
    Bazel. By @xander-zitara

- 0.3.6

  - Fix the build for the combination of cgemm and no_std (#76)

- 0.3.5

  - Significant improvements to complex matrix packing and kernels (#75)

  - Use a specialized AVX2 matrix packing function for sgemm, dgemm when this
    feature is detected on x86-64

- 0.3.4

  - Sgemm, dgemm microkernel implementations for AArch64 NEON (ARM)

    Matrixmultiply now uses autocfg to detect rust version to enable these kernels
    when AArch64 intrinsics are available from Rust 1.61.

  - Small change to matrix packing functions so that they in some cases optimize
    better due to improvements to pointer alias information.

- 0.3.3

  - Attempt to fix macos bug #55 again (manifesting as a debug assertion, only
    in debug builds.)

  - Updated comments for x86 kernels by @Tastaturtaste

  - Updates to MIRI/CI by @jturner314

  - Silenced Send/Sync future compatibility warnings for a raw pointer wrapper

- 0.3.2

  - Add optional feature ``cgemm`` for complex matmult functions ``cgemm`` and
    ``zgemm``

  - Add optional feature ``constconf`` for compile-time configuration of matrix
    kernel parameters for chunking. Improved scripts for benchmarking over ranges
    of different settings. With thanks to @DutchGhost for the const-time
    parsing functions.

  - Improved benchmarking and testing.

  - Threading is now slightly more eager to threads (depending on matrix element count).

- 0.3.1

  - Attempt to fix bug #55 were the mask buffer in TLS did not seem to
    get its requested alignment on macos. The mask buffer pointer is now
    aligned manually (again, like it was in 0.2.x).

  - Fix a minor issue where we were passing a buffer pointer as ``&T``
    when it should have been ``&[T]``.

- 0.3.0

  - Implement initial support for threading using a bespoke thread pool with
    little contention.
    To use, enable feature ``threading`` (and configure number of threads with the
    variable ``MATMUL_NUM_THREADS``).

    Initial support is for up to 4 threads - will be updated with more
    experience in coming versions.

  - Added a better benchmarking program for arbitrary size and layout, see
    ``examples/benchmark.rs`` for this; it supports csv output for better
    recording of measurements

  - Minimum supported rust version is 1.41.1 and the version update policy
    has been updated.

  - Updated to Rust 2018 edition

  - Moved CI to github actions (so long travis and thanks for all the fish).

- 0.2.4

  - Support no-std mode by @vadixidav and @jturner314
    New (default) feature flag "std"; use default-features = false to disable
    and use no-std.
    Note that runtime CPU feature detection requires std.

  - Fix tests so that they build correctly on non-x86 #49 platforms, and manage
    the release by @bluss

- 0.2.3

  - Update rawpointer dependency to 0.2
  - Minor changes to inlining for ``-Ctarget-cpu=native`` use (not recommended -
    use automatic runtime feature detection.
  - Minor improvements to kernel masking (#42, #41) by @bluss and @SuperFluffy

- 0.2.2

  - New dgemm avx and fma kernels implemented by R. Janis Goldschmidt
    (@SuperFluffy). With fast cases for both row and column major output.

    Benchmark improvements: Using fma instructions reduces execution time on
    dgemm benchmarks by 25-35% compared with the avx kernel, see issue `#35`_

    Using the avx dgemm kernel reduces execution time on dgemm benchmarks by
    5-7% compared with the previous version's autovectorized kernel.

  - New fma adaption of the sgemm avx kernel by R. Janis Goldschmidt
    (@SuperFluffy).

    Benchmark improvement: Using fma instructions reduces execution time on
    sgemm benchmarks by 10-15% compared with the avx kernel, see issue `#35`_

  - More flexible kernel selection allows kernels to individually set all
    their parameters, ensures the fallback (plain Rust) kernels can be tuned
    for performance as well, and moves feature detection out of the gemm loop.

    Benchmark improvement: Reduces execution time on various benchmarks
    by 1-2% in the avx kernels, see `#37`_.

  - Improved testing to cover input/output strides of more diversity.

.. _#35: https://github.com/bluss/matrixmultiply/issues/35
.. _#37: https://github.com/bluss/matrixmultiply/issues/37

- 0.2.1

  - Improve matrix packing by taking better advantage of contiguous inputs.

    Benchmark improvement: execution time for 64×64 problem where inputs are either
    both row major or both column major changed by -5% sgemm and -1% for dgemm.
    (#26)

  - In the sgemm avx kernel, handle column major output arrays just like
    it does row major arrays.

    Benchmark improvement: execution time for 32×32 problem where output is column
    major changed by -11%. (#27)

- 0.2.0

  - Use runtime feature detection on x86 and x86-64 platforms, to enable
    AVX-specific microkernels at runtime if available on the currently
    executing configuration.

    This means no special compiler flags are needed to enable native
    instruction performance!

  - Implement a specialized 8×8 sgemm (f32) AVX microkernel, this speeds up
    matrix multiplication by another 25%.

  - Use ``std::alloc`` for allocation of aligned packing buffers

  - We now require Rust 1.28 as the minimal version

- 0.1.15

  - Fix bug where the result matrix C was not updated in the case of a M × K by
    K × N matrix multiplication where K was zero. (This resulted in the output
    C potentially being left uninitialized or with incorrect values in this
    specific scenario.) By @jturner314 (PR #21)

- 0.1.14

  - Avoid an unused code warning

- 0.1.13

  - Pick 8x8 sgemm (f32) kernel when AVX target feature is enabled
    (with Rust 1.14 or later, no effect otherwise).
  - Use ``rawpointer``, a µcrate with raw pointer methods taken from this
    project.

- 0.1.12

  - Internal cleanup with retained performance

- 0.1.11

  - Adjust sgemm (f32) kernel to optimize better on recent Rust.

- 0.1.10

  - Update doc links to docs.rs

- 0.1.9

  - Workaround optimization regression in rust nightly (1.12-ish) (#9)

- 0.1.8

  - Improved docs

- 0.1.7

  - Reduce overhead slightly for small matrix multiplication problems by using
    only one allocation call for both packing buffers.

- 0.1.6

  - Disable manual loop unrolling in debug mode (quicker debug builds)

- 0.1.5

  - Update sgemm to use a 4x8 microkernel (“still in simplistic rust”),
    which improves throughput by 10%.

- 0.1.4

  - Prepare support for aligned packed buffers
  - Update dgemm to use a 8x4 microkernel, still in simplistic rust,
    which improves throughput by 10-20% when using AVX.

- 0.1.3

  - Silence some debug prints

- 0.1.2

  - Major performance improvement for sgemm and dgemm (20-30% when using AVX).
    Since it all depends on what the optimizer does, I'd love to get
    issue reports that report good or bad performance.
  - Made the kernel masking generic, which is a cleaner design

- 0.1.1

  - Minor improvement in the kernel
