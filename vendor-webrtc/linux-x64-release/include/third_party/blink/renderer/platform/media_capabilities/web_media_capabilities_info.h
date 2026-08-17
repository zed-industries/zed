// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CAPABILITIES_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CAPABILITIES_INFO_H_

namespace blink {

// Represents a MediaCapabilitiesInfo dictionary to be used outside of Blink.
// This is set by consumers and sent back to Blink.
struct WebMediaCapabilitiesInfo {
  bool supported = false;
  bool smooth = false;
  bool power_efficient = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CAPABILITIES_INFO_H_
