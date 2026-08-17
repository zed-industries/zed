// Copyright 2016 - 2018 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature="std")]
use core::cell::UnsafeCell;
use core::cmp::min;
use core::mem::size_of;
use core::slice;

use crate::aligned_alloc::Alloc;

use crate::ptr::Ptr;
use crate::util::range_chunk;
use crate::util::round_up_to;

use crate::kernel::Element;
use crate::kernel::GemmKernel;
use crate::kernel::GemmSelect;
#[cfg(feature = "cgemm")]
use crate::kernel::{c32, c64};
use crate::threading::{get_thread_pool, ThreadPoolCtx, LoopThreadConfig};
use crate::sgemm_kernel;
use crate::dgemm_kernel;
#[cfg(feature = "cgemm")]
use crate::cgemm_kernel;
#[cfg(feature = "cgemm")]
use crate::zgemm_kernel;
use rawpointer::PointerExt;

/// General matrix multiplication (f32)
///
/// C ← α A B + β C
///
/// + m, k, n: dimensions
/// + a, b, c: pointer to the first element in the matrix
/// + A: m by k matrix
/// + B: k by n matrix
/// + C: m by n matrix
/// + rs<em>x</em>: row stride of *x*
/// + cs<em>x</em>: col stride of *x*
///
/// Strides for A and B may be arbitrary. Strides for C must not result in
/// elements that alias each other, for example they can not be zero.
///
/// If β is zero, then C does not need to be initialized.
pub unsafe fn sgemm(
    m: usize, k: usize, n: usize,
    alpha: f32,
    a: *const f32, rsa: isize, csa: isize,
    b: *const f32, rsb: isize, csb: isize,
    beta: f32,
    c: *mut f32, rsc: isize, csc: isize)
{
    sgemm_kernel::detect(GemmParameters { m, k, n,
                alpha,
                a, rsa, csa,
                b, rsb, csb,
                beta,
                c, rsc, csc})
}

/// General matrix multiplication (f64)
///
/// C ← α A B + β C
///
/// + m, k, n: dimensions
/// + a, b, c: pointer to the first element in the matrix
/// + A: m by k matrix
/// + B: k by n matrix
/// + C: m by n matrix
/// + rs<em>x</em>: row stride of *x*
/// + cs<em>x</em>: col stride of *x*
///
/// Strides for A and B may be arbitrary. Strides for C must not result in
/// elements that alias each other, for example they can not be zero.
///
/// If β is zero, then C does not need to be initialized.
pub unsafe fn dgemm(
    m: usize, k: usize, n: usize,
    alpha: f64,
    a: *const f64, rsa: isize, csa: isize,
    b: *const f64, rsb: isize, csb: isize,
    beta: f64,
    c: *mut f64, rsc: isize, csc: isize)
{
    dgemm_kernel::detect(GemmParameters { m, k, n,
                alpha,
                a, rsa, csa,
                b, rsb, csb,
                beta,
                c, rsc, csc})
}

/// cgemm/zgemm per-operand options
///
/// TBD.
#[cfg(feature = "cgemm")]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum CGemmOption {
    /// Standard
    Standard,
}

#[cfg(feature = "cgemm")]
/// General matrix multiplication (complex f32)
///
/// C ← α A B + β C
///
/// + m, k, n: dimensions
/// + a, b, c: pointer to the first element in the matrix
/// + A: m by k matrix
/// + B: k by n matrix
/// + C: m by n matrix
/// + rs<em>x</em>: row stride of *x*
/// + cs<em>x</em>: col stride of *x*
///
/// Strides for A and B may be arbitrary. Strides for C must not result in
/// elements that alias each other, for example they can not be zero.
///
/// If β is zero, then C does not need to be initialized.
///
/// Requires crate feature `"cgemm"`
pub unsafe fn cgemm(
    flaga: CGemmOption, flagb: CGemmOption,
    m: usize, k: usize, n: usize,
    alpha: c32,
    a: *const c32, rsa: isize, csa: isize,
    b: *const c32, rsb: isize, csb: isize,
    beta: c32,
    c: *mut c32, rsc: isize, csc: isize)
{
    let _ = (flaga, flagb);
    cgemm_kernel::detect(GemmParameters { m, k, n,
                alpha,
                a, rsa, csa,
                b, rsb, csb,
                beta,
                c, rsc, csc})
}

#[cfg(feature = "cgemm")]
/// General matrix multiplication (complex f64)
///
/// C ← α A B + β C
///
/// + m, k, n: dimensions
/// + a, b, c: pointer to the first element in the matrix
/// + A: m by k matrix
/// + B: k by n matrix
/// + C: m by n matrix
/// + rs<em>x</em>: row stride of *x*
/// + cs<em>x</em>: col stride of *x*
///
/// Strides for A and B may be arbitrary. Strides for C must not result in
/// elements that alias each other, for example they can not be zero.
///
/// If β is zero, then C does not need to be initialized.
///
/// Requires crate feature `"cgemm"`
pub unsafe fn zgemm(
    flaga: CGemmOption, flagb: CGemmOption,
    m: usize, k: usize, n: usize,
    alpha: c64,
    a: *const c64, rsa: isize, csa: isize,
    b: *const c64, rsb: isize, csb: isize,
    beta: c64,
    c: *mut c64, rsc: isize, csc: isize)
{
    let _ = (flaga, flagb);
    zgemm_kernel::detect(GemmParameters { m, k, n,
                alpha,
                a, rsa, csa,
                b, rsb, csb,
                beta,
                c, rsc, csc})
}

struct GemmParameters<T> {
    // Parameters grouped logically in rows
    m: usize, k: usize, n: usize,
    alpha: T,
    a: *const T, rsa: isize, csa: isize,
    beta: T,
    b: *const T, rsb: isize, csb: isize,
    c:   *mut T, rsc: isize, csc: isize,
}

impl<T> GemmSelect<T> for GemmParameters<T> {
    fn select<K>(self, _kernel: K)
       where K: GemmKernel<Elem=T>,
             T: Element,
    {
        // This is where we enter with the configuration specific kernel
        // We could cache kernel specific function pointers here, if we
        // needed to support more constly configuration detection.
        let GemmParameters {
            m, k, n,
            alpha,
            a, rsa, csa,
            b, rsb, csb,
            beta,
            c, rsc, csc} = self;

        unsafe {
            gemm_loop::<K>(
                m, k, n,
                alpha,
                a, rsa, csa,
                b, rsb, csb,
                beta,
                c, rsc, csc)
        }
    }
}


/// Ensure that GemmKernel parameters are supported
/// (alignment, microkernel size).
///
/// This function is optimized out for a supported configuration.
#[inline(always)]
fn ensure_kernel_params<K>()
    where K: GemmKernel
{
    let mr = K::MR;
    let nr = K::NR;
    // These are current limitations,
    // can change if corresponding code in gemm_loop is updated.
    assert!(mr > 0 && mr <= 8);
    assert!(nr > 0 && nr <= 8);
    assert!(mr * nr * size_of::<K::Elem>() <= 8 * 4 * 8);
    assert!(K::align_to() <= 32);
    // one row/col of the kernel is limiting the max align we can provide
    let max_align = size_of::<K::Elem>() * min(mr, nr);
    assert!(K::align_to() <= max_align);

    assert!(K::MR <= K::mc());
    assert!(K::mc() <= K::kc());
    assert!(K::kc() <= K::nc());
    assert!(K::nc() <= 65536);
}

/// Implement matrix multiply using packed buffers and a microkernel
/// strategy, the type parameter `K` is the gemm microkernel.
// no inline is best for the default case, where we support many K per
// gemm entry point. FIXME: make this conditional on feature detection
#[inline(never)]
unsafe fn gemm_loop<K>(
    m: usize, k: usize, n: usize,
    alpha: K::Elem,
    a: *const K::Elem, rsa: isize, csa: isize,
    b: *const K::Elem, rsb: isize, csb: isize,
    beta: K::Elem,
    c: *mut K::Elem, rsc: isize, csc: isize)
    where K: GemmKernel
{
    debug_assert!(m <= 1 || n == 0 || rsc != 0);
    debug_assert!(m == 0 || n <= 1 || csc != 0);

    // if A or B have no elements, compute C ← βC and return
    if m == 0 || k == 0 || n == 0 {
        return c_to_beta_c(m, n, beta, c, rsc, csc);
    }

    let knc = K::nc();
    let kkc = K::kc();
    let kmc = K::mc();
    ensure_kernel_params::<K>();

    let a = Ptr(a);
    let b = Ptr(b);
    let c = Ptr(c);

    let (nthreads, tp) = get_thread_pool();
    let thread_config = LoopThreadConfig::new::<K>(m, k, n, nthreads);
    let nap = thread_config.num_pack_a();

    let (mut packing_buffer, ap_size, bp_size) = make_packing_buffer::<K>(m, k, n, nap);
    let app = Ptr(packing_buffer.ptr_mut());
    let bpp = app.add(ap_size * nap);

    // LOOP 5: split n into nc parts (B, C)
    for (l5, nc) in range_chunk(n, knc) {
        dprint!("LOOP 5, {}, nc={}", l5, nc);
        let b = b.stride_offset(csb, knc * l5);
        let c = c.stride_offset(csc, knc * l5);

        // LOOP 4: split k in kc parts (A, B)
        // This particular loop can't be parallelized because the
        // C chunk (writable) is shared between iterations.
        for (l4, kc) in range_chunk(k, kkc) {
            dprint!("LOOP 4, {}, kc={}", l4, kc);
            let b = b.stride_offset(rsb, kkc * l4);
            let a = a.stride_offset(csa, kkc * l4);

            // Pack B -> B~
            K::pack_nr(kc, nc, slice::from_raw_parts_mut(bpp.ptr(), bp_size),
                       b.ptr(), csb, rsb);

            // First time writing to C, use user's `beta`, else accumulate
            let betap = if l4 == 0 { beta } else { <_>::one() };

            // LOOP 3: split m into mc parts (A, C)
            range_chunk(m, kmc)
                .parallel(thread_config.loop3, tp)
                .thread_local(move |i, _nt| {
                    // a packing buffer A~ per thread
                    debug_assert!(i < nap);
                    app.add(ap_size * i)
                })
                .for_each(move |tp, &mut app, l3, mc| {
                    dprint!("LOOP 3, {}, mc={}", l3, mc);
                    let a = a.stride_offset(rsa, kmc * l3);
                    let c = c.stride_offset(rsc, kmc * l3);

                    // Pack A -> A~
                    K::pack_mr(kc, mc, slice::from_raw_parts_mut(app.ptr(), ap_size),
                               a.ptr(), rsa, csa);

                    // LOOP 2 and 1
                    gemm_packed::<K>(nc, kc, mc,
                                     alpha,
                                     app.to_const(), bpp.to_const(),
                                     betap,
                                     c, rsc, csc,
                                     tp, thread_config);
                });
        }
    }
}

// set up buffer for masked (redirected output of) kernel
const KERNEL_MAX_SIZE: usize = 8 * 8 * 4;
const KERNEL_MAX_ALIGN: usize = 32;
const MASK_BUF_SIZE: usize = KERNEL_MAX_SIZE + KERNEL_MAX_ALIGN - 1;

// Pointers into buffer will be manually aligned anyway, due to
// bugs we have seen on certain platforms (macos) that look like
// we don't get aligned allocations out of TLS - 16- and 8-byte
// allocations have been seen, make the minimal align request we can.
// Align(32) would not work with TLS for s390x.
#[cfg_attr(
    not(any(
        target_os = "macos",
        // Target i686-win7-windows-msvc <https://github.com/rust-lang/rust/issues/138903>
        all(
            target_arch = "x86",
            target_vendor = "win7",
            target_os = "windows",
            target_env = "msvc"
        )
    )),
    repr(align(16))
)]
struct MaskBuffer {
    buffer: [u8; MASK_BUF_SIZE],
}

// Use thread local if we can; this is faster even in the single threaded case because
// it is possible to skip zeroing out the array.
#[cfg(feature = "std")]
thread_local! {
    static MASK_BUF: UnsafeCell<MaskBuffer> =
        UnsafeCell::new(MaskBuffer { buffer: [0; MASK_BUF_SIZE] });
}

/// Loops 1 and 2 around the µ-kernel
///
/// + app: packed A (A~)
/// + bpp: packed B (B~)
/// + nc: columns of packed B
/// + kc: columns of packed A / rows of packed B
/// + mc: rows of packed A
unsafe fn gemm_packed<K>(nc: usize, kc: usize, mc: usize,
                         alpha: K::Elem,
                         app: Ptr<*const K::Elem>, bpp: Ptr<*const K::Elem>,
                         beta: K::Elem,
                         c: Ptr<*mut K::Elem>, rsc: isize, csc: isize,
                         tp: ThreadPoolCtx, thread_config: LoopThreadConfig)
    where K: GemmKernel,
{
    let mr = K::MR;
    let nr = K::NR;
    // check for the mask buffer that fits 8 x 8 f32 and 8 x 4 f64 kernels and alignment
    assert!(mr * nr * size_of::<K::Elem>() <= KERNEL_MAX_SIZE && K::align_to() <= KERNEL_MAX_ALIGN);

    #[cfg(not(feature = "std"))]
    let mut mask_buf = MaskBuffer { buffer: [0; MASK_BUF_SIZE] };

    // LOOP 2: through micropanels in packed `b` (B~, C)
    range_chunk(nc, nr)
        .parallel(thread_config.loop2, tp)
        .thread_local(|_i, _nt| {
            let mut ptr;
            #[cfg(not(feature = "std"))]
            {
                debug_assert_eq!(_nt, 1);
                ptr = mask_buf.buffer.as_mut_ptr();
            }
            #[cfg(feature = "std")]
            {
                ptr = MASK_BUF.with(|buf| (*buf.get()).buffer.as_mut_ptr());
            }
            ptr = align_ptr(K::align_to(), ptr);
            slice::from_raw_parts_mut(ptr as *mut K::Elem, KERNEL_MAX_SIZE / size_of::<K::Elem>())
        })
        .for_each(move |_tp, mask_buf, l2, nr_| {
            let bpp = bpp.stride_offset(1, kc * nr * l2);
            let c = c.stride_offset(csc, nr * l2);

            // LOOP 1: through micropanels in packed `a` while `b` is constant (A~, C)
            for (l1, mr_) in range_chunk(mc, mr) {
                let app = app.stride_offset(1, kc * mr * l1);
                let c = c.stride_offset(rsc, mr * l1);

                // GEMM KERNEL
                // NOTE: For the rust kernels, it performs better to simply
                // always use the masked kernel function!
                if K::always_masked() || nr_ < nr || mr_ < mr {
                    masked_kernel::<_, K>(kc, alpha, app.ptr(), bpp.ptr(),
                                          beta, c.ptr(), rsc, csc,
                                          mr_, nr_, mask_buf);
                    continue;
                } else {
                    K::kernel(kc, alpha, app.ptr(), bpp.ptr(), beta, c.ptr(), rsc, csc);
                }
            }
        });
}

/// Allocate a vector of uninitialized data to be used for both packing buffers.
///
/// + A~ needs be KC x MC
/// + B~ needs be KC x NC
/// but we can make them smaller if the matrix is smaller than this (just ensure
/// we have rounded up to a multiple of the kernel size).
///
/// na: Number of buffers to alloc for A
///
/// Return packing buffer and size of A~ (The offset to B~ is A~ size times `na`), size of B~.
unsafe fn make_packing_buffer<K>(m: usize, k: usize, n: usize, na: usize)
    -> (Alloc<K::Elem>, usize, usize)
    where K: GemmKernel,
{
    // max alignment requirement is a multiple of min(MR, NR) * sizeof<Elem>
    // because apack_size is a multiple of MR, start of b aligns fine
    let m = min(m, K::mc());
    let k = min(k, K::kc());
    let n = min(n, K::nc());
    // round up k, n to multiples of mr, nr
    // round up to multiple of kc
    debug_assert_ne!(na, 0);
    debug_assert!(na <= 128);
    let apack_size = k * round_up_to(m, K::MR);
    let bpack_size = k * round_up_to(n, K::NR);
    let nelem = apack_size * na + bpack_size;

    dprint!("packed nelem={}, apack={}, bpack={},
             m={} k={} n={}, na={}",
             nelem, apack_size, bpack_size,
             m,k,n, na);

    (Alloc::new(nelem, K::align_to()), apack_size, bpack_size)
}

/// offset the ptr forwards to align to a specific byte count
/// Safety: align_to must be a power of two and ptr valid for the pointer arithmetic
#[inline]
unsafe fn align_ptr<T>(mut align_to: usize, mut ptr: *mut T) -> *mut T {
    // always ensure minimal alignment on macos
    if cfg!(target_os = "macos") {
        align_to = Ord::max(align_to, 8);
    }

    if align_to != 0 {
        let cur_align = ptr as usize % align_to;
        if cur_align != 0 {
            ptr = ptr.offset(((align_to - cur_align) / size_of::<T>()) as isize);
        }
    }
    ptr
}

/// Call the GEMM kernel with a "masked" output C.
/// 
/// Simply redirect the MR by NR kernel output to the passed
/// in `mask_buf`, and copy the non masked region to the real
/// C.
///
/// + rows: rows of kernel unmasked
/// + cols: cols of kernel unmasked
#[inline(never)]
unsafe fn masked_kernel<T, K>(k: usize, alpha: T,
                              a: *const T,
                              b: *const T,
                              beta: T,
                              c: *mut T, rsc: isize, csc: isize,
                              rows: usize, cols: usize,
                              mask_buf: &mut [T])
    where K: GemmKernel<Elem=T>, T: Element,
{
    // use column major order for `mask_buf`
    K::kernel(k, alpha, a, b, T::zero(), mask_buf.as_mut_ptr(), 1, K::MR as isize);
    c_to_masked_ab_beta_c::<_, K>(beta, c, rsc, csc, rows, cols, &*mask_buf);
}

/// Copy output in `mask_buf` to the actual c matrix
///
/// C ← M + βC  where M is the `mask_buf`
#[inline]
unsafe fn c_to_masked_ab_beta_c<T, K>(beta: T,
                                      c: *mut T, rsc: isize, csc: isize,
                                      rows: usize, cols: usize,
                                      mask_buf: &[T])
    where K: GemmKernel<Elem=T>, T: Element,
{
    // note: use separate function here with `&T` argument for mask buf,
    // so that the compiler sees that `c` and `mask_buf` never alias.
    let mr = K::MR;
    let nr = K::NR;
    let mut ab = mask_buf.as_ptr();
    for j in 0..nr {
        for i in 0..mr {
            if i < rows && j < cols {
                let cptr = c.stride_offset(rsc, i)
                            .stride_offset(csc, j);
                if beta.is_zero() {
                    *cptr = *ab; // initialize
                } else {
                    (*cptr).mul_assign(beta);
                    (*cptr).add_assign(*ab);
                }
            }
            ab.inc();
        }
    }
}

// Compute just C ← βC
#[inline(never)]
unsafe fn c_to_beta_c<T>(m: usize, n: usize, beta: T,
                         c: *mut T, rsc: isize, csc: isize)
    where T: Element
{
    for i in 0..m {
        for j in 0..n {
            let cptr = c.stride_offset(rsc, i)
                        .stride_offset(csc, j);
            if beta.is_zero() {
                *cptr = T::zero(); // initialize C
            } else {
                (*cptr).mul_assign(beta);
            }
        }
    }
}
