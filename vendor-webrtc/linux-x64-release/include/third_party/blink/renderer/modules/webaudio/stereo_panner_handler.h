// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_HANDLER_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/audio/stereo_panner.h"

namespace blink {

class BaseAudioContext;
class StereoPannerOptions;

class StereoPannerHandler final : public AudioHandler {
 public:
  static scoped_refptr<StereoPannerHandler> Create(AudioNode&,
                                                   float sample_rate,
                                                   AudioParamHandler& pan);
  ~StereoPannerHandler() override;

  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void Initialize() override;

  void SetChannelCount(unsigned, ExceptionState&) final;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) final;

  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const final { return false; }

 private:
  StereoPannerHandler(AudioNode&, float sample_rate, AudioParamHandler& pan);

  std::unique_ptr<StereoPanner> stereo_panner_;
  scoped_refptr<AudioParamHandler> pan_;

  AudioFloatArray sample_accurate_pan_values_;

  FRIEND_TEST_ALL_PREFIXES(StereoPannerNodeTest, StereoPannerLifetime);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_HANDLER_H_
