// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TASK_ENVIRONMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TASK_ENVIRONMENT_H_

#include <optional>

#include "base/test/task_environment.h"
#include "third_party/blink/renderer/platform/scheduler/common/task_priority.h"
#include "third_party/blink/renderer/platform/testing/main_thread_isolate.h"
#include "third_party/blink/renderer/platform/testing/scoped_main_thread_overrider.h"
#include "v8/include/v8-forward.h"

namespace blink::scheduler {
class MainThreadSchedulerImpl;
}

namespace blink::test {

// TaskEnvironment is a convenience class which allows usage of these
// APIs within its scope:
// - Same APIs as base::test::TaskEnvironment.
// - Blink Main Thread isolate.
// - blink::scheduler::WebThreadScheduler.
//
// Only tests that need blink APIs should instantiate a
// blink::test::TaskEnvironment. Use base::test::SingleThreadTaskEnvironment or
// base::test::TaskEnvironment otherwise.
//
// Tests that render <video> may also need CSSDefaultStyleSheets::TestingScope
// (see comments on that class).
class TaskEnvironment : public base::test::TaskEnvironment {
 public:
  using ValidTraits = base::test::TaskEnvironment::ValidTraits;

  template <typename... Traits>
    requires base::trait_helpers::AreValidTraits<ValidTraits, Traits...>
  explicit TaskEnvironment(Traits... traits)
      : TaskEnvironment(CreateTaskEnvironmentWithPriorities(
            blink::scheduler::CreatePrioritySettings(),
            SubclassCreatesDefaultTaskRunner{},
            traits...)) {}

  ~TaskEnvironment() override;

  scheduler::MainThreadSchedulerImpl* main_thread_scheduler() {
    return scheduler_.get();
  }
  v8::Isolate* isolate() { return main_thread_isolate_->isolate(); }
  void ResetIsolate() { main_thread_isolate_.reset(); }

  static bool IsSupported();

 private:
  // When |real_main_thread_scheduler|, instantiate a full featured
  // blink::MainThreadScheduler as opposed to a simple Thread scheduler.
  explicit TaskEnvironment(
      base::test::TaskEnvironment&& scoped_task_environment);

  std::unique_ptr<scheduler::MainThreadSchedulerImpl> scheduler_;
  std::optional<MainThreadIsolate> main_thread_isolate_;
  std::optional<ScopedMainThreadOverrider> main_thread_overrider_;
};

}  // namespace blink::test

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_TASK_ENVIRONMENT_H_
