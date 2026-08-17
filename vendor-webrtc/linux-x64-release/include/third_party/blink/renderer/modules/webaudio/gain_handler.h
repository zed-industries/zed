// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_GAIN_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_GAIN_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"

namespace blink {

class AudioNode;
class AudioNodeInput;

class GainHandler final : public AudioHandler {
 public:
  static scoped_refptr<GainHandler> Create(AudioNode&,
                                           float sample_rate,
                                           AudioParamHandler& gain);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;

  // Called in the main thread when the number of channels for the input may
  // have changed.
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;

  // AudioNode
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const final { return false; }

 private:
  GainHandler(AudioNode&, float sample_rate, AudioParamHandler& gain);

  scoped_refptr<AudioParamHandler> gain_;

  AudioFloatArray sample_accurate_gain_values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_GAIN_HANDLER_H_
