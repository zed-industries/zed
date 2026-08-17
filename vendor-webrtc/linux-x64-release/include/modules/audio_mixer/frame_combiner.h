/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_MIXER_FRAME_COMBINER_H_
#define MODULES_AUDIO_MIXER_FRAME_COMBINER_H_

#include <memory>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_frame.h"
#include "modules/audio_processing/agc2/limiter.h"

namespace webrtc {
class ApmDataDumper;

class FrameCombiner {
 public:
  explicit FrameCombiner(bool use_limiter);
  ~FrameCombiner();

  // Combine several frames into one. Assumes sample_rate,
  // samples_per_channel of the input frames match the parameters. The
  // parameters 'number_of_channels' and 'sample_rate' are needed
  // because 'mix_list' can be empty. The parameter
  // 'number_of_streams' is used for determining whether to pass the
  // data through a limiter.
  void Combine(ArrayView<AudioFrame* const> mix_list,
               size_t number_of_channels,
               int sample_rate,
               size_t number_of_streams,
               AudioFrame* audio_frame_for_mixing);

  // Stereo, 48 kHz, 10 ms.
  static constexpr size_t kMaximumNumberOfChannels = 8;
  static constexpr size_t kMaximumChannelSize = 48 * 10;

 private:
  std::unique_ptr<ApmDataDumper> data_dumper_;
  Limiter limiter_;
  const bool use_limiter_;
  std::array<float, kMaximumChannelSize * kMaximumNumberOfChannels>
      mixing_buffer_ = {};
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_MIXER_FRAME_COMBINER_H_
