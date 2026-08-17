// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file provides the RegisterContext cross-platform typedef that represents
// the native register context for the platform.

#ifndef BASE_PROFILER_REGISTER_CONTEXT_H_
#define BASE_PROFILER_REGISTER_CONTEXT_H_

#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
typedef struct _CONTEXT CONTEXT;
#elif BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
#include <sys/ucontext.h>
#elif BUILDFLAG(IS_APPLE) && \
    (defined(ARCH_CPU_X86_64) || defined(ARCH_CPU_ARM64))
#include <mach/machine/thread_status.h>
#else
#include <stdint.h>
#endif

namespace base {

#if BUILDFLAG(IS_WIN)

using RegisterContext = ::CONTEXT;

#elif BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

using RegisterContext = mcontext_t;

#elif BUILDFLAG(IS_APPLE) && defined(ARCH_CPU_X86_64)

using RegisterContext = x86_thread_state64_t;

#elif BUILDFLAG(IS_APPLE) && defined(ARCH_CPU_ARM64)

using RegisterContext = arm_thread_state64_t;

#else

// Placeholders for other cases.
struct RegisterContext {
  uintptr_t stack_pointer;
  uintptr_t frame_pointer;
  uintptr_t instruction_pointer;
};

#endif

}  // namespace base

#endif  // BASE_PROFILER_REGISTER_CONTEXT_H_
