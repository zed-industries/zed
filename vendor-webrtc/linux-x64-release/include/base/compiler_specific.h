// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_COMPILER_SPECIFIC_H_
#define BASE_COMPILER_SPECIFIC_H_

#include "build/build_config.h"

#if defined(COMPILER_MSVC) && !defined(__clang__)
#error "Only clang-cl is supported on Windows, see https://crbug.com/988071"
#endif

// A wrapper around `__has_attribute()`, which is similar to the C++20-standard
// `__has_cpp_attribute()`, but tests for support for `__attribute__(())`s.
// Compilers that do not support this (e.g. MSVC) are also assumed not to
// support `__attribute__`, so this is simply mapped to `0` there.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-attribute
#if defined(__has_attribute)
#define HAS_ATTRIBUTE(x) __has_attribute(x)
#else
#define HAS_ATTRIBUTE(x) 0
#endif

// A wrapper around `__has_builtin`, similar to `HAS_ATTRIBUTE()`.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-builtin
#if defined(__has_builtin)
#define HAS_BUILTIN(x) __has_builtin(x)
#else
#define HAS_BUILTIN(x) 0
#endif

// A wrapper around `__has_feature`, similar to `HAS_ATTRIBUTE()`.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#has-feature-and-has-extension
#if defined(__has_feature)
#define HAS_FEATURE(FEATURE) __has_feature(FEATURE)
#else
#define HAS_FEATURE(FEATURE) 0
#endif

// Annotates a function indicating it should not be inlined.
//
// You may also want `NOOPT` if your goal is to preserve a function call even
// for the most trivial cases; see
// https://stackoverflow.com/questions/54481855/clang-ignoring-attribute-noinline/54482070#54482070.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#noinline
//
// Usage:
// ```
//   NOINLINE void Func() {
//     // This body will not be inlined into callers.
//   }
// ```
#if __has_cpp_attribute(clang::noinline)
#define NOINLINE [[clang::noinline]]
#elif __has_cpp_attribute(gnu::noinline)
#define NOINLINE [[gnu::noinline]]
#elif __has_cpp_attribute(msvc::noinline)
#define NOINLINE [[msvc::noinline]]
#else
#define NOINLINE
#endif

// Annotates a call site indicating that the callee should not be inlined.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#noinline
//
// Usage:
// ```
//   void Func() {
//      // This specific call to `DoSomething` should not be inlined.
//      NOINLINE_CALL DoSomething();
//   }
// ```
#if __has_cpp_attribute(clang::noinline)
#define NOINLINE_CALL [[clang::noinline]]
#else
#define NOINLINE_CALL
#endif

// Annotates a function indicating it should not be optimized.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#optnone
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-optimize-function-attribute
//
// Usage:
// ```
//   NOOPT void Func() {
//     // This body will not be optimized.
//   }
// ```
#if __has_cpp_attribute(clang::optnone)
#define NOOPT [[clang::optnone]]
#elif __has_cpp_attribute(gnu::optimize)
#define NOOPT [[gnu::optimize(0)]]
#else
#define NOOPT
#endif

// Annotates a function indicating it should always be inlined.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#always-inline-force-inline
//
// Usage:
// ```
//   ALWAYS_INLINE void Func() {
//     // This body will be inlined into callers whenever possible.
//   }
// ```
//
// Since `ALWAYS_INLINE` is performance-oriented but can hamper debugging,
// ignore it in debug mode.
#if defined(NDEBUG)
#if __has_cpp_attribute(clang::always_inline)
#define ALWAYS_INLINE [[clang::always_inline]] inline
#elif __has_cpp_attribute(gnu::always_inline)
#define ALWAYS_INLINE [[gnu::always_inline]] inline
#elif defined(COMPILER_MSVC)
#define ALWAYS_INLINE __forceinline
#endif
#endif
#if !defined(ALWAYS_INLINE)
#define ALWAYS_INLINE inline
#endif

// Annotates a call site indicating the calee should always be inlined.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#always-inline-force-inline
//
// Usage:
// ```
//   void Func() {
//     // This specific call will be inlined if possible.
//     ALWAYS_INLINE_CALL DoSomething();
//   }
// ```
//
// Since `ALWAYS_INLINE_CALL` is performance-oriented but can hamper debugging,
// ignore it in debug mode.
#if defined(NDEBUG)
#if __has_cpp_attribute(clang::always_inline)
#define ALWAYS_INLINE_CALL [[clang::always_inline]]
#endif
#endif
#if !defined(ALWAYS_INLINE_CALL)
#define ALWAYS_INLINE_CALL
#endif

// Annotates a function indicating it should never be tail called. Useful to
// make sure callers of the annotated function are never omitted from call
// stacks. Often useful with `NOINLINE` to make sure the function itself is also
// not omitted from call stacks. Note: this does not prevent code folding of
// multiple identical callers into a single signature; to do that, see
// `NO_CODE_FOLDING()` in base/debug/alias.h.
//
// For a caller-side version of this, see `DISABLE_TAIL_CALLS`.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#not-tail-called
//
// Usage:
// ```
//   // Calls to this function will not be tail calls.
//   NOT_TAIL_CALLED void Func();
// ```
#if __has_cpp_attribute(clang::not_tail_called)
#define NOT_TAIL_CALLED [[clang::not_tail_called]]
#else
#define NOT_TAIL_CALLED
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
//     MUSTTAIL return Func1(d + 1);  // `Func1()` will be tail-called.
//   }
// ```
#if __has_cpp_attribute(clang::musttail)
#define MUSTTAIL [[clang::musttail]]
#else
#define MUSTTAIL
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
//     NO_UNIQUE_ADDRESS T t;
//   };
// ```
//
// Unfortunately MSVC ignores [[no_unique_address]] (see
// https://devblogs.microsoft.com/cppblog/msvc-cpp20-and-the-std-cpp20-switch/#msvc-extensions-and-abi),
// and clang-cl matches it for ABI compatibility reasons. We need to prefer
// [[msvc::no_unique_address]] when available if we actually want any effect.
#if __has_cpp_attribute(msvc::no_unique_address)
#define NO_UNIQUE_ADDRESS [[msvc::no_unique_address]]
#elif __has_cpp_attribute(no_unique_address)
#define NO_UNIQUE_ADDRESS [[no_unique_address]]
#else
#define NO_UNIQUE_ADDRESS
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
//   PRINTF_FORMAT(1, 2)
//   void Print(const char* format, ...);
//   void Func() {
//     // The following call will not compile; diagnosed as format and argument
//     // types mismatching.
//     Print("%s", 1);
//   }
// ```
#if __has_cpp_attribute(gnu::format)
#define PRINTF_FORMAT(format_param, dots_param) \
  [[gnu::format(printf, format_param, dots_param)]]
#else
#define PRINTF_FORMAT(format_param, dots_param)
#endif

// Annotates a function disabling the named sanitizer within its body.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#no-sanitize
//   https://clang.llvm.org/docs/UsersManual.html#controlling-code-generation
//
// Usage:
// ```
//   NO_SANITIZE("cfi-icall") void Func() {
//     // CFI indirect call checks will not be performed in this body.
//   }
// ```
#if __has_cpp_attribute(clang::no_sanitize)
#define NO_SANITIZE(sanitizer) [[clang::no_sanitize(sanitizer)]]
#else
#define NO_SANITIZE(sanitizer)
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
//   MSAN_UNPOISON(ptr, sizeof(T));
// ```
#if defined(MEMORY_SANITIZER) && !BUILDFLAG(IS_NACL)
#include <sanitizer/msan_interface.h>
#define MSAN_UNPOISON(p, size) __msan_unpoison(p, size)
#else
#define MSAN_UNPOISON(p, size)
#endif

// Annotates a pointer and size directing MSAN to check whether that memory
// region is initialized, as if it was being read from. If any bits are
// uninitialized, crashes with an MSAN report. Useful for e.g. sanitizing data
// MSAN won't be able to track, such as data that is about to be passed to
// another process via shared memory.
//
// See also:
//   https://www.chromium.org/developers/testing/memorysanitizer/#debugging-msan-reports
//
// Usage:
// ```
//   T* ptr = ...;
//   // The following line will crash at runtime in MSAN builds if `ptr` does
//   // not point to an initialized `T`.
//   MSAN_CHECK_MEM_IS_INITIALIZED(ptr, sizeof(T));
// ```
#if defined(MEMORY_SANITIZER) && !BUILDFLAG(IS_NACL)
#define MSAN_CHECK_MEM_IS_INITIALIZED(p, size) \
  __msan_check_mem_is_initialized(p, size)
#else
#define MSAN_CHECK_MEM_IS_INITIALIZED(p, size)
#endif

// Annotates a function disabling Control Flow Integrity checks due to perf
// impact.
//
// See also:
//   https://clang.llvm.org/docs/ControlFlowIntegrity.html#performance
//   https://www.chromium.org/developers/testing/control-flow-integrity/#overhead-only-tested-on-x64
//
// Usage:
// ```
//   DISABLE_CFI_PERF void Func() {
//     // CFI checks will not be performed in this body, due to perf reasons.
//   }
// ```
#if !defined(DISABLE_CFI_PERF)
#if defined(__clang__) && defined(OFFICIAL_BUILD)
#define DISABLE_CFI_PERF NO_SANITIZE("cfi")
#else
#define DISABLE_CFI_PERF
#endif
#endif

// Annotates a function disabling Control Flow Integrity indirect call checks.
// NOTE: Prefer `DISABLE_CFI_DLSYM()` if you just need to allow calling of dlsym
// functions.
//
// See also:
//   https://clang.llvm.org/docs/ControlFlowIntegrity.html#available-schemes
//   https://www.chromium.org/developers/testing/control-flow-integrity/#indirect-call-failures
//
// Usage:
// ```
//   DISABLE_CFI_ICALL void Func() {
//     // CFI indirect call checks will not be performed in this body.
//   }
// ```
#if !defined(DISABLE_CFI_ICALL)
#if BUILDFLAG(IS_WIN)
#define DISABLE_CFI_ICALL NO_SANITIZE("cfi-icall") __declspec(guard(nocf))
#else
#define DISABLE_CFI_ICALL NO_SANITIZE("cfi-icall")
#endif
#endif

// Annotates a function disabling Control Flow Integrity indirect call checks if
// doing so is necessary to call dlsym functions. The checks are retained on
// platforms where loaded modules participate in CFI (viz. Windows).
//
// See also:
//   https://www.chromium.org/developers/testing/control-flow-integrity/#indirect-call-failures
//
// Usage:
// ```
//   DISABLE_CFI_DLSYM void Func() {
//     // On non-Windows platforms, CFI indirect call checks will not be
//     // performed in this body.
//   }
// ```
#if !defined(DISABLE_CFI_DLSYM)
#if BUILDFLAG(IS_WIN)
#define DISABLE_CFI_DLSYM
#else
#define DISABLE_CFI_DLSYM DISABLE_CFI_ICALL
#endif
#endif

// Evaluates to a string constant containing the function signature.
//
// See also:
//   https://clang.llvm.org/docs/LanguageExtensions.html#source-location-builtins
//   https://en.cppreference.com/w/c/language/function_definition#func
//
// Usage:
// ```
//   void Func(int arg) {
//     std::cout << PRETTY_FUNCTION;  // Prints `void Func(int)` or similar.
//   }
// ```
#if defined(COMPILER_GCC)
#define PRETTY_FUNCTION __PRETTY_FUNCTION__
#elif defined(COMPILER_MSVC)
#define PRETTY_FUNCTION __FUNCSIG__
#else
#define PRETTY_FUNCTION __func__
#endif

// Annotates a variable indicating that its storage should not be filled with a
// fixed pattern when uninitialized.
//
// The `init_stack_vars` gn arg (enabled on most build configs) causes the
// compiler to generate code that writes a fixed pattern into uninitialized
// parts of all local variables, to mitigate security risks. In most cases, e.g.
// when such memory is either never accessed or will be initialized later before
// reading, the compiler is able to remove the additional stores, and any
// remaining stores are unlikely to affect program performance.
//
// If hot code suffers unavoidable perf penalties, this can disable the
// pattern-filling there. This should only be done when necessary, since reads
// from uninitialized variables are not only UB, they can in practice allow
// attackers to control logic by pre-filling the variable's memory with a
// desirable value.
//
// NOTE: This behavior also increases the likelihood the compiler will generate
// `memcpy()`/`memset()` calls to init variables. If this causes link errors for
// targets that don't link against the CRT, this macro can help; you may instead
// want 'configs -= [ "//build/config/compiler:default_init_stack_vars" ]' in
// the relevant .gn file to disable this on the whole target.
//
// See also:
//   https://source.chromium.org/chromium/chromium/src/+/main:build/config/compiler/BUILD.gn;l=3088;drc=24ccaf63ff5b1883be1ebe5f979d917ce28b0131
//   https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-ftrivial-auto-var-init
//   https://clang.llvm.org/docs/AttributeReference.html#uninitialized
//
// Usage:
// ```
//   // The following line declares `i` without ensuring it initially contains
//   // any particular pattern.
//   STACK_UNINITIALIZED int i;
// ```
#if __has_cpp_attribute(clang::uninitialized)
#define STACK_UNINITIALIZED [[clang::uninitialized]]
#elif __has_cpp_attribute(gnu::uninitialized)
#define STACK_UNINITIALIZED [[gnu::uninitialized]]
#else
#define STACK_UNINITIALIZED
#endif

// Annotates a function disabling stack canary checks.
//
// The `-fstack-protector` compiler flag (passed on most non-Windows builds)
// causes the compiler to extend some function prologues and epilogues to set
// and check a canary value, to detect stack buffer overflows and crash in
// response. If hot code suffers unavoidable perf penalties, or intentionally
// modifies the canary value, this can disable the behavior there.
//
// See also:
//   https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fstack-protector
//   https://clang.llvm.org/docs/AttributeReference.html#no-stack-protector-safebuffers
//
// Usage:
// ```
//   NO_STACK_PROTECTOR void Func() {
//     // Stack canary checks will not be performed in this body.
//   }
// ```
#if __has_cpp_attribute(gnu::no_stack_protector)
#define NO_STACK_PROTECTOR [[gnu::no_stack_protector]]
#elif __has_cpp_attribute(gnu::optimize)
#define NO_STACK_PROTECTOR [[gnu::optimize("-fno-stack-protector")]]
#else
#define NO_STACK_PROTECTOR
#endif

// Annotates a codepath suppressing static analysis along that path. Useful when
// code is safe in practice for reasons the analyzer can't detect, e.g. because
// the condition leading to that path guarantees a param is non-null.
//
// Usage:
// ```
//   if (cond) {
//     ANALYZER_SKIP_THIS_PATH();
//     // Static analysis will be disabled for the remainder of this block.
//     delete ptr;
//   }
// ```
#if defined(__clang_analyzer__)
inline constexpr bool AnalyzerNoReturn()
#if HAS_ATTRIBUTE(analyzer_noreturn)
    __attribute__((analyzer_noreturn))
#endif
{
  return false;
}
#define ANALYZER_SKIP_THIS_PATH() static_cast<void>(::AnalyzerNoReturn())
#else
// The above definition would be safe even outside the analyzer, but defining
// the macro away entirely avoids the need for the optimizer to eliminate it.
#define ANALYZER_SKIP_THIS_PATH()
#endif

// Annotates a condition directing static analysis to assume it is always true.
// Evaluates to the provided `arg` as a `bool`.
//
// Usage:
// ```
//   // Static analysis will assume the following condition always holds.
//   if (ANALYZER_ASSUME_TRUE(cond)) ...
// ```
#if defined(__clang_analyzer__)
inline constexpr bool AnalyzerAssumeTrue(bool arg) {
  return arg || AnalyzerNoReturn();
}
#define ANALYZER_ASSUME_TRUE(arg) ::AnalyzerAssumeTrue(!!(arg))
#else
// Again, the above definition is safe, this is just simpler for the optimizer.
#define ANALYZER_ASSUME_TRUE(arg) (arg)
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
//   NOMERGE void Func();  // No direct calls to `Func()` will be merged.
//
//   using Ptr = decltype(&Func);
//   NOMERGE Ptr ptr = &Func;  // No calls through `ptr` will be merged.
//
//   NOMERGE if (cond) {
//     // No calls in this block will be merged.
//   }
// ```
#if __has_cpp_attribute(clang::nomerge)
#define NOMERGE [[clang::nomerge]]
#else
#define NOMERGE
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
//   struct TRIVIAL_ABI S { ~S(); }
// ```
#if __has_cpp_attribute(clang::trivial_abi)
#define TRIVIAL_ABI [[clang::trivial_abi]]
#else
#define TRIVIAL_ABI
#endif

// Determines whether a type is trivially relocatable, i.e. a move-and-destroy
// sequence can safely be replaced with `memcpy()`. This is true of types with
// trivial copy or move construction plus trivial destruction, as well as types
// marked `TRIVIAL_ABI`. Useful to optimize container implementations.
//
// See also:
//   https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/p1144r8.html
//   https://clang.llvm.org/docs/LanguageExtensions.html#:~:text=__is_trivially_relocatable
//
// Usage:
// ```
//   if constexpr (IS_TRIVIALLY_RELOCATABLE(T)) {
//     // This block will only be executed if type `T` is trivially relocatable.
//   }
// ```
#if HAS_BUILTIN(__is_trivially_relocatable)
#define IS_TRIVIALLY_RELOCATABLE(t) __is_trivially_relocatable(t)
#else
#define IS_TRIVIALLY_RELOCATABLE(t) false
#endif

// Annotates a member function as safe to call on a moved-from object, which it
// will reinitialize.
//
// See also:
//   https://clang.llvm.org/extra/clang-tidy/checks/bugprone/use-after-move.html#reinitialization
//
// Usage:
// ```
//   struct S {
//     REINITIALIZES_AFTER_MOVE void Reset();
//   };
//   void Func1(const S&);
//   void Func2() {
//     S s1;
//     S s2 = std::move(s1);
//     s1.Reset();
//     // clang-tidy's `bugprone-use-after-move` check will not flag the
//     // following call as a use-after-move, due to the intervening `Reset()`.
//     Func1(s1);
//   }
// ```
#if __has_cpp_attribute(clang::reinitializes)
#define REINITIALIZES_AFTER_MOVE [[clang::reinitializes]]
#else
#define REINITIALIZES_AFTER_MOVE
#endif

// Annotates a type as owning an object or memory region whose address may be
// vended to or stored by other objects. For example, `std::unique_ptr<T>` owns
// a `T` and vends its address via `.get()`, and `std::string` owns a block of
// `char` and vends its address via `.data()`. Used to detect lifetime errors in
// conjunction with `GSL_POINTER`; see documentation there.
//
// See also:
//   https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#SS-ownership
//   https://clang.llvm.org/docs/AttributeReference.html#owner
//   https://clang.llvm.org/docs/DiagnosticsReference.html#wdangling-gsl
//
// Usage:
// ```
//   // Marking `S` as `GSL_OWNER` enables `-Wdangling-gsl` to detect misuse by
//   // types annotated as `GSL_POINTER`.
//   struct GSL_OWNER S;
// ```
#if __has_cpp_attribute(gsl::Owner)
#define GSL_OWNER [[gsl::Owner]]
#else
#define GSL_OWNER
#endif

// Annotates a type as holding a pointer into an owner object (an appropriate
// STL or `GSL_OWNER`-annotated type). If an instance of the pointer type is
// constructed from an instance of the owner type, and the owner instance is
// destroyed, the pointer instance is considered to be dangling. Useful to
// diagnose some cases of lifetime errors.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#pointer
//
// Usage:
// ```
//  struct GSL_OWNER T {};
//  struct GSL_POINTER S {
//    S(const T&);
//  };
//  S Func() {
//    // The following return will not compile; diagnosed as returning address
//    // of local temporary.
//    return S(T());
//  }
// ```
#if __has_cpp_attribute(gsl::Pointer)
#define GSL_POINTER [[gsl::Pointer]]
#else
#define GSL_POINTER
#endif

// Annotates a type or variable to add a "logically_const" ABI tag to any
// corresponding mangled symbol name(s). Useful to suppress warnings from the
// "Mutable Constants" trybot check [1] when logically const instances are named
// like `kConstants` but for some reason should not be marked `const`.
//
// [1]:
// https://chromium.googlesource.com/chromium/src/+/main/docs/speed/binary_size/android_binary_size_trybot.md#Mutable-Constants
//
// Usage:
// ```
//   struct S {};
//   S kConstS;                      // Fails on some trybots.
//   LOGICALLY_CONST S kAlsoConstS;  // OK
//
//   struct LOGICALLY_CONST T {};
//   T kConstT;                      // OK
// ```
#if __has_cpp_attribute(gnu::abi_tag)
#define LOGICALLY_CONST [[gnu::abi_tag("logically_const")]]
#else
#define LOGICALLY_CONST
#endif

// Annotates a function indicating it is cold, but called from hot functions.
// Useful when a performance-sensitive function is usually simple, but in edge
// cases must fall back to a more complex handler.
//
// On X86-64 and AArch64, this changes the calling convention so most registers
// are callee-saved, reducing register spills in the caller. This can improve
// caller performance in the common case, at the cost of pessimizing the callee.
// On other platforms, this attribute has no effect as of Clang 20.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#preserve-most
//
// Usage:
// ```
//   // Calls to this function will not require most registers to be saved.
//   PRESERVE_MOST void Func();
// ```
//
// Disable `PRESERVE_MOST` in component builds, since `_dl_runtime_resolve()`
// clobbers registers on platforms where it's used, and the component build is
// not perf-critical anyway; see
// https://github.com/llvm/llvm-project/issues/105588.
//
// Also disable for Win ARM64 due to as-yet-uninvestigated crashes.
// TODO(crbug.com/42204008): Investigate, fix, and re-enable.
#if __has_cpp_attribute(clang::preserve_most) &&             \
    (defined(ARCH_CPU_ARM64) || defined(ARCH_CPU_X86_64)) && \
    !defined(COMPONENT_BUILD) &&                             \
    !(BUILDFLAG(IS_WIN) && defined(ARCH_CPU_ARM64))
#define PRESERVE_MOST [[clang::preserve_most]]
#else
#define PRESERVE_MOST
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
//      S(int* p LIFETIME_BOUND);
//      int* Get() LIFETIME_BOUND;
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
#if __has_cpp_attribute(clang::lifetimebound)
#define LIFETIME_BOUND [[clang::lifetimebound]]
#else
#define LIFETIME_BOUND
#endif

// Annotates a function or variable to indicate that it should have weak
// linkage. Useful for library code that wants code linking against it to be
// able to override its functionality; inside a single target, this is better
// accomplished via virtual methods and other more standard mechanisms.
//
// Any weak definition of a symbol will be overridden at link time by a non-weak
// definition. Marking a `const` or `constexpr` variable weak makes it no longer
// be considered a compile-time constant, since its value may be different after
// linking.
//
// Multiple weak definitions of a symbol may exist, in which case the linker is
// free to select any when there are no non-weak definitions. Like with symbols
// marked `inline`, this can lead to subtle, difficult-to-diagnose bugs if not
// all definitions are identical.
//
// A weak declaration that has no definitions at link time will be linked as if
// the corresponding address is null. Therefore library code can use weak
// declarations and conditionals to allow consumers to provide optional
// customizations.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#weak
//
// Usage:
// ```
//   // The following definition defaults `x` to 10, but allows other object
//   // files to override its value. Thus, despite `constexpr`, `x` is not
//   // considered a compile-time constant (and cannot be used in a `constexpr`
//   // context).
//   extern const int x;
//   WEAK_SYMBOL constexpr int x = 10;
//
//   // The following declaration allows linking to occur whether a definition
//   // of `Func()` is provided or not; if none is present, `&Func` will
//   // evaluate to `nullptr` at runtime.
//   WEAK_SYMBOL void Func();
//
//   // The following definition provides a default implementation of `Func2()`,
//   // but allows other object files to override.
//   WEAK_SYMBOL void Func2() { ... }
// ```
#if __has_cpp_attribute(gnu::weak)
#define WEAK_SYMBOL [[gnu::weak]]
#else
#define WEAK_SYMBOL
#endif

// Annotates a function indicating that the compiler should not convert calls
// within it to tail calls.
//
// For a callee-side version of this, see `NOT_TAIL_CALLED`.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#disable-tail-calls
// Usage:
// ```
//   DISABLE_TAIL_CALLS void Func() {
//     // Function calls in this body will not be tail calls.
//   }
// ```
#if __has_cpp_attribute(clang::disable_tail_calls)
#define DISABLE_TAIL_CALLS [[clang::disable_tail_calls]]
#else
#define DISABLE_TAIL_CALLS
#endif

// Annotates a type or member indicating the minimum possible alignment (one bit
// for bitfields, one byte otherwise) should be used. This can be used to
// eliminate padding inside objects, at the cost of potentially pessimizing
// code, or even generating invalid code (depending on platform restrictions) if
// underaligned objects have their addresses taken and passed elsewhere.
//
// This is similar to the more-broadly-supported `#pragma pack(1)`.
//
// See also:
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Variable-Attributes.html#index-packed-variable-attribute
//
// Usage:
// ```
//   struct PACKED_OBJ S1 {
//     int8_t a;   // Alignment 1, offset 0, size 1
//     int32_t b;  // Alignment 1, offset 1 (0 bytes padding), size 4
//   };  // Overall alignment 1, 0 bytes trailing padding, overall size 5
//
//   struct S2 {
//     int8_t a;              // Alignment 1, offset 0, size 1
//     int32_t b;             // Alignment 4, offset 4 (3 bytes padding), size 4
//     int8_t c;              // Alignment 1, offset 8 (0 bytes padding), size 1
//     PACKED_OBJ int32_t d;  // Alignment 1, offset 9 (0 bytes padding), size 4
//   };  // Overall alignment 4, 3 bytes trailing padding, overall size 16
// ```
#if __has_cpp_attribute(gnu::packed)
#define PACKED_OBJ [[gnu::packed]]
#else
#define PACKED_OBJ
#endif

// Annotates a function indicating that the returned pointer will never be null.
// This may allow the compiler to assume null checks on the caller side are
// unnecessary.
//
// In practice, this is usually better-handled by returning a value or
// reference, which enforce such guarantees at the type level.
//
// See also:
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-returns_005fnonnull-function-attribute
//   https://clang.llvm.org/docs/AttributeReference.html#nullability-attributes
//
// Usage:
// ```
//   // The following function will never return `nullptr`.
//   RETURNS_NONNULL int* Func();
// ```
#if __has_cpp_attribute(gnu::returns_nonnull)
#define RETURNS_NONNULL [[gnu::returns_nonnull]]
#else
#define RETURNS_NONNULL
#endif

// Annotates a function indicating it is const, meaning that it has no
// observable side effects and its return value depends only on its arguments.
// Const functions may not read external memory other than unchanging objects
// (e.g. non-volatile constants), and the compiler is free to replace calls to
// them with the return values of earlier calls with the same arguments no
// matter what other state might have changed in the meantime.
//
// This is a much stronger restriction than `const`-qualified functions, and is
// rarely appropriate outside small local helpers, which are frequently
// inlineable anyway and would not really benefit.
//
// WARNING: Misusing this attribute can lead to silent miscompilation, UB, and
// difficult-to-diagnose bugs. For this and the above reason, usage should be
// very rare.
//
// See also:
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-const-function-attribute
//
// Usage:
// ```
//   // The compiler may replace calls to this function with values returned
//   // from earlier calls, assuming the args match.
//   CONST_FUNCTION int Func(int);
// ```
#if __has_cpp_attribute(gnu::const)
#define CONST_FUNCTION [[gnu::const]]
#else
#define CONST_FUNCTION
#endif

// Annotates a function indicating it is pure, meaning that it has no observable
// side effects. Unlike functions annotated `CONST_FUNCTION`, pure functions may
// still read external memory, and thus their return values may change between
// calls. `strlen()` and `memcmp()` are examples of pure functions. Useful to
// allow folding/reordering calls for optimization purposes.
//
// WARNING: Misusing this attribute can lead to silent miscompilation, UB, and
// difficult-to-diagnose bugs. Because apparently-safe invocations can sometimes
// have side effects (especially when invoking "overridable" functionality like
// virtual or templated methods), such misuse is far more likely than it seems.
// Therefore, this macro should generally be used only in key vocabulary types,
// where the perf and ergonomic benefits of callers not needing to worry about
// caching results in local variables in hot code outweighs the risks.
//
// See also:
//   https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-pure-function-attribute
//
// Usage:
// ```
//   // Calls to this function may be subject to more aggressive common
//   // subexpression (CSE) optimization.
//   PURE_FUNCTION int Func(int);
// ```
#if __has_cpp_attribute(gnu::pure)
#define PURE_FUNCTION [[gnu::pure]]
#else
#define PURE_FUNCTION
#endif

// Annotates a function or class data member indicating it can lead to
// out-of-bounds accesses (OOB) if given incorrect inputs.
//
// For functions, this commonly includes functions which take pointers, sizes,
// iterators, sentinels, etc. and cannot fully check their preconditions (e.g.
// that the provided pointer actually points to an allocation of at least the
// provided size). Useful to diagnose potential misuse via
// `-Wunsafe-buffer-usage`, as well as to mark functions potentially in need of
// safer alternatives.
//
// For fields, this would be used to annotate both pointer and size fields that
// have not yet been converted to a span.
//
// All functions or fields annotated with this macro should come with a `#
// Safety` comment that explains what the caller must guarantee to prevent OOB.
// Ideally, unsafe functions should also be paired with a safer version, e.g.
// one that replaces pointer parameters with `span`s; otherwise, document safer
// replacement coding patterns callers can migrate to.
//
// Annotating a function `UNSAFE_BUFFER_USAGE` means all call sites (that do not
// disable the warning) must wrap calls in `UNSAFE_BUFFERS()`; see documentation
// there. Annotating a field `UNSAFE_BUFFER_USAGE` means that `UNSAFE_BUFFERS()`
// must wrap expressions that mutate of the field.
//
// See also:
//   https://chromium.googlesource.com/chromium/src/+/main/docs/unsafe_buffers.md
//   https://clang.llvm.org/docs/SafeBuffers.html
//   https://clang.llvm.org/docs/DiagnosticsReference.html#wunsafe-buffer-usage
//
// Usage:
// ```
//   // Calls to this function must be wrapped in `UNSAFE_BUFFERS()`.
//   UNSAFE_BUFFER_USAGE void Func(T* input, T* end);
//
//   struct S {
//     // Changing this pointer requires `UNSAFE_BUFFERS()`.
//     UNSAFE_BUFFER_USAGE int* p;
//   };
// ```
#if __has_cpp_attribute(clang::unsafe_buffer_usage)
#define UNSAFE_BUFFER_USAGE [[clang::unsafe_buffer_usage]]
#else
#define UNSAFE_BUFFER_USAGE
#endif

// Annotates code indicating that it should be permanently exempted from
// `-Wunsafe-buffer-usage`. For temporary cases such as migrating callers to
// safer patterns, use `UNSAFE_TODO()` instead; see documentation there.
//
// All calls to functions annotated with `UNSAFE_BUFFER_USAGE` must be marked
// with one of these two macros; they can also be used around pointer
// arithmetic, pointer subscripting, and the like.
//
// ** USE OF THIS MACRO SHOULD BE VERY RARE.** Using this macro indicates that
// the compiler cannot verify that the code avoids OOB, and manual review is
// required. Even with manual review, it's easy for assumptions to change and
// security bugs to creep in over time. Prefer safer patterns instead.
//
// Usage should wrap the minimum necessary code, and *must* include a
// `// SAFETY: ...` comment that explains how the code guarantees safety or
// meets the requirements of called `UNSAFE_BUFFER_USAGE` functions. Guarantees
// must be manually verifiable by the Chrome security team using only local
// invariants; contact security@chromium.org to schedule such a review. Valid
// invariants include:
// - Runtime conditions or `CHECK()`s nearby
// - Invariants guaranteed by types in the surrounding code
// - Invariants guaranteed by function calls in the surrounding code
// - Caller requirements, if the containing function is itself annotated with
//   `UNSAFE_BUFFER_USAGE`; this is less safe and should be a last resort
//
// See also:
//   https://chromium.googlesource.com/chromium/src/+/main/docs/unsafe_buffers.md
//   https://clang.llvm.org/docs/SafeBuffers.html
//   https://clang.llvm.org/docs/DiagnosticsReference.html#wunsafe-buffer-usage
//
// Usage:
// ```
//   // The following call will not trigger a compiler warning even if `Func()`
//   // is annotated `UNSAFE_BUFFER_USAGE`.
//   return UNSAFE_BUFFERS(Func(input, end));
// ```
//
// Test for `__clang__` directly, as there's no `__has_pragma` or similar (see
// https://github.com/llvm/llvm-project/issues/51887).
#if defined(__clang__)
// Disabling `clang-format` allows each `_Pragma` to be on its own line, as
// recommended by https://gcc.gnu.org/onlinedocs/cpp/Pragmas.html.
// clang-format off
#define UNSAFE_BUFFERS(...)                  \
  _Pragma("clang unsafe_buffer_usage begin") \
  __VA_ARGS__                                \
  _Pragma("clang unsafe_buffer_usage end")
// clang-format on
#else
#define UNSAFE_BUFFERS(...) __VA_ARGS__
#endif

// Annotates code indicating that it should be temporarily exempted from
// `-Wunsafe-buffer-usage`. While this is functionally the same as
// `UNSAFE_BUFFERS()`, semantically it indicates that this is for migration
// purposes, and should be cleaned up as soon as possible.
//
// Usage:
// ```
//   // The following call will not trigger a compiler warning even if `Func()`
//   // is annotated `UNSAFE_BUFFER_USAGE`.
//   return UNSAFE_TODO(Func(input, end));
// ```
#define UNSAFE_TODO(...) UNSAFE_BUFFERS(__VA_ARGS__)

// Annotates a function restricting its availability based on compile-time
// information in the evaluated context. Useful to convert runtime errors to
// compile-time errors if functions' arguments are always known at compile time.
//
// SFINAE and `requires` clauses can restrict function availability based on the
// unevaluated context (type information and syntactic correctness). This
// provides a similar capability based on the evaluated context (variable
// values). If the condition fails, or cannot be determined at compile time, the
// function is excluded from the overload set.
//
// Some use cases could be satisfied without this by marking the function
// `consteval` and breaking compile when the condition fails (e.g. via
// `CHECK()`/`assert()`). However, `ENABLE_IF_ATTR()` is generally superior:
//   - Not all desired functions can be made `consteval`; e.g. most
//     constructors.
//   - The error message in the macro case is clearer and more actionable.
//   - `ENABLE_IF_ATTR()` interacts better with template metaprogramming.
//
// See also:
//   https://clang.llvm.org/docs/AttributeReference.html#enable-if
//   https://github.com/chromium/subspace/issues/266
//
// Usage:
// ```
//   void NotConsteval(int a) {
//     assert(a > 0);
//   }
//   consteval void WithoutEnableIf(int a) {
//     assert(a > 0);
//   }
//   void WithEnableIf(int a) ENABLE_IF_ATTR(a > 0, "arg must be positive") {}
//   void Func(int i) {
//     // Compiles; assertion fails at runtime.
//     NotConsteval(-1);
//
//     // Will not compile; diagnosed as not a constant expression.
//     WithoutEnableIf(-1);
//
//     // Will not compile; diagnosed as no matching function call with
//     // "note: candidate disabled: arg must be positive".
//     WithEnableIf(-1);
//
//     // Will not compile (same reason). Marking `Func()` as
//     // `ENABLE_IF_ATTR(i > 0, ...)` will not help; the compiler's analysis is
//     // not sufficiently sophisticated to propagate this constraint.
//     WithEnableIf(i);
//   }
// ```
#if HAS_ATTRIBUTE(enable_if)
#define ENABLE_IF_ATTR(cond, msg) __attribute__((enable_if(cond, msg)))
#else
#define ENABLE_IF_ATTR(cond, msg)
#endif

#endif  // BASE_COMPILER_SPECIFIC_H_
