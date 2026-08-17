// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_SPLITTER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_SPLITTER_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class ExceptionState;

class ChannelSplitterHandler final : public AudioHandler {
 public:
  static scoped_refptr<ChannelSplitterHandler>
  Create(AudioNode&, float sample_rate, unsigned number_of_outputs);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void SetChannelCount(unsigned, ExceptionState&) final;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) final;
  void SetChannelInterpretation(V8ChannelInterpretation::Enum,
                                ExceptionState&) final;

  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool RequiresTailProcessing() const final { return false; }

 private:
  ChannelSplitterHandler(AudioNode&,
                         float sample_rate,
                         unsigned number_of_outputs);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CHANNEL_SPLITTER_HANDLER_H_
