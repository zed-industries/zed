// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_DOM_VIEW_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_DOM_VIEW_TRANSITION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_view_transition_callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/view_transition/view_transition.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_type_set.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ExecutionContext;
class ScriptState;
class ViewTransition;

// This class handles script interaction for the ViewTransition object. It
// implements the ViewTransition IDL interface.
class CORE_EXPORT DOMViewTransition : public ScriptWrappable,
                                      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();
  using PromiseProperty = ScriptPromiseProperty<IDLUndefined, IDLAny>;

 public:
  // Constructor for navigation-initiated view transition.
  explicit DOMViewTransition(ExecutionContext&, ViewTransition&);

  // Constructor for script-initiated view transition. Also delegated from the
  // navigation-initiated constructor.
  explicit DOMViewTransition(ExecutionContext&,
                             ViewTransition&,
                             V8ViewTransitionCallback*);

  ~DOMViewTransition() override;

  // ExecutionContextLifecycleObserver implementation.
  void ContextDestroyed() override;

  // IDL implementation. Refer to view_transition.idl for additional comments.
  void skipTransition();
  ScriptPromise<IDLUndefined> finished(ScriptState*) const;
  ScriptPromise<IDLUndefined> ready(ScriptState*) const;
  ScriptPromise<IDLUndefined> updateCallbackDone(ScriptState*) const;

  ViewTransitionTypeSet* types() const;

  // Called from ViewTransition when the transition is skipped/aborted for any
  // reason.
  void DidSkipTransition(ViewTransition::PromiseResponse);

  // Called just after the associated ViewTransition advances into the
  // kAnimating state but before any animation frames have been produced.
  void DidStartAnimating();
  // Called just before the associated ViewTransition advances from kAnimating
  // to kFinished state but before any finalization has run.
  void DidFinishAnimating();

  void InvokeDOMChangeCallback();

  ViewTransition* GetViewTransitionForTest() { return view_transition_.Get(); }

  void Trace(Visitor* visitor) const override;

 private:
  void AtMicrotask(ViewTransition::PromiseResponse response,
                   PromiseProperty* resolver);
  void HandlePromise(ViewTransition::PromiseResponse response,
                     PromiseProperty* property);

  friend class DOMChangeFinishedCallback;
  friend class DOMChangeRejectedCallback;
  void NotifyDOMCallbackFinished();
  void NotifyDOMCallbackRejected(ScriptValue value);

  // Cleared when the context is destroyed.
  Member<ExecutionContext> execution_context_;

  Member<ViewTransition> view_transition_;

  Member<V8ViewTransitionCallback> update_dom_callback_;
  Member<PromiseProperty> finished_promise_property_;
  Member<PromiseProperty> ready_promise_property_;
  Member<PromiseProperty> dom_updated_promise_property_;

  // The result of running the `update_dom_callback_`. This is set from
  // InvokeDOMChangeCallback.
  //
  // kNotInvoked: The state before InvokeDOMChangeCallback is run.
  // kRunning: Indicates that the callback is in running state. Note that even
  //           if the callback is synchronous, or there is no callback, the
  //           notification that it has finished running is async.
  // kFailed: Indicates that there was a failure in running the callback and the
  //          transition should be skipped.
  // kSucceeded: Indicates that the callback has completed successfully.
  enum class DOMCallbackResult { kNotInvoked, kRunning, kFailed, kSucceeded };
  DOMCallbackResult dom_callback_result_ = DOMCallbackResult::kNotInvoked;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_DOM_VIEW_TRANSITION_H_
