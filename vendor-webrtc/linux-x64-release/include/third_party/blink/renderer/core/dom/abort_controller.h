// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class AbortSignal;
class ScriptState;
class ScriptValue;

// Implementation of https://dom.spec.whatwg.org/#interface-abortcontroller
// See also design doc at
// https://docs.google.com/document/d/1OuoCG2uiijbAwbCw9jaS7tHEO0LBO_4gMNio1ox0qlY/edit
class CORE_EXPORT AbortController : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(AbortController, Dispose);

 public:
  static AbortController* Create(ScriptState*);

  explicit AbortController(AbortSignal*);
  ~AbortController() override;

  // abort_controller.idl

  // https://dom.spec.whatwg.org/#dom-abortcontroller-signal
  AbortSignal* signal() const { return signal_.Get(); }

  // https://dom.spec.whatwg.org/#dom-abortcontroller-abort
  void abort(ScriptState*);
  void abort(ScriptState*, ScriptValue reason);

  void Dispose();

  void Trace(Visitor*) const override;

 private:
  Member<AbortSignal> signal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_CONTROLLER_H_
