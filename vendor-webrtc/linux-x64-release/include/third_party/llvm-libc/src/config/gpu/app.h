//===-- Classes to capture properites of GPU applications -------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_CONFIG_GPU_APP_H
#define LLVM_LIBC_CONFIG_GPU_APP_H

#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {

// TODO: Move other global values here and export them to the host.
struct DataEnvironment {
  uintptr_t *env_ptr;
};

extern DataEnvironment app;

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_CONFIG_GPU_APP_H
