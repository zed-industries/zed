// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/spatial_navigation.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

struct FocusCandidate;
class KeyboardEvent;
class Node;
class Page;
struct PhysicalRect;

// Encapsulates logic and state related to "spatial navigation". Spatial
// Navigation is used to move and interact with a page in a purely directional
// way, e.g. keyboard arrows. We use the term "interest" to specify which
// element the user is currently on.
class CORE_EXPORT SpatialNavigationController final
    : public GarbageCollected<SpatialNavigationController> {
 public:
  explicit SpatialNavigationController(Page& page);

  bool HandleArrowKeyboardEvent(KeyboardEvent* event);
  bool HandleEnterKeyboardEvent(KeyboardEvent* event);
  bool HandleEscapeKeyboardEvent(KeyboardEvent* event);
  bool HandleImeSubmitKeyboardEvent(KeyboardEvent* event);

  // Called when the enter key is released to clear local state because we don't
  // get a consistent event stream when the Enter key is partially handled.
  void ResetEnterKeyState();

  void Trace(Visitor*) const;

 private:
  // Entry-point into SpatialNavigation advancement. Will return true if an
  // action (moving interest or scrolling), false otherwise.
  bool Advance(SpatialNavigationDirection direction);

  /*
   * Advances interest only within the specified container. Returns true if
   * interest was advanced or the container was scrolled, false if no
   * advancement was possible within the container.
   *
   * container - The scrollable container within which to limit advancement.
   * starting_rect_in_root_frame - The rect to use to begin searching for the
   *                               next node. Intuitively, the interest node's
   *                               rect (but sometimes different for scrollers).
   * direction - Direction of advancement
   * interest_child_in_container - The inner-most child _within this container_
   *                               where interest is located. This may differ
   *                               from the starting_rect as the interest node
   *                               may be in a nested container.
   */
  bool AdvanceWithinContainer(Node& container,
                              const PhysicalRect& starting_rect_in_root_frame,
                              SpatialNavigationDirection direction,
                              Node* interest_child_in_container);

  // Parameters have same meanings as method above.
  FocusCandidate FindNextCandidateInContainer(
      Node& container,
      const PhysicalRect& starting_rect_in_root_frame,
      SpatialNavigationDirection direction,
      Node* interest_child_in_container);

  // Returns which Node we're starting navigation from or nullptr if we should
  // abort navigation.
  Node* StartingNode();
  void MoveInterestTo(Node* next_node);

  // Dispatches a fake mouse move event at the center of the given element to
  // produce hover state and mouse enter/exit events. If no element is given,
  // we dispatch a mouse event outside of the page to simulate the pointer
  // leaving the page (and clearing hover, producing mouse leave).
  void DispatchMouseMoveAt(Element* element);

  // Returns true if the element should be considered for navigation.
  bool IsValidCandidate(const Element* element) const;

  // Returns the element that's currently interested. i.e. the Element that's
  // currently indicated to the user.
  Element* GetInterestedElement() const;

  Element* GetFocusedElement() const;

  Member<Page> page_;

  // We need to track whether the enter key has been handled in down or press to
  // know whether to generate a click on the up.
  bool enter_key_down_seen_ = false;
  bool enter_key_press_seen_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SPATIAL_NAVIGATION_CONTROLLER_H_
