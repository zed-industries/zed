// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_H_

#include "third_party/blink/public/mojom/keyboard_lock/keyboard_lock.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/keyboard/keyboard_layout_map.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class ExceptionState;

class KeyboardLayout final : public GarbageCollected<KeyboardLayout>,
                             public ExecutionContextClient {
 public:
  explicit KeyboardLayout(ExecutionContext*);

  KeyboardLayout(const KeyboardLayout&) = delete;
  KeyboardLayout& operator=(const KeyboardLayout&) = delete;

  ~KeyboardLayout() = default;

  ScriptPromise<KeyboardLayoutMap> GetKeyboardLayoutMap(ScriptState*,
                                                        ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  // Returns true if the local frame is attached to the renderer.
  bool IsLocalFrameAttached();

  // Returns true if |service_| is initialized and ready to be called.
  bool EnsureServiceConnected();

  void GotKeyboardLayoutMap(ScriptPromiseResolver<KeyboardLayoutMap>*,
                            mojom::blink::GetKeyboardLayoutMapResultPtr);

  Member<ScriptPromiseResolver<KeyboardLayoutMap>> script_promise_resolver_;

  HeapMojoRemote<mojom::blink::KeyboardLockService> service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_H_
