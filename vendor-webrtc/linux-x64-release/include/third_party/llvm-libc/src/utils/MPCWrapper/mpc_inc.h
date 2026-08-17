//===-- MPCUtils.h ----------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_MPCWRAPPER_MPC_INC_H
#define LLVM_LIBC_UTILS_MPCWRAPPER_MPC_INC_H

#ifdef CUSTOM_MPC_INCLUDER
// Some downstream repos are monoliths carrying MPC sources in their third
// party directory. In such repos, including the MPC header as
// `#include <mpc.h>` is either disallowed or not possible. If that is the
// case, a file named `CustomMPCIncluder.h` should be added through which the
// MPC header can be included in manner allowed in that repo.
#include "CustomMPCIncluder.h"
#else
#include <mpc.h>
#endif

#endif // LLVM_LIBC_UTILS_MPCWRAPPER_MPC_INC_H
