/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_FFT_CONVOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_FFT_CONVOLVER_H_

#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/audio/fft_frame.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FFTConvolver final {
  USING_FAST_MALLOC(FFTConvolver);

 public:
  // fftSize must be a power of two
  explicit FFTConvolver(unsigned fft_size);
  FFTConvolver(const FFTConvolver&) = delete;
  FFTConvolver& operator=(const FFTConvolver&) = delete;

  // For now, with multiple calls to Process(), framesToProcess MUST add up
  // EXACTLY to fftSize / 2
  //
  // FIXME: Later, we can do more sophisticated buffering to relax this
  // requirement...
  //
  // The input to output latency is equal to fftSize / 2
  //
  // Processing in-place is allowed...
  void Process(const FFTFrame* fft_kernel,
               const float* source_p,
               float* dest_p,
               uint32_t frames_to_process);

  void Reset();

  unsigned FftSize() const { return frame_.FftSize(); }

 private:
  FFTFrame frame_;

  // Buffer input until we get fftSize / 2 samples then do an FFT
  size_t read_write_index_;
  AudioFloatArray input_buffer_;

  // Stores output which we read a little at a time
  AudioFloatArray output_buffer_;

  // Saves the 2nd half of the FFT buffer, so we can do an overlap-add with the
  // 1st half of the next one
  AudioFloatArray last_overlap_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_FFT_CONVOLVER_H_
