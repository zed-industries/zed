//===-- memprof_flags.h ---------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is a part of MemProfiler, a memory profiler.
//
// MemProf runtime flags.
//===----------------------------------------------------------------------===//

#ifndef MEMPROF_FLAGS_H
#define MEMPROF_FLAGS_H

#include "sanitizer_common/sanitizer_flag_parser.h"
#include "sanitizer_common/sanitizer_internal_defs.h"

// Default MemProf flags are defined in memprof_flags.inc and sancov_flags.inc.
// These values can be overridden in a number of ways, each option overrides the
// prior one:
//  1) by setting MEMPROF_DEFAULT_OPTIONS during the compilation of the MemProf
//     runtime
//  2) by setting the LLVM flag -memprof-runtime-default-options during the
//     compilation of your binary
//  3) by overriding the user-specified function __memprof_default_options()
//  4) by setting the environment variable MEMPROF_OPTIONS during runtime

namespace __memprof {

struct Flags {
#define MEMPROF_FLAG(Type, Name, DefaultValue, Description) Type Name;
#include "memprof_flags.inc"
#undef MEMPROF_FLAG

  void SetDefaults();
};

extern Flags memprof_flags_dont_use_directly;
inline Flags *flags() { return &memprof_flags_dont_use_directly; }

void InitializeFlags();

} // namespace __memprof

#endif // MEMPROF_FLAGS_H
