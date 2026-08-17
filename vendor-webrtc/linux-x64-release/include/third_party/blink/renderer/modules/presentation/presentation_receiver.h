// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_RECEIVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_RECEIVER_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class PresentationConnectionList;
class ReceiverPresentationConnection;

// Implements the PresentationReceiver interface from the Presentation API from
// which websites can implement the receiving side of a presentation. This needs
// to be eagerly created in order to have the receiver associated with the
// client.
class MODULES_EXPORT PresentationReceiver final
    : public ScriptWrappable,
      public mojom::blink::PresentationReceiver {
  DEFINE_WRAPPERTYPEINFO();
  using ConnectionListProperty =
      ScriptPromiseProperty<PresentationConnectionList, DOMException>;

 public:
  explicit PresentationReceiver(LocalDOMWindow*);
  ~PresentationReceiver() override = default;

  // PresentationReceiver.idl implementation
  ScriptPromise<PresentationConnectionList> connectionList(ScriptState*);

  // mojom::blink::PresentationReceiver
  void OnReceiverConnectionAvailable(
      mojom::blink::PresentationConnectionResultPtr result) override;

  void RegisterConnection(ReceiverPresentationConnection*);
  void RemoveConnection(ReceiverPresentationConnection*);
  void Terminate();

  LocalDOMWindow* GetWindow() const { return window_.Get(); }

  void Trace(Visitor*) const override;

 private:
  friend class PresentationReceiverTest;

  static void RecordOriginTypeAccess(ExecutionContext&);

  Member<ConnectionListProperty> connection_list_property_;
  Member<PresentationConnectionList> connection_list_;

  HeapMojoReceiver<mojom::blink::PresentationReceiver, PresentationReceiver>
      presentation_receiver_receiver_;
  HeapMojoRemote<mojom::blink::PresentationService>
      presentation_service_remote_;
  Member<LocalDOMWindow> window_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_RECEIVER_H_
