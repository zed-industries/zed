// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_CRASH_HELPERS_H_
#define THIRD_PARTY_BLINK_COMMON_CRASH_HELPERS_H_

#include "base/compiler_specific.h"

namespace blink {

namespace internal {

// These functions are defined in a separate file from the calling
// file to try to prevent the compiler from inlining them. Some tests
// which verify crashpad, and stack symbolization from minidumps,
// expect to find the names of these functions.

NOINLINE void CrashIntentionally();
NOINLINE void BadCastCrashIntentionally();

}  // namespace internal

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_COMMON_CRASH_HELPERS_H_
