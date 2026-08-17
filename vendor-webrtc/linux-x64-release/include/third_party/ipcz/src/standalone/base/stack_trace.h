// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_STANDALONE_BASE_STACK_TRACE_H_
#define IPCZ_SRC_STANDALONE_BASE_STACK_TRACE_H_

#include <string>

#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

namespace ipcz::standalone {

// Minimal alternative to Chromium's base::debug::StackTrace, for use when not
// linking against other Chromium sources. This uses Abseil's stack tracing and
// symbolization facilities instead, but presents an API similar to Chromium's
// StackTrace.
class StackTrace {
 public:
  static constexpr size_t kDefaultFrameCount = 16;

  explicit StackTrace(size_t frame_count = kDefaultFrameCount);
  StackTrace(const StackTrace&);
  StackTrace& operator=(const StackTrace&);
  ~StackTrace();

  static void EnableStackTraceSymbolization(const char* argv0);

  std::string ToString() const;

 private:
  absl::InlinedVector<void*, kDefaultFrameCount> frames_;
};

}  // namespace ipcz::standalone

#endif  // IPCZ_SRC_STANDALONE_BASE_STACK_TRACE_H_
