// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_AGENT_GROUP_SCHEDULER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_AGENT_GROUP_SCHEDULER_IMPL_H_

#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/agent_group_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {
namespace scheduler {
class MainThreadSchedulerImpl;
class MainThreadTaskQueue;
class PageSchedulerImpl;
class PolicyUpdater;
class WebThreadScheduler;

// AgentGroupScheduler implementation which schedules per-AgentSchedulingGroup
// tasks.
class PLATFORM_EXPORT AgentGroupSchedulerImpl : public AgentGroupScheduler {
  // TODO(dtapuska): Remove usage of this prefinalizer. The MainThreadTaskQueues
  // need to be removed from the MainThreadScheduler and are created from both
  // oilpanned objects and non-oilpanned objects. This finalizer should be able
  // to be removed once more scheduling classes are moved to oilpan.
  USING_PRE_FINALIZER(AgentGroupSchedulerImpl, Dispose);

 public:
  explicit AgentGroupSchedulerImpl(
      MainThreadSchedulerImpl& main_thread_scheduler);
  AgentGroupSchedulerImpl(const AgentGroupSchedulerImpl&) = delete;
  AgentGroupSchedulerImpl& operator=(const AgentGroupSchedulerImpl&) = delete;
  ~AgentGroupSchedulerImpl() override;

  std::unique_ptr<PageScheduler> CreatePageScheduler(
      PageScheduler::Delegate*) override;
  scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> CompositorTaskRunner() override;
  scoped_refptr<MainThreadTaskQueue> CompositorTaskQueue();
  WebThreadScheduler& GetMainThreadScheduler() override;
  v8::Isolate* Isolate() override;

  void AddAgent(Agent* agent) override;
  void Trace(Visitor*) const override;
  void OnUrgentMessageReceived() override;
  void OnUrgentMessageProcessed() override;

  void PerformMicrotaskCheckpoint();

  void Dispose();

  // Associate a page scheduler which was not created by `CreatePageScheduler()`
  // with this agent group scheduler.
  void AddPageSchedulerForTesting(PageSchedulerImpl* page_scheduler);

  // Disassociate a page scheduler from this agent group scheduler.
  void RemovePageScheduler(PageSchedulerImpl* page_scheduler);

  // Increments / decrements the number of visible frames for an agent. May
  // schedule a policy update via `policy_updater`.
  void IncrementVisibleFramesForAgent(
      const base::UnguessableToken& agent_cluster_id,
      PolicyUpdater& policy_updater);
  void DecrementVisibleFramesForAgent(
      const base::UnguessableToken& agent_cluster_id,
      PolicyUpdater& policy_updater);

  // Returns true iff there is at least one visible frame for
  // `agent_cluster_id`.
  bool IsAgentVisible(const base::UnguessableToken& agent_cluster_id) const;

  // Update policy for all frames.
  void UpdatePolicy();

 private:
  scoped_refptr<MainThreadTaskQueue> default_task_queue_;
  scoped_refptr<base::SingleThreadTaskRunner> default_task_runner_;
  scoped_refptr<MainThreadTaskQueue> compositor_task_queue_;
  scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
  const raw_ref<MainThreadSchedulerImpl, DanglingUntriaged>
      main_thread_scheduler_;  // Not owned.
  HeapHashSet<WeakMember<Agent>> agents_;
  HashSet<PageSchedulerImpl*> page_schedulers_;
  std::map<base::UnguessableToken, int> num_visible_frames_per_agent_
      ALLOW_DISCOURAGED_TYPE(
          "There is no compelling reason to make base::UnguessableToken "
          "compatible with WTF::HashMap");
  bool is_updating_policy_ = false;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_AGENT_GROUP_SCHEDULER_IMPL_H_
