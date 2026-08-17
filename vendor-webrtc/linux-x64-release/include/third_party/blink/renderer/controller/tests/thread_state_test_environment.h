// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CONTROLLER_TESTS_THREAD_STATE_TEST_ENVIRONMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CONTROLLER_TESTS_THREAD_STATE_TEST_ENVIRONMENT_H_

#include <optional>

#include "base/run_loop.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/scheduler/test/renderer_scheduler_test_support.h"
#include "third_party/blink/renderer/platform/heap/heap_test_utilities.h"
#include "third_party/blink/renderer/platform/heap/thread_state.h"

class ThreadStateTestEnvironment : public ::testing::Environment {
 public:
  ThreadStateTestEnvironment() = default;

  ThreadStateTestEnvironment(const ThreadStateTestEnvironment&) = delete;
  ThreadStateTestEnvironment& operator=(const ThreadStateTestEnvironment&) =
      delete;

  void SetUp() override {
    conservative_gc_scope_.emplace(blink::ThreadState::Current());
  }
  void TearDown() override {
    // Collect garbage (including threadspecific persistent handles) in order
    // to release mock objects referred from v8 or Oilpan heap. Otherwise false
    // mock leaks will be reported.
    blink::ThreadState::Current()->CollectAllGarbageForTesting();
  }

 private:
  STACK_ALLOCATED_IGNORE("https://crbug.com/1409156")
  std::optional<blink::HeapPointersOnStackScope> conservative_gc_scope_;
};

#endif  // THIRD_PARTY_BLINK_RENDERER_CONTROLLER_TESTS_THREAD_STATE_TEST_ENVIRONMENT_H_
