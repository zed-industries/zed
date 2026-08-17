// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DYNAMICS_COMPRESSOR_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DYNAMICS_COMPRESSOR_HANDLER_H_

#include <atomic>
#include <memory>

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"

namespace blink {

class BaseAudioContext;
class DynamicsCompressor;
class DynamicsCompressorOptions;

class MODULES_EXPORT DynamicsCompressorHandler final : public AudioHandler {
 public:
  static scoped_refptr<DynamicsCompressorHandler> Create(
      AudioNode&,
      float sample_rate,
      AudioParamHandler& threshold,
      AudioParamHandler& knee,
      AudioParamHandler& ratio,
      AudioParamHandler& attack,
      AudioParamHandler& release);

  ~DynamicsCompressorHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override;
  void Initialize() override;

  float ReductionValue() const {
    return reduction_.load(std::memory_order_relaxed);
  }

  void SetChannelCount(unsigned, ExceptionState&) final;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) final;

 private:
  DynamicsCompressorHandler(AudioNode&,
                            float sample_rate,
                            AudioParamHandler& threshold,
                            AudioParamHandler& knee,
                            AudioParamHandler& ratio,
                            AudioParamHandler& attack,
                            AudioParamHandler& release);
  bool RequiresTailProcessing() const final;
  double TailTime() const override;
  double LatencyTime() const override;

  std::unique_ptr<DynamicsCompressor> dynamics_compressor_;
  scoped_refptr<AudioParamHandler> threshold_;
  scoped_refptr<AudioParamHandler> knee_;
  scoped_refptr<AudioParamHandler> ratio_;
  std::atomic<float> reduction_;
  scoped_refptr<AudioParamHandler> attack_;
  scoped_refptr<AudioParamHandler> release_;

  FRIEND_TEST_ALL_PREFIXES(DynamicsCompressorNodeTest, ProcessorLifetime);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_DYNAMICS_COMPRESSOR_HANDLER_H_
