// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_AGENT_GROUP_SCHEDULER_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_AGENT_GROUP_SCHEDULER_SCHEDULER_H_

#include "base/memory/raw_ref.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"
#include "third_party/blink/public/platform/scheduler/test/renderer_scheduler_test_support.h"
#include "third_party/blink/public/platform/scheduler/web_thread_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/agent_group_scheduler.h"

namespace blink {
namespace scheduler {

class FakeAgentGroupScheduler : public AgentGroupScheduler {
 public:
  explicit FakeAgentGroupScheduler(WebThreadScheduler& web_thread_scheduler)
      : web_thread_scheduler_(web_thread_scheduler) {}
  ~FakeAgentGroupScheduler() override = default;

  scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner() override {
    return GetSingleThreadTaskRunnerForTesting();
  }

  scoped_refptr<base::SingleThreadTaskRunner> CompositorTaskRunner() override {
    return GetSingleThreadTaskRunnerForTesting();
  }

  std::unique_ptr<PageScheduler> CreatePageScheduler(
      PageScheduler::Delegate*) override {
    return nullptr;
  }

  WebThreadScheduler& GetMainThreadScheduler() override {
    return *web_thread_scheduler_;
  }

  v8::Isolate* Isolate() override { NOTREACHED(); }

  void AddAgent(Agent* agent) override {}

  void OnUrgentMessageReceived() override {}

  void OnUrgentMessageProcessed() override {}

 private:
  const raw_ref<WebThreadScheduler> web_thread_scheduler_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_FAKE_AGENT_GROUP_SCHEDULER_SCHEDULER_H_
