// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_INFO_H_

#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class SoftNavigationContext;
}  // namespace blink

namespace blink::scheduler {

// Interface for task state that can be propagated to descendant tasks and
// continuations.
class PLATFORM_EXPORT TaskAttributionInfo : public GarbageCollectedMixin {
 public:
  // Returns an id for this object, which is unique for the associated tracker.
  // This is primarily used for tracking task state across IPCs, e.g. for
  // navigation and postMessage.
  virtual TaskAttributionId Id() const = 0;

  // Returns the `SoftNavigationContext` associated with the task state, which
  // can be null.
  virtual SoftNavigationContext* GetSoftNavigationContext() = 0;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_INFO_H_
