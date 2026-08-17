//===-- Implementation header of difftime -----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_TIME_DIFFTIME_H
#define LLVM_LIBC_SRC_TIME_DIFFTIME_H

#include "hdr/types/time_t.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

double difftime(time_t end, time_t beginning);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_TIME_DIFFTIME_H
