// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_MEDIA_CAPABILITY_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_MEDIA_CAPABILITY_H_

#include "third_party/blink/public/platform/web_string.h"

namespace blink {

struct WebMediaKeySystemMediaCapability {
  enum class EncryptionScheme {
    kNotSpecified,
    kCenc,
    kCbcs,
    kCbcs_1_9,  // CBCS with a specific encrypt:skip pattern of 1:9.
    kUnrecognized
  };

  WebMediaKeySystemMediaCapability() = default;

  WebString content_type;
  WebString mime_type;
  WebString codecs;
  WebString robustness;
  EncryptionScheme encryption_scheme = EncryptionScheme::kNotSpecified;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_MEDIA_CAPABILITY_H_
