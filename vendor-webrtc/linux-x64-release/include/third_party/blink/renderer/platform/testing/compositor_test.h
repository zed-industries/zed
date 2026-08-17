// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_COMPOSITOR_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_COMPOSITOR_TEST_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/test/test_mock_time_task_runner.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace blink {

class CompositorTest : public testing::Test {
 public:
  CompositorTest();
  CompositorTest(const CompositorTest&) = delete;
  CompositorTest& operator=(const CompositorTest&) = delete;
  ~CompositorTest() override;

 protected:
  // cc::LayerTreeHost requires a task runner, so we use a mock task runner and
  // bind it as the current SingleThreadTaskRunner::CurrentDefaultHandle for
  // this thread.
  scoped_refptr<base::TestMockTimeTaskRunner> runner_;
  base::SingleThreadTaskRunner::CurrentDefaultHandle
      runner_current_default_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_COMPOSITOR_TEST_H_
