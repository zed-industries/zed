// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DELEGATED_INK_DELEGATED_INK_TRAIL_PRESENTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DELEGATED_INK_DELEGATED_INK_TRAIL_PRESENTER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Element;
class ExceptionState;
class InkTrailStyle;
class LocalFrame;
class PointerEvent;
class ScriptState;

// This class collects the required metadata for rendering a delegated ink
// trail and sends it to cc in a unique_ptr<viz::DelegatedInkMetadata>. This
// information is collected from the presentation_area_ and provided
// PointerEvent and InkTrailStyle, and is transformed into root frame
// coordinates before being packed up and sent to cc.
//
// Explainer for the feature:
// https://github.com/WICG/ink-enhancement/blob/main/README.md
class MODULES_EXPORT DelegatedInkTrailPresenter : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DelegatedInkTrailPresenter(Element* element, LocalFrame* frame);

  void updateInkTrailStartPoint(ScriptState* state,
                                PointerEvent* evt,
                                InkTrailStyle* style,
                                ExceptionState& exception_state);
  Element* presentationArea() const { return presentation_area_.Get(); }

  void Trace(Visitor* visitor) const override;

 private:
  Member<Element> presentation_area_;
  Member<LocalFrame> local_frame_;
  base::TimeTicks last_delegated_ink_metadata_timestamp_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DELEGATED_INK_DELEGATED_INK_TRAIL_PRESENTER_H_
