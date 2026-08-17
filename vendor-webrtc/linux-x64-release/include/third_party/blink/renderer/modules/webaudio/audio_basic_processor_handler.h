/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BASIC_PROCESSOR_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BASIC_PROCESSOR_HANDLER_H_

#include <memory>

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class AudioNodeInput;
class AudioProcessor;

// AudioBasicProcessorHandler is an AudioHandler with one input and one output
// where the input and output have the same number of channels.
class MODULES_EXPORT AudioBasicProcessorHandler : public AudioHandler {
 public:
  ~AudioBasicProcessorHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) final;
  void PullInputs(uint32_t frames_to_process) final;
  void Initialize() final;
  void Uninitialize() final;

  // Called in the main thread when the number of channels for the input may
  // have changed.
  void CheckNumberOfChannelsForInput(AudioNodeInput*) final;

  // Returns the number of channels for both the input and the output.
  unsigned NumberOfChannels();
  AudioProcessor* Processor() { return processor_.get(); }

 protected:
  AudioBasicProcessorHandler(NodeType,
                             AudioNode&,
                             float sample_rate,
                             std::unique_ptr<AudioProcessor>);

  // Returns true if the first output sample of any channel is non-finite.  This
  // is a proxy for determining if the filter state is bad.  For
  // BiquadFilterNodes and IIRFilterNodes, if the internal state has non-finite
  // values, the non-finite value propagates pretty much forever in the output.
  // This is because infinities and NaNs are sticky.
  bool HasNonFiniteOutput() const;

 private:
  bool RequiresTailProcessing() const final;
  double TailTime() const final;
  double LatencyTime() const final;

  std::unique_ptr<AudioProcessor> processor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BASIC_PROCESSOR_HANDLER_H_
