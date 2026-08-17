/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_NODE_H_

#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class AudioListener;
class AudioParam;
class BaseAudioContext;
class PannerHandler;
class PannerOptions;
class V8DistanceModelType;
class V8PanningModelType;

// PannerNode is an AudioNode with one input and one output.
// It positions a sound in 3D space, with the exact effect dependent on the
// panning model.  It has a position and an orientation in 3D space which is
// relative to the position and orientation of the context's AudioListener.  A
// distance effect will attenuate the gain as the position moves away from the
// listener.  A cone effect will attenuate the gain as the orientation moves
// away from the listener.  All of these effects follow the OpenAL specification
// very closely.
class PannerNode final : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PannerNode* Create(BaseAudioContext&, ExceptionState&);
  static PannerNode* Create(BaseAudioContext*,
                            const PannerOptions*,
                            ExceptionState&);
  PannerHandler& GetPannerHandler() const;

  explicit PannerNode(BaseAudioContext&);

  void Trace(Visitor*) const override;

  // Uses a 3D cartesian coordinate system
  AudioParam* positionX() const { return position_x_.Get(); }
  AudioParam* positionY() const { return position_y_.Get(); }
  AudioParam* positionZ() const { return position_z_.Get(); }

  AudioParam* orientationX() const { return orientation_x_.Get(); }
  AudioParam* orientationY() const { return orientation_y_.Get(); }
  AudioParam* orientationZ() const { return orientation_z_.Get(); }

  V8PanningModelType panningModel() const;
  void setPanningModel(const V8PanningModelType&);
  void setPosition(float x, float y, float z, ExceptionState&);
  void setOrientation(float x, float y, float z, ExceptionState&);
  V8DistanceModelType distanceModel() const;
  void setDistanceModel(const V8DistanceModelType&);
  double refDistance() const;
  void setRefDistance(double, ExceptionState&);
  double maxDistance() const;
  void setMaxDistance(double, ExceptionState&);
  double rolloffFactor() const;
  void setRolloffFactor(double, ExceptionState&);
  double coneInnerAngle() const;
  void setConeInnerAngle(double);
  double coneOuterAngle() const;
  void setConeOuterAngle(double);
  double coneOuterGain() const;
  void setConeOuterGain(double, ExceptionState&);

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  Member<AudioParam> position_x_;
  Member<AudioParam> position_y_;
  Member<AudioParam> position_z_;

  Member<AudioParam> orientation_x_;
  Member<AudioParam> orientation_y_;
  Member<AudioParam> orientation_z_;

  // This listener is held alive here to allow referencing it from PannerHandler
  // via weak reference.
  Member<AudioListener> listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PANNER_NODE_H_
