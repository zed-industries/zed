// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_FAKE_THREAD_SCHEDULER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_FAKE_THREAD_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"
#include "third_party/blink/public/platform/scheduler/web_thread_scheduler.h"

namespace blink {
namespace scheduler {

class WebFakeThreadScheduler : public WebThreadScheduler {
 public:
  WebFakeThreadScheduler();
  WebFakeThreadScheduler(const WebFakeThreadScheduler&) = delete;
  WebFakeThreadScheduler& operator=(const WebFakeThreadScheduler&) = delete;
  ~WebFakeThreadScheduler() override;

  // RendererScheduler implementation.
  std::unique_ptr<MainThread> CreateMainThread() override;
  std::unique_ptr<WebAgentGroupScheduler> CreateWebAgentGroupScheduler()
      override;
  void SetRendererHidden(bool hidden) override;
  void SetRendererBackgrounded(bool backgrounded) override;
#if BUILDFLAG(IS_ANDROID)
  void PauseTimersForAndroidWebView() override;
  void ResumeTimersForAndroidWebView() override;
#endif
  void Shutdown() override;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_TEST_WEB_FAKE_THREAD_SCHEDULER_H_
