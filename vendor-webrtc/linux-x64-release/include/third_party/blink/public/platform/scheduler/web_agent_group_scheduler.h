// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_AGENT_GROUP_SCHEDULER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_AGENT_GROUP_SCHEDULER_H_

#include "base/task/single_thread_task_runner.h"
#include "ipc/urgent_message_observer.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {
class AgentGroupScheduler;
namespace scheduler {

// WebAgentGroupScheduler schedules per-AgentSchedulingGroup tasks.
// AgentSchedulingGroup is Blink's unit of scheduling and performance isolation.
// And WebAgentGroupScheduler is a dedicated scheduler for each
// AgentSchedulingGroup. Any task posted on WebAgentGroupScheduler shouldn’t be
// run on a different WebAgentGroupScheduler.
class BLINK_PLATFORM_EXPORT WebAgentGroupScheduler
    : public IPC::UrgentMessageObserver {
 public:
  // Create a dummy AgentGroupScheduler only for testing
  static std::unique_ptr<blink::scheduler::WebAgentGroupScheduler>
  CreateForTesting();

  WebAgentGroupScheduler() = delete;
  ~WebAgentGroupScheduler() override;

  // Default task runner for an AgentSchedulingGroup.
  // Default task runners for different AgentSchedulingGroup would be
  // independent and won't have any ordering guarantees between them.
  scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner();

  // Compositor task runner for an AgentSchedulingGroup.
  // Compositor task runners for different AgentSchedulingGroup would be
  // independent and won't have any ordering guarantees between them.
  scoped_refptr<base::SingleThreadTaskRunner> CompositorTaskRunner();

  // The isolate for this WebAgentGroupScheduler.
  v8::Isolate* Isolate();

#if INSIDE_BLINK
  explicit WebAgentGroupScheduler(AgentGroupScheduler*);
  AgentGroupScheduler& GetAgentGroupScheduler();
#endif

  // IPC::Channel::UrgentMessageDelegate implementation:
  void OnUrgentMessageReceived() override;
  void OnUrgentMessageProcessed() override;

 protected:
  WebPrivatePtrForGC<AgentGroupScheduler> private_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_SCHEDULER_WEB_AGENT_GROUP_SCHEDULER_H_
