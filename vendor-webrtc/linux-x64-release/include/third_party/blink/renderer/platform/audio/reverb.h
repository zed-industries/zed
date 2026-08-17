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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_H_

#include <memory>
#include "third_party/blink/renderer/platform/audio/reverb_convolver.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioBus;

// Multi-channel convolution reverb with channel matrixing - one or more
// ReverbConvolver objects are used internally.

class PLATFORM_EXPORT Reverb final {
  USING_FAST_MALLOC(Reverb);

 public:
  // |render_slice_size| is a rendering hint, so the FFTs can be optimized to
  // not all occur at the same time (very bad when rendering on a real-time
  // thread).
  Reverb(AudioBus* impulse_response_buffer,
         unsigned render_slice_size,
         unsigned max_fft_size,
         bool use_background_threads,
         bool normalize);
  Reverb(const Reverb&) = delete;
  Reverb& operator=(const Reverb&) = delete;

  void Process(const AudioBus* source_bus,
               AudioBus* destination_bus,
               uint32_t frames_to_process);
  void Reset();

  size_t ImpulseResponseLength() const { return impulse_response_length_; }
  size_t LatencyFrames() const;

 private:
  void Initialize(AudioBus* impulse_response_buffer,
                  unsigned render_slice_size,
                  unsigned max_fft_size,
                  bool use_background_threads,
                  float scale);

  size_t impulse_response_length_;
  // The actual number of channels in the response.  This can be less
  // than the number of ReverbConvolver's in |m_convolvers|.
  unsigned number_of_response_channels_;

  Vector<std::unique_ptr<ReverbConvolver>> convolvers_;

  // For "True" stereo processing
  scoped_refptr<AudioBus> temp_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_H_
