// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

// This file provides functions that provide access to key registers in the
// native register context.

#ifndef BASE_PROFILER_REGISTER_CONTEXT_REGISTERS_H_
#define BASE_PROFILER_REGISTER_CONTEXT_REGISTERS_H_

#include <stdint.h>

#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>
#elif BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
#include <sys/ucontext.h>
#elif BUILDFLAG(IS_APPLE)
#include <mach/machine/thread_status.h>
#else
#include "base/profiler/register_context.h"
#endif

namespace base {

// Helper function to account for the fact that platform-specific register state
// types may be of the same size as uintptr_t, but not of the same type or
// signedness -- e.g. unsigned int vs. unsigned long on 32-bit Windows, unsigned
// long vs. unsigned long long on Mac, long long vs. unsigned long long on
// Linux.
template <typename T>
uintptr_t& AsUintPtr(T* value) {
  static_assert(sizeof(T) == sizeof(uintptr_t),
                "register state type must be of equivalent size to uintptr_t");
  return *reinterpret_cast<uintptr_t*>(value);
}

#if BUILDFLAG(IS_WIN)

inline uintptr_t& RegisterContextStackPointer(::CONTEXT* context) {
#if defined(ARCH_CPU_X86_64)
  return context->Rsp;
#elif defined(ARCH_CPU_ARM64)
  return context->Sp;
#else
  return AsUintPtr(&context->Esp);
#endif
}

inline uintptr_t& RegisterContextFramePointer(::CONTEXT* context) {
#if defined(ARCH_CPU_X86_64)
  return context->Rbp;
#elif defined(ARCH_CPU_ARM64)
  return context->Fp;
#else
  return AsUintPtr(&context->Ebp);
#endif
}

inline uintptr_t& RegisterContextInstructionPointer(::CONTEXT* context) {
#if defined(ARCH_CPU_X86_64)
  return context->Rip;
#elif defined(ARCH_CPU_ARM64)
  return context->Pc;
#else
  return AsUintPtr(&context->Eip);
#endif
}

#elif BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

inline uintptr_t& RegisterContextStackPointer(mcontext_t* context) {
#if defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->arm_sp);
#elif defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_64_BITS)
  return AsUintPtr(&context->sp);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->gregs[REG_ESP]);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_64_BITS)
  return AsUintPtr(&context->gregs[REG_RSP]);
#else
  // The implementations here and below are placeholders for other POSIX
  // platforms that just return the first three register slots in the context.
  return *reinterpret_cast<uintptr_t*>(context);
#endif
}

inline uintptr_t& RegisterContextFramePointer(mcontext_t* context) {
#if defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->arm_fp);
#elif defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_64_BITS)
  // r29 is the FP register on 64-bit ARM per the Procedure Call Standard,
  // section 5.1.1.
  return AsUintPtr(&context->regs[29]);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->gregs[REG_EBP]);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_64_BITS)
  return AsUintPtr(&context->gregs[REG_RBP]);
#else
  return *(reinterpret_cast<uintptr_t*>(context) + 1);
#endif
}

inline uintptr_t& RegisterContextInstructionPointer(mcontext_t* context) {
#if defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->arm_pc);
#elif defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_64_BITS)
  return AsUintPtr(&context->pc);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_32_BITS)
  return AsUintPtr(&context->gregs[REG_EIP]);
#elif defined(ARCH_CPU_X86_FAMILY) && defined(ARCH_CPU_64_BITS)
  return AsUintPtr(&context->gregs[REG_RIP]);
#else
  return *(reinterpret_cast<uintptr_t*>(context) + 2);
#endif
}

#elif BUILDFLAG(IS_APPLE) && defined(ARCH_CPU_X86_64)

inline uintptr_t& RegisterContextStackPointer(x86_thread_state64_t* context) {
  return AsUintPtr(&context->__rsp);
}

inline uintptr_t& RegisterContextFramePointer(x86_thread_state64_t* context) {
  return AsUintPtr(&context->__rbp);
}

inline uintptr_t& RegisterContextInstructionPointer(
    x86_thread_state64_t* context) {
  return AsUintPtr(&context->__rip);
}

#elif BUILDFLAG(IS_APPLE) && defined(ARCH_CPU_ARM64)

// TODO(thakis): Have getter/setter functions instead of returning a ref to
// prepare for arm64e. See __DARWIN_OPAQUE_ARM_THREAD_STATE6 in
// mach/arm/_structs.h
inline uintptr_t& RegisterContextStackPointer(arm_thread_state64_t* context) {
  return AsUintPtr(&context->__sp);
}

inline uintptr_t& RegisterContextFramePointer(arm_thread_state64_t* context) {
  return AsUintPtr(&context->__fp);
}

inline uintptr_t& RegisterContextInstructionPointer(
    arm_thread_state64_t* context) {
  return AsUintPtr(&context->__pc);
}

#else

// Placeholders for other cases.
inline uintptr_t& RegisterContextStackPointer(RegisterContext* context) {
  return context->stack_pointer;
}

inline uintptr_t& RegisterContextFramePointer(RegisterContext* context) {
  return context->frame_pointer;
}

inline uintptr_t& RegisterContextInstructionPointer(RegisterContext* context) {
  return context->instruction_pointer;
}

#endif

}  // namespace base

#endif  // BASE_PROFILER_REGISTER_CONTEXT_REGISTERS_H_
