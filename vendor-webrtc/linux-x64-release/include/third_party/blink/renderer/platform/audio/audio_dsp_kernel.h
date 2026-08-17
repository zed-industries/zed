/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/audio/audio_dsp_kernel_processor.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// AudioDSPKernel does the processing for one channel of an
// AudioDSPKernelProcessor.

class PLATFORM_EXPORT AudioDSPKernel {
  USING_FAST_MALLOC(AudioDSPKernel);

 public:
  explicit AudioDSPKernel(AudioDSPKernelProcessor* kernel_processor)
      : kernel_processor_(kernel_processor),
        sample_rate_(kernel_processor->SampleRate()),
        render_quantum_frames_(kernel_processor->RenderQuantumFrames()) {}

  explicit AudioDSPKernel(float sample_rate, unsigned render_quantum_frames)
      : kernel_processor_(nullptr),
        sample_rate_(sample_rate),
        render_quantum_frames_(render_quantum_frames) {}

  virtual ~AudioDSPKernel();

  // Subclasses must override process() to do the processing and reset() to
  // reset DSP state.
  virtual void Process(const float* source,
                       float* destination,
                       uint32_t frames_to_process) = 0;
  // Subclasses that have AudioParams must override this to process the
  // AudioParams.
  virtual void ProcessOnlyAudioParams(uint32_t frames_to_process) {}
  virtual void Reset() = 0;

  float SampleRate() const { return sample_rate_; }
  unsigned RenderQuantumFrames() const { return render_quantum_frames_; }
  double Nyquist() const { return 0.5 * SampleRate(); }

  AudioDSPKernelProcessor* Processor() { return kernel_processor_; }
  const AudioDSPKernelProcessor* Processor() const { return kernel_processor_; }

  virtual double TailTime() const = 0;
  virtual double LatencyTime() const = 0;
  virtual bool RequiresTailProcessing() const = 0;

 protected:
  // This raw pointer is safe because the AudioDSPKernelProcessor object is
  // guaranteed to be kept alive while the AudioDSPKernel object is alive.
  raw_ptr<AudioDSPKernelProcessor> kernel_processor_;
  float sample_rate_;
  unsigned render_quantum_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_H_
