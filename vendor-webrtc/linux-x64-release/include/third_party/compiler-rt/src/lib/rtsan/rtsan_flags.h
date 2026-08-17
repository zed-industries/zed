//===----------------------- rtsan_flags.h ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of RealtimeSanitizer.
//
//===----------------------------------------------------------------------===//
#pragma once

namespace __rtsan {

struct Flags {
#define RTSAN_FLAG(Type, Name, DefaultValue, Description)                      \
  Type Name{DefaultValue};
#include "rtsan_flags.inc"
#undef RTSAN_FLAG

  bool ContainsSuppresionFile() { return suppressions[0] != '\0'; }
};

extern Flags flags_data;
inline Flags &flags() { return flags_data; }

void InitializeFlags();

} // namespace __rtsan
