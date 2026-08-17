//===-- Implementation header for stdc_has_single_bit_uc --------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDBIT_STDC_HAS_SINGLE_BIT_UC_H
#define LLVM_LIBC_SRC_STDBIT_STDC_HAS_SINGLE_BIT_UC_H

#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

bool stdc_has_single_bit_uc(unsigned char value);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDBIT_STDC_HAS_SINGLE_BIT_UC_H
