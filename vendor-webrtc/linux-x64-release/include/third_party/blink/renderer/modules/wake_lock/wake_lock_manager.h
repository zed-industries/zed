// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_MANAGER_H_

#include "base/gtest_prod_util.h"
#include "services/device/public/mojom/wake_lock.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_wake_lock_type.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/wake_lock/wake_lock_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ExecutionContext;
class WakeLockSentinel;

// https://w3c.github.io/screen-wake-lock/#dfn-activelocks
// Per-document and per-wake lock type internal data.
class MODULES_EXPORT WakeLockManager final
    : public GarbageCollected<WakeLockManager> {
 public:
  WakeLockManager(ExecutionContext*, V8WakeLockType::Enum);

  void AcquireWakeLock(ScriptPromiseResolver<WakeLockSentinel>*);
  void ClearWakeLocks();

  void UnregisterSentinel(WakeLockSentinel*);

  void Trace(Visitor* visitor) const;

 private:
  // Handle connection errors from |wake_lock_|.
  void OnWakeLockConnectionError();

  // A set with all WakeLockSentinel instances belonging to this
  // Navigator/WorkerNavigator.
  HeapHashSet<Member<WakeLockSentinel>> wake_lock_sentinels_;

  // An actual platform WakeLock. If bound, it means there is an active wake
  // lock for a given type.
  HeapMojoRemote<device::mojom::blink::WakeLock> wake_lock_;
  V8WakeLockType::Enum wake_lock_type_;

  // ExecutionContext from which we will connect to |wake_lock_service_|.
  Member<ExecutionContext> execution_context_;

  FRIEND_TEST_ALL_PREFIXES(WakeLockManagerTest, AcquireWakeLock);
  FRIEND_TEST_ALL_PREFIXES(WakeLockManagerTest, ReleaseAllWakeLocks);
  FRIEND_TEST_ALL_PREFIXES(WakeLockManagerTest, ReleaseOneWakeLock);
  FRIEND_TEST_ALL_PREFIXES(WakeLockManagerTest, ClearWakeLocks);
  FRIEND_TEST_ALL_PREFIXES(WakeLockManagerTest, WakeLockConnectionError);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WAKE_LOCK_WAKE_LOCK_MANAGER_H_
