// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class KeyboardLayout;
class KeyboardLayoutMap;
class KeyboardLock;
class ScriptState;

class Keyboard final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit Keyboard(ExecutionContext*);

  Keyboard(const Keyboard&) = delete;
  Keyboard& operator=(const Keyboard&) = delete;

  ~Keyboard() override;

  // KeyboardLock API: https://w3c.github.io/keyboard-lock/
  ScriptPromise<IDLUndefined> lock(ScriptState*,
                                   const Vector<String>&,
                                   ExceptionState&);
  void unlock(ScriptState*);

  ScriptPromise<KeyboardLayoutMap> getLayoutMap(ScriptState*, ExceptionState&);

  // ScriptWrappable override.
  void Trace(Visitor*) const override;

 private:
  Member<KeyboardLock> keyboard_lock_;
  Member<KeyboardLayout> keyboard_layout_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_H_
