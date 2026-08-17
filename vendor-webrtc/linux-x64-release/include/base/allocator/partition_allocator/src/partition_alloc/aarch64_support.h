// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_AARCH64_SUPPORT_H_
#define PARTITION_ALLOC_AARCH64_SUPPORT_H_

#include <stdint.h>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(IS_ANDROID) || PA_BUILDFLAG(IS_LINUX)
#define HAS_HW_CAPS
#endif

#if PA_BUILDFLAG(PA_ARCH_CPU_ARM64) && defined(HAS_HW_CAPS)
#include <asm/hwcap.h>
#include <sys/ifunc.h>
#else
struct __ifunc_arg_t;
#endif

namespace partition_alloc::internal {

constexpr bool IsBtiEnabled(uint64_t ifunc_hwcap,
                            struct __ifunc_arg_t* ifunc_hw) {
#if PA_BUILDFLAG(PA_ARCH_CPU_ARM64) && defined(HAS_HW_CAPS) && \
    !PA_BUILDFLAG(IS_HWASAN)
  return (ifunc_hwcap & _IFUNC_ARG_HWCAP) && (ifunc_hw->_hwcap2 & HWCAP2_BTI);
#else
  return false;
#endif
}

constexpr bool IsMteEnabled(uint64_t ifunc_hwcap,
                            struct __ifunc_arg_t* ifunc_hw) {
#if PA_BUILDFLAG(PA_ARCH_CPU_ARM64) && defined(HAS_HW_CAPS) && \
    PA_BUILDFLAG(HAS_MEMORY_TAGGING)
  return (ifunc_hwcap & _IFUNC_ARG_HWCAP) && (ifunc_hw->_hwcap2 & HWCAP2_MTE);
#else
  return false;
#endif
}

}  // namespace partition_alloc::internal

#undef HAS_HW_CAPS

#endif  // PARTITION_ALLOC_AARCH64_SUPPORT_H_
