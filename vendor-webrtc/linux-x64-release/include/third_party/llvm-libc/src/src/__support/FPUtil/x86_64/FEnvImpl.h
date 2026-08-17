//===-- x86_64 floating point env manipulation functions --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_FPUTIL_X86_64_FENVIMPL_H
#define LLVM_LIBC_SRC___SUPPORT_FPUTIL_X86_64_FENVIMPL_H

#include "src/__support/macros/attributes.h" // LIBC_INLINE
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"

#if !defined(LIBC_TARGET_ARCH_IS_X86)
#error "Invalid include"
#endif

#include <stdint.h>

#include "hdr/types/fenv_t.h"
#include "src/__support/macros/sanitizer.h"

namespace LIBC_NAMESPACE_DECL {
namespace fputil {

namespace internal {

// Normally, one should be able to define FE_* macros to the exact rounding mode
// encodings. However, since we want LLVM libc to be compiled against headers
// from other libcs, we cannot assume that FE_* macros are always defined in
// such a manner. So, we will define enums corresponding to the x86_64 bit
// encodings. The implementations can map from FE_* to the corresponding enum
// values.

// The rounding control values in the x87 control register and the MXCSR
// register have the same 2-bit enoding but have different bit positions.
// See below for the bit positions.
struct RoundingControlValue {
  static constexpr uint16_t TO_NEAREST = 0x0;
  static constexpr uint16_t DOWNWARD = 0x1;
  static constexpr uint16_t UPWARD = 0x2;
  static constexpr uint16_t TOWARD_ZERO = 0x3;
};

static constexpr uint16_t X87_ROUNDING_CONTROL_BIT_POSITION = 10;
static constexpr uint16_t MXCSR_ROUNDING_CONTROL_BIT_POSITION = 13;

// The exception flags in the x87 status register and the MXCSR have the same
// encoding as well as the same bit positions.
struct ExceptionFlags {
  static constexpr uint16_t INVALID_F = 0x1;
  // Some libcs define __FE_DENORM corresponding to the denormal input
  // exception and include it in FE_ALL_EXCEPTS. We define and use it to
  // support compiling against headers provided by such libcs.
  static constexpr uint16_t DENORMAL_F = 0x2;
  static constexpr uint16_t DIV_BY_ZERO_F = 0x4;
  static constexpr uint16_t OVERFLOW_F = 0x8;
  static constexpr uint16_t UNDERFLOW_F = 0x10;
  static constexpr uint16_t INEXACT_F = 0x20;
};

// The exception control bits occupy six bits, one bit for each exception.
// In the x87 control word, they occupy the first 6 bits. In the MXCSR
// register, they occupy bits 7 to 12.
static constexpr uint16_t X87_EXCEPTION_CONTROL_BIT_POSITION = 0;
static constexpr uint16_t X87_EXCEPTION_CONTROL_BIT_POSITION_HIGH = 24;
static constexpr uint16_t MXCSR_EXCEPTION_CONTOL_BIT_POISTION = 7;

// Exception flags are individual bits in the corresponding registers.
// So, we just OR the bit values to get the full set of exceptions.
LIBC_INLINE uint16_t get_status_value_for_except(int excepts) {
  // We will make use of the fact that exception control bits are single
  // bit flags in the control registers.
  return ((excepts & FE_INVALID) ? ExceptionFlags::INVALID_F : 0) |
#ifdef __FE_DENORM
         ((excepts & __FE_DENORM) ? ExceptionFlags::DENORMAL_F : 0) |
#endif // __FE_DENORM
         ((excepts & FE_DIVBYZERO) ? ExceptionFlags::DIV_BY_ZERO_F : 0) |
         ((excepts & FE_OVERFLOW) ? ExceptionFlags::OVERFLOW_F : 0) |
         ((excepts & FE_UNDERFLOW) ? ExceptionFlags::UNDERFLOW_F : 0) |
         ((excepts & FE_INEXACT) ? ExceptionFlags::INEXACT_F : 0);
}

LIBC_INLINE int exception_status_to_macro(uint16_t status) {
  return ((status & ExceptionFlags::INVALID_F) ? FE_INVALID : 0) |
#ifdef __FE_DENORM
         ((status & ExceptionFlags::DENORMAL_F) ? __FE_DENORM : 0) |
#endif // __FE_DENORM
         ((status & ExceptionFlags::DIV_BY_ZERO_F) ? FE_DIVBYZERO : 0) |
         ((status & ExceptionFlags::OVERFLOW_F) ? FE_OVERFLOW : 0) |
         ((status & ExceptionFlags::UNDERFLOW_F) ? FE_UNDERFLOW : 0) |
         ((status & ExceptionFlags::INEXACT_F) ? FE_INEXACT : 0);
}

struct X87StateDescriptor {
  uint16_t control_word;
  uint16_t unused1;
  uint16_t status_word;
  uint16_t unused2;
  // TODO: Elaborate the remaining 20 bytes as required.
  uint32_t _[5];
};

LIBC_INLINE uint16_t get_x87_control_word() {
  uint16_t w;
  __asm__ __volatile__("fnstcw %0" : "=m"(w)::);
  MSAN_UNPOISON(&w, sizeof(w));
  return w;
}

LIBC_INLINE void write_x87_control_word(uint16_t w) {
  __asm__ __volatile__("fldcw %0" : : "m"(w) :);
}

LIBC_INLINE uint16_t get_x87_status_word() {
  uint16_t w;
  __asm__ __volatile__("fnstsw %0" : "=m"(w)::);
  MSAN_UNPOISON(&w, sizeof(w));
  return w;
}

LIBC_INLINE void clear_x87_exceptions() {
  __asm__ __volatile__("fnclex" : : :);
}

LIBC_INLINE uint32_t get_mxcsr() {
  uint32_t w;
  __asm__ __volatile__("stmxcsr %0" : "=m"(w)::);
  MSAN_UNPOISON(&w, sizeof(w));
  return w;
}

LIBC_INLINE void write_mxcsr(uint32_t w) {
  __asm__ __volatile__("ldmxcsr %0" : : "m"(w) :);
}

LIBC_INLINE void get_x87_state_descriptor(X87StateDescriptor &s) {
  __asm__ __volatile__("fnstenv %0" : "=m"(s));
  MSAN_UNPOISON(&s, sizeof(s));
}

LIBC_INLINE void write_x87_state_descriptor(const X87StateDescriptor &s) {
  __asm__ __volatile__("fldenv %0" : : "m"(s) :);
}

LIBC_INLINE void fwait() { __asm__ __volatile__("fwait"); }

} // namespace internal

LIBC_INLINE int enable_except(int excepts) {
  // In the x87 control word and in MXCSR, an exception is blocked
  // if the corresponding bit is set. That is the reason for all the
  // bit-flip operations below as we need to turn the bits to zero
  // to enable them.

  uint16_t bit_mask = internal::get_status_value_for_except(excepts);

  uint16_t x87_cw = internal::get_x87_control_word();
  uint16_t old_excepts = ~x87_cw & 0x3F; // Save previously enabled exceptions.
  x87_cw &= ~bit_mask;
  internal::write_x87_control_word(x87_cw);

  // Enabling SSE exceptions via MXCSR is a nice thing to do but
  // might not be of much use practically as SSE exceptions and the x87
  // exceptions are independent of each other.
  uint32_t mxcsr = internal::get_mxcsr();
  mxcsr &= ~(bit_mask << internal::MXCSR_EXCEPTION_CONTOL_BIT_POISTION);
  internal::write_mxcsr(mxcsr);

  // Since the x87 exceptions and SSE exceptions are independent of each,
  // it doesn't make much sence to report both in the return value. Most
  // often, the standard floating point functions deal with FPU operations
  // so we will retrun only the old x87 exceptions.
  return internal::exception_status_to_macro(old_excepts);
}

LIBC_INLINE int disable_except(int excepts) {
  // In the x87 control word and in MXCSR, an exception is blocked
  // if the corresponding bit is set.

  uint16_t bit_mask = internal::get_status_value_for_except(excepts);

  uint16_t x87_cw = internal::get_x87_control_word();
  uint16_t old_excepts = ~x87_cw & 0x3F; // Save previously enabled exceptions.
  x87_cw |= bit_mask;
  internal::write_x87_control_word(x87_cw);

  // Just like in enable_except, it is not clear if disabling SSE exceptions
  // is required. But, we will still do it only as a "nice thing to do".
  uint32_t mxcsr = internal::get_mxcsr();
  mxcsr |= (bit_mask << internal::MXCSR_EXCEPTION_CONTOL_BIT_POISTION);
  internal::write_mxcsr(mxcsr);

  return internal::exception_status_to_macro(old_excepts);
}

LIBC_INLINE int get_except() {
  uint16_t mxcsr = static_cast<uint16_t>(internal::get_mxcsr());
  uint16_t enabled_excepts = ~(mxcsr >> 7) & 0x3F;
  return internal::exception_status_to_macro(enabled_excepts);
}

LIBC_INLINE int clear_except(int excepts) {
  internal::X87StateDescriptor state;
  internal::get_x87_state_descriptor(state);
  state.status_word &=
      static_cast<uint16_t>(~internal::get_status_value_for_except(excepts));
  internal::write_x87_state_descriptor(state);

  uint32_t mxcsr = internal::get_mxcsr();
  mxcsr &= ~internal::get_status_value_for_except(excepts);
  internal::write_mxcsr(mxcsr);
  return 0;
}

LIBC_INLINE int test_except(int excepts) {
  uint16_t status_word = internal::get_x87_status_word();
  uint32_t mxcsr = internal::get_mxcsr();
  // Check both x87 status word and MXCSR.
  uint16_t status_value = internal::get_status_value_for_except(excepts);
  return internal::exception_status_to_macro(
      static_cast<uint16_t>(status_value & (status_word | mxcsr)));
}

// Sets the exception flags but does not trigger the exception handler.
LIBC_INLINE int set_except(int excepts) {
  uint16_t status_value = internal::get_status_value_for_except(excepts);
  internal::X87StateDescriptor state;
  internal::get_x87_state_descriptor(state);
  state.status_word |= status_value;
  internal::write_x87_state_descriptor(state);

  uint32_t mxcsr = internal::get_mxcsr();
  mxcsr |= status_value;
  internal::write_mxcsr(mxcsr);

  return 0;
}

LIBC_INLINE int raise_except(int excepts) {
  uint16_t status_value = internal::get_status_value_for_except(excepts);

  // We set the status flag for exception one at a time and call the
  // fwait instruction to actually get the processor to raise the
  // exception by calling the exception handler. This scheme is per
  // the description in "8.6 X87 FPU EXCEPTION SYNCHRONIZATION"
  // of the "Intel 64 and IA-32 Architectures Software Developer's
  // Manual, Vol 1".

  // FPU status word is read for each exception separately as the
  // exception handler can potentially write to it (typically to clear
  // the corresponding exception flag). By reading it separately, we
  // ensure that the writes by the exception handler are maintained
  // when raising the next exception.

  auto raise_helper = [](uint16_t singleExceptFlag) {
    internal::X87StateDescriptor state;
    uint32_t mxcsr = 0;
    internal::get_x87_state_descriptor(state);
    mxcsr = internal::get_mxcsr();
    state.status_word |= singleExceptFlag;
    mxcsr |= singleExceptFlag;
    internal::write_x87_state_descriptor(state);
    internal::write_mxcsr(mxcsr);
    internal::fwait();
  };

  if (status_value & internal::ExceptionFlags::INVALID_F)
    raise_helper(internal::ExceptionFlags::INVALID_F);
  if (status_value & internal::ExceptionFlags::DIV_BY_ZERO_F)
    raise_helper(internal::ExceptionFlags::DIV_BY_ZERO_F);
  if (status_value & internal::ExceptionFlags::OVERFLOW_F)
    raise_helper(internal::ExceptionFlags::OVERFLOW_F);
  if (status_value & internal::ExceptionFlags::UNDERFLOW_F)
    raise_helper(internal::ExceptionFlags::UNDERFLOW_F);
  if (status_value & internal::ExceptionFlags::INEXACT_F)
    raise_helper(internal::ExceptionFlags::INEXACT_F);
#ifdef __FE_DENORM
  if (status_value & internal::ExceptionFlags::DENORMAL_F) {
    raise_helper(internal::ExceptionFlags::DENORMAL_F);
  }
#endif // __FE_DENORM

  // There is no special synchronization scheme available to
  // raise SEE exceptions. So, we will ignore that for now.
  // Just plain writing to the MXCSR register does not guarantee
  // the exception handler will be called.

  return 0;
}

LIBC_INLINE int get_round() {
  uint16_t bit_value =
      (internal::get_mxcsr() >> internal::MXCSR_ROUNDING_CONTROL_BIT_POSITION) &
      0x3;
  switch (bit_value) {
  case internal::RoundingControlValue::TO_NEAREST:
    return FE_TONEAREST;
  case internal::RoundingControlValue::DOWNWARD:
    return FE_DOWNWARD;
  case internal::RoundingControlValue::UPWARD:
    return FE_UPWARD;
  case internal::RoundingControlValue::TOWARD_ZERO:
    return FE_TOWARDZERO;
  default:
    return -1; // Error value.
  }
}

LIBC_INLINE int set_round(int mode) {
  uint16_t bit_value;
  switch (mode) {
  case FE_TONEAREST:
    bit_value = internal::RoundingControlValue::TO_NEAREST;
    break;
  case FE_DOWNWARD:
    bit_value = internal::RoundingControlValue::DOWNWARD;
    break;
  case FE_UPWARD:
    bit_value = internal::RoundingControlValue::UPWARD;
    break;
  case FE_TOWARDZERO:
    bit_value = internal::RoundingControlValue::TOWARD_ZERO;
    break;
  default:
    return 1; // To indicate failure
  }

  uint16_t x87_value = static_cast<uint16_t>(
      bit_value << internal::X87_ROUNDING_CONTROL_BIT_POSITION);
  uint16_t x87_control = internal::get_x87_control_word();
  x87_control = static_cast<uint16_t>(
      (x87_control &
       ~(uint16_t(0x3) << internal::X87_ROUNDING_CONTROL_BIT_POSITION)) |
      x87_value);
  internal::write_x87_control_word(x87_control);

  uint32_t mxcsr_value = bit_value
                         << internal::MXCSR_ROUNDING_CONTROL_BIT_POSITION;
  uint32_t mxcsr_control = internal::get_mxcsr();
  mxcsr_control = (mxcsr_control &
                   ~(0x3 << internal::MXCSR_ROUNDING_CONTROL_BIT_POSITION)) |
                  mxcsr_value;
  internal::write_mxcsr(mxcsr_control);

  return 0;
}

namespace internal {

#if defined(_WIN32)
// MSVC fenv.h defines a very simple representation of the floating point state
// which just consists of control and status words of the x87 unit.
struct FPState {
  uint32_t control_word;
  uint32_t status_word;
};
#elif defined(__APPLE__)
struct FPState {
  uint16_t control_word;
  uint16_t status_word;
  uint32_t mxcsr;
  uint8_t reserved[8];
};
#else
struct FPState {
  X87StateDescriptor x87_status;
  uint32_t mxcsr;
};
#endif // _WIN32

} // namespace internal

static_assert(
    sizeof(fenv_t) == sizeof(internal::FPState),
    "Internal floating point state does not match the public fenv_t type.");

#ifdef _WIN32

// The exception flags in the Windows FEnv struct and the MXCSR have almost
// reversed bit positions.
struct WinExceptionFlags {
  static constexpr uint32_t INEXACT_WIN = 0x01;
  static constexpr uint32_t UNDERFLOW_WIN = 0x02;
  static constexpr uint32_t OVERFLOW_WIN = 0x04;
  static constexpr uint32_t DIV_BY_ZERO_WIN = 0x08;
  static constexpr uint32_t INVALID_WIN = 0x10;
  static constexpr uint32_t DENORMAL_WIN = 0x20;

  // The Windows FEnv struct has a second copy of all of these bits in the high
  // byte of the 32 bit control word. These are used as the source of truth when
  // calling fesetenv.
  static constexpr uint32_t HIGH_OFFSET = 24;

  static constexpr uint32_t HIGH_INEXACT = INEXACT_WIN << HIGH_OFFSET;
  static constexpr uint32_t HIGH_UNDERFLOW = UNDERFLOW_WIN << HIGH_OFFSET;
  static constexpr uint32_t HIGH_OVERFLOW = OVERFLOW_WIN << HIGH_OFFSET;
  static constexpr uint32_t HIGH_DIV_BY_ZERO = DIV_BY_ZERO_WIN << HIGH_OFFSET;
  static constexpr uint32_t HIGH_INVALID = INVALID_WIN << HIGH_OFFSET;
  static constexpr uint32_t HIGH_DENORMAL = DENORMAL_WIN << HIGH_OFFSET;
};

/*
    fenv_t control word format:

    Windows (at least for x64) uses a 4 byte control fenv control word stored in
    a 32 bit integer. The first byte contains just the rounding mode and the
    exception masks, while the last two bytes contain that same information as
    well as the flush-to-zero and denormals-are-zero flags. The flags are
    represented with a truth table:

    00 - No flags set
    01 - Flush-to-zero and Denormals-are-zero set
    11 - Flush-to-zero set
    10 - Denormals-are-zero set

    U represents unused.

     +-----Rounding Mode-----+
     |                       |
    ++                      ++
    ||                      ||
    RRMMMMMM UUUUUUUU UUUUFFRR UUMMMMMM
      |    |              ||     |    |
      +----+      flags---++     +----+
           |                          |
           +------Exception Masks-----+


    fenv_t status word format:

    The status word is a lot simpler for this conversion, since only the
    exception flags are used in the MXCSR.

      +----+---Exception Flags---+----+
      |    |                     |    |
    UUEEEEEE UUUUUUUU UUUUUUUU UUEEEEEE



    MXCSR Format:

    The MXCSR format is the same information, just organized differently. Since
    the fenv_t struct for windows doesn't include the mxcsr bits, they must be
    generated from the control word bits.

      Exception Masks---+           +---Exception Flags
                        |           |
     Flush-to-zero---+  +----+ +----+
                     |  |    | |    |
                     FRRMMMMMMDEEEEEE
                      ||      |
                      ++      +---Denormals-are-zero
                      |
                      +---Rounding Mode


    The mask and flag order is as follows:

    fenv_t      mxcsr

    denormal    inexact
    invalid     underflow
    div by 0    overflow
    overflow    div by 0
    underflow   denormal
    inexact     invalid

    This is almost reverse, except for denormal and invalid which are in the
    same order in both.
  */

LIBC_INLINE int get_env(fenv_t *envp) {
  internal::FPState *state = reinterpret_cast<internal::FPState *>(envp);

  uint32_t status_word = 0;
  uint32_t control_word = 0;

  uint32_t mxcsr = internal::get_mxcsr();

  // Set exception flags in the status word
  status_word |= (mxcsr & (internal::ExceptionFlags::INVALID_F |
                           internal::ExceptionFlags::DENORMAL_F))
                 << 4;
  status_word |= (mxcsr & internal::ExceptionFlags::DIV_BY_ZERO_F) << 1;
  status_word |= (mxcsr & internal::ExceptionFlags::OVERFLOW_F) >> 1;
  status_word |= (mxcsr & internal::ExceptionFlags::UNDERFLOW_F) >> 3;
  status_word |= (mxcsr & internal::ExceptionFlags::INEXACT_F) >> 5;
  status_word |= status_word << WinExceptionFlags::HIGH_OFFSET;

  // Set exception masks in bits 0-5 and 24-29
  control_word |= (mxcsr & ((internal::ExceptionFlags::INVALID_F |
                             internal::ExceptionFlags::DENORMAL_F)
                            << 7)) >>
                  3;
  control_word |= (mxcsr & (internal::ExceptionFlags::DIV_BY_ZERO_F << 7)) >> 6;
  control_word |= (mxcsr & (internal::ExceptionFlags::OVERFLOW_F << 7)) >> 8;
  control_word |= (mxcsr & (internal::ExceptionFlags::UNDERFLOW_F << 7)) >> 10;
  control_word |= (mxcsr & (internal::ExceptionFlags::INEXACT_F << 7)) >> 12;
  control_word |= control_word << WinExceptionFlags::HIGH_OFFSET;

  // Set rounding in bits 8-9 and 30-31
  control_word |= (mxcsr & 0x6000) >> 5;
  control_word |= (mxcsr & 0x6000) << 17;

  // Set flush-to-zero in bit 10
  control_word |= (mxcsr & 0x8000) >> 5;

  // Set denormals-are-zero xor flush-to-zero in bit 11
  control_word |= (((mxcsr & 0x8000) >> 9) ^ (mxcsr & 0x0040)) << 5;

  state->control_word = control_word;
  state->status_word = status_word;
  return 0;
}

LIBC_INLINE int set_env(const fenv_t *envp) {
  const internal::FPState *state =
      reinterpret_cast<const internal::FPState *>(envp);

  uint32_t mxcsr = 0;

  // Set exception flags from the status word
  mxcsr |= static_cast<uint16_t>(
      (state->status_word &
       (WinExceptionFlags::HIGH_DENORMAL | WinExceptionFlags::HIGH_INVALID)) >>
      28);
  mxcsr |= static_cast<uint16_t>(
      (state->status_word & WinExceptionFlags::HIGH_DIV_BY_ZERO) >> 25);
  mxcsr |= static_cast<uint16_t>(
      (state->status_word & WinExceptionFlags::HIGH_OVERFLOW) >> 23);
  mxcsr |= static_cast<uint16_t>(
      (state->status_word & WinExceptionFlags::HIGH_UNDERFLOW) >> 21);
  mxcsr |= static_cast<uint16_t>(
      (state->status_word & WinExceptionFlags::HIGH_INEXACT) >> 19);

  // Set denormals-are-zero from bit 10 xor bit 11
  mxcsr |= static_cast<uint16_t>(
      (((state->control_word & 0x800) >> 1) ^ (state->control_word & 0x400)) >>
      4);

  // Set exception masks from bits 24-29
  mxcsr |= static_cast<uint16_t>(
      (state->control_word &
       (WinExceptionFlags::HIGH_DENORMAL | WinExceptionFlags::HIGH_INVALID)) >>
      21);
  mxcsr |= static_cast<uint16_t>(
      (state->control_word & WinExceptionFlags::HIGH_DIV_BY_ZERO) >> 18);
  mxcsr |= static_cast<uint16_t>(
      (state->control_word & WinExceptionFlags::HIGH_OVERFLOW) >> 16);
  mxcsr |= static_cast<uint16_t>(
      (state->control_word & WinExceptionFlags::HIGH_UNDERFLOW) >> 14);
  mxcsr |= static_cast<uint16_t>(
      (state->control_word & WinExceptionFlags::HIGH_INEXACT) >> 12);

  // Set rounding from bits 30-31
  mxcsr |= static_cast<uint16_t>((state->control_word & 0xc0000000) >> 17);

  // Set flush-to-zero from bit 10
  mxcsr |= static_cast<uint16_t>((state->control_word & 0x400) << 5);

  internal::write_mxcsr(mxcsr);
  return 0;
}
#else
LIBC_INLINE int get_env(fenv_t *envp) {
  internal::FPState *state = reinterpret_cast<internal::FPState *>(envp);
#ifdef __APPLE__
  internal::X87StateDescriptor x87_status;
  internal::get_x87_state_descriptor(x87_status);
  state->control_word = x87_status.control_word;
  state->status_word = x87_status.status_word;
#else
  internal::get_x87_state_descriptor(state->x87_status);
#endif // __APPLE__
  state->mxcsr = internal::get_mxcsr();
  return 0;
}

LIBC_INLINE int set_env(const fenv_t *envp) {
  // envp contains everything including pieces like the current
  // top of FPU stack. We cannot arbitrarily change them. So, we first
  // read the current status and update only those pieces which are
  // not disruptive.
  internal::X87StateDescriptor x87_status;
  internal::get_x87_state_descriptor(x87_status);

  if (envp == FE_DFL_ENV) {
    // Reset the exception flags in the status word.
    x87_status.status_word &= ~uint16_t(0x3F);
    // Reset other non-sensitive parts of the status word.
    for (int i = 0; i < 5; i++)
      x87_status._[i] = 0;
    // In the control word, we do the following:
    // 1. Mask all exceptions
    // 2. Set rounding mode to round-to-nearest
    // 3. Set the internal precision to double extended precision.
    x87_status.control_word |= uint16_t(0x3F);         // Mask all exceptions.
    x87_status.control_word &= ~(uint16_t(0x3) << 10); // Round to nearest.
    x87_status.control_word |= (uint16_t(0x3) << 8);   // Extended precision.
    internal::write_x87_state_descriptor(x87_status);

    // We take the exact same approach MXCSR register as well.
    // MXCSR has two additional fields, "flush-to-zero" and
    // "denormals-are-zero". We reset those bits. Also, MXCSR does not
    // have a field which controls the precision of internal operations.
    uint32_t mxcsr = internal::get_mxcsr();
    mxcsr &= ~uint16_t(0x3F);        // Clear exception flags.
    mxcsr &= ~(uint16_t(0x1) << 6);  // Reset denormals-are-zero
    mxcsr |= (uint16_t(0x3F) << 7);  // Mask exceptions
    mxcsr &= ~(uint16_t(0x3) << 13); // Round to nearest.
    mxcsr &= ~(uint16_t(0x1) << 15); // Reset flush-to-zero
    internal::write_mxcsr(mxcsr);

    return 0;
  }

  const internal::FPState *fpstate =
      reinterpret_cast<const internal::FPState *>(envp);

  // Copy the exception status flags from envp.
  x87_status.status_word &= ~uint16_t(0x3F);
#ifdef __APPLE__
  x87_status.status_word |= (fpstate->status_word & 0x3F);
  // We can set the x87 control word as is as there no sensitive bits.
  x87_status.control_word = fpstate->control_word;
#else
  x87_status.status_word |= (fpstate->x87_status.status_word & 0x3F);
  // Copy other non-sensitive parts of the status word.
  for (int i = 0; i < 5; i++)
    x87_status._[i] = fpstate->x87_status._[i];
  // We can set the x87 control word as is as there no sensitive bits.
  x87_status.control_word = fpstate->x87_status.control_word;
#endif // __APPLE__
  internal::write_x87_state_descriptor(x87_status);

  // We can write the MXCSR state as is as there are no sensitive bits.
  internal::write_mxcsr(fpstate->mxcsr);
  return 0;
}
#endif

} // namespace fputil
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC___SUPPORT_FPUTIL_X86_64_FENVIMPL_H
