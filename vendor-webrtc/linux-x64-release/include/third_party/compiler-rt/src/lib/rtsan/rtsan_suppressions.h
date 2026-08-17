//===--- rtsan_suppressions.h - Realtime Sanitizer --------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of the RTSan runtime, providing support for suppressions
//
//===----------------------------------------------------------------------===//

#pragma once

#include "sanitizer_common/sanitizer_stacktrace.h"

namespace __rtsan {

void InitializeSuppressions();
bool IsStackTraceSuppressed(const __sanitizer::StackTrace &stack);
bool IsFunctionSuppressed(const char *function_name);

} // namespace __rtsan
