// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTEXT_LIFECYCLE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTEXT_LIFECYCLE_OBSERVER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ContextLifecycleNotifier;

// Observer that gets notified when the context lifecycle is changed (e.g.
// destroyed, moved into back/forward cache). Used to observe ExecutionContext
// from platform/.
class PLATFORM_EXPORT ContextLifecycleObserver : public GarbageCollectedMixin {
 public:
  virtual ~ContextLifecycleObserver();
  void NotifyContextDestroyed();

  ContextLifecycleNotifier* GetContextLifecycleNotifier() const {
    return notifier_.Get();
  }
  void SetContextLifecycleNotifier(ContextLifecycleNotifier*);

  virtual bool IsExecutionContextLifecycleObserver() const { return false; }

  void Trace(Visitor*) const override;

 protected:
  ContextLifecycleObserver() = default;

  virtual void ContextDestroyed() = 0;

 private:
  WeakMember<ContextLifecycleNotifier> notifier_;
#if DCHECK_IS_ON()
  bool waiting_for_context_destroyed_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_CONTEXT_LIFECYCLE_OBSERVER_H_
