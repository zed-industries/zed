// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_VISIBILITY_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_VISIBILITY_OBSERVER_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

enum class LayerPositionVisibility : uint8_t;
class Element;
class IntersectionObserver;
class IntersectionObserverEntry;
class ScrollSnapshotClient;

// Monitors visibility of an anchor element for an anchored element, to support
// `position-visibility: anchors-visible` [1]. When the anchor is detected
// as newly-visible or newly-invisible, the anchored element's `PaintLayer` is
// updated via `PaintLayer::SetInvisibleForPositionVisibility`.
//
// There are two aspects of `anchors-visible` visibility:
// 1. Intersection, which is updated with a post-layout intersection observer
//    setup in `MonitorAnchor`.
// 2. CSS visibility, which is checked for all used anchors during all lifecycle
//    updates with `UpdateForCssAnchorVisibility`. This is needed to ensure we
//    catch CSS visibility changes on anchor elements.
//
// [1] Spec: https://drafts.csswg.org/css-anchor-position-1/#position-visibility
class AnchorPositionVisibilityObserver final
    : public GarbageCollected<AnchorPositionVisibilityObserver> {
 public:
  explicit AnchorPositionVisibilityObserver(Element& anchored_element);

  // Sets the anchor element which will be monitored for intersection and CSS
  // visibility changes.
  void MonitorAnchor(const Element*);

  // Update visibility based on the anchor element's CSS visibility.
  void UpdateForCssAnchorVisibility();

  // Update visibility based on the visibility of chained anchor positioned
  // elements. See: AnchorPositionScrollData::DefaultAnchorHasChainedAnchor().
  static void UpdateForChainedAnchorVisibility(
      const HeapHashSet<WeakMember<ScrollSnapshotClient>>&);

  void Trace(Visitor*) const;

 private:
  bool IsInvisibleForChainedAnchorVisibility() const;

  void SetLayerInvisible(LayerPositionVisibility, bool invisible);

  void OnIntersectionVisibilityChanged(
      const HeapVector<Member<IntersectionObserverEntry>>&);

  Member<IntersectionObserver> observer_;
  Member<Element> anchored_element_;
  Member<const Element> anchor_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_VISIBILITY_OBSERVER_H_
