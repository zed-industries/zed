/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DOWN_SAMPLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DOWN_SAMPLER_H_

#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/audio/simple_fft_convolver.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// DownSampler down-samples the source stream by a factor of 2x.

class PLATFORM_EXPORT DownSampler final {
  USING_FAST_MALLOC(DownSampler);

 public:
  explicit DownSampler(unsigned input_block_size);
  DownSampler(const DownSampler&) = delete;
  DownSampler& operator=(const DownSampler&) = delete;

  // The destination buffer |destP| is of size sourceFramesToProcess / 2.
  void Process(const float* source_p,
               float* dest_p,
               uint32_t source_frames_to_process);

  void Reset();

  // Latency based on the destination sample-rate.
  size_t LatencyFrames() const;

 private:
  enum { kDefaultKernelSize = 256 };

  unsigned input_block_size_;

  // Half-band filter. SimpleFFTConvolver is always faster than DirectConvolver.
  SimpleFFTConvolver convolver_;

  AudioFloatArray temp_buffer_;

  // Used as delay-line (FIR filter history) for the input samples to account
  // for the 0.5 term right in the middle of the kernel.
  AudioFloatArray input_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DOWN_SAMPLER_H_
