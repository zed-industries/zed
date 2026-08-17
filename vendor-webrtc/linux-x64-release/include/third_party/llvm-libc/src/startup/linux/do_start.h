//===-- Header file of do_start -------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "config/app.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {
// setup the libc runtime and invoke the main routine.
[[noreturn]] void do_start();
} // namespace LIBC_NAMESPACE_DECL
