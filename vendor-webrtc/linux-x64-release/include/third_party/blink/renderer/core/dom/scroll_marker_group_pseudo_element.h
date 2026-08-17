// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_PSEUDO_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_PSEUDO_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/dom/scroll_marker_group_data.h"
#include "third_party/blink/renderer/core/dom/scroll_marker_pseudo_element.h"

namespace blink {

// Represents ::scroll-marker-group pseudo element and manages
// implicit focus group, formed by ::scroll-marker pseudo elements.
// This focus group is needed to cycle through its element with
// arrow keys.
class ScrollMarkerGroupPseudoElement : public PseudoElement {
 public:
  // pseudo_id is needed, as ::scroll-marker-group can be after or before.
  ScrollMarkerGroupPseudoElement(Element* originating_element,
                                 PseudoId pseudo_id);

  bool IsScrollMarkerGroupPseudoElement() const final { return true; }

  void AddToFocusGroup(ScrollMarkerPseudoElement& scroll_marker);
  void RemoveFromFocusGroup(ScrollMarkerPseudoElement& scroll_marker);
  void ClearFocusGroup();
  // Set selected scroll marker. Returns true if the selected marker changed.
  CORE_EXPORT bool SetSelected(ScrollMarkerPseudoElement& scroll_marker,
                               bool apply_snap_alignment = true);
  ScrollMarkerPseudoElement* Selected() const;
  void ActivateNextScrollMarker();
  void ActivatePrevScrollMarker();
  CORE_EXPORT void ActivateScrollMarker(
      ScrollMarkerPseudoElement* scroll_marker,
      bool apply_snap_alignment = true);

  void DetachLayoutTree(bool performing_reattach) final;
  void Dispose() final;
  void Trace(Visitor* v) const final;

  void UpdateSelectedScrollMarker();

  // When a "targeted" scroll occurs, we should consider the selected scroll
  // marker pinned until a non-targeted scroll occurs.
  void PinSelectedMarker(ScrollMarkerPseudoElement* scroll_marker);
  void UnPinSelectedMarker();
  bool SelectedMarkerIsPinned() const;

  void ScrollSelectedIntoView(bool apply_snap_alignment);

 private:

  ScrollMarkerPseudoElement* FindNextScrollMarker(const Element* current);
  ScrollMarkerPseudoElement* FindPreviousScrollMarker(const Element* current);

  Member<ScrollMarkerGroupData> scroll_marker_group_data_;
};

template <>
struct DowncastTraits<ScrollMarkerGroupPseudoElement> {
  static bool AllowFrom(const Node& node) {
    return node.IsScrollMarkerGroupPseudoElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_PSEUDO_ELEMENT_H_
