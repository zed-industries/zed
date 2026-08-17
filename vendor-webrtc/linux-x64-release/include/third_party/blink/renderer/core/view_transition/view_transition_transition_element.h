// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TRANSITION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TRANSITION_ELEMENT_H_

#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_pseudo_element_base.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {
class ViewTransitionStyleTracker;

// This class represents the ::view-transition pseudo-element.
// Once the transition starts, it tracks the nested ::view-transition-group(*)
// pseudo-elements for quick lookup.
class CORE_EXPORT ViewTransitionTransitionElement
    : public ViewTransitionPseudoElementBase {
 public:
  ViewTransitionTransitionElement(
      Element* parent,
      const ViewTransitionStyleTracker* style_tracker);
  ~ViewTransitionTransitionElement() override = default;

  PseudoElement* FindViewTransitionGroupPseudoElement(
      const AtomicString& view_transition_name);
  PseudoElement* FindViewTransitionGroupPseudoElementParent(
      const AtomicString& view_transition_name);
};

template <>
struct DowncastTraits<ViewTransitionTransitionElement> {
  static bool AllowFrom(const Node& node) {
    return node.GetPseudoId() == kPseudoIdViewTransition;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_VIEW_TRANSITION_VIEW_TRANSITION_TRANSITION_ELEMENT_H_
