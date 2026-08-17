// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_IO_TASK_RUNNER_TESTING_PLATFORM_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_IO_TASK_RUNNER_TESTING_PLATFORM_SUPPORT_H_

#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/scheduler/public/non_main_thread.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class IOTaskRunnerTestingPlatformSupport : public TestingPlatformSupport {
 public:
  IOTaskRunnerTestingPlatformSupport();

  scoped_refptr<base::SingleThreadTaskRunner> GetIOTaskRunner() const override;

 private:
  std::unique_ptr<NonMainThread> io_thread_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_IO_TASK_RUNNER_TESTING_PLATFORM_SUPPORT_H_
