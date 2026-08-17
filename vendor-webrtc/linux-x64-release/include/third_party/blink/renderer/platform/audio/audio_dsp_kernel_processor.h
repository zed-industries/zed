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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_PROCESSOR_H_

#include <memory>

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/audio/audio_processor.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioBus;
class AudioDSPKernel;
class AudioProcessor;

// AudioDSPKernelProcessor processes one input -> one output (N channels each)
// It uses one AudioDSPKernel object per channel to do the processing, thus
// there is no cross-channel processing.  Despite this limitation it turns out
// to be a very common and useful type of processor.

class PLATFORM_EXPORT AudioDSPKernelProcessor : public AudioProcessor {
 public:
  // numberOfChannels may be later changed if object is not yet in an
  // "initialized" state
  AudioDSPKernelProcessor(float sample_rate,
                          unsigned number_of_channels,
                          unsigned render_quantum_frames);

  // Subclasses create the appropriate type of processing kernel here.
  // We'll call this to create a kernel for each channel.
  virtual std::unique_ptr<AudioDSPKernel> CreateKernel() = 0;

  // AudioProcessor methods
  void Initialize() override;
  void Uninitialize() override;
  void Process(const AudioBus* source,
               AudioBus* destination,
               uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void Reset() override;
  void SetNumberOfChannels(unsigned) override;
  unsigned NumberOfChannels() const override { return number_of_channels_; }

  double TailTime() const override;
  double LatencyTime() const override;
  bool RequiresTailProcessing() const override;

 protected:
  Vector<std::unique_ptr<AudioDSPKernel>> kernels_ GUARDED_BY(process_lock_);
  mutable base::Lock process_lock_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DSP_KERNEL_PROCESSOR_H_
