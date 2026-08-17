// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CONFIGURATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CONFIGURATION_H_

#include <optional>

#include "third_party/blink/renderer/platform/media_capabilities/web_audio_configuration.h"
#include "third_party/blink/renderer/platform/media_capabilities/web_video_configuration.h"

namespace blink {

enum class MediaConfigurationType {
  // for decodingInfo
  kFile,
  kMediaSource,
  // for encodingInfo
  kRecord,
  kTransmission,
};

// Represents a MediaConfiguration dictionary to be used outside of Blink. At
// least one of `audioConfiguration` or `videoConfiguration` will be set.
// It is created by Blink and passed to consumers that can assume that all
// required fields are properly set.
struct WebMediaConfiguration {
  WebMediaConfiguration() = default;

  WebMediaConfiguration(
      MediaConfigurationType type,
      std::optional<WebAudioConfiguration> audio_configuration,
      std::optional<WebVideoConfiguration> video_configuration)
      : type(type),
        audio_configuration(audio_configuration),
        video_configuration(video_configuration) {}

  MediaConfigurationType type;

  std::optional<WebAudioConfiguration> audio_configuration;
  std::optional<WebVideoConfiguration> video_configuration;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CAPABILITIES_WEB_MEDIA_CONFIGURATION_H_
