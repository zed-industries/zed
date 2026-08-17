/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_BLOCK_H_
#define MODULES_AUDIO_PROCESSING_AEC3_BLOCK_H_

#include <array>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"

namespace webrtc {

// Contains one or more channels of 4 milliseconds of audio data.
// The audio is split in one or more frequency bands, each with a sampling
// rate of 16 kHz.
class Block {
 public:
  Block(int num_bands, int num_channels, float default_value = 0.0f)
      : num_bands_(num_bands),
        num_channels_(num_channels),
        data_(num_bands * num_channels * kBlockSize, default_value) {}

  // Returns the number of bands.
  int NumBands() const { return num_bands_; }

  // Returns the number of channels.
  int NumChannels() const { return num_channels_; }

  // Modifies the number of channels and sets all samples to zero.
  void SetNumChannels(int num_channels) {
    num_channels_ = num_channels;
    data_.resize(num_bands_ * num_channels_ * kBlockSize);
    std::fill(data_.begin(), data_.end(), 0.0f);
  }

  // Iterators for accessing the data.
  auto begin(int band, int channel) {
    return data_.begin() + GetIndex(band, channel);
  }

  auto begin(int band, int channel) const {
    return data_.begin() + GetIndex(band, channel);
  }

  auto end(int band, int channel) { return begin(band, channel) + kBlockSize; }

  auto end(int band, int channel) const {
    return begin(band, channel) + kBlockSize;
  }

  // Access data via ArrayView.
  ArrayView<float, kBlockSize> View(int band, int channel) {
    return ArrayView<float, kBlockSize>(&data_[GetIndex(band, channel)],
                                        kBlockSize);
  }

  ArrayView<const float, kBlockSize> View(int band, int channel) const {
    return ArrayView<const float, kBlockSize>(&data_[GetIndex(band, channel)],
                                              kBlockSize);
  }

  // Lets two Blocks swap audio data.
  void Swap(Block& b) {
    std::swap(num_bands_, b.num_bands_);
    std::swap(num_channels_, b.num_channels_);
    data_.swap(b.data_);
  }

 private:
  // Returns the index of the first sample of the requested |band| and
  // |channel|.
  int GetIndex(int band, int channel) const {
    return (band * num_channels_ + channel) * kBlockSize;
  }

  int num_bands_;
  int num_channels_;
  std::vector<float> data_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_PROCESSING_AEC3_BLOCK_H_
