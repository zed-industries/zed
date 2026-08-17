// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_MOCK_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_MOCK_THREAD_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"
#include "components/viz/common/frame_sinks/begin_frame_args.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/common/input/web_input_event_attribution.h"
#include "third_party/blink/public/platform/scheduler/web_thread_scheduler.h"

namespace base {
class TaskObserver;
}

namespace blink {
namespace scheduler {

class WebMockThreadScheduler : public WebThreadScheduler {
 public:
  WebMockThreadScheduler() = default;
  WebMockThreadScheduler(const WebMockThreadScheduler&) = delete;
  WebMockThreadScheduler& operator=(const WebMockThreadScheduler&) = delete;
  ~WebMockThreadScheduler() override = default;

  MOCK_METHOD0(DefaultTaskRunner,
               scoped_refptr<base::SingleThreadTaskRunner>());
  MOCK_METHOD0(DeprecatedDefaultTaskRunner,
               scoped_refptr<base::SingleThreadTaskRunner>());
  MOCK_METHOD0(InputTaskRunner, scoped_refptr<base::SingleThreadTaskRunner>());
  MOCK_METHOD0(LoadingTaskRunner,
               scoped_refptr<base::SingleThreadTaskRunner>());
  MOCK_METHOD0(IPCTaskRunner, scoped_refptr<base::SingleThreadTaskRunner>());
  MOCK_METHOD0(CreateWebAgentGroupScheduler,
               std::unique_ptr<WebAgentGroupScheduler>());
  MOCK_METHOD1(SetRendererHidden, void(bool));
  MOCK_METHOD1(SetRendererBackgrounded, void(bool));
#if BUILDFLAG(IS_ANDROID)
  MOCK_METHOD0(PauseTimersForAndroidWebView, void());
  MOCK_METHOD0(ResumeTimersForAndroidWebView, void());
#endif
  MOCK_METHOD0(OnNavigate, void());
  MOCK_METHOD1(AddTaskObserver, void(base::TaskObserver*));
  MOCK_METHOD1(RemoveTaskObserver, void(base::TaskObserver*));
  MOCK_METHOD0(Shutdown, void());
  MOCK_METHOD0(VirtualTimePaused, void());
  MOCK_METHOD0(VirtualTimeResumed, void());
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_MOCK_THREAD_SCHEDULER_H_
