// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_AUDIO_WEB_AUDIO_DEVICE_SOURCE_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_AUDIO_WEB_AUDIO_DEVICE_SOURCE_TYPE_H_

namespace blink {

// Types of audio sources. Each source can have individual mixing and/or
// latency requirements for output. The source is specified by the client when
// requesting output sink from the factory, and the factory creates the output
// sink basing on those requirements.
//
// TODO(crbug.com/704136): This enum originally belongs to AudioDeviceFactory
// class (currently in //content). Move it back to the aforementiened class,
// when audio_device_factory.cc|h gets Onion souped.
enum class WebAudioDeviceSourceType {
  kNone = 0,
  kMediaElement,
  kWebRtc,
  kNonRtcAudioTrack,
  kWebAudioInteractive,
  kWebAudioBalanced,
  kWebAudioPlayback,
  kWebAudioExact,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_AUDIO_WEB_AUDIO_DEVICE_SOURCE_TYPE_H_
