// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_REQUEST_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/presentation/presentation_availability.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class PresentationConnection;

// Implements the PresentationRequest interface from the Presentation API from
// which websites can start or join presentation connections.
class MODULES_EXPORT PresentationRequest final
    : public EventTarget,
      public ActiveScriptWrappable<PresentationRequest>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PresentationRequest(ExecutionContext*, const Vector<KURL>&);
  ~PresentationRequest() override = default;

  static PresentationRequest* Create(ExecutionContext*,
                                     const String& url,
                                     ExceptionState&);
  static PresentationRequest* Create(ExecutionContext*,
                                     const Vector<String>& urls,
                                     ExceptionState&);

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  ScriptPromise<PresentationConnection> start(ScriptState*, ExceptionState&);
  ScriptPromise<PresentationConnection> reconnect(ScriptState*,
                                                  const String& id,
                                                  ExceptionState&);
  ScriptPromise<PresentationAvailability> getAvailability(ScriptState*,
                                                          ExceptionState&);

  const Vector<KURL>& Urls() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(connectionavailable, kConnectionavailable)

  void Trace(Visitor*) const override;

 protected:
  // EventTarget implementation.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  Vector<KURL> urls_;
  Member<PresentationAvailability> availability_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_REQUEST_H_
