// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_CONFIGURATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_CONFIGURATION_H_

#include <vector>

#include "media/base/eme_constants.h"
#include "third_party/blink/public/platform/web_encrypted_media_types.h"
#include "third_party/blink/public/platform/web_media_key_system_media_capability.h"

namespace blink {

struct WebMediaKeySystemConfiguration {
  enum class Requirement {
    kRequired,
    kOptional,
    kNotAllowed,
  };

  WebString label;
  std::vector<media::EmeInitDataType> init_data_types;
  std::vector<WebMediaKeySystemMediaCapability> audio_capabilities;
  std::vector<WebMediaKeySystemMediaCapability> video_capabilities;
  Requirement distinctive_identifier = Requirement::kOptional;
  Requirement persistent_state = Requirement::kOptional;
  std::vector<WebEncryptedMediaSessionType> session_types;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_KEY_SYSTEM_CONFIGURATION_H_
