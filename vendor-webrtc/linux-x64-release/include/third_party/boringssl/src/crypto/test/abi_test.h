// Copyright 2018 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_TEST_ABI_TEST_H
#define OPENSSL_HEADER_CRYPTO_TEST_ABI_TEST_H

#include <gtest/gtest.h>

#include <string>
#include <type_traits>
#include <vector>

#include <openssl/base.h>

#include "../internal.h"


// abi_test provides routines for verifying that functions satisfy platform ABI
// requirements.
namespace abi_test {

// Result stores the result of an ABI test.
struct Result {
  bool ok() const { return errors.empty(); }

  std::vector<std::string> errors;
};

namespace internal {

// DeductionGuard wraps |T| in a template, so that template argument deduction
// does not apply to it. This may be used to force C++ to deduce template
// arguments from another parameter.
template <typename T>
struct DeductionGuard {
  using Type = T;
};

// Reg128 contains storage space for a 128-bit register.
struct alignas(16) Reg128 {
  bool operator==(const Reg128 &x) const { return x.lo == lo && x.hi == hi; }
  bool operator!=(const Reg128 &x) const { return !((*this) == x); }
  uint64_t lo, hi;
};

// LOOP_CALLER_STATE_REGISTERS is a macro that iterates over all registers the
// callee is expected to save for the caller, with the exception of the stack
// pointer. The stack pointer is tested implicitly by the function successfully
// returning at all.
#if defined(OPENSSL_X86_64)

// References:
// SysV64: https://github.com/hjl-tools/x86-psABI/wiki/x86-64-psABI-1.0.pdf
// Win64: https://docs.microsoft.com/en-us/cpp/build/x64-software-conventions?view=vs-2017#register-usage
#if defined(OPENSSL_WINDOWS)
#define LOOP_CALLER_STATE_REGISTERS()  \
  CALLER_STATE_REGISTER(uint64_t, rbx) \
  CALLER_STATE_REGISTER(uint64_t, rbp) \
  CALLER_STATE_REGISTER(uint64_t, rdi) \
  CALLER_STATE_REGISTER(uint64_t, rsi) \
  CALLER_STATE_REGISTER(uint64_t, r12) \
  CALLER_STATE_REGISTER(uint64_t, r13) \
  CALLER_STATE_REGISTER(uint64_t, r14) \
  CALLER_STATE_REGISTER(uint64_t, r15) \
  CALLER_STATE_REGISTER(Reg128, xmm6)  \
  CALLER_STATE_REGISTER(Reg128, xmm7)  \
  CALLER_STATE_REGISTER(Reg128, xmm8)  \
  CALLER_STATE_REGISTER(Reg128, xmm9)  \
  CALLER_STATE_REGISTER(Reg128, xmm10) \
  CALLER_STATE_REGISTER(Reg128, xmm11) \
  CALLER_STATE_REGISTER(Reg128, xmm12) \
  CALLER_STATE_REGISTER(Reg128, xmm13) \
  CALLER_STATE_REGISTER(Reg128, xmm14) \
  CALLER_STATE_REGISTER(Reg128, xmm15)
#else
#define LOOP_CALLER_STATE_REGISTERS()  \
  CALLER_STATE_REGISTER(uint64_t, rbx) \
  CALLER_STATE_REGISTER(uint64_t, rbp) \
  CALLER_STATE_REGISTER(uint64_t, r12) \
  CALLER_STATE_REGISTER(uint64_t, r13) \
  CALLER_STATE_REGISTER(uint64_t, r14) \
  CALLER_STATE_REGISTER(uint64_t, r15)
#endif  // OPENSSL_WINDOWS

#elif defined(OPENSSL_X86)

// References:
// SysV32: https://uclibc.org/docs/psABI-i386.pdf and
// Win32: https://docs.microsoft.com/en-us/cpp/cpp/argument-passing-and-naming-conventions?view=vs-2017
#define LOOP_CALLER_STATE_REGISTERS()  \
  CALLER_STATE_REGISTER(uint32_t, esi) \
  CALLER_STATE_REGISTER(uint32_t, edi) \
  CALLER_STATE_REGISTER(uint32_t, ebx) \
  CALLER_STATE_REGISTER(uint32_t, ebp)

#elif defined(OPENSSL_ARM)

// References:
// AAPCS: https://developer.arm.com/docs/ihi0042/latest
// iOS32: https://developer.apple.com/library/archive/documentation/Xcode/Conceptual/iPhoneOSABIReference/Articles/ARMv6FunctionCallingConventions.html
// Linux: http://sourcery.mentor.com/sgpp/lite/arm/portal/kbattach142/arm_gnu_linux_%20abi.pdf
//
// ARM specifies a common calling convention, except r9 is left to the platform.
// Linux treats r9 as callee-saved, while iOS 3+ treats it as caller-saved. Most
// of our assembly treats it as callee-saved to be uniform, but we match the
// platform to avoid false positives when testing compiler-generated output.
#define LOOP_CALLER_STATE_REGISTERS_PRE_R9() \
  CALLER_STATE_REGISTER(uint64_t, d8)        \
  CALLER_STATE_REGISTER(uint64_t, d9)        \
  CALLER_STATE_REGISTER(uint64_t, d10)       \
  CALLER_STATE_REGISTER(uint64_t, d11)       \
  CALLER_STATE_REGISTER(uint64_t, d12)       \
  CALLER_STATE_REGISTER(uint64_t, d13)       \
  CALLER_STATE_REGISTER(uint64_t, d14)       \
  CALLER_STATE_REGISTER(uint64_t, d15)       \
  CALLER_STATE_REGISTER(uint32_t, r4)        \
  CALLER_STATE_REGISTER(uint32_t, r5)        \
  CALLER_STATE_REGISTER(uint32_t, r6)        \
  CALLER_STATE_REGISTER(uint32_t, r7)        \
  CALLER_STATE_REGISTER(uint32_t, r8)
#define LOOP_CALLER_STATE_REGISTERS_POST_R9() \
  CALLER_STATE_REGISTER(uint32_t, r10)        \
  CALLER_STATE_REGISTER(uint32_t, r11)
#if defined(OPENSSL_APPLE)
#define LOOP_CALLER_STATE_REGISTERS()  \
  LOOP_CALLER_STATE_REGISTERS_PRE_R9() \
  LOOP_CALLER_STATE_REGISTERS_POST_R9()
#else  // !OPENSSL_APPLE
#define LOOP_CALLER_STATE_REGISTERS()  \
  LOOP_CALLER_STATE_REGISTERS_PRE_R9() \
  CALLER_STATE_REGISTER(uint32_t, r9)  \
  LOOP_CALLER_STATE_REGISTERS_POST_R9()
#endif  // OPENSSL_APPLE

#elif defined(OPENSSL_AARCH64)

// References:
// AAPCS64: https://developer.arm.com/docs/ihi0055/latest
// iOS64: https://developer.apple.com/library/archive/documentation/Xcode/Conceptual/iPhoneOSABIReference/Articles/ARM64FunctionCallingConventions.html
//
// In aarch64, r18 (accessed as w18 or x18 in a 64-bit context) is the platform
// register. iOS says user code may not touch it. We found no clear reference
// for Linux. The iOS behavior implies portable assembly cannot use it, and
// aarch64 has many registers. Thus this framework ignores register's existence.
// We test r18 violations in arm-xlate.pl.
#define LOOP_CALLER_STATE_REGISTERS()                                \
  /* Per AAPCS64, section 5.1.2, only the bottom 64 bits of v8-v15 */ \
  /* are preserved. These are accessed as dN. */                     \
  CALLER_STATE_REGISTER(uint64_t, d8)                                \
  CALLER_STATE_REGISTER(uint64_t, d9)                                \
  CALLER_STATE_REGISTER(uint64_t, d10)                               \
  CALLER_STATE_REGISTER(uint64_t, d11)                               \
  CALLER_STATE_REGISTER(uint64_t, d12)                               \
  CALLER_STATE_REGISTER(uint64_t, d13)                               \
  CALLER_STATE_REGISTER(uint64_t, d14)                               \
  CALLER_STATE_REGISTER(uint64_t, d15)                               \
  /* For consistency with dN, use the 64-bit name xN, rather than */ \
  /* the generic rN. */                                              \
  CALLER_STATE_REGISTER(uint64_t, x19)                               \
  CALLER_STATE_REGISTER(uint64_t, x20)                               \
  CALLER_STATE_REGISTER(uint64_t, x21)                               \
  CALLER_STATE_REGISTER(uint64_t, x22)                               \
  CALLER_STATE_REGISTER(uint64_t, x23)                               \
  CALLER_STATE_REGISTER(uint64_t, x24)                               \
  CALLER_STATE_REGISTER(uint64_t, x25)                               \
  CALLER_STATE_REGISTER(uint64_t, x26)                               \
  CALLER_STATE_REGISTER(uint64_t, x27)                               \
  CALLER_STATE_REGISTER(uint64_t, x28)                               \
  CALLER_STATE_REGISTER(uint64_t, x29)

#endif  // X86_64 || X86 || ARM || AARCH64

// Enable ABI testing if all of the following are true.
//
// - We have CallerState and trampoline support for the architecture.
//
// - Assembly is enabled.
//
// - This is not a shared library build. Assembly functions are not reachable
//   from tests in shared library builds.
#if defined(LOOP_CALLER_STATE_REGISTERS) && !defined(OPENSSL_NO_ASM) && \
    !defined(BORINGSSL_SHARED_LIBRARY)
#define SUPPORTS_ABI_TEST

// CallerState contains all caller state that the callee is expected to
// preserve.
struct CallerState {
#define CALLER_STATE_REGISTER(type, name) type name;
  LOOP_CALLER_STATE_REGISTERS()
#undef CALLER_STATE_REGISTER
};

// RunTrampoline runs |func| on |argv|, recording ABI errors in |out|. It does
// not perform any type-checking. If |unwind| is true and unwind tests have been
// enabled, |func| is single-stepped under an unwind test.
crypto_word_t RunTrampoline(Result *out, crypto_word_t func,
                            const crypto_word_t *argv, size_t argc,
                            bool unwind);

template <typename T>
inline crypto_word_t ToWord(T t) {
  // ABIs typically pass floats and structs differently from integers and
  // pointers. We only need to support the latter.
  static_assert(std::is_integral<T>::value || std::is_pointer<T>::value,
                "parameter types must be integral or pointer types");
  // We only support types which fit in registers.
  static_assert(sizeof(T) <= sizeof(crypto_word_t),
                "parameter types must be at most word-sized");

  // ABIs are complex around arguments that are smaller than native words.
  // Parameters passed in memory are sometimes packed and sometimes padded to a
  // word. When parameters are padded in memory or passed in a larger register,
  // the unused bits may be undefined or sign- or zero-extended.
  //
  // We could simply cast to |crypto_word_t| everywhere but, on platforms where
  // padding is undefined, we perturb the bits to test the function accounts for
  // for this.
#if defined(OPENSSL_32_BIT)
  // We never pass parameters smaller than int, so require word-sized parameters
  // on 32-bit architectures for simplicity.
  static_assert(sizeof(T) == 4, "parameter types must be word-sized");
  return (crypto_word_t)t;
#elif defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)
  // AAPCS64, section 5.4.2, clauses C.7 and C.14 says any remaining bits in
  // aarch are unspecified. iOS64 contradicts this and says the callee extends
  // arguments up to 32 bits, and only the upper 32 bits are unspecified.
  //
  // On x86_64, Win64 leaves all unused bits unspecified. SysV also leaves
  // unused bits in stack parameters unspecified, but it behaves like iOS64 for
  // register parameters. This was determined via experimentation.
  //
  // We limit to 32-bit and 64-bit parameters, the subset where the above all
  // align, and then test that functions tolerate arbitrary unused bits.
  //
  // TODO(davidben): Find authoritative citations for x86_64. For x86_64, I
  // observed the behavior of Clang, GCC, and MSVC. ABI rules here may be
  // inferred from two kinds of experiments:
  //
  // 1. When passing a value to a small-argument-taking function, does the
  //    compiler ensure unused bits are cleared, sign-extended, etc.? Tests for
  //    register parameters are confounded by x86_64's implicit clearing of
  //    registers' upper halves, but passing some_u64 >> 1 usually clears this.
  //
  // 2. When compiling a small-argument-taking function, does the compiler make
  //    assumptions about unused bits of arguments?
  //
  // MSVC was observed to tolerate and produce arbitrary values for unused bits,
  // which is conclusive. GCC and Clang, targeting Linux, were similarly
  // conclusive on stack parameters. Clang was also conclusive for register
  // parameters. Callers only extended parameters up to 32 bits, and callees
  // took advantage of the 32-bit extension. GCC only exhibited the callee
  // behavior.
  static_assert(sizeof(T) >= 4, "parameters must be at least 32 bits wide");
  crypto_word_t ret;
  // Filling extra bits with 0xaa will be vastly out of bounds for code
  // expecting either sign- or zero-extension. (0xaa is 0b10101010.)
  OPENSSL_memset(&ret, 0xaa, sizeof(ret));
  OPENSSL_memcpy(&ret, &t, sizeof(t));
  return ret;
#else
#error "unknown architecture"
#endif
}

// CheckImpl runs |func| on |args|, recording ABI errors in |out|. If |unwind|
// is true and unwind tests have been enabled, |func| is single-stepped under an
// unwind test.
//
// It returns the value as a |crypto_word_t| to work around problems when |R| is
// void. |args| is wrapped in a |DeductionGuard| so |func| determines the
// template arguments. Otherwise, |args| may deduce |Args| incorrectly. For
// instance, if |func| takes const int *, and the caller passes an int *, the
// compiler will complain the deduced types do not match.
template <typename R, typename... Args>
inline crypto_word_t CheckImpl(Result *out, bool unwind, R (*func)(Args...),
                               typename DeductionGuard<Args>::Type... args) {
  // We only support up to 8 arguments, so all arguments on aarch64 are passed
  // in registers. This is simpler and avoids the iOS discrepancy around packing
  // small arguments on the stack. (See the iOS64 reference.)
  static_assert(sizeof...(args) <= 8,
                "too many arguments for abi_test_trampoline");

  // Allocate one extra entry so MSVC does not complain about zero-size arrays.
  crypto_word_t argv[sizeof...(args) + 1] = {
      ToWord(args)...,
  };
  return RunTrampoline(out, reinterpret_cast<crypto_word_t>(func), argv,
                       sizeof...(args), unwind);
}
#else
// To simplify callers when ABI testing support is unavoidable, provide a backup
// CheckImpl implementation. It must be specialized for void returns because we
// call |func| directly.
template <typename R, typename... Args>
inline std::enable_if_t<!std::is_void<R>::value, crypto_word_t> CheckImpl(
    Result *out, bool /* unwind */, R (*func)(Args...),
    typename DeductionGuard<Args>::Type... args) {
  *out = Result();
  return func(args...);
}

template <typename... Args>
inline crypto_word_t CheckImpl(Result *out, bool /* unwind */,
                               void (*func)(Args...),
                               typename DeductionGuard<Args>::Type... args) {
  *out = Result();
  func(args...);
  return 0;
}
#endif  // SUPPORTS_ABI_TEST

// FixVAArgsString takes a string like "f, 1, 2" and returns a string like
// "f(1, 2)".
//
// This is needed because the |CHECK_ABI| macro below cannot be defined as
// CHECK_ABI(func, ...). The C specification requires that variadic macros bind
// at least one variadic argument. Clang, GCC, and MSVC all ignore this, but
// there are issues with trailing commas and different behaviors across
// compilers.
std::string FixVAArgsString(const char *str);

// CheckGTest behaves like |CheckImpl|, but it returns the correct type and
// raises GTest assertions on failure. If |unwind| is true and unwind tests are
// enabled, |func| is single-stepped under an unwind test.
template <typename R, typename... Args>
inline R CheckGTest(const char *va_args_str, const char *file, int line,
                    bool unwind, R (*func)(Args...),
                    typename DeductionGuard<Args>::Type... args) {
  Result result;
  crypto_word_t ret = CheckImpl(&result, unwind, func, args...);
  if (!result.ok()) {
    testing::Message msg;
    msg << "ABI failures in " << FixVAArgsString(va_args_str) << ":\n";
    for (const auto &error : result.errors) {
      msg << "    " << error << "\n";
    }
    ADD_FAILURE_AT(file, line) << msg;
  }
  return (R)ret;
}

}  // namespace internal

// Check runs |func| on |args| and returns the result. If ABI-testing is
// supported in this build configuration, it writes any ABI failures to |out|.
// Otherwise, it runs the function transparently.
template <typename R, typename... Args>
inline R Check(Result *out, R (*func)(Args...),
               typename internal::DeductionGuard<Args>::Type... args) {
  return (R)internal::CheckImpl(out, false, func, args...);
}

// EnableUnwindTests enables unwind tests, if supported. If not supported, it
// does nothing.
void EnableUnwindTests();

// UnwindTestsEnabled returns true if unwind tests are enabled and false
// otherwise.
bool UnwindTestsEnabled();

}  // namespace abi_test

// CHECK_ABI calls the first argument on the remaining arguments and returns the
// result. If ABI-testing is supported in this build configuration, it adds a
// non-fatal GTest failure if the call did not satisfy ABI requirements.
//
// |CHECK_ABI| does return the value and thus may replace any function call,
// provided it takes only simple parameters. However, it is recommended to test
// ABI separately from functional tests of assembly. Fully instrumenting a
// function for ABI checking requires single-stepping the function, which is
// inefficient.
//
// Functional testing requires coverage of input values, while ABI testing only
// requires branch coverage. Most of our assembly is constant-time, so usually
// only a few instrumented calls are necessary.
//
// TODO(https://crbug.com/boringssl/259): Most of Windows assembly currently
// fails SEH testing. For now, |CHECK_ABI| behaves like |CHECK_ABI_NO_UNWIND|
// on Windows. Functions which work with unwind testing on Windows should use
// |CHECK_ABI_SEH|.
#if defined(OPENSSL_WINDOWS)
#define CHECK_ABI(...) CHECK_ABI_NO_UNWIND(__VA_ARGS__)
#else
#define CHECK_ABI(...) CHECK_ABI_SEH(__VA_ARGS__)
#endif

// CHECK_ABI_SEH behaves like |CHECK_ABI| but enables unwind testing on Windows.
#define CHECK_ABI_SEH(...)                                               \
  abi_test::internal::CheckGTest(#__VA_ARGS__, __FILE__, __LINE__, true, \
                                 __VA_ARGS__)

// CHECK_ABI_NO_UNWIND behaves like |CHECK_ABI| but disables unwind testing.
#define CHECK_ABI_NO_UNWIND(...)                                          \
  abi_test::internal::CheckGTest(#__VA_ARGS__, __FILE__, __LINE__, false, \
                                 __VA_ARGS__)


// Internal functions.

#if defined(SUPPORTS_ABI_TEST)
struct Uncallable {
  Uncallable() = delete;
};

extern "C" {

// abi_test_trampoline loads callee-saved registers from |state|, calls |func|
// with |argv|, then saves the callee-saved registers into |state|. It returns
// the result of |func|. If |unwind| is non-zero, this function triggers unwind
// instrumentation.
//
// We give |func| type |crypto_word_t| to avoid tripping MSVC's warning 4191.
crypto_word_t abi_test_trampoline(crypto_word_t func,
                                  abi_test::internal::CallerState *state,
                                  const crypto_word_t *argv, size_t argc,
                                  crypto_word_t unwind);

#if defined(OPENSSL_X86_64)
// abi_test_unwind_start points at the instruction that starts unwind testing in
// |abi_test_trampoline|. This is the value of the instruction pointer at the
// first |SIGTRAP| during unwind testing.
//
// This symbol is not a function and should not be called.
void abi_test_unwind_start(Uncallable);

// abi_test_unwind_return points at the instruction immediately after the call in
// |abi_test_trampoline|. When unwinding the function under test, this is the
// expected address in the |abi_test_trampoline| frame. After this address, the
// unwind tester should ignore |SIGTRAP| until |abi_test_unwind_stop|.
//
// This symbol is not a function and should not be called.
void abi_test_unwind_return(Uncallable);

// abi_test_unwind_stop is the value of the instruction pointer at the final
// |SIGTRAP| during unwind testing.
//
// This symbol is not a function and should not be called.
void abi_test_unwind_stop(Uncallable);

// abi_test_bad_unwind_wrong_register preserves the ABI, but annotates the wrong
// register in unwind metadata.
void abi_test_bad_unwind_wrong_register(void);

// abi_test_bad_unwind_temporary preserves the ABI, but temporarily corrupts the
// storage space for a saved register, breaking unwind.
void abi_test_bad_unwind_temporary(void);

#if defined(OPENSSL_WINDOWS)
// abi_test_bad_unwind_epilog preserves the ABI, and correctly annotates the
// prolog, but the epilog does not match Win64's rules, breaking unwind during
// the epilog.
void abi_test_bad_unwind_epilog(void);
#endif
#endif  // OPENSSL_X86_64

#if defined(OPENSSL_X86_64) || defined(OPENSSL_X86)
// abi_test_get_and_clear_direction_flag clears the direction flag. If the flag
// was previously set, it returns one. Otherwise, it returns zero.
int abi_test_get_and_clear_direction_flag(void);

// abi_test_set_direction_flag sets the direction flag. This does not conform to
// ABI requirements and must only be called within a |CHECK_ABI| guard to avoid
// errors later in the program.
int abi_test_set_direction_flag(void);
#endif  // OPENSSL_X86_64 || OPENSSL_X86

}  // extern "C"
#endif  // SUPPORTS_ABI_TEST


#endif  // OPENSSL_HEADER_CRYPTO_TEST_ABI_TEST_H
