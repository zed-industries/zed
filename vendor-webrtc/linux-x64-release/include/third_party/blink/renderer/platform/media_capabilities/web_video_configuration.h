// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_VIDEO_CONFIGURATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_VIDEO_CONFIGURATION_H_

#include <optional>

#include "third_party/blink/public/platform/web_string.h"

namespace blink {

// Represents a VideoConfiguration dictionary to be used outside of Blink.
// It is created by Blink and passed to consumers that can assume that all
// required fields are properly set.
struct WebVideoConfiguration {
  WebString mime_type;
  WebString codec;
  unsigned width;
  unsigned height;
  unsigned bitrate;
  double framerate;
  std::optional<WebString> hdr_metadata_type;
  std::optional<WebString> color_gamut;
  std::optional<WebString> transfer_function;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_VIDEO_CONFIGURATION_H_
