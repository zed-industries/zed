/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_WAV_BASED_SIMULATOR_H_
#define MODULES_AUDIO_PROCESSING_TEST_WAV_BASED_SIMULATOR_H_

#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/audio/audio_processing.h"
#include "api/scoped_refptr.h"
#include "modules/audio_processing/test/audio_processing_simulator.h"

namespace webrtc {
namespace test {

// Used to perform an audio processing simulation from wav files.
class WavBasedSimulator final : public AudioProcessingSimulator {
 public:
  WavBasedSimulator(
      const SimulationSettings& settings,
      absl_nonnull scoped_refptr<AudioProcessing> audio_processing);

  WavBasedSimulator() = delete;
  WavBasedSimulator(const WavBasedSimulator&) = delete;
  WavBasedSimulator& operator=(const WavBasedSimulator&) = delete;

  ~WavBasedSimulator() override;

  // Processes the WAV input.
  void Process() override;

  // Only analyzes the data for the simulation, instead of perform any
  // processing.
  void Analyze() override;

 private:
  enum SimulationEventType {
    kProcessStream,
    kProcessReverseStream,
  };

  void Initialize();
  bool HandleProcessStreamCall();
  bool HandleProcessReverseStreamCall();
  void PrepareProcessStreamCall();
  void PrepareReverseProcessStreamCall();
  static std::vector<SimulationEventType> GetDefaultEventChain();
  static std::vector<SimulationEventType> GetCustomEventChain(
      absl::string_view filename);

  std::vector<SimulationEventType> call_chain_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_WAV_BASED_SIMULATOR_H_
