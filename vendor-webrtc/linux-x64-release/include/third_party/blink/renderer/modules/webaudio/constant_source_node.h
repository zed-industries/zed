// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_NODE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/audio_scheduled_source_node.h"

namespace blink {

class BaseAudioContext;
class ConstantSourceHandler;
class ConstantSourceOptions;
class ExceptionState;

// ConstantSourceNode is an audio generator for a constant source
class ConstantSourceNode final : public AudioScheduledSourceNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ConstantSourceNode* Create(BaseAudioContext&, ExceptionState&);
  static ConstantSourceNode* Create(BaseAudioContext*,
                                    const ConstantSourceOptions*,
                                    ExceptionState&);

  explicit ConstantSourceNode(BaseAudioContext&);
  void Trace(Visitor*) const override;

  AudioParam* offset();

  ConstantSourceHandler& GetConstantSourceHandler() const;

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  Member<AudioParam> offset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONSTANT_SOURCE_NODE_H_
