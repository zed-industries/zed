/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_RESAMPLER_PUSH_SINC_RESAMPLER_H_
#define COMMON_AUDIO_RESAMPLER_PUSH_SINC_RESAMPLER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "api/audio/audio_view.h"
#include "common_audio/resampler/sinc_resampler.h"

namespace webrtc {

// A thin wrapper over SincResampler to provide a push-based interface as
// required by WebRTC. SincResampler uses a pull-based interface, and will
// use SincResamplerCallback::Run() to request data upon a call to Resample().
// These Run() calls will happen on the same thread Resample() is called on.
class PushSincResampler : public SincResamplerCallback {
 public:
  // Provide the size of the source and destination blocks in samples. These
  // must correspond to the same time duration (typically 10 ms) as the sample
  // ratio is inferred from them.
  PushSincResampler(size_t source_frames, size_t destination_frames);
  ~PushSincResampler() override;

  PushSincResampler(const PushSincResampler&) = delete;
  PushSincResampler& operator=(const PushSincResampler&) = delete;

  // Perform the resampling. `source_frames` must always equal the
  // `source_frames` provided at construction. `destination_capacity` must be
  // at least as large as `destination_frames`. Returns the number of samples
  // provided in destination (for convenience, since this will always be equal
  // to `destination_frames`).
  template <typename S, typename D>
  size_t Resample(const MonoView<S>& source, const MonoView<D>& destination) {
    return Resample(&source[0], SamplesPerChannel(source), &destination[0],
                    SamplesPerChannel(destination));
  }

  size_t Resample(const int16_t* source,
                  size_t source_frames,
                  int16_t* destination,
                  size_t destination_capacity);
  size_t Resample(const float* source,
                  size_t source_frames,
                  float* destination,
                  size_t destination_capacity);

  // Delay due to the filter kernel. Essentially, the time after which an input
  // sample will appear in the resampled output.
  static float AlgorithmicDelaySeconds(int source_rate_hz) {
    return 1.f / source_rate_hz * SincResampler::kKernelSize / 2;
  }

 protected:
  // Implements SincResamplerCallback.
  void Run(size_t frames, float* destination) override;

 private:
  friend class PushSincResamplerTest;
  SincResampler* get_resampler_for_testing() { return resampler_.get(); }

  std::unique_ptr<SincResampler> resampler_;
  std::unique_ptr<float[]> float_buffer_;
  const float* source_ptr_;
  const int16_t* source_ptr_int_;
  const size_t destination_frames_;

  // True on the first call to Resample(), to prime the SincResampler buffer.
  bool first_pass_;

  // Used to assert we are only requested for as much data as is available.
  size_t source_available_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_RESAMPLER_PUSH_SINC_RESAMPLER_H_
