/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_FILTER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_FILTER_NODE_H_

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/biquad_processor.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class BaseAudioContext;
class BiquadFilterOptions;
class ExceptionState;
class V8BiquadFilterType;

class BiquadFilterNode final : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // These must be defined as in the .idl file and must match those in the
  // BiquadProcessor class.
  enum {
    LOWPASS = 0,
    HIGHPASS = 1,
    BANDPASS = 2,
    LOWSHELF = 3,
    HIGHSHELF = 4,
    PEAKING = 5,
    NOTCH = 6,
    ALLPASS = 7
  };

  static BiquadFilterNode* Create(BaseAudioContext&, ExceptionState&);
  static BiquadFilterNode* Create(BaseAudioContext*,
                                  const BiquadFilterOptions*,
                                  ExceptionState&);

  explicit BiquadFilterNode(BaseAudioContext&);

  void Trace(Visitor*) const override;

  V8BiquadFilterType type() const;
  void setType(const V8BiquadFilterType&);

  AudioParam* frequency() { return frequency_.Get(); }
  AudioParam* q() { return q_.Get(); }
  AudioParam* gain() { return gain_.Get(); }
  AudioParam* detune() { return detune_.Get(); }

  // Get the magnitude and phase response of the filter at the given
  // set of frequencies (in Hz). The phase response is in radians.
  void getFrequencyResponse(NotShared<const DOMFloat32Array> frequency_hz,
                            NotShared<DOMFloat32Array> mag_response,
                            NotShared<DOMFloat32Array> phase_response,
                            ExceptionState&);

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  BiquadProcessor* GetBiquadProcessor() const;
  bool SetType(BiquadProcessor::FilterType);  // Returns true on success.

  Member<AudioParam> frequency_;
  Member<AudioParam> q_;
  Member<AudioParam> gain_;
  Member<AudioParam> detune_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_BIQUAD_FILTER_NODE_H_
