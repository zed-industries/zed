//===-- sanitizer_win_interception.h ----------------------    --*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Windows-specific export surface to provide interception for parts of the
// runtime that are always statically linked, both for overriding user-defined
// functions as well as registering weak functions that the ASAN runtime should
// use over defaults.
//
//===----------------------------------------------------------------------===//

#ifndef SANITIZER_WIN_INTERCEPTION_H
#define SANITIZER_WIN_INTERCEPTION_H

#include "sanitizer_platform.h"
#if SANITIZER_WINDOWS

#  include "sanitizer_common.h"
#  include "sanitizer_internal_defs.h"

namespace __sanitizer {
using RegisterWeakFunctionCallback = void (*)();
void AddRegisterWeakFunctionCallback(uptr export_address,
                                     RegisterWeakFunctionCallback cb);
}  // namespace __sanitizer

#endif  // SANITIZER_WINDOWS
#endif  // SANITIZER_WIN_INTERCEPTION_H