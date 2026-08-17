// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/presentation/presentation_availability_observer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class KURL;

// Expose whether there is a presentation display available for |url|. The
// object will be initialized with a default value passed via ::take() and will
// then subscribe to receive callbacks if the status for |url| were to
// change. The object will only listen to changes when required.
class MODULES_EXPORT PresentationAvailability final
    : public EventTarget,
      public ActiveScriptWrappable<PresentationAvailability>,
      public ExecutionContextLifecycleStateObserver,
      public PageVisibilityObserver,
      public PresentationAvailabilityObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static constexpr char kNotSupportedErrorInfo[] =
      "Requesting PresentationAvailability isn't supported at the moment. It "
      "can be due to a permanent or temporary system limitation. It is "
      "recommended to try to blindly start a presentation in that case.";

  static PresentationAvailability* Take(ExecutionContext*,
                                        const WTF::Vector<KURL>&,
                                        bool);

  PresentationAvailability(ExecutionContext*, const WTF::Vector<KURL>&, bool);
  ~PresentationAvailability() override;

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // PresentationAvailabilityObserver implementation.
  void AvailabilityChanged(blink::mojom::ScreenAvailability) override;
  const Vector<KURL>& Urls() const override;

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleStateObserver implementation.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) override;
  void ContextDestroyed() override;

  // PageVisibilityObserver implementation.
  void PageVisibilityChanged() override;

  bool value() const { return value_; }

  void AddResolver(ScriptPromiseResolver<PresentationAvailability>* resolver);
  void RejectPendingPromises();
  void ResolvePendingPromises();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  void Trace(Visitor*) const override;

 protected:
  // EventTarget implementation.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  // Current state of the ExecutionContextLifecycleStateObserver. It is Active
  // when created. It becomes Suspended when suspend() is called and moves back
  // to Active if resume() is called. It becomes Inactive when stop() is called
  // or at destruction time.
  enum class State : char {
    kActive,
    kSuspended,
    kInactive,
  };

  void SetState(State);
  void UpdateListening();

  Vector<KURL> urls_;
  bool value_;
  State state_;
  HeapVector<Member<ScriptPromiseResolver<PresentationAvailability>>>
      availability_resolvers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_H_
