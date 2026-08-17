// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STACK_CANARY_LINUX_H_
#define BASE_STACK_CANARY_LINUX_H_

#include "base/base_export.h"

namespace base {

// This resets the reference stack canary to a new random value, which is
// useful when forking so multiple processes don't have the same canary (which
// makes it easy to brute force). All functions called from here on out will
// use the new stack canary. However, functions that are on the call stack at
// the time of calling this function are now unsafe to return from unless they
// have the no_stack_protector attribute.
//
// On ARM we require the process to be single-threaded, as this function needs
// to edit a read-only page containing the canary.
void BASE_EXPORT ResetStackCanaryIfPossible();

// After this is called, any canary mismatch is considered to be due to a
// change in the reference canary (see ResetStackCanaryIfPossible()) rather
// than a stack corruption. Instead of immediately crashing, emit a useful
// debug message that explains how to avoid the crash.
// Has no effect is non-debug builds.
void BASE_EXPORT SetStackSmashingEmitsDebugMessage();

}  // namespace base

#endif  // BASE_STACK_CANARY_LINUX_H_
