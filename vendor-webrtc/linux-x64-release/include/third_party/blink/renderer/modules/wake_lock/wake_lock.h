// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_H_

#include <array>

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_wake_lock_type.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/wake_lock/wake_lock_type.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class NavigatorBase;
class ScriptState;
class WakeLockManager;
class WakeLockSentinel;

class MODULES_EXPORT WakeLock final : public ScriptWrappable,
                                      public Supplement<NavigatorBase>,
                                      public ExecutionContextLifecycleObserver,
                                      public PageVisibilityObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Getter for navigator.wakelock
  static WakeLock* wakeLock(NavigatorBase&);

  explicit WakeLock(NavigatorBase&);

  ScriptPromise<WakeLockSentinel> request(ScriptState*,
                                          V8WakeLockType type,
                                          ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  // While this could be part of request() itself, having it as a separate
  // function makes testing (which uses a custom ScriptPromiseResolverBase) a
  // lot easier.
  void DoRequest(V8WakeLockType::Enum,
                 ScriptPromiseResolver<WakeLockSentinel>*);

  void DidReceivePermissionResponse(V8WakeLockType::Enum,
                                    ScriptPromiseResolver<WakeLockSentinel>*,
                                    mojom::blink::PermissionStatus);

  // ExecutionContextLifecycleObserver implementation
  void ContextDestroyed() override;

  // PageVisibilityObserver implementation
  void PageVisibilityChanged() override;

  // Permission handling
  mojom::blink::PermissionService* GetPermissionService();

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;

  // https://w3c.github.io/screen-wake-lock/#dfn-activelocks
  // An ordered map of wake lock types to a list of WakeLockSentinel objects
  // associated with this Document.
  std::array<Member<WakeLockManager>, V8WakeLockType::kEnumSize> managers_;

  FRIEND_TEST_ALL_PREFIXES(WakeLockSentinelTest, ContextDestruction);
  FRIEND_TEST_ALL_PREFIXES(WakeLockTest, RequestWakeLockGranted);
  FRIEND_TEST_ALL_PREFIXES(WakeLockTest, RequestWakeLockDenied);
  FRIEND_TEST_ALL_PREFIXES(WakeLockTest, LossOfDocumentActivity);
  FRIEND_TEST_ALL_PREFIXES(WakeLockTest, PageVisibilityHidden);
  FRIEND_TEST_ALL_PREFIXES(WakeLockTest,
                           PageVisibilityHiddenBeforeLockAcquisition);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_H_
