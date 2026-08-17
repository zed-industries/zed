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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_H_

#include <sys/types.h>

#include <atomic>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param_handler.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param_timeline.h"
#include "third_party/blink/renderer/modules/webaudio/audio_summing_junction.h"
#include "third_party/blink/renderer/modules/webaudio/base_audio_context.h"
#include "third_party/blink/renderer/modules/webaudio/inspector_helper_mixin.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class AudioNodeOutput;
class V8AutomationRate;

// AudioParam class represents web-exposed AudioParam interface.
class AudioParam final : public ScriptWrappable, public InspectorHelperMixin {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioParam* Create(
      BaseAudioContext&,
      const String& parent_uuid,
      AudioParamHandler::AudioParamType,
      double default_value,
      AudioParamHandler::AutomationRate rate,
      AudioParamHandler::AutomationRateMode rate_mode,
      float min_value = -std::numeric_limits<float>::max(),
      float max_value = std::numeric_limits<float>::max());

  AudioParam(BaseAudioContext&,
             const String& parent_uuid,
             AudioParamHandler::AudioParamType,
             double default_value,
             AudioParamHandler::AutomationRate rate,
             AudioParamHandler::AutomationRateMode rate_mode,
             float min,
             float max);

  ~AudioParam() override;

  void Trace(Visitor*) const override;
  // `Handler()` always returns a valid object.
  AudioParamHandler& Handler() const { return *handler_; }
  // `Context()` always returns a valid object.
  BaseAudioContext* Context() const { return context_.Get(); }

  AudioParamHandler::AudioParamType GetParamType() const {
    return Handler().GetParamType();
  }
  String GetParamName() const { return Handler().GetParamName(); }
  void SetParamType(AudioParamHandler::AudioParamType);
  void SetCustomParamName(const String name);

  float value() const;
  void setValue(float, ExceptionState&);
  void setValue(float);

  V8AutomationRate automationRate() const;
  void setAutomationRate(const V8AutomationRate&, ExceptionState&);

  float defaultValue() const;

  float minValue() const;
  float maxValue() const;

  AudioParam* setValueAtTime(float value, double time, ExceptionState&);
  AudioParam* linearRampToValueAtTime(float value,
                                      double time,
                                      ExceptionState&);
  AudioParam* exponentialRampToValueAtTime(float value,
                                           double time,
                                           ExceptionState&);
  AudioParam* setTargetAtTime(float target,
                              double time,
                              double time_constant,
                              ExceptionState&);
  AudioParam* setValueCurveAtTime(const Vector<float>& curve,
                                  double time,
                                  double duration,
                                  ExceptionState&);
  AudioParam* cancelScheduledValues(double start_time, ExceptionState&);
  AudioParam* cancelAndHoldAtTime(double start_time, ExceptionState&);

  // InspectorHelperMixin: an AudioParam is always owned by an AudioNode so
  // its notification is done by the parent AudioNode.
  void ReportDidCreate() final {}
  void ReportWillBeDestroyed() final {}

 private:
  void WarnIfOutsideRange(const String& param_methd, float value);

  scoped_refptr<AudioParamHandler> handler_;
  Member<BaseAudioContext> context_;
  scoped_refptr<DeferredTaskHandler> deferred_task_handler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_H_
