// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_LIST_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/presentation/presentation_connection.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Implements the PresentationConnectionList interface from the Presentation API
// from which represents set of presentation connections in the set of
// presentation controllers.
class MODULES_EXPORT PresentationConnectionList final
    : public EventTarget,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PresentationConnectionList(ExecutionContext*);
  ~PresentationConnectionList() override = default;

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextClient::GetExecutionContext();
  }

  // PresentationConnectionList.idl implementation.
  const HeapVector<Member<PresentationConnection>>& connections() const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(connectionavailable, kConnectionavailable)

  void AddConnection(PresentationConnection*);
  // Remove connection from connection list. Returns true if connection is
  // successfully removed; Returns false if connection does not exist.
  bool RemoveConnection(PresentationConnection*);
  void DispatchConnectionAvailableEvent(PresentationConnection*);
  bool IsEmpty();

  void Trace(Visitor*) const override;

 protected:
  // EventTarget implementation.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  friend class PresentationReceiverTest;

  HeapVector<Member<PresentationConnection>> connections_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_LIST_H_
