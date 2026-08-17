// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPILER_SPECIFIC_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPILER_SPECIFIC_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"

// A wrapper around `__has_cpp_attribute()`, which is in C++20 and thus not yet
// available for all targets PA supports (since PA's minimum C++ version is 17).
// This works similarly to `PA_HAS_ATTRIBUTE()` below, in that where it's
// unavailable it will map to `0`.
#if defined(__has_cpp_attribute)
#define PA_HAS_CPP_ATTRIBUTE(x) __has_cpp_attribute(x)
#else
#define PA_HAS_CPP_ATTRIBUTE(x) 0
#endif

// A wrapper around `__has_attribute()`, which is similar to the C++20-standard
// `__has_cpp_attribute()`, but tests for support for `__attribute__(())`s.
// Compilers that do not support this (e.g. MSVC) are also assumed not to
// support `__attribute__`, so this is simply mapped to `0` there.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-attribute
#if defined(__has_attribute)
#define PA_HAS_ATTRIBUTE(x) __has_attribute(x)
#else
#define PA_HAS_ATTRIBUTE(x) 0
#endif

// A wrapper around `__has_builtin`, similar to `PA_HAS_ATTRIBUTE()`.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-builtin
#if defined(__has_builtin)
#define PA_HAS_BUILTIN(x) __has_builtin(x)
#else
#define PA_HAS_BUILTIN(x) 0
#endif

// A wrapper around `__has_feature`, similar to `PA_HAS_ATTRIBUTE()`.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-feature-and-has-extension
#if defined(__has_feature)
#define PA_HAS_FEATURE(FEATURE) __has_feature(FEATURE)
#else
#define PA_HAS_FEATURE(FEATURE) 0
#endif

// Annotates a function indicating it should not be inlined.
//
// Note that this may still fail to preserve function calls in the most trivial
// cases, due to optimizations like constant folding; see
// https://stackoverflow.com/questions/54481855/clang-ignoring-attribute-noinline/54482070#54482070.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#noinline
//
// Usage:
// ```
//   PA_NOINLINE void Func() {
//     // This body will not be inlined into callers.
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(gnu::noinline)
#define PA_NOINLINE [[gnu::noinline]]
#elif PA_HAS_CPP_ATTRIBUTE(msvc::noinline)
#define PA_NOINLINE [[msvc::noinline]]
#else
#define PA_NOINLINE
#endif

// Annotates a function indicating it should always be inlined.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#always-inline-force-inline
//
// Usage:
// ```
//   PA_ALWAYS_INLINE void Func() {
//     // This body will be inlined into callers whenever possible.
//   }
// ```
//
// Since `ALWAYS_INLINE` is performance-oriented but can hamper debugging,
// ignore it in debug mode.
#if !PA_BUILDFLAG(IS_DEBUG)
#if PA_HAS_CPP_ATTRIBUTE(clang::always_inline)
#define PA_ALWAYS_INLINE [[clang::always_inline]] inline
#elif PA_HAS_CPP_ATTRIBUTE(gnu::always_inline)
#define PA_ALWAYS_INLINE [[gnu::always_inline]] inline
#elif defined(PA_COMPILER_MSVC)
#define PA_ALWAYS_INLINE __forceinline
#endif
#endif  // !PA_BUILDFLAG(IS_DEBUG)
#if !defined(PA_ALWAYS_INLINE)
#define PA_ALWAYS_INLINE inline
#endif

// Annotates a function indicating it should never be tail called. Useful to
// make sure callers of the annotated function are never omitted from call
// stacks. Often useful with `PA_NOINLINE` to make sure the function itself is
// also not omitted from call stacks. Note: this does not prevent code folding
// of multiple identical callers into a single signature; to do that, see
// `PA_NO_CODE_FOLDING()` in partition_alloc_base/debug/alias.h.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#not-tail-called
//
// Usage:
// ```
//   // Calls to this method will not be tail calls.
//   PA_NOT_TAIL_CALLED void Func();
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::not_tail_called)
#define PA_NOT_TAIL_CALLED [[clang::not_tail_called]]
#else
#define PA_NOT_TAIL_CALLED
#endif

// Annotates a return statement indicating the compiler must convert it to a
// tail call. Can be used only on return statements, even for functions
// returning void. Caller and callee must have the same number of arguments and
// the argument types must be "similar". While the compiler may automatically
// convert compatible calls to tail calls when optimizing, this annotation
// requires it to occur if doing so is valid, and will not compile otherwise.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#musttail
//
// Usage:
// ```
//   int Func1(double);
//   int Func2(double d) {
//     PA_MUSTTAIL return Func1(d + 1);  // `Func1()` will be tail-called.
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::musttail)
#define PA_MUSTTAIL [[clang::musttail]]
#else
#define PA_MUSTTAIL
#endif

// Annotates a data member indicating it need not have an address distinct from
// all other non-static data members of the class, and its tail padding may be
// used for other objects' storage. This can have subtle and dangerous effects,
// including on containing objects; use with caution.
//
// See also:
//   https://en.cppreference.com/w/cpp/language/attributes/no_unique_address
//   https://wg21.link/dcl.attr.nouniqueaddr
// Usage:
// ```
//   // In the following struct, `t` might not have a unique address from `i`,
//   // and `t`'s tail padding (if any) may be reused by subsequent objects.
//   struct S {
//     int i;
//     PA_NO_UNIQUE_ADDRESS T t;
//   };
// ```
//
// Unfortunately MSVC ignores [[no_unique_address]] (see
// https://devblogs.microsoft.com/cppblog/msvc-cpp20-and-the-std-cpp20-switch/#msvc-extensions-and-abi),
// and clang-cl matches it for ABI compatibility reasons. We need to prefer
// [[msvc::no_unique_address]] when available if we actually want any effect.
#if PA_HAS_CPP_ATTRIBUTE(msvc::no_unique_address)
#define PA_NO_UNIQUE_ADDRESS [[msvc::no_unique_address]]
#elif PA_HAS_CPP_ATTRIBUTE(no_unique_address)
#define PA_NO_UNIQUE_ADDRESS [[no_unique_address]]
#else
#define PA_NO_UNIQUE_ADDRESS
#endif

// Annotates a function indicating it takes a `printf()`-style format string.
// The compiler will check that the provided arguments match the type specifiers
// in the format string. Useful to detect mismatched format strings/args.
//
// `format_param` is the one-based index of the format string parameter;
// `dots_param` is the one-based index of the "..." parameter.
// For `v*printf()` functions (which take a `va_list`), `dots_param` should be
// 0. For member functions, the implicit `this` parameter is at index 1.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#format
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-format-function-attribute
//
// Usage:
// ```
//   PA_PRINTF_FORMAT(1, 2)
//   void Print(const char* format, ...);
//   void Func() {
//     // The following call will not compile; diagnosed as format and argument
//     // types mismatching.
//     Print("%s", 1);
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(gnu::format)
#define PA_PRINTF_FORMAT(format_param, dots_param) \
  [[gnu::format(printf, format_param, dots_param)]]
#else
#define PA_PRINTF_FORMAT(format_param, dots_param)
#endif

// Annotates a function disabling the named sanitizer within its body.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#no-sanitize
//   https://clang.llvm.org/docs/UsersManual.html#controlling-code-generation
//
// Usage:
// ```
//   PA_NO_SANITIZE("cfi-icall") void Func() {
//     // CFI indirect call checks will not be performed in this body.
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::no_sanitize)
#define PA_NO_SANITIZE(sanitizer) [[clang::no_sanitize(sanitizer)]]
#else
#define PA_NO_SANITIZE(sanitizer)
#endif

// Annotates a pointer and size directing MSAN to treat that memory region as
// fully initialized. Useful for e.g. code that deliberately reads uninitialized
// data, such as a GC scavenging root set pointers from the stack.
//
// See also:
//   https://github.com/google/sanitizers/wiki/MemorySanitizer
//
// Usage:
// ```
//   T* ptr = ...;
//   // After the next statement, MSAN will assume `ptr` points to an
//   // initialized `T`.
//   PA_MSAN_UNPOISON(ptr, sizeof(T));
// ```
#if defined(MEMORY_SANITIZER)
#include <sanitizer/msan_interface.h>
#define PA_MSAN_UNPOISON(p, size) __msan_unpoison(p, size)
#else
#define PA_MSAN_UNPOISON(p, size)
#endif

// Annotates a codepath suppressing static analysis along that path. Useful when
// code is safe in practice for reasons the analyzer can't detect, e.g. because
// the condition leading to that path guarantees a param is non-null.
//
// Usage:
// ```
//   if (cond) {
//     PA_ANALYZER_SKIP_THIS_PATH();
//     // Static analysis will be disabled for the remainder of this block.
//     delete ptr;
//   }
// ```
#if defined(__clang_analyzer__)
namespace partition_alloc::internal {
inline constexpr bool AnalyzerNoReturn()
#if PA_HAS_ATTRIBUTE(analyzer_noreturn)
    __attribute__((analyzer_noreturn))
#endif
{
  return false;
}
}  // namespace partition_alloc::internal
#define PA_ANALYZER_SKIP_THIS_PATH() \
  static_cast<void>(::partition_alloc::internal::AnalyzerNoReturn())
#else
// The above definition would be safe even outside the analyzer, but defining
// the macro away entirely avoids the need for the optimizer to eliminate it.
#define PA_ANALYZER_SKIP_THIS_PATH()
#endif

// Annotates a condition directing static analysis to assume it is always true.
// Evaluates to the provided `arg` as a `bool`.
//
// Usage:
// ```
//   // Static analysis will assume the following condition always holds.
//   if (PA_ANALYZER_ASSUME_TRUE(cond)) ...
// ```
#if defined(__clang_analyzer__)
namespace partition_alloc::internal {
inline constexpr bool AnalyzerAssumeTrue(bool arg) {
  return arg || AnalyzerNoReturn();
}
}  // namespace partition_alloc::internal
#define PA_ANALYZER_ASSUME_TRUE(arg) \
  ::partition_alloc::internal::AnalyzerAssumeTrue(!!(arg))
#else
// Again, the above definition is safe, this is just simpler for the optimizer.
#define PA_ANALYZER_ASSUME_TRUE(arg) (arg)
#endif

// Annotates a function, function pointer, or statement to disallow
// optimizations that merge calls. Useful to ensure the source locations of such
// calls are not obscured.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#nomerge
//
// Usage:
// ```
//   PA_NOMERGE void Func();  // No direct calls to `Func()` will be merged.
//
//   using Ptr = decltype(&Func);
//   PA_NOMERGE Ptr ptr = &Func;  // No calls through `ptr` will be merged.
//
//   PA_NOMERGE if (cond) {
//     // No calls in this block will be merged.
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::nomerge)
#define PA_NOMERGE [[clang::nomerge]]
#else
#define PA_NOMERGE
#endif

// Annotates a type as being suitable for passing in registers despite having a
// non-trivial copy or move constructor or destructor. This requires the type
// not be concerned about its address remaining constant, be safely usable after
// copying its memory, and have a destructor that may be safely omitted on
// moved-from instances; an example is `std::unique_ptr`. Unnecessary if the
// copy/move constructor(s) and destructor are unconditionally trivial; likely
// ineffective if the type is too large to be passed in one or two registers
// with the target ABI. However, annotating a type this way will also cause
// `IS_TRIVIALLY_RELOCATABLE()` to return true for that type, and so may be
// desirable even for large types, if they are placed in containers that
// optimize based on that check.
//
// NOTE: Use with caution; this has subtle effects on constructor/destructor
// ordering. When used with types passed or returned by value, values may be
// constructed in the source stack frame, passed in a register, and then used
// and destroyed in the target stack frame.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#trivial-abi
//   https://libcxx.llvm.org/docs/DesignDocs/UniquePtrTrivialAbi.html
//
// Usage:
// ```
//   // Instances of type `S` will be eligible to be passed in registers despite
//   // `S`'s nontrivial destructor.
//   struct PA_TRIVIAL_ABI S { ~S(); }
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::trivial_abi)
#define PA_TRIVIAL_ABI [[clang::trivial_abi]]
#else
#define PA_TRIVIAL_ABI
#endif

// Makes C++20's `constinit` functionality available even pre-C++20, by falling
// back to a custom attribute.
// TODO(crbug.com/365046216): Use `constinit` directly when C++20 is available
// and all usage sites have been reordered to be compatible with doing so.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#require-constant-initialization-constinit-c-20
//
// Usage:
// ```
// struct S {
//   constexpr S() = default;
//   S(int) {}
// };
//
// // Compiles (constant initialization via `constexpr` default constructor).
// PA_CONSTINIT S s0;
//
// // Will not compile; diagnosed as usage of non-constexpr constructor in a
// // constant expression.
// PA_CONSTINIT S s1(1);
//
// // Compiles (non-constant initialization via non-`constexpr` constructor).
// S s2(2);
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::require_constant_initialization)
#define PA_CONSTINIT [[clang::require_constant_initialization]]
#else
#define PA_CONSTINIT
#endif

// Annotates a type as holding a pointer into an owner object (an appropriate
// STL or `[[gsl::Owner]]`-annotated type). If an instance of the pointer type
// is constructed from an instance of the owner type, and the owner instance is
// destroyed, the pointer instance is considered to be dangling. Useful to
// diagnose some cases of lifetime errors.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#pointer
//
// Usage:
// ```
//  struct [[gsl::Owner]] T {};
//  struct PA_GSL_POINTER S {
//    S(const T&);
//  };
//  S Func() {
//    // The following return will not compile; diagnosed as returning address
//    // of local temporary.
//    return S(T());
//  }
// ```
#if PA_HAS_CPP_ATTRIBUTE(gsl::Pointer)
#define PA_GSL_POINTER [[gsl::Pointer]]
#else
#define PA_GSL_POINTER
#endif

// Annotates a destructor marking it `constexpr` only if the language supports
// it (C++20 and onward).
//
// Usage:
// ```
//  struct S {
//    PA_CONSTEXPR_DTOR ~S() {}  // N.B.: Compiles even pre-C++20
//  };
//  // The following declaration will only compile in C++20; diagnosed as an
//  // invalid constexpr variable of non-literal type otherwise.
//  constexpr S s;
// ```
#if defined(__cpp_constexpr) && __cpp_constexpr >= 201907L
#define PA_CONSTEXPR_DTOR constexpr
#else
#define PA_CONSTEXPR_DTOR
#endif

// Annotates a pointer or reference parameter or return value for a member
// function as having lifetime intertwined with the instance on which the
// function is called. For parameters, the function is assumed to store the
// value into the called-on object, so if the referred-to object is later
// destroyed, the called-on object is also considered to be dangling. For return
// values, the value is assumed to point into the called-on object, so if that
// object is destroyed, the returned value is also considered to be dangling.
// Useful to diagnose some cases of lifetime errors.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#lifetimebound
//
// Usage:
// ```
//   struct S {
//      S(int* p PA_LIFETIME_BOUND);
//      int* Get() PA_LIFETIME_BOUND;
//   };
//   S Func1() {
//     int i = 0;
//     // The following return will not compile; diagnosed as returning address
//     // of a stack object.
//     return S(&i);
//   }
//   int* Func2(int* p) {
//     // The following return will not compile; diagnosed as returning address
//     // of a local temporary.
//     return S(p).Get();
//   }
// ```
#if PA_HAS_CPP_ATTRIBUTE(clang::lifetimebound)
#define PA_LIFETIME_BOUND [[clang::lifetimebound]]
#else
#define PA_LIFETIME_BOUND
#endif

// Annotates a function disabling PGO profiling. This may be necessary to avoid
// runtime crashes due to re-entrancy when allocator functions are instrumented
// for PGO profiling and the instrumentation attempts to allocate.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#no-profile-instrument-function
//
// Usage:
// ```
// ```
#if PA_HAS_CPP_ATTRIBUTE(gnu::no_profile_instrument_function)
#define PA_NOPROFILE [[gnu::no_profile_instrument_function]]
#else
#define PA_NOPROFILE
#endif

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_COMPILER_SPECIFIC_H_
