/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ALIGNMENT_MIXER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ALIGNMENT_MIXER_H_

#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/block.h"

namespace webrtc {

// Performs channel conversion to mono for the purpose of providing a decent
// mono input for the delay estimation. This is achieved by analyzing all
// incoming channels and produce one single channel output.
class AlignmentMixer {
 public:
  AlignmentMixer(size_t num_channels,
                 const EchoCanceller3Config::Delay::AlignmentMixing& config);

  AlignmentMixer(size_t num_channels,
                 bool downmix,
                 bool adaptive_selection,
                 float excitation_limit,
                 bool prefer_first_two_channels);

  void ProduceOutput(const Block& x, ArrayView<float, kBlockSize> y);

  enum class MixingVariant { kDownmix, kAdaptive, kFixed };

 private:
  const size_t num_channels_;
  const float one_by_num_channels_;
  const float excitation_energy_threshold_;
  const bool prefer_first_two_channels_;
  const MixingVariant selection_variant_;
  std::array<size_t, 2> strong_block_counters_;
  std::vector<float> cumulative_energies_;
  int selected_channel_ = 0;
  size_t block_counter_ = 0;

  void Downmix(const Block& x, ArrayView<float, kBlockSize> y) const;
  int SelectChannel(const Block& x);
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ALIGNMENT_MIXER_H_
