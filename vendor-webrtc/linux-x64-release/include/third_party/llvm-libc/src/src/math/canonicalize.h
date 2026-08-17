//===-- Implementation header for canonicalize -------------------*- C++-*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_MATH_CANONICALIZE_H
#define LLVM_LIBC_SRC_MATH_CANONICALIZE_H

#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

int canonicalize(double *cx, const double *x);

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_MATH_CANONICALIZE_H
