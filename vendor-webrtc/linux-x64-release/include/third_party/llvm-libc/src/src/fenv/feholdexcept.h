//===-- Implementation header for feholdexcept ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_FENV_FEHOLDEXCEPT_H
#define LLVM_LIBC_SRC_FENV_FEHOLDEXCEPT_H

#include "hdr/types/fenv_t.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int feholdexcept(fenv_t *);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_FENV_FEHOLDEXCEPT_H
