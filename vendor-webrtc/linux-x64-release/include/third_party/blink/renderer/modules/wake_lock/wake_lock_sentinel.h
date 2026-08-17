// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_SENTINEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_SENTINEL_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_wake_lock_type.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/wake_lock/wake_lock_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class ExecutionContext;
class ScriptState;
class WakeLockManager;

class MODULES_EXPORT WakeLockSentinel final
    : public EventTarget,
      public ActiveScriptWrappable<WakeLockSentinel>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  WakeLockSentinel(ScriptState* script_state,
                   V8WakeLockType::Enum type,
                   WakeLockManager* manager);
  ~WakeLockSentinel() override;

  // Web-exposed interfaces
  DEFINE_ATTRIBUTE_EVENT_LISTENER(release, kRelease)
  ScriptPromise<IDLUndefined> release(ScriptState*);
  bool released() const;
  V8WakeLockType type() const;

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;
  void Trace(Visitor*) const override;

  // ActiveScriptWrappable overrides.
  bool HasPendingActivity() const override;

  // ExecutionContextLifecycleObserver overrides.
  void ContextDestroyed() override;

 private:
  friend class WakeLockManager;

  // This function, which only has any effect once, detaches this sentinel from
  // its |manager_|, and fires a "release" event.
  // It is implemented separately from release() itself so that |manager_| can
  // call it without triggering the creation of a new ScriptPromise, as
  // it is not relevant to |manager_| and this function may be called from a
  // context where |script_state_|'s context is no longer valid.
  void DoRelease();

  Member<WakeLockManager> manager_;
  bool released_ = false;
  const V8WakeLockType::Enum type_;

  FRIEND_TEST_ALL_PREFIXES(WakeLockSentinelTest, MultipleReleaseCalls);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_SENTINEL_H_
