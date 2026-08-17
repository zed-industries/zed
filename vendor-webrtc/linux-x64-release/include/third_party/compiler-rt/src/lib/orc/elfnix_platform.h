//===- elfnix_platform.h ----------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// ORC Runtime support for dynamic loading features on ELF-based platforms.
//
//===----------------------------------------------------------------------===//

#ifndef ORC_RT_ELFNIX_PLATFORM_H
#define ORC_RT_ELFNIX_PLATFORM_H

#include "common.h"
#include "executor_address.h"

// Atexit functions.
ORC_RT_INTERFACE int __orc_rt_elfnix_cxa_atexit(void (*func)(void *), void *arg,
                                                void *dso_handle);
ORC_RT_INTERFACE int __orc_rt_elfnix_atexit(void (*func)(void *));
ORC_RT_INTERFACE void __orc_rt_elfnix_cxa_finalize(void *dso_handle);

// dlfcn functions.
ORC_RT_INTERFACE const char *__orc_rt_elfnix_jit_dlerror();
ORC_RT_INTERFACE void *__orc_rt_elfnix_jit_dlopen(const char *path, int mode);
ORC_RT_INTERFACE int __orc_rt_elfnix_jit_dlupdate(void *dso_handle);
ORC_RT_INTERFACE int __orc_rt_elfnix_jit_dlclose(void *dso_handle);
ORC_RT_INTERFACE void *__orc_rt_elfnix_jit_dlsym(void *dso_handle,
                                                 const char *symbol);

namespace orc_rt {
namespace elfnix {

struct ELFNixPerObjectSectionsToRegister {
  ExecutorAddrRange EHFrameSection;
  ExecutorAddrRange ThreadDataSection;
};

using ELFNixJITDylibDepInfo = std::vector<ExecutorAddr>;

using ELFNixJITDylibDepInfoMap =
    std::unordered_map<ExecutorAddr, ELFNixJITDylibDepInfo>;

enum dlopen_mode : int {
  ORC_RT_RTLD_LAZY = 0x1,
  ORC_RT_RTLD_NOW = 0x2,
  ORC_RT_RTLD_LOCAL = 0x4,
  ORC_RT_RTLD_GLOBAL = 0x8
};

} // namespace elfnix

using SPSELFNixPerObjectSectionsToRegister =
    SPSTuple<SPSExecutorAddrRange, SPSExecutorAddrRange>;

template <>
class SPSSerializationTraits<SPSELFNixPerObjectSectionsToRegister,
                             elfnix::ELFNixPerObjectSectionsToRegister> {

public:
  static size_t size(const elfnix::ELFNixPerObjectSectionsToRegister &MOPOSR) {
    return SPSELFNixPerObjectSectionsToRegister::AsArgList::size(
        MOPOSR.EHFrameSection, MOPOSR.ThreadDataSection);
  }

  static bool
  serialize(SPSOutputBuffer &OB,
            const elfnix::ELFNixPerObjectSectionsToRegister &MOPOSR) {
    return SPSELFNixPerObjectSectionsToRegister::AsArgList::serialize(
        OB, MOPOSR.EHFrameSection, MOPOSR.ThreadDataSection);
  }

  static bool deserialize(SPSInputBuffer &IB,
                          elfnix::ELFNixPerObjectSectionsToRegister &MOPOSR) {
    return SPSELFNixPerObjectSectionsToRegister::AsArgList::deserialize(
        IB, MOPOSR.EHFrameSection, MOPOSR.ThreadDataSection);
  }
};

using SPSELFNixJITDylibDepInfo = SPSSequence<SPSExecutorAddr>;
using SPSELFNixJITDylibDepInfoMap =
    SPSSequence<SPSTuple<SPSExecutorAddr, SPSELFNixJITDylibDepInfo>>;

} // namespace orc_rt

#endif // ORC_RT_ELFNIX_PLATFORM_H
