//===--- rtsan_stats.h - Realtime Sanitizer ---------------------*- C++ -*-===//
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

namespace __rtsan {

void IncrementTotalErrorCount();
void IncrementUniqueErrorCount();
void IncrementSuppressedCount();

void PrintStatisticsSummary();

} // namespace __rtsan
