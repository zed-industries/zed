/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_H_
#define MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_H_

#include <memory>

namespace webrtc {
class ApmDataDumper;

// Saturation protector. Analyzes peak levels and recommends a headroom to
// reduce the chances of clipping.
class SaturationProtector {
 public:
  virtual ~SaturationProtector() = default;

  // Returns the recommended headroom in dB.
  virtual float HeadroomDb() = 0;

  // Analyzes the peak level of a 10 ms frame along with its speech probability
  // and the current speech level estimate to update the recommended headroom.
  virtual void Analyze(float speech_probability,
                       float peak_dbfs,
                       float speech_level_dbfs) = 0;

  // Resets the internal state.
  virtual void Reset() = 0;
};

// Creates a saturation protector that starts at `initial_headroom_db`.
std::unique_ptr<SaturationProtector> CreateSaturationProtector(
    float initial_headroom_db,
    int adjacent_speech_frames_threshold,
    ApmDataDumper* apm_data_dumper);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_H_
