/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MERGE_H_
#define MODULES_AUDIO_CODING_NETEQ_MERGE_H_

#include "modules/audio_coding/neteq/audio_multi_vector.h"

namespace webrtc {

// Forward declarations.
class Expand;
class SyncBuffer;

// This class handles the transition from expansion to normal operation.
// When a packet is not available for decoding when needed, the expand operation
// is called to generate extrapolation data. If the missing packet arrives,
// i.e., it was just delayed, it can be decoded and appended directly to the
// end of the expanded data (thanks to how the Expand class operates). However,
// if a later packet arrives instead, the loss is a fact, and the new data must
// be stitched together with the end of the expanded data. This stitching is
// what the Merge class does.
class Merge {
 public:
  Merge(int fs_hz,
        size_t num_channels,
        Expand* expand,
        SyncBuffer* sync_buffer);
  virtual ~Merge();

  Merge(const Merge&) = delete;
  Merge& operator=(const Merge&) = delete;

  // The main method to produce the audio data. The decoded data is supplied in
  // `input`, having `input_length` samples in total for all channels
  // (interleaved). The result is written to `output`. The number of channels
  // allocated in `output` defines the number of channels that will be used when
  // de-interleaving `input`.
  virtual size_t Process(int16_t* input,
                         size_t input_length,
                         AudioMultiVector* output);

  virtual size_t RequiredFutureSamples();

 protected:
  const int fs_hz_;
  const size_t num_channels_;

 private:
  static const int kMaxSampleRate = 48000;
  static const size_t kExpandDownsampLength = 100;
  static const size_t kInputDownsampLength = 40;
  static const size_t kMaxCorrelationLength = 60;

  // Calls `expand_` to get more expansion data to merge with. The data is
  // written to `expanded_signal_`. Returns the length of the expanded data,
  // while `expand_period` will be the number of samples in one expansion period
  // (typically one pitch period). The value of `old_length` will be the number
  // of samples that were taken from the `sync_buffer_`.
  size_t GetExpandedSignal(size_t* old_length, size_t* expand_period);

  // Analyzes `input` and `expanded_signal` and returns muting factor (Q14) to
  // be used on the new data.
  int16_t SignalScaling(const int16_t* input,
                        size_t input_length,
                        const int16_t* expanded_signal) const;

  // Downsamples `input` (`input_length` samples) and `expanded_signal` to
  // 4 kHz sample rate. The downsampled signals are written to
  // `input_downsampled_` and `expanded_downsampled_`, respectively.
  void Downsample(const int16_t* input,
                  size_t input_length,
                  const int16_t* expanded_signal,
                  size_t expanded_length);

  // Calculates cross-correlation between `input_downsampled_` and
  // `expanded_downsampled_`, and finds the correlation maximum. The maximizing
  // lag is returned.
  size_t CorrelateAndPeakSearch(size_t start_position,
                                size_t input_length,
                                size_t expand_period) const;

  const int fs_mult_;  // fs_hz_ / 8000.
  const size_t timestamps_per_call_;
  Expand* expand_;
  SyncBuffer* sync_buffer_;
  int16_t expanded_downsampled_[kExpandDownsampLength];
  int16_t input_downsampled_[kInputDownsampLength];
  AudioMultiVector expanded_;
  std::vector<int16_t> temp_data_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MERGE_H_
