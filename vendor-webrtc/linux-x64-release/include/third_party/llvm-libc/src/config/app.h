//===-- Classes to capture properites of applications -----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_CONFIG_APP_H
#define LLVM_LIBC_CONFIG_APP_H

#include "src/__support/macros/properties/architectures.h"

#if defined(LIBC_TARGET_ARCH_IS_GPU)
#include "gpu/app.h"
#elif defined(__linux__)
#include "linux/app.h"
#endif

#endif // LLVM_LIBC_CONFIG_APP_H
