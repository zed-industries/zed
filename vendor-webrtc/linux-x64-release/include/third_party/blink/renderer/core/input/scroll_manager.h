// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_SCROLL_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_SCROLL_MANAGER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/event_with_hit_test_results.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class AutoscrollController;
class LayoutBox;
class LocalFrame;
class PaintLayer;
class PaintLayerScrollableArea;

// Scroll directions used to check whether propagation is possible in a given
// direction. Used in CanPropagate.
enum class ScrollPropagationDirection { kHorizontal, kVertical, kBoth, kNone };

// This class is deprecated as scrolling is now handled by cc::InputHandler.
// It is still involved with the following main-thread operations:
// - keyboard scrolls
// - middle-click autoscroll
// - resizer-control interactions
// For Javascript scrolls, see ProgrammaticScrollAnimator.
// Do not add new things to this class.
// TODO(crbug.com/1503711): Remove keyboard scrolling.
class CORE_EXPORT ScrollManager : public GarbageCollected<ScrollManager> {
 public:
  explicit ScrollManager(LocalFrame&);
  ScrollManager(const ScrollManager&) = delete;
  ScrollManager& operator=(const ScrollManager&) = delete;
  virtual ~ScrollManager() = default;
  void Trace(Visitor*) const;

  void Clear();

  bool MiddleClickAutoscrollInProgress() const;
  void StopMiddleClickAutoscroll();
  AutoscrollController* GetAutoscrollController() const;
  void StopAutoscroll();

  // Performs a chaining logical scroll, within a *single* frame, starting
  // from either a provided starting node or a default based on the focused or
  // most recently clicked node, falling back to the frame.
  // Returns true if the scroll was consumed.
  // direction - The logical direction to scroll in. This will be converted to
  //             a physical direction for each LayoutBox we try to scroll
  //             based on that box's writing mode.
  // granularity - The units that the  scroll delta parameter is in.
  // startNode - Optional. If provided, start chaining from the given node.
  //             If not, use the current focus or last clicked node.
  bool LogicalScroll(mojom::blink::ScrollDirection,
                     ui::ScrollGranularity,
                     Node* start_node,
                     Node* mouse_press_node,
                     bool scrolling_via_key = false);

  // Performs a logical scroll that chains, crossing frames, starting from
  // the given node or a reasonable default (focus/last clicked).
  bool BubblingScroll(mojom::blink::ScrollDirection,
                      ui::ScrollGranularity,
                      Node* starting_node,
                      Node* mouse_press_node,
                      bool scrolling_via_key = false);

  // These functions are related to |m_resizeScrollableArea|.
  bool InResizeMode() const;
  void Resize(const WebMouseEvent&);
  // Clears |m_resizeScrollableArea|. if |shouldNotBeNull| is true this
  // function DCHECKs to make sure that variable is indeed not null.
  void ClearResizeScrollableArea(bool should_not_be_null);
  void SetResizeScrollableArea(PaintLayer*, gfx::Point);

  // Determines whether the scroll-chain should be propagated upwards given a
  // scroll direction.
  static bool CanPropagate(const LayoutBox* layout_box,
                           ScrollPropagationDirection direction);

 private:
  void RecomputeScrollChain(const Node& start_node,
                            Deque<DOMNodeId>& scroll_chain,
                            bool is_autoscroll);
  bool CanScroll(const Node& current_node, bool for_autoscroll);

  const Member<LocalFrame> frame_;

  Member<PaintLayerScrollableArea> resize_scrollable_area_;

  // In the coords of resize_scrollable_area_.
  gfx::Vector2d offset_from_resize_corner_;
  gfx::Transform resize_position_to_size_transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_SCROLL_MANAGER_H_
