//===-- Implementation header for getauxval function ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_SYS_AUXV_GETAUXVAL_H
#define LLVM_LIBC_SRC_SYS_AUXV_GETAUXVAL_H

#include "hdr/sys_auxv_macros.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

unsigned long getauxval(unsigned long id);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_SYS_AUXV_GETAUXVAL_H
