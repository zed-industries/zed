//===-- MPFRUtils.h ---------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_MPFRWRAPPER_MPFR_INC_H
#define LLVM_LIBC_UTILS_MPFRWRAPPER_MPFR_INC_H

#ifdef CUSTOM_MPFR_INCLUDER
// Some downstream repos are monoliths carrying MPFR sources in their third
// party directory. In such repos, including the MPFR header as
// `#include <mpfr.h>` is either disallowed or not possible. If that is the
// case, a file named `CustomMPFRIncluder.h` should be added through which the
// MPFR header can be included in manner allowed in that repo.
#include "CustomMPFRIncluder.h"
#else
#include <mpfr.h>
#endif

#endif // LLVM_LIBC_UTILS_MPFRWRAPPER_MPFR_INC_H
