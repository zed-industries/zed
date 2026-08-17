/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_UTILITY_CHANNEL_MIXER_H_
#define AUDIO_UTILITY_CHANNEL_MIXER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/audio/audio_frame.h"
#include "api/audio/channel_layout.h"

namespace webrtc {

// ChannelMixer is for converting audio between channel layouts.  The conversion
// matrix is built upon construction and used during each Transform() call.  The
// algorithm works by generating a conversion matrix mapping each output channel
// to list of input channels.  The transform renders all of the output channels,
// with each output channel rendered according to a weighted sum of the relevant
// input channels as defined in the matrix.
// This file is derived from Chromium's media/base/channel_mixer.h.
class ChannelMixer {
 public:
  // To mix two channels into one and preserve loudness, we must apply
  // (1 / sqrt(2)) gain to each.
  static constexpr float kHalfPower = 0.707106781186547524401f;

  ChannelMixer(ChannelLayout input_layout,
               size_t input_channels,
               ChannelLayout output_layout,
               size_t output_channels);
  ChannelMixer(ChannelLayout input_layout, ChannelLayout output_layout);
  ~ChannelMixer();

  // Transforms all input channels corresponding to the selected `input_layout`
  // to the number of channels in the selected `output_layout`.
  // Example usage (downmix from stereo to mono):
  //
  //   ChannelMixer mixer(CHANNEL_LAYOUT_STEREO, CHANNEL_LAYOUT_MONO);
  //   AudioFrame frame;
  //   frame.samples_per_channel_ = 160;
  //   frame.num_channels_ = 2;
  //   EXPECT_EQ(2u, frame.channels());
  //   mixer.Transform(&frame);
  //   EXPECT_EQ(1u, frame.channels());
  //
  void Transform(AudioFrame* frame);

 private:
  bool IsUpMixing() const { return output_channels_ > input_channels_; }

  // Selected channel layouts.
  const ChannelLayout input_layout_;
  const ChannelLayout output_layout_;

  // Channel counts for input and output.
  const size_t input_channels_;
  const size_t output_channels_;

  // 2D matrix of output channels to input channels.
  std::vector<std::vector<float> > matrix_;

  // 1D array used as temporary storage during the transformation.
  std::unique_ptr<int16_t[]> audio_vector_;

  // Number of elements allocated for `audio_vector_`.
  size_t audio_vector_size_ = 0;

  // Optimization case for when we can simply remap the input channels to output
  // channels, i.e., when all scaling factors in `matrix_` equals 1.0.
  bool remapping_;

  // Delete the copy constructor and assignment operator.
  ChannelMixer(const ChannelMixer& other) = delete;
  ChannelMixer& operator=(const ChannelMixer& other) = delete;
};

}  // namespace webrtc

#endif  // AUDIO_UTILITY_CHANNEL_MIXER_H_
