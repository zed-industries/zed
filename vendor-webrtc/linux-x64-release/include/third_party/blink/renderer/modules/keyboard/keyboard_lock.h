// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LOCK_H_

#include "third_party/blink/public/mojom/keyboard_lock/keyboard_lock.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ExceptionState;

class KeyboardLock final : public GarbageCollected<KeyboardLock>,
                           public ExecutionContextClient {
 public:
  explicit KeyboardLock(ExecutionContext*);

  KeyboardLock(const KeyboardLock&) = delete;
  KeyboardLock& operator=(const KeyboardLock&) = delete;

  ~KeyboardLock();

  ScriptPromise<IDLUndefined> lock(ScriptState*,
                                   const Vector<String>&,
                                   ExceptionState&);
  void unlock(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  // Returns true if the local frame is attached to the renderer.
  bool IsLocalFrameAttached();

  // Returns true if |service_| is initialized and ready to be called.
  bool EnsureServiceConnected();

  // Returns true if the current frame is a top-level browsing context.
  bool CalledFromSupportedContext(ExecutionContext*);

  void LockRequestFinished(ScriptPromiseResolver<IDLUndefined>*,
                           mojom::KeyboardLockRequestResult);

  HeapMojoRemote<mojom::blink::KeyboardLockService> service_;
  Member<ScriptPromiseResolver<IDLUndefined>> request_keylock_resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LOCK_H_
