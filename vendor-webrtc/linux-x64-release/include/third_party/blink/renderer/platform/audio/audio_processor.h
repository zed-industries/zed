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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_PROCESSOR_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AudioBus;

// AudioProcessor is an abstract base class representing an audio signal
// processing object with a single input and a single output, where the number
// of input channels equals the number of output channels.  It can be used as
// one part of a complex DSP algorithm, or as the processor for a basic (one
// input - one output) AudioNode.

class PLATFORM_EXPORT AudioProcessor {
  USING_FAST_MALLOC(AudioProcessor);

 public:
  AudioProcessor(float sample_rate,
                 unsigned number_of_channels,
                 unsigned render_quantum_frames)
      : number_of_channels_(number_of_channels),
        sample_rate_(sample_rate),
        render_quantum_frames_(render_quantum_frames) {}

  virtual ~AudioProcessor();

  // Full initialization can be done here instead of in the constructor.
  virtual void Initialize() = 0;
  virtual void Uninitialize() = 0;

  // Processes the source to destination bus.  The number of channels must match
  // in source and destination.
  virtual void Process(const AudioBus* source,
                       AudioBus* destination,
                       uint32_t frames_to_process) = 0;

  // Forces all AudioParams in the processor to run the timeline,
  // bypassing any other processing the processor would do in
  // process().
  virtual void ProcessOnlyAudioParams(uint32_t frames_to_process) {}

  // Resets filter state
  virtual void Reset() = 0;

  virtual void SetNumberOfChannels(unsigned) = 0;
  virtual unsigned NumberOfChannels() const = 0;

  bool IsInitialized() const { return initialized_; }

  float SampleRate() const { return sample_rate_; }

  unsigned RenderQuantumFrames() const { return render_quantum_frames_; }

  virtual double TailTime() const = 0;
  virtual double LatencyTime() const = 0;
  virtual bool RequiresTailProcessing() const = 0;

 protected:
  bool initialized_ = false;
  unsigned number_of_channels_;
  float sample_rate_;
  unsigned render_quantum_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_PROCESSOR_H_
