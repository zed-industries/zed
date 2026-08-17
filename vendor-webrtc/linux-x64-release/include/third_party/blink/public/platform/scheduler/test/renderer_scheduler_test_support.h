// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_RENDERER_SCHEDULER_TEST_SUPPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_RENDERER_SCHEDULER_TEST_SUPPORT_H_

#include <memory>

#include "base/functional/bind.h"

namespace base {
class SequencedTaskRunner;
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

namespace scheduler {

class WebThreadScheduler;
class WebMockThreadScheduler;

// Creates simple scheduling infrastructure for unit tests.  It allows creation
// of FrameSchedulers and PageSchedulers, but doesn't provide any task running
// infrastructure, relying on the presence of
// SingleThreadTaskRunner::GetCurrentDefault() instead, meaning that the users
// also have to create base::debug::TaskEnvironment.
std::unique_ptr<WebThreadScheduler> CreateWebMainThreadSchedulerForTests();

// Simple scheduling infrastructure for unit tests, with the addition of mocked
// methods.
std::unique_ptr<WebMockThreadScheduler>
CreateMockWebMainThreadSchedulerForTests();

// Returns a SequencedTaskRunner. This implementation is same as
// base::SequencedTaskRunner::GetCurrentDefault(), but this is intended to be
// used for testing. See crbug.com/794123.
scoped_refptr<base::SequencedTaskRunner> GetSequencedTaskRunnerForTesting();

// Returns the SingleThreadTaskRunner for the current thread for testing. This
// implementation is same as base::SingleThreadTaskRunner::GetCurrentDefault(),
// but this is intended to be used for testing. See crbug.com/794123.
scoped_refptr<base::SingleThreadTaskRunner>
GetSingleThreadTaskRunnerForTesting();

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_RENDERER_SCHEDULER_TEST_SUPPORT_H_
