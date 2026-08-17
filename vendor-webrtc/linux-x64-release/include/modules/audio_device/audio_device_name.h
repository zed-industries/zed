/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_AUDIO_DEVICE_NAME_H_
#define MODULES_AUDIO_DEVICE_AUDIO_DEVICE_NAME_H_

#include <deque>
#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {

struct AudioDeviceName {
  // Represents a default device. Note that, on Windows there are two different
  // types of default devices (Default and Default Communication). They can
  // either be two different physical devices or be two different roles for one
  // single device. Hence, this id must be combined with a "role parameter" on
  // Windows to uniquely identify a default device.
  static const char kDefaultDeviceId[];

  AudioDeviceName() = default;
  AudioDeviceName(absl::string_view device_name, absl::string_view unique_id);

  ~AudioDeviceName() = default;

  // Support copy and move.
  AudioDeviceName(const AudioDeviceName& other) = default;
  AudioDeviceName(AudioDeviceName&&) = default;
  AudioDeviceName& operator=(const AudioDeviceName&) = default;
  AudioDeviceName& operator=(AudioDeviceName&&) = default;

  bool IsValid();

  std::string device_name;  // Friendly name of the device.
  std::string unique_id;    // Unique identifier for the device.
};

typedef std::deque<AudioDeviceName> AudioDeviceNames;

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_AUDIO_DEVICE_NAME_H_
