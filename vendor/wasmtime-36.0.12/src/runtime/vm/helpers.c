// When using _FORTIFY_SOURCE with `longjmp` causes longjmp_chk to be used
// instead. longjmp_chk ensures that the jump target is on the existing stack.
// For our use case of jumping between stacks we need to disable it.
#undef _FORTIFY_SOURCE

#include <setjmp.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if (defined(__GNUC__) && !defined(__clang__))
#define WASMTIME_GCC 1
#endif

#ifdef CFG_TARGET_OS_windows

// Windows is required to use normal `setjmp` and `longjmp`.
#define platform_setjmp(buf) setjmp(buf)
#define platform_longjmp(buf, arg) longjmp(buf, arg)
typedef jmp_buf platform_jmp_buf;

#elif defined(WASMTIME_GCC) || defined(__x86_64__)

// clang-format off

// GCC and Clang on x86_64 provide `__builtin_setjmp`/`__builtin_longjmp`, which
// differ from plain `setjmp` and `longjmp` in that they're implemented by
// the compiler inline rather than in libc, and the compiler can avoid saving
// and restoring most of the registers. See the [GCC docs] and [clang docs]
// for more information.
//
// Per the caveat in the GCC docs, this assumes that the host compiler (which
// may be compiling for a generic architecture family) knows about all the
// register state that Cranelift (which may be specializing for the hardware at
// runtime) is assuming is callee-saved.
//
// [GCC docs]: https://gcc.gnu.org/onlinedocs/gcc/Nonlocal-Gotos.html
// [clang docs]: https://llvm.org/docs/ExceptionHandling.html#llvm-eh-sjlj-setjmp

// clang-format on
#define platform_setjmp(buf) __builtin_setjmp(buf)
#define platform_longjmp(buf, arg) __builtin_longjmp(buf, arg)
typedef void *platform_jmp_buf[5]; // this is the documented size; see the docs
                                   // links for details.

#else

// All other platforms/compilers funnel in here.
//
// Note that `sigsetjmp` and `siglongjmp` are used here where possible to
// explicitly pass a 0 argument to `sigsetjmp` that we don't need to preserve
// the process signal mask. This should make this call a bit faster b/c it
// doesn't need to touch the kernel signal handling routines.
#define platform_setjmp(buf) sigsetjmp(buf, 0)
#define platform_longjmp(buf, arg) siglongjmp(buf, arg)
typedef sigjmp_buf platform_jmp_buf;

#endif

#define CONCAT2(a, b) a##b
#define CONCAT(a, b) CONCAT2(a, b)
#define VERSIONED_SYMBOL(a) CONCAT(a, VERSIONED_SUFFIX)

bool VERSIONED_SYMBOL(wasmtime_setjmp)(void **buf_storage,
                                       bool (*body)(void *, void *),
                                       void *payload, void *callee) {
  platform_jmp_buf buf;
  if (platform_setjmp(buf) != 0) {
    return false;
  }
  *buf_storage = &buf;
  return body(payload, callee);
}

void VERSIONED_SYMBOL(wasmtime_longjmp)(void *JmpBuf) {
  platform_jmp_buf *buf = (platform_jmp_buf *)JmpBuf;
  platform_longjmp(*buf, 1);
}

#ifdef FEATURE_DEBUG_BUILTINS
#ifdef CFG_TARGET_OS_windows
#define DEBUG_BUILTIN_EXPORT __declspec(dllexport)
#else
#define DEBUG_BUILTIN_EXPORT
#endif

// This set of symbols is defined here in C because Rust's #[export_name]
// functions are not dllexported on Windows when building an executable. These
// symbols are directly referenced by name from the native DWARF info.
void *VERSIONED_SYMBOL(resolve_vmctx_memory_ptr)(void *);
DEBUG_BUILTIN_EXPORT void *
VERSIONED_SYMBOL(wasmtime_resolve_vmctx_memory_ptr)(void *p) {
  return VERSIONED_SYMBOL(resolve_vmctx_memory_ptr)(p);
}
void VERSIONED_SYMBOL(set_vmctx_memory)(void *);
DEBUG_BUILTIN_EXPORT void VERSIONED_SYMBOL(wasmtime_set_vmctx_memory)(void *p) {
  VERSIONED_SYMBOL(set_vmctx_memory)(p);
}

// Helper symbol called from Rust to force the above two functions to not get
// stripped by the linker.
void VERSIONED_SYMBOL(wasmtime_debug_builtins_init)() {
#ifndef CFG_TARGET_OS_windows
  void *volatile p;
  p = (void *)&VERSIONED_SYMBOL(wasmtime_resolve_vmctx_memory_ptr);
  p = (void *)&VERSIONED_SYMBOL(wasmtime_set_vmctx_memory);
  (void)p;
#endif
}
#endif // FEATURE_DEBUG_BUILTINS

// For more information about this see `unix/unwind.rs` and the
// `using_libunwind` function. The basic idea is that weak symbols aren't stable
// in Rust so we use a bit of C to work around that.
#ifndef CFG_TARGET_OS_windows
__attribute__((weak)) extern void __unw_add_dynamic_fde();

bool VERSIONED_SYMBOL(wasmtime_using_libunwind)() {
  return __unw_add_dynamic_fde != NULL;
}
#endif
