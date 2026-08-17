// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_DUMMY_SCHEDULERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_DUMMY_SCHEDULERS_H_

#include <memory>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "v8/include/v8-forward.h"

namespace blink {
class FrameScheduler;
class PageScheduler;
class AgentGroupScheduler;
class MainThread;

namespace scheduler {
class WebThreadScheduler;

// These methods create a simple implementations of the core scheduler classes.
// They are useful in the situation when you want to return a non-null scheduler
// (to ensure that your callers don't have to check for this) but don't have one
// available.
//
// The actual implementation is no-op, with two exceptions:
// - It returns a valid task runner (default one).
// - Creating new schedulers (e.g. DummyPageScheduler's CreateFrameScheduler
//   method returns a DummyFrameScheduler.
//
// Overall, their usage is discouraged except in the following two cases:
// - Detached frames (should be fixed with frame:document lifetime refactoring).
// - Tests

PLATFORM_EXPORT std::unique_ptr<FrameScheduler> CreateDummyFrameScheduler(
    v8::Isolate* isolate);
PLATFORM_EXPORT std::unique_ptr<PageScheduler> CreateDummyPageScheduler(
    v8::Isolate* isolate = nullptr);
PLATFORM_EXPORT AgentGroupScheduler* CreateDummyAgentGroupScheduler(
    v8::Isolate* isolate = nullptr);
PLATFORM_EXPORT std::unique_ptr<WebThreadScheduler>
CreateDummyWebMainThreadScheduler();

// This sets up a minimally viable implementation of blink::Thread without
// changing the current Platform. This is essentially a workaround for the
// initialization order in ScopedUnittestsEnvironmentSetup, and nobody else
// should use this.
PLATFORM_EXPORT std::unique_ptr<MainThread> CreateSimpleMainThread();

// These are dirty workaround for tests requiring the main thread task runner
// from a non-main thread. These functions are not thread safe, and the caller
// should ensure proper synchronization with MainThread()->GetTaskRunner(), e.g.
// if your test needs base::TaskEnvironment and a non-main thread may call
// MainThread()->GetTaskRunner(), call SetMainThreadTaskRunnerForTesting() in
// your test fixture's SetUp() before any task is posted, and call
// UnsetMainThreadTaskRunnerForTesting() in TearDown() after all tasks
// completed.
//
// TODO(crbug.com/1315595): These should be packed in a custom test fixture
// along with TaskEnvironment for reusability.
PLATFORM_EXPORT void SetMainThreadTaskRunnerForTesting();
PLATFORM_EXPORT void UnsetMainThreadTaskRunnerForTesting();

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_DUMMY_SCHEDULERS_H_
