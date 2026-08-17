// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_NODE_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/stereo_panner_handler.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/audio/stereo_panner.h"

namespace blink {

class BaseAudioContext;
class StereoPannerOptions;

// StereoPannerNode is an AudioNode with one input and one output. It is
// specifically designed for equal-power stereo panning.
class StereoPannerNode final : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static StereoPannerNode* Create(BaseAudioContext&, ExceptionState&);
  static StereoPannerNode* Create(BaseAudioContext*,
                                  const StereoPannerOptions*,
                                  ExceptionState&);

  explicit StereoPannerNode(BaseAudioContext&);

  void Trace(Visitor*) const override;

  AudioParam* pan() const;

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  Member<AudioParam> pan_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_STEREO_PANNER_NODE_H_
