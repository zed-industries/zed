// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATE_EVENT_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_focus_reset.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_scroll_behavior.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_navigation_type.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/focused_element_change_observer.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/navigation_api/navigation_api.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class AbortController;
class AbortSignal;
class NavigationDestination;
class NavigateEventInit;
class NavigationInterceptOptions;
class NavigationReloadOptions;
class ExceptionState;
class FormData;
class V8NavigationInterceptHandler;
class V8NavigationInterceptPrecommitHandler;

class NavigateEvent final : public Event,
                            public ExecutionContextClient,
                            public FocusedElementChangeObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static NavigateEvent* Create(ExecutionContext* context,
                               const AtomicString& type,
                               NavigateEventInit* init,
                               AbortController* controller = nullptr) {
    return MakeGarbageCollected<NavigateEvent>(context, type, init, controller);
  }

  NavigateEvent(ExecutionContext* context,
                const AtomicString& type,
                NavigateEventInit* init,
                AbortController* controller);

  void SetDispatchParams(NavigateEventDispatchParams* dispatch_params) {
    dispatch_params_ = dispatch_params;
  }

  V8NavigationType navigationType() {
    return V8NavigationType(navigation_type_);
  }
  NavigationDestination* destination() { return destination_.Get(); }
  bool canIntercept() const { return can_intercept_; }
  bool userInitiated() const { return user_initiated_; }
  bool hashChange() const { return hash_change_; }
  AbortSignal* signal() { return signal_.Get(); }
  FormData* formData() const { return form_data_.Get(); }
  String downloadRequest() const { return download_request_; }
  ScriptValue info() const { return info_; }
  bool hasUAVisualTransition() const { return has_ua_visual_transition_; }
  Element* sourceElement() const { return source_element_.Get(); }
  void intercept(NavigationInterceptOptions*, ExceptionState&);

  // If intercept() was called, this is called after dispatch to either commit
  // the navigation or set the appropritate state for a deferred commit.
  void MaybeCommitImmediately(ScriptState*);

  void Redirect(const String& url, NavigationReloadOptions*, ExceptionState&);

  void React(ScriptState* script_state);

  void scroll(ExceptionState&);

  bool HasNavigationActions() const {
    return intercept_state_ != InterceptState::kNone;
  }
  void FinalizeNavigationActionPromisesList();

  void Abort(ScriptState*, ScriptValue error, CancelNavigationReason);

  // FocusedElementChangeObserver implementation:
  void DidChangeFocus() final;

  const AtomicString& InterfaceName() const final;
  void Trace(Visitor*) const final;

 private:
  bool PerformSharedChecks(const String& function_name, ExceptionState&);

  void CommitNow(ScriptState*);

  void PotentiallyResetTheFocus();
  void PotentiallyProcessScrollBehavior();
  void ProcessScrollBehavior();

  class FulfillReaction;
  class RejectReaction;
  void ReactDone(ScriptValue, bool did_fulfill);

  void DelayedLoadStartTimerFired();

  V8NavigationType::Enum navigation_type_;
  Member<NavigationDestination> destination_;
  bool can_intercept_;
  bool user_initiated_;
  bool hash_change_;
  Member<AbortController> controller_;
  Member<AbortSignal> signal_;
  Member<FormData> form_data_;
  String download_request_;
  ScriptValue info_;
  bool has_ua_visual_transition_ = false;
  Member<Element> source_element_;
  std::optional<V8NavigationFocusReset> focus_reset_behavior_ = std::nullopt;
  std::optional<V8NavigationScrollBehavior> scroll_behavior_ = std::nullopt;

  Member<NavigateEventDispatchParams> dispatch_params_;

  enum class InterceptState {
    kNone,
    kIntercepted,
    kCommitted,
    kScrolled,
    kFinished
  };
  InterceptState intercept_state_ = InterceptState::kNone;

  HeapVector<Member<V8NavigationInterceptPrecommitHandler>>
      navigation_action_precommit_handlers_list_;

  HeapVector<MemberScriptPromise<IDLUndefined>>
      navigation_action_promises_list_;
  HeapVector<Member<V8NavigationInterceptHandler>>
      navigation_action_handlers_list_;

  bool did_change_focus_during_intercept_ = false;

  // Used to delay the start of the loading UI when the navigation is
  // intercepted, in order to minimize jittering if any handlers are short.
  static constexpr base::TimeDelta kDelayLoadStart = base::Milliseconds(50);
  TaskHandle delayed_load_start_task_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_NAVIGATION_API_NAVIGATE_EVENT_H_
