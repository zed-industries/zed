// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_H_

#include "components/content_settings/core/common/content_settings_pattern.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/peerconnection/webrtc_ip_handling_policy.mojom-shared.h"

namespace blink {

struct BLINK_COMMON_EXPORT WebRtcIpHandlingUrlEntry {
  ContentSettingsPattern url_pattern;
  blink::mojom::WebRtcIpHandlingPolicy handling;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_H_
