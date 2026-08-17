// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_IDENTIFIABILITY_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_IDENTIFIABILITY_METRICS_H_

namespace blink {

class RTCRtpCapabilities;
class IdentifiableTokenBuilder;

void IdentifiabilityAddRTCRtpCapabilitiesToBuilder(
    IdentifiableTokenBuilder& builder,
    const RTCRtpCapabilities& capabilities);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_IDENTIFIABILITY_METRICS_H_
