// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_INTERNAL_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_INTERNAL_OBSERVER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_script_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"

namespace blink {

// Implementation of the DOM `Observable` API's "internal observer" concept.
// See: https://wicg.github.io/observable/#internal-observer. It is responsible
// for holding onto the concrete "next", "error", and "complete" algorithms that
// `Subscriber::{next(), error(), complete()} ultimately invoke.
//
// Most of the time these algorithms are whatever JavaScript passes in as
// callbacks in the Web IDL `Observer` dictionary. But for the various
// Promise-returning operators on the Observable interface [1], it is C++ that
// subscribes to an Observable, passing in its own native "next", "error", and
// "complete" algorithms in the `ObservableInternalObserver`.
//
// [1]: https://wicg.github.io/observable/#promise-returning-operators)
class CORE_EXPORT ObservableInternalObserver
    : public GarbageCollected<ObservableInternalObserver> {
 public:
  // See https://wicg.github.io/observable/#internal-observer:
  //
  // An internal observer is a struct with the following items:
  // next steps
  //   An algorithm that takes a single parameter of type `any`. Initially,
  //   these steps do nothing.
  virtual void Next(ScriptValue) = 0;

  // error steps
  //   An algorithm that takes a single parameter of type `any`. Initially, the
  //   default error algorithm.
  virtual void Error(ScriptState* script_state, ScriptValue error_value) {
    // The given observer's `error()` handler can be null here if the error
    // callback was simply not passed in (it is not required).
    //
    // Reporting the exception requires a valid `ScriptState`, which we don't
    // have if we're in a detached context. See observable-constructor.window.js
    // for tests.
    if (!script_state->ContextIsValid()) {
      return;
    }
    ScriptState::Scope scope(script_state);
    V8ScriptRunner::ReportException(script_state->GetIsolate(),
                                    error_value.V8Value());
  }

  // complete steps
  //   An algorithm with no parameters. Initially, these steps do nothing.
  virtual void Complete() = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_INTERNAL_OBSERVER_H_
