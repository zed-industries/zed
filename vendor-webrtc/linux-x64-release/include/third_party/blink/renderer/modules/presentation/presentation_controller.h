// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONTROLLER_H_

#include <vector>

#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/presentation/presentation.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class ControllerPresentationConnection;
class LocalDOMWindow;
class PresentationAvailabilityObserver;
class PresentationAvailabilityState;

// Implements the PresentationController interface from the Presentation API
// from which websites can implement the controlling side of a presentation.
class MODULES_EXPORT PresentationController
    : public GarbageCollected<PresentationController>,
      public Supplement<LocalDOMWindow>,
      public mojom::blink::PresentationController {
 public:
  static const char kSupplementName[];

  explicit PresentationController(LocalDOMWindow&);

  PresentationController(const PresentationController&) = delete;
  PresentationController& operator=(const PresentationController&) = delete;

  ~PresentationController() override;

  static PresentationController* From(LocalDOMWindow&);
  static PresentationController* FromContext(ExecutionContext*);

  // Implementation of Supplement.
  void Trace(Visitor*) const override;

  // Called by the Presentation object to advertize itself to the controller.
  // The Presentation object is kept as a WeakMember in order to avoid keeping
  // it alive when it is no longer in the tree.
  void SetPresentation(Presentation*);

  // Handling of running connections.
  void RegisterConnection(ControllerPresentationConnection*);

  // Return a connection in |m_connections| with id equals to |presentationId|,
  // url equals to one of |presentationUrls|, and state is not terminated.
  // Return null if such a connection does not exist.
  ControllerPresentationConnection* FindExistingConnection(
      const std::vector<blink::WebURL>& presentation_urls,
      const WebString& presentation_id);

  // Returns a reference to the PresentationService remote, requesting the
  // remote service if needed. May return an invalid remote if the associated
  // Document is detached.
  HeapMojoRemote<mojom::blink::PresentationService>& GetPresentationService();

  // Returns the PresentationAvailabilityState owned by |this|, creating it if
  // needed. Always non-null.
  PresentationAvailabilityState* GetAvailabilityState();

  // Marked virtual for testing.
  virtual void AddAvailabilityObserver(PresentationAvailabilityObserver*);
  virtual void RemoveAvailabilityObserver(PresentationAvailabilityObserver*);

 private:
  // mojom::blink::PresentationController implementation.
  void OnScreenAvailabilityUpdated(const KURL&,
                                   mojom::blink::ScreenAvailability) override;
  void OnConnectionStateChanged(
      mojom::blink::PresentationInfoPtr,
      mojom::blink::PresentationConnectionState) override;
  void OnConnectionClosed(mojom::blink::PresentationInfoPtr,
                          mojom::blink::PresentationConnectionCloseReason,
                          const String& message) override;
  void OnDefaultPresentationStarted(
      mojom::blink::PresentationConnectionResultPtr result) override;

  // Return the connection associated with the given |presentation_info| or
  // null if it doesn't exist.
  ControllerPresentationConnection* FindConnection(
      const mojom::blink::PresentationInfo&) const;

  // Lazily-instantiated when the page queries for availability.
  Member<PresentationAvailabilityState> availability_state_;

  // The Presentation instance associated with that frame.
  WeakMember<Presentation> presentation_;

  // The presentation connections associated with that frame.
  HeapHashSet<WeakMember<ControllerPresentationConnection>> connections_;

  // Holder of the Mojo connection to the PresentationService remote.
  blink::HeapMojoRemote<mojom::blink::PresentationService>
      presentation_service_remote_;

  // Lazily-initialized binding for mojom::blink::PresentationController. Sent
  // to |presentation_service_|'s implementation.
  HeapMojoReceiver<mojom::blink::PresentationController, PresentationController>
      presentation_controller_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONTROLLER_H_
