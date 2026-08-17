// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_MOJOM_TRAITS_H_

#include "components/content_settings/core/common/content_settings_mojom_traits.h"
#include "components/content_settings/core/common/content_settings_pattern.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/peerconnection/webrtc_ip_handling_url_entry.h"
#include "third_party/blink/public/mojom/peerconnection/webrtc_ip_handling_policy.mojom-shared.h"
#include "third_party/blink/public/mojom/peerconnection/webrtc_ip_handling_url_entry.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::WebRtcIpHandlingUrlEntryDataView,
                 ::blink::WebRtcIpHandlingUrlEntry> {
  static const ContentSettingsPattern& url_pattern(
      const ::blink::WebRtcIpHandlingUrlEntry& data) {
    return data.url_pattern;
  }

  static const blink::mojom::WebRtcIpHandlingPolicy& handling(
      const ::blink::WebRtcIpHandlingUrlEntry& data) {
    return data.handling;
  }

  static bool Read(blink::mojom::WebRtcIpHandlingUrlEntryDataView data,
                   ::blink::WebRtcIpHandlingUrlEntry* out) {
    if (!data.ReadUrlPattern(&out->url_pattern) ||
        !data.ReadHandling(&out->handling)) {
      return false;
    }
    return true;
  }
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PEERCONNECTION_WEBRTC_IP_HANDLING_URL_ENTRY_MOJOM_TRAITS_H_
