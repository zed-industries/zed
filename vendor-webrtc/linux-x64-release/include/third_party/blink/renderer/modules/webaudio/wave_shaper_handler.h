// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_WAVE_SHAPER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_WAVE_SHAPER_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_over_sample_type.h"
#include "third_party/blink/renderer/modules/webaudio/audio_basic_processor_handler.h"

namespace blink {

class AudioNode;
class AudioProcessor;
class WaveShaperProcessor;

// WaveShaperHandler implements non-linear distortion effects.
class WaveShaperHandler final : public AudioHandler {
 public:
  static scoped_refptr<WaveShaperHandler> Create(AudioNode&, float sample_rate);

  void SetCurve(const float* curve_data, unsigned curve_length);
  const Vector<float>* Curve() const;
  void SetOversample(V8OverSampleType::Enum oversample);
  V8OverSampleType::Enum Oversample() const;

 private:
  WaveShaperHandler(AudioNode& iirfilter_node, float sample_rate);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void Initialize() override;
  void Uninitialize() override;
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;
  bool RequiresTailProcessing() const override;
  double TailTime() const override;
  double LatencyTime() const override;
  void PullInputs(uint32_t frames_to_process) override;

  unsigned NumberOfChannels();
  AudioProcessor* Processor() { return processor_.get(); }
  const AudioProcessor* Processor() const { return processor_.get(); }
  WaveShaperProcessor* GetWaveShaperProcessor();
  const WaveShaperProcessor* GetWaveShaperProcessor() const;

  std::unique_ptr<AudioProcessor> processor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_WAVE_SHAPER_HANDLER_H_
