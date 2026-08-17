// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_FEATURES_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_FEATURES_H_
#include "base/feature_list.h"
#include "base/metrics/field_trial_params.h"
#include "third_party/blink/renderer/modules/modules_export.h"
namespace blink {
MODULES_EXPORT BASE_DECLARE_FEATURE(kWebRtcEncodedTransformDirectCallback);
MODULES_EXPORT BASE_DECLARE_FEATURE(kWebRtcEncryptedRtpHeaderExtensions);
MODULES_EXPORT BASE_DECLARE_FEATURE(
    kWebRtcRtpScriptTransformerFrameRestrictions);
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_FEATURES_H_
