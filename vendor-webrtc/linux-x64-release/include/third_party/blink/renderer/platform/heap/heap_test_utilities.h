// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_UTILITIES_H_

#include "base/test/task_environment.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/platform/heap/thread_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/cppgc/testing.h"
#include "v8/include/v8.h"
#include "v8/include/v8-cppgc.h"

namespace blink {

// Allows for overriding the stack state for the purpose of testing. Any garbage
// collection calls scoped with `HeapPointersOnStackScope` will perform
// conservative stack scanning, even if other (more local) hints indicate that
// there's no need for it.
class HeapPointersOnStackScope final {
  STACK_ALLOCATED();

 public:
  explicit HeapPointersOnStackScope(const ThreadState* state)
      : embedder_stack_state_(
            state->cpp_heap().GetHeapHandle(),
            cppgc::EmbedderStackState::kMayContainHeapPointers) {}

  HeapPointersOnStackScope(const HeapPointersOnStackScope&) = delete;
  HeapPointersOnStackScope& operator=(const HeapPointersOnStackScope&) = delete;

 private:
  cppgc::testing::OverrideEmbedderStackStateScope embedder_stack_state_;
};

class TestSupportingGC : public testing::Test {
 public:
  ~TestSupportingGC() override;

  // Performs a precise garbage collection with eager sweeping.
  static void PreciselyCollectGarbage();

  // Performs a conservative garbage collection with eager sweeping.
  static void ConservativelyCollectGarbage();

  // Performs multiple rounds of garbage collections until no more memory can be
  // freed. This is useful to avoid other garbage collections having to deal
  // with stale memory.
  static void ClearOutOldGarbage();

 protected:
  base::test::TaskEnvironment task_environment_;
};

// Test driver for compaction.
class CompactionTestDriver {
 public:
  explicit CompactionTestDriver(ThreadState*);

  void ForceCompactionForNextGC();

 protected:
  cppgc::testing::StandaloneTestingHeap heap_;
};

// Test driver for incremental marking. Assumes that no stack handling is
// required.
class IncrementalMarkingTestDriver {
 public:
  explicit IncrementalMarkingTestDriver(ThreadState*);
  ~IncrementalMarkingTestDriver();

  virtual void StartGC();
  virtual void TriggerMarkingSteps();
  virtual void FinishGC();

  void TriggerMarkingStepsWithStack();

 protected:
  cppgc::testing::StandaloneTestingHeap heap_;
};

// Test driver for concurrent marking. Assumes that no stack handling is
// required.
class ConcurrentMarkingTestDriver : public IncrementalMarkingTestDriver {
 public:
  explicit ConcurrentMarkingTestDriver(ThreadState*);

  void StartGC() override;
  void TriggerMarkingSteps() override;
  void FinishGC() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TEST_UTILITIES_H_
