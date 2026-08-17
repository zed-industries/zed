/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_RESAMPLER_INCLUDE_PUSH_RESAMPLER_H_
#define COMMON_AUDIO_RESAMPLER_INCLUDE_PUSH_RESAMPLER_H_

#include <memory>
#include <vector>

#include "api/audio/audio_view.h"

namespace webrtc {

class PushSincResampler;

// Wraps PushSincResampler to provide stereo support.
// Note: This implementation assumes 10ms buffer sizes throughout.
template <typename T>
class PushResampler final {
 public:
  PushResampler();
  PushResampler(size_t src_samples_per_channel,
                size_t dst_samples_per_channel,
                size_t num_channels);
  ~PushResampler();

  // Returns the total number of samples provided in destination (e.g. 32 kHz,
  // 2 channel audio gives 640 samples).
  int Resample(InterleavedView<const T> src, InterleavedView<T> dst);
  // For when a deinterleaved/mono channel already exists and we can skip the
  // deinterleaved operation.
  int Resample(MonoView<const T> src, MonoView<T> dst);

 private:
  // Ensures that source and destination buffers for deinterleaving are
  // correctly configured prior to resampling that requires deinterleaving.
  void EnsureInitialized(size_t src_samples_per_channel,
                         size_t dst_samples_per_channel,
                         size_t num_channels);

  // Buffers used for when a deinterleaving step is necessary.
  std::unique_ptr<T[]> source_;
  std::unique_ptr<T[]> destination_;
  DeinterleavedView<T> source_view_;
  DeinterleavedView<T> destination_view_;

  std::vector<std::unique_ptr<PushSincResampler>> resamplers_;
};
}  // namespace webrtc

#endif  // COMMON_AUDIO_RESAMPLER_INCLUDE_PUSH_RESAMPLER_H_
