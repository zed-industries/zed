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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_NODE_H_

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class AnalyserHandler;
class AnalyserOptions;
class BaseAudioContext;
class ExceptionState;

class AnalyserNode final : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AnalyserNode* Create(BaseAudioContext&, ExceptionState&);
  static AnalyserNode* Create(BaseAudioContext*,
                              const AnalyserOptions*,
                              ExceptionState&);

  explicit AnalyserNode(BaseAudioContext&);

  unsigned fftSize() const;
  void setFftSize(unsigned size, ExceptionState&);
  unsigned frequencyBinCount() const;
  void setMinDecibels(double, ExceptionState&);
  double minDecibels() const;
  void setMaxDecibels(double, ExceptionState&);
  double maxDecibels() const;
  void setSmoothingTimeConstant(double, ExceptionState&);
  double smoothingTimeConstant() const;
  void getFloatFrequencyData(NotShared<DOMFloat32Array>);
  void getByteFrequencyData(NotShared<DOMUint8Array>);
  void getFloatTimeDomainData(NotShared<DOMFloat32Array>);
  void getByteTimeDomainData(NotShared<DOMUint8Array>);

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  AnalyserHandler& GetAnalyserHandler() const;

  void SetMinMaxDecibels(double min, double max, ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ANALYSER_NODE_H_
