/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_H_

#include "third_party/blink/renderer/modules/webaudio/audio_listener_handler.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/inspector_helper_mixin.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/vector3d_f.h"

namespace blink {

// This interface represents the position and orientation of the person
// listening to the audio scene. All PannerNode objects spatialize in relation
// to the BaseAudioContext's listener.
//
// Spec: https://www.w3.org/TR/webaudio/#AudioListener
class AudioListener final : public ScriptWrappable,
                            public InspectorHelperMixin {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit AudioListener(BaseAudioContext&);
  ~AudioListener() final;

  AudioListenerHandler& Handler() const { return *handler_; }

  // https://www.w3.org/TR/webaudio/#AudioListener-attributes
  AudioParam* positionX() const { return position_x_.Get(); }
  AudioParam* positionY() const { return position_y_.Get(); }
  AudioParam* positionZ() const { return position_z_.Get(); }
  AudioParam* forwardX() const { return forward_x_.Get(); }
  AudioParam* forwardY() const { return forward_y_.Get(); }
  AudioParam* forwardZ() const { return forward_z_.Get(); }
  AudioParam* upX() const { return up_x_.Get(); }
  AudioParam* upY() const { return up_y_.Get(); }
  AudioParam* upZ() const { return up_z_.Get(); }

  // https://www.w3.org/TR/webaudio/#AudioListener-methods
  void setOrientation(float x, float y, float z,
                      float up_x, float up_y, float up_z,
                      ExceptionState& exceptionState);
  void setPosition(float x, float y, float z, ExceptionState& exceptionState);

  // InspectorHelperMixin: Note that this object belongs to a BaseAudioContext,
  // so these methods get called by the parent context.
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

  void Trace(Visitor*) const override;

 private:
  void SetHandler(scoped_refptr<AudioListenerHandler>);

  void SetPosition(const gfx::Point3F&, ExceptionState&);
  void SetOrientation(const gfx::Vector3dF&, ExceptionState&);
  void SetUpVector(const gfx::Vector3dF&, ExceptionState&);

  scoped_refptr<AudioListenerHandler> handler_;
  scoped_refptr<DeferredTaskHandler> deferred_task_handler_;

  Member<AudioParam> position_x_;
  Member<AudioParam> position_y_;
  Member<AudioParam> position_z_;
  Member<AudioParam> forward_x_;
  Member<AudioParam> forward_y_;
  Member<AudioParam> forward_z_;
  Member<AudioParam> up_x_;
  Member<AudioParam> up_y_;
  Member<AudioParam> up_z_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_LISTENER_H_
