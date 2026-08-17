//===--- rtsan_diagnostics.h - Realtime Sanitizer ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Part of the RealtimeSanitizer runtime library
//
//===----------------------------------------------------------------------===//

#pragma once

#include "sanitizer_common/sanitizer_common.h"
#include "sanitizer_common/sanitizer_internal_defs.h"

namespace __rtsan {

enum class DiagnosticsInfoType {
  InterceptedCall,
  BlockingCall,
};

struct DiagnosticsInfo {
  DiagnosticsInfoType type;
  const char *func_name;
  __sanitizer::uptr pc;
  __sanitizer::uptr bp;
};

void PrintDiagnostics(const DiagnosticsInfo &info);
void PrintErrorSummary(const DiagnosticsInfo &info,
                       const __sanitizer::BufferedStackTrace &stack);
} // namespace __rtsan
