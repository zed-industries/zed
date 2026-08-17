/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DATA_OBSERVER_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DATA_OBSERVER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "api/audio/audio_device.h"
#include "api/scoped_refptr.h"

namespace webrtc {

// This interface will capture the raw PCM data of both the local captured as
// well as the mixed/rendered remote audio.
class AudioDeviceDataObserver {
 public:
  virtual void OnCaptureData(const void* audio_samples,
                             size_t num_samples,
                             size_t bytes_per_sample,
                             size_t num_channels,
                             uint32_t samples_per_sec) = 0;

  virtual void OnRenderData(const void* audio_samples,
                            size_t num_samples,
                            size_t bytes_per_sample,
                            size_t num_channels,
                            uint32_t samples_per_sec) = 0;

  AudioDeviceDataObserver() = default;
  virtual ~AudioDeviceDataObserver() = default;
};

// Creates an ADMWrapper around an ADM instance that registers
// the provided AudioDeviceDataObserver.
scoped_refptr<AudioDeviceModule> CreateAudioDeviceWithDataObserver(
    scoped_refptr<AudioDeviceModule> impl,
    std::unique_ptr<AudioDeviceDataObserver> observer);

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_INCLUDE_AUDIO_DEVICE_DATA_OBSERVER_H_
