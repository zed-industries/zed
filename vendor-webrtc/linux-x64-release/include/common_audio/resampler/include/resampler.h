/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

/*
 * A wrapper for resampling a numerous amount of sampling combinations.
 */

#ifndef COMMON_AUDIO_RESAMPLER_INCLUDE_RESAMPLER_H_
#define COMMON_AUDIO_RESAMPLER_INCLUDE_RESAMPLER_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

// All methods return 0 on success and -1 on failure.
class Resampler {
 public:
  Resampler();
  Resampler(int inFreq, int outFreq, size_t num_channels);
  ~Resampler();

  // Reset all states
  int Reset(int inFreq, int outFreq, size_t num_channels);

  // Reset all states if any parameter has changed
  int ResetIfNeeded(int inFreq, int outFreq, size_t num_channels);

  // Resample samplesIn to samplesOut.
  int Push(const int16_t* samplesIn,
           size_t lengthIn,
           int16_t* samplesOut,
           size_t maxLen,
           size_t& outLen);  // NOLINT: to avoid changing APIs

 private:
  enum ResamplerMode {
    kResamplerMode1To1,
    kResamplerMode1To2,
    kResamplerMode1To3,
    kResamplerMode1To4,
    kResamplerMode1To6,
    kResamplerMode1To12,
    kResamplerMode2To3,
    kResamplerMode2To11,
    kResamplerMode4To11,
    kResamplerMode8To11,
    kResamplerMode11To16,
    kResamplerMode11To32,
    kResamplerMode2To1,
    kResamplerMode3To1,
    kResamplerMode4To1,
    kResamplerMode6To1,
    kResamplerMode12To1,
    kResamplerMode3To2,
    kResamplerMode11To2,
    kResamplerMode11To4,
    kResamplerMode11To8
  };

  // Computes the resampler mode for a given sampling frequency pair.
  // Returns -1 for unsupported frequency pairs.
  static int ComputeResamplerMode(int in_freq_hz,
                                  int out_freq_hz,
                                  ResamplerMode* mode);

  // Generic pointers since we don't know what states we'll need
  void* state1_;
  void* state2_;
  void* state3_;

  // Storage if needed
  int16_t* in_buffer_;
  int16_t* out_buffer_;
  size_t in_buffer_size_;
  size_t out_buffer_size_;
  size_t in_buffer_size_max_;
  size_t out_buffer_size_max_;

  int my_in_frequency_khz_;
  int my_out_frequency_khz_;
  ResamplerMode my_mode_;
  size_t num_channels_;

  // Extra instance for stereo
  Resampler* helper_left_;
  Resampler* helper_right_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_RESAMPLER_INCLUDE_RESAMPLER_H_
