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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_STAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_STAGE_H_

#include <memory>
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/audio/fft_frame.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ReverbAccumulationBuffer;
class ReverbConvolver;
class FFTConvolver;
class DirectConvolver;

// A ReverbConvolverStage represents the convolution associated with a
// sub-section of a large impulse response.  It incorporates a delay line to
// account for the offset of the sub-section within the larger impulse
// response.
class ReverbConvolverStage final {
  USING_FAST_MALLOC(ReverbConvolverStage);

 public:
  // renderPhase is useful to know so that we can manipulate the pre versus post
  // delay so that stages will perform their heavy work (FFT processing) on
  // different slices to balance the load in a real-time thread.
  ReverbConvolverStage(const float* impulse_response,
                       size_t response_length,
                       size_t reverb_total_latency,
                       size_t stage_offset,
                       unsigned stage_length,
                       unsigned fft_size,
                       size_t render_phase,
                       unsigned render_slice_size,
                       ReverbAccumulationBuffer*,
                       float scale,
                       bool direct_mode = false);
  ReverbConvolverStage(const ReverbConvolverStage&) = delete;
  ReverbConvolverStage& operator=(const ReverbConvolverStage&) = delete;

  // WARNING: framesToProcess must be such that it evenly divides the delay
  // buffer size (stage_offset).
  void Process(const float* source, uint32_t frames_to_process);

  void ProcessInBackground(ReverbConvolver* convolver,
                           uint32_t frames_to_process);

  void Reset();

  // Useful for background processing
  size_t InputReadIndex() const { return input_read_index_; }

 private:
  std::unique_ptr<FFTFrame> fft_kernel_;
  std::unique_ptr<FFTConvolver> fft_convolver_;

  AudioFloatArray pre_delay_buffer_;

  raw_ptr<ReverbAccumulationBuffer> accumulation_buffer_;
  uint32_t accumulation_read_index_;
  size_t input_read_index_;

  size_t pre_delay_length_;
  size_t post_delay_length_;
  size_t pre_read_write_index_;
  size_t frames_processed_;

  AudioFloatArray temporary_buffer_;

  bool direct_mode_;
  std::unique_ptr<DirectConvolver> direct_convolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_STAGE_H_
