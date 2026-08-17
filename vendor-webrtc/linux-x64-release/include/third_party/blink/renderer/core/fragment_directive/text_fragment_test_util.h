// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_TEST_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_TEST_UTIL_H_

#include "third_party/blink/renderer/core/fragment_directive/text_fragment_anchor.h"
#include "third_party/blink/renderer/core/testing/sim/sim_test.h"
#include "third_party/blink/renderer/platform/testing/task_environment.h"

namespace blink {

class TextFragmentAnchorTestBase : public SimTest {
 public:
  explicit TextFragmentAnchorTestBase(
      base::test::TaskEnvironment::TimeSource time_source);
  TextFragmentAnchorTestBase();
  // SimTest overrides
  void SetUp() override;
  void TearDown() override;

  void RunAsyncMatchingTasks();
  void RunUntilTextFragmentFinalization();

  // Helps ensure that tests disable the virtual time controller during teardown
  // or to test real time behavior from tests. This only works if the
  // VirtualTimeController::EnableVirtualTime() has been called, otherwise this
  // function no-ops.
  void DisableVirtualTimeIfSet();

 private:
  // This is mostly used to run tests in blink_unittests which doesn't yet
  // support blink::test::TaskEnvironment, and also provides a way for tests
  // verifying real time behavior in blink_unittests_v2 to turn off virtual
  // time.
  bool enable_virtual_time_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_TEXT_FRAGMENT_TEST_UTIL_H_
