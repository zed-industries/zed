// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_BLINK_PERF_TEST_SUITE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_BLINK_PERF_TEST_SUITE_H_

#include "base/test/test_suite.h"
#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"

namespace blink {

class BlinkPerfTestSuite : public base::TestSuite {
  STACK_ALLOCATED();

 public:
  BlinkPerfTestSuite(int argc, char** argv);

  void Initialize() override;
  void Shutdown() override;

 private:
  ScopedUnittestsEnvironmentSetup env;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_BLINK_PERF_TEST_SUITE_H_
