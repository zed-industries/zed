// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_HANDLER_H_

#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_handler.h"
#include "third_party/blink/renderer/modules/webaudio/realtime_analyser.h"

namespace blink {

class AudioNode;
class ExceptionState;

class AnalyserHandler final : public AudioHandler {
 public:
  static scoped_refptr<AnalyserHandler> Create(AudioNode&, float sample_rate);
  ~AnalyserHandler() override;

  unsigned FftSize() const { return analyser_.FftSize(); }
  void SetFftSize(unsigned size, ExceptionState&);

  unsigned FrequencyBinCount() const { return analyser_.FrequencyBinCount(); }

  double MinDecibels() const { return analyser_.MinDecibels(); }
  void SetMinDecibels(double k, ExceptionState&);

  double MaxDecibels() const { return analyser_.MaxDecibels(); }
  void SetMaxDecibels(double k, ExceptionState&);

  void SetMinMaxDecibels(double min, double max, ExceptionState&);

  double SmoothingTimeConstant() const {
    return analyser_.SmoothingTimeConstant();
  }
  void SetSmoothingTimeConstant(double k, ExceptionState&);

  void GetFloatFrequencyData(DOMFloat32Array* array, double current_time) {
    analyser_.GetFloatFrequencyData(array, current_time);
  }
  void GetByteFrequencyData(DOMUint8Array* array, double current_time) {
    analyser_.GetByteFrequencyData(array, current_time);
  }
  void GetFloatTimeDomainData(DOMFloat32Array* array) {
    analyser_.GetFloatTimeDomainData(array);
  }
  void GetByteTimeDomainData(DOMUint8Array* array) {
    analyser_.GetByteTimeDomainData(array);
  }

 private:
  AnalyserHandler(AudioNode&, float sample_rate);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;
  bool RequiresTailProcessing() const override;
  double TailTime() const override;
  double LatencyTime() const override { return 0; }
  bool PropagatesSilence() const override {
    // An AnalyserNode does actually propagate silence, but to get the
    // time and FFT data updated correctly, process() needs to be
    // called even if all the inputs are silent.
    return false;
  }
  void PullInputs(uint32_t frames_to_process) override;
  void UpdatePullStatusIfNeeded() override;

  RealtimeAnalyser analyser_;

  // When setting to true, handler will be pulled automatically by
  // BaseAudioContext before the end of each render quantum.
  bool need_automatic_pull_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_HANDLER_H_
