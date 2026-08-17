// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_PSEUDO_ELEMENT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_PSEUDO_ELEMENT_BASE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
class ViewTransitionStyleTracker;

class CORE_EXPORT ViewTransitionPseudoElementBase : public PseudoElement {
 public:
  ViewTransitionPseudoElementBase(
      Element* parent,
      PseudoId,
      const AtomicString& view_transition_name,
      bool is_generated_name,
      const ViewTransitionStyleTracker* style_tracker);
  ~ViewTransitionPseudoElementBase() override = default;

  bool CanGeneratePseudoElement(PseudoId) const override;
  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) override;
  void Trace(Visitor* visitor) const override;

  const Vector<AtomicString>& ViewTransitionClassList() const;

  // Returns true if this pseudo element is bound to a transition using
  // `tracker`.
  bool IsBoundTo(const blink::ViewTransitionStyleTracker* tracker) const;

  const Vector<AtomicString>& GetViewTransitionNames() const;

 protected:
  Vector<AtomicString> view_transition_class_;
  Member<const ViewTransitionStyleTracker> style_tracker_;
};

template <>
struct DowncastTraits<ViewTransitionPseudoElementBase> {
  static bool AllowFrom(const Node& node) {
    return IsTransitionPseudoElement(node.GetPseudoId());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_PSEUDO_ELEMENT_BASE_H_
