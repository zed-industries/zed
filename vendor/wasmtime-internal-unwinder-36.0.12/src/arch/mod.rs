//! Architecture-specific runtime support corresponding to details of
//! Cranelift codegen or ABI support.
//!
//! This crate houses any architecture-specific tidbits required when
//! building a runtime that executes Cranelift-produced code.
//!
//! All architectures have the same interface when exposed to the rest of the
//! crate.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86;
        use x86 as imp;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        use aarch64 as imp;
    } else if #[cfg(target_arch = "s390x")] {
        mod s390x;
        use s390x as imp;
    } else if #[cfg(target_arch = "riscv64")] {
        mod riscv64;
        use riscv64 as imp;
    }
}

// Re re-export functions from the `imp` module with one set of `pub
// use` declarations here so we can share doc-comments.

cfg_if::cfg_if! {
    if #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "s390x",
        target_arch = "riscv64"
    ))] {
        /// Get the current stack pointer (at the time this function is
        /// executing). This may be used to check, e.g., approximate space
        /// remaining on a stack, but cannot be relied upon for anything exact
        /// because the stack pointer from *within this function* is read and
        /// the frame is later popped.
        pub use imp::get_stack_pointer;

        /// Resume execution at the given PC, SP, and FP, with the given
        /// payload values, according to the tail-call ABI's exception
        /// scheme. Note that this scheme does not restore any other
        /// registers, so the given state is all that we need.
        ///
        /// # Safety
        ///
        /// This method requires:
        ///
        /// - the `sp` and `fp` to correspond to an active stack frame
        ///   (above the current function), in code using Cranelift's
        ///   `tail` calling convention.
        ///
        /// - The `pc` to correspond to a `try_call` handler
        ///   destination, as emitted in Cranelift metadata, or
        ///   otherwise a target that is expecting the tail-call ABI's
        ///   exception ABI.
        ///
        /// - The Rust frames between the unwind destination and this
        ///   frame to be unwind-safe: that is, they cannot have `Drop`
        ///   handlers for which safety requires that they run.
        pub unsafe fn resume_to_exception_handler(
            pc: usize,
            sp: usize,
            fp: usize,
            payload1: usize,
            payload2: usize,
        ) -> ! {
            // Without this ASAN seems to nondeterministically trigger an
            // internal assertion when running tests with threads. Not entirely
            // clear what's going on here but it seems related to the fact that
            // there's Rust code on the stack which is never cleaned up due to
            // the jump out of `imp::resume_to_exception_handler`.
            //
            // This function is documented as something that should be called to
            // clean up the entire thread's shadow memory and stack which isn't
            // exactly what we want but this at least seems to resolve ASAN
            // issues for now. Probably a heavy hammer but better than false
            // positives I suppose...
            #[cfg(asan)]
            {
                unsafe extern "C" {
                    fn __asan_handle_no_return();
                }
                unsafe {
                    __asan_handle_no_return();
                }
            }
            unsafe {
                imp::resume_to_exception_handler(pc, sp, fp, payload1, payload2)
            }
        }

        /// Get the return address in the function at the next-older
        /// frame from the given FP.
        ///
        /// # Safety
        ///
        /// - Requires that `fp` is a valid frame-pointer value for an
        ///   active stack frame (above the current function), in code
        ///   using Cranelift's `tail` calling convention.
        pub use imp::get_next_older_pc_from_fp;


        /// The offset of the saved old-FP value in a frame, from the
        /// location pointed to by a given FP.
        pub const NEXT_OLDER_FP_FROM_FP_OFFSET: usize = imp::NEXT_OLDER_FP_FROM_FP_OFFSET;

        /// The offset of the next older SP value, from the value of a
        /// given FP.
        pub const NEXT_OLDER_SP_FROM_FP_OFFSET: usize = imp::NEXT_OLDER_SP_FROM_FP_OFFSET;

        /// Assert that the given `fp` is aligned as expected by the
        /// host platform's implementation of the Cranelift tail-call
        /// ABI.
        pub use imp::assert_fp_is_aligned;

        /// If we have the above host-specific implementations, we can
        /// implement `Unwind`.
        pub struct UnwindHost;

        unsafe impl crate::stackwalk::Unwind for UnwindHost {
            fn next_older_fp_from_fp_offset(&self) -> usize {
                NEXT_OLDER_FP_FROM_FP_OFFSET
            }
            fn next_older_sp_from_fp_offset(&self) -> usize {
                NEXT_OLDER_SP_FROM_FP_OFFSET
            }
            unsafe fn get_next_older_pc_from_fp(&self, fp: usize) -> usize {
                unsafe {
                    get_next_older_pc_from_fp(fp)
                }
            }
            fn assert_fp_is_aligned(&self, fp: usize) {
                assert_fp_is_aligned(fp)
            }
        }
    }
}
