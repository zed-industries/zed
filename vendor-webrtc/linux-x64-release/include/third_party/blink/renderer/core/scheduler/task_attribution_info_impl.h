// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_INFO_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_INFO_IMPL_H_

#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scheduler/script_wrappable_task_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/scheduler/public/task_attribution_info.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
class ExecutionContext;
class SchedulerTaskContext;
class SoftNavigationContext;

class CORE_EXPORT TaskAttributionInfoImpl final
    : public GarbageCollected<TaskAttributionInfoImpl>,
      public WrappableTaskState,
      public scheduler::TaskAttributionInfo {
 public:
  TaskAttributionInfoImpl(scheduler::TaskAttributionId, SoftNavigationContext*);

  // `WrappableTaskState` implementation:
  scheduler::TaskAttributionInfo* GetTaskAttributionInfo() override;
  SchedulerTaskContext* GetSchedulerTaskContextFor(
      const ExecutionContext&) override;

  // `scheduler::TaskAttributionInfo` implementation:
  scheduler::TaskAttributionId Id() const override;
  SoftNavigationContext* GetSoftNavigationContext() override;

  void Trace(Visitor*) const override;

 private:
  const scheduler::TaskAttributionId id_;
  Member<SoftNavigationContext> soft_navigation_context_;
};

// `TaskAttributionInfoImpl` is the only implementation of
// `scheduler::TaskAttributionInfo`, so this cast is always safe.
template <>
struct DowncastTraits<TaskAttributionInfoImpl> {
  static bool AllowFrom(const scheduler::TaskAttributionInfo&) { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_INFO_IMPL_H_
