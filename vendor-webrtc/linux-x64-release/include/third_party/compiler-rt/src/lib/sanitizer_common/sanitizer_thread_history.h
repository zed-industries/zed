//===-- sanitizer_thread_history.h ------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Utility to print thread histroy from ThreadRegistry.
//
//===----------------------------------------------------------------------===//

#ifndef SANITIZER_THREAD_HISTORY_H
#define SANITIZER_THREAD_HISTORY_H

#include "sanitizer_thread_registry.h"

namespace __sanitizer {

void PrintThreadHistory(ThreadRegistry& registry, InternalScopedString& out);

}  // namespace __sanitizer

#endif  // SANITIZER_THREAD_HISTORY_H
