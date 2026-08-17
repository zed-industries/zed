// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_POLICY_UPDATER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_POLICY_UPDATER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/stack_allocated.h"

namespace blink::scheduler {

class AgentGroupSchedulerImpl;
class FrameSchedulerImpl;
class PageSchedulerImpl;

// Updates policy for a frame, page or agent at the end of a scope.
class PolicyUpdater {
  STACK_ALLOCATED();

 public:
  PolicyUpdater();
  ~PolicyUpdater();

  // No copy or assignment allowed.
  PolicyUpdater(const PolicyUpdater&) = delete;
  PolicyUpdater& operator=(const PolicyUpdater&) = delete;

  // Schedules a call to `UpdatePolicy()` on a frame, page or agent group when
  // this object is deleted.
  void UpdateFramePolicy(FrameSchedulerImpl* frame);
  void UpdatePagePolicy(PageSchedulerImpl* page);
  void UpdateAgentGroupPolicy(AgentGroupSchedulerImpl* agent_group);

 private:
  raw_ptr<FrameSchedulerImpl> frame_;
  raw_ptr<PageSchedulerImpl> page_;
  AgentGroupSchedulerImpl* agent_group_{nullptr};
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_POLICY_UPDATER_H_
