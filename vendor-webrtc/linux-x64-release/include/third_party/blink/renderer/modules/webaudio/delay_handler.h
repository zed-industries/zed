// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DELAY_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DELAY_HANDLER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_handler.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class AudioParamHandler;
class AudioNodeInput;
class Delay;

class DelayHandler final : public AudioHandler {
 public:
  static scoped_refptr<DelayHandler> Create(AudioNode&,
                                            float sample_rate,
                                            AudioParamHandler& delay_time,
                                            double max_delay_time);

  ~DelayHandler() override;

 private:
  DelayHandler(AudioNode&,
               float sample_rate,
               AudioParamHandler& delay_time,
               double max_delay_time);

  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;

  void Initialize() override;
  void Uninitialize() override;

  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;

  bool RequiresTailProcessing() const override;
  double TailTime() const override;
  double LatencyTime() const override;

  void PullInputs(uint32_t frames_to_process) override;

  unsigned number_of_channels_;
  float sample_rate_;
  unsigned render_quantum_frames_;

  Vector<std::unique_ptr<Delay>> kernels_ GUARDED_BY(process_lock_);
  mutable base::Lock process_lock_;

  scoped_refptr<AudioParamHandler> delay_time_;
  double max_delay_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DELAY_HANDLER_H_
