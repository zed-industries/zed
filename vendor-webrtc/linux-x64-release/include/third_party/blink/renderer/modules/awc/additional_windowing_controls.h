// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AWC_ADDITIONAL_WINDOWING_CONTROLS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AWC_ADDITIONAL_WINDOWING_CONTROLS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class LocalDOMWindow;
class ScriptState;

// Complements LocalDOMWindow with additional windowing controls.
class AdditionalWindowingControls {
  STATIC_ONLY(AdditionalWindowingControls);

 public:
  // Web-exposed interfaces:
  static ScriptPromise<IDLUndefined> maximize(ScriptState*,
                                              LocalDOMWindow&,
                                              ExceptionState& exception_state);
  static ScriptPromise<IDLUndefined> minimize(ScriptState*,
                                              LocalDOMWindow&,
                                              ExceptionState& exception_state);
  static ScriptPromise<IDLUndefined> restore(ScriptState*,
                                             LocalDOMWindow&,
                                             ExceptionState& exception_state);
  static ScriptPromise<IDLUndefined> setResizable(
      ScriptState*,
      LocalDOMWindow&,
      bool resizable,
      ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AWC_ADDITIONAL_WINDOWING_CONTROLS_H_
