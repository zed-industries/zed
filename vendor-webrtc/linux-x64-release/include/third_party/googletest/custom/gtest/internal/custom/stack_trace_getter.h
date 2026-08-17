// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_STACK_TRACE_GETTER_H_
#define THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_STACK_TRACE_GETTER_H_

#include <optional>

#include "base/debug/stack_trace.h"
#include "third_party/googletest/src/googletest/src/gtest-internal-inl.h"

// An implementation of Google Test's OsStackTraceGetterInterface that uses
// Chromium's base::debug::StackTrace to obtain stringified stack traces.
class StackTraceGetter
    : public ::testing::internal::OsStackTraceGetterInterface {
 public:
  StackTraceGetter() = default;
  ~StackTraceGetter() override = default;
  StackTraceGetter(const StackTraceGetter&) = delete;
  StackTraceGetter& operator=(const StackTraceGetter&) = delete;

  // ::testing::internal::OsStackTraceGetterInterface:
  std::string CurrentStackTrace(int max_depth, int skip_count) override;
  void UponLeavingGTest() override;

 private:
  std::optional<base::debug::StackTrace> stack_trace_upon_leaving_gtest_;
};

#endif  // THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_STACK_TRACE_GETTER_H_
