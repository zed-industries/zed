// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_NODE_H_

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/iir_filter_handler.h"

namespace blink {

class BaseAudioContext;
class ExceptionState;
class IIRFilterOptions;

class IIRFilterNode : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static IIRFilterNode* Create(BaseAudioContext&,
                               const Vector<double>& feedforward,
                               const Vector<double>& feedback,
                               ExceptionState&);

  static IIRFilterNode* Create(BaseAudioContext*,
                               const IIRFilterOptions*,
                               ExceptionState&);

  IIRFilterNode(BaseAudioContext&,
                const Vector<double>& denominator,
                const Vector<double>& numerator,
                bool is_filter_stable);

  void Trace(Visitor*) const override;

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
  IIRFilterHandler& GetIIRFilterHandler() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_NODE_H_
