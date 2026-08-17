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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_KERNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_KERNEL_H_

#include <memory>
#include <utility>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/platform/audio/fft_frame.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioChannel;

// HRTF stands for Head-Related Transfer Function.
// HRTFKernel is a frequency-domain representation of an impulse-response used
// as part of the spatialized panning system.  For a given azimuth / elevation
// angle there will be one HRTFKernel for the left ear transfer function, and
// one for the right ear.  The leading delay (average group delay) for each
// impulse response is extracted:
//      m_fftFrame is the frequency-domain representation of the impulse
//      response with the delay removed
//      m_frameDelay is the leading delay of the original impulse response.
class HRTFKernel final {
  USING_FAST_MALLOC(HRTFKernel);

 public:
  // Note: this is destructive on the passed in AudioChannel.
  // The length of channel must be a power of two.
  HRTFKernel(AudioChannel*, unsigned fft_size, float sample_rate);
  HRTFKernel(std::unique_ptr<FFTFrame> fft_frame,
             float frame_delay,
             float sample_rate)
      : fft_frame_(std::move(fft_frame)),
        frame_delay_(frame_delay),
        sample_rate_(sample_rate) {}
  HRTFKernel(const HRTFKernel&) = delete;
  HRTFKernel& operator=(const HRTFKernel&) = delete;

  // Given two HRTFKernels, and an interpolation factor x: 0 -> 1, returns an
  // interpolated HRTFKernel.
  static std::unique_ptr<HRTFKernel>
  CreateInterpolatedKernel(HRTFKernel* kernel1, HRTFKernel* kernel2, float x);

  FFTFrame* FftFrame() { return fft_frame_.get(); }
  float FrameDelay() const { return frame_delay_; }

 private:
  std::unique_ptr<FFTFrame> fft_frame_;
  float frame_delay_;
  const float sample_rate_;
};

typedef Vector<std::unique_ptr<HRTFKernel>> HRTFKernelList;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_KERNEL_H_
