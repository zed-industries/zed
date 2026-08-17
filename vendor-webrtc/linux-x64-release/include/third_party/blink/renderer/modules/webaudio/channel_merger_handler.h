// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_MERGER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_MERGER_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class AudioNode;
class ExceptionState;

class ChannelMergerHandler final : public AudioHandler {
 public:
  static scoped_refptr<ChannelMergerHandler> Create(AudioNode&,
                                                    float sample_rate,
                                                    unsigned number_of_inputs);

  void Process(uint32_t frames_to_process) override;
  void SetChannelCount(unsigned, ExceptionState&) final;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) final;

  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const final { return false; }

 private:
  ChannelMergerHandler(AudioNode&,
                       float sample_rate,
                       unsigned number_of_inputs);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_MERGER_HANDLER_H_
