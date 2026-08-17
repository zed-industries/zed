/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_UTILITY_CHANNEL_MIXING_MATRIX_H_
#define AUDIO_UTILITY_CHANNEL_MIXING_MATRIX_H_

#include <vector>

#include "api/audio/channel_layout.h"

namespace webrtc {

class ChannelMixingMatrix {
 public:
  ChannelMixingMatrix(ChannelLayout input_layout,
                      int input_channels,
                      ChannelLayout output_layout,
                      int output_channels);

  ~ChannelMixingMatrix();

  // Create the transformation matrix of input channels to output channels.
  // Updates the empty matrix with the transformation, and returns true
  // if the transformation is just a remapping of channels (no mixing).
  // The size of `matrix` is `output_channels` x `input_channels`, i.e., the
  // number of rows equals the number of output channels and the number of
  // columns corresponds to the number of input channels.
  // This file is derived from Chromium's media/base/channel_mixing_matrix.h.
  bool CreateTransformationMatrix(std::vector<std::vector<float>>* matrix);

 private:
  // Result transformation of input channels to output channels
  std::vector<std::vector<float>>* matrix_;

  // Input and output channel layout provided during construction.
  const ChannelLayout input_layout_;
  const int input_channels_;
  const ChannelLayout output_layout_;
  const int output_channels_;

  // Helper variable for tracking which inputs are currently unaccounted,
  // should be empty after construction completes.
  std::vector<Channels> unaccounted_inputs_;

  // Helper methods for managing unaccounted input channels.
  void AccountFor(Channels ch);
  bool IsUnaccounted(Channels ch) const;

  // Helper methods for checking if `ch` exists in either `input_layout_` or
  // `output_layout_` respectively.
  bool HasInputChannel(Channels ch) const;
  bool HasOutputChannel(Channels ch) const;

  // Helper methods for updating `matrix_` with the proper value for
  // mixing `input_ch` into `output_ch`.  MixWithoutAccounting() does not
  // remove the channel from `unaccounted_inputs_`.
  void Mix(Channels input_ch, Channels output_ch, float scale);
  void MixWithoutAccounting(Channels input_ch, Channels output_ch, float scale);

  // Delete the copy constructor and assignment operator.
  ChannelMixingMatrix(const ChannelMixingMatrix& other) = delete;
  ChannelMixingMatrix& operator=(const ChannelMixingMatrix& other) = delete;
};

}  // namespace webrtc

#endif  // AUDIO_UTILITY_CHANNEL_MIXING_MATRIX_H_
