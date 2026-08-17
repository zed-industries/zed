// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/audio_scheduled_source_node.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"

namespace blink {

class AudioNode;

class ConstantSourceHandler final : public AudioScheduledSourceHandler {
 public:
  static scoped_refptr<ConstantSourceHandler> Create(AudioNode&,
                                                     float sample_rate,
                                                     AudioParamHandler& offset);

  ConstantSourceHandler(const ConstantSourceHandler&) = delete;
  ConstantSourceHandler& operator=(const ConstantSourceHandler&) = delete;

  ~ConstantSourceHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  // If we are no longer playing, propagate silence ahead to downstream nodes.
  bool PropagatesSilence() const override;

  // AudioScheduledSourceHandler
  void HandleStoppableSourceNode() override;

 private:
  ConstantSourceHandler(AudioNode&,
                        float sample_rate,
                        AudioParamHandler& offset);

  base::WeakPtr<AudioScheduledSourceHandler> AsWeakPtr() override;

  scoped_refptr<AudioParamHandler> offset_;
  AudioFloatArray sample_accurate_values_;

  base::WeakPtrFactory<AudioScheduledSourceHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_HANDLER_H_
