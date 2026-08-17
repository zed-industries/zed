// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Part of generating an artificial Rust crash for testing purposes.
// We call through this C++ function to ensure we can cope with mixed
// language stacks.

#ifndef THIRD_PARTY_BLINK_COMMON_RUST_CRASH_RUST_CRASH_H_
#define THIRD_PARTY_BLINK_COMMON_RUST_CRASH_RUST_CRASH_H_

namespace blink {

// Called from Rust, calls back into Rust then crashes. See src/lib.rs for the
// sequence.
__attribute__((noinline)) void EnterCppForRustCrash();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_COMMON_RUST_CRASH_RUST_CRASH_H_
