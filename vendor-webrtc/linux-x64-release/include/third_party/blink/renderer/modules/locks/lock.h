// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKS_LOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKS_LOCK_H_

#include "third_party/blink/public/mojom/feature_observer/feature_observer.mojom-blink.h"
#include "third_party/blink/public/mojom/locks/lock_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_lock_mode.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LockManager;
class ScriptState;

class Lock final : public ScriptWrappable,
                   public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Lock(ScriptState*,
       const String& name,
       mojom::blink::LockMode,
       mojo::PendingAssociatedRemote<mojom::blink::LockHandle>,
       mojo::PendingRemote<mojom::blink::ObservedFeature>,
       LockManager*);
  ~Lock() override;

  void Trace(Visitor*) const override;

  // Lock.idl
  String name() const { return name_; }
  V8LockMode mode() const;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // The lock is held until the passed promise resolves. When it is released,
  // the passed resolver is invoked with the promise's result.
  void HoldUntil(ScriptPromise<IDLAny>, ScriptPromiseResolver<IDLAny>*);

  static mojom::blink::LockMode EnumToMode(V8LockMode::Enum);
  static V8LockMode::Enum ModeToEnum(mojom::blink::LockMode);

 private:
  class ThenFunction;

  void ReleaseIfHeld();

  void OnConnectionError();

  Member<ScriptPromiseResolver<IDLAny>> resolver_;

  const String name_;
  const mojom::blink::LockMode mode_;

  // An opaque handle; this one end of a mojo pipe. When this is closed,
  // the lock is released by the back end.
  HeapMojoAssociatedRemote<mojom::blink::LockHandle> handle_;

  HeapMojoRemote<mojom::blink::ObservedFeature> lock_lifetime_;

  // LockManager::OnLockReleased() is called when this lock is released, to
  // stop artificially keeping this instance alive. It is necessary in the
  // case where the resolver's promise could potentially be GC'd.
  Member<LockManager> manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKS_LOCK_H_
