// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_CONTROLLER_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

// This enum is used to track usage of Complex SDP with UKM.
// The prefix indicates the SDP format used (PlanB or UnifiedPlan) or error if
// the SDP could not be parsed.
// The suffix indicates whether the peer connection has been configured with
// explicit SDP semantics (ExplicitSemantics) or not (ImplicitSemantics). The
// SDP format used does not necessarily match the semantics with which the peer
// connection has been configured.
enum class ComplexSdpCategory {
  kPlanBImplicitSemantics = 0,
  kPlanBExplicitSemantics = 1,
  kUnifiedPlanImplicitSemantics = 2,
  kUnifiedPlanExplicitSemantics = 3,
  kErrorImplicitSemantics = 4,
  kErrorExplicitSemantics = 5,
};

class RTCPeerConnectionController
    : public GarbageCollected<RTCPeerConnectionController>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];
  static RTCPeerConnectionController& From(Document&);

  void MaybeReportComplexSdp(ComplexSdpCategory);

  explicit RTCPeerConnectionController(Document&);

  void Trace(Visitor*) const override;

 private:
  bool has_reported_ukm_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_CONTROLLER_H_
