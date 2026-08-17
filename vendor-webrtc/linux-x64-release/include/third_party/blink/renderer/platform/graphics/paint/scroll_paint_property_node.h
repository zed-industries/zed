// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCROLL_PAINT_PROPERTY_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCROLL_PAINT_PROPERTY_NODE_H_

#include <algorithm>
#include <optional>

#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "cc/input/main_thread_scrolling_reason.h"
#include "cc/input/overscroll_behavior.h"
#include "cc/input/scroll_snap_data.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_property_node.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ClipPaintPropertyNode;

using MainThreadScrollingReasons = uint32_t;

enum class CompositedScrollingPreference : uint8_t {
  kDefault,
  kPreferred,
  kNotPreferred,
};

// A scroll node contains auxiliary scrolling information which includes how far
// an area can be scrolled, main thread scrolling reasons, etc. Scroll nodes
// are referenced by TransformPaintPropertyNodes that are used for the scroll
// offset translation, though scroll offset translation can exist without a
// scroll node (e.g., overflow: hidden).
//
// Main thread scrolling reasons force scroll updates to go to the main thread
// and can have dependencies on other nodes. For example, all parents of a
// scroll node with background attachment fixed set should also have it set.
//
// The scroll tree differs from the other trees because it does not affect
// geometry directly.
class PLATFORM_EXPORT ScrollPaintPropertyNode final
    : public PaintPropertyNodeBase<ScrollPaintPropertyNode,
                                   ScrollPaintPropertyNode> {
 public:
  // To make it less verbose and more readable to construct and update a node,
  // a struct with default values is used to represent the state.
  struct PLATFORM_EXPORT State {
    DISALLOW_NEW();

   public:
    gfx::Rect container_rect;
    gfx::Size contents_size;
    Member<const ClipPaintPropertyNode> overflow_clip_node;
    bool user_scrollable_horizontal = false;
    bool user_scrollable_vertical = false;

    // This bit tells the compositor whether the inner viewport should be
    // scrolled using the full viewport mechanism (overscroll, top control
    // movement, inner+outer panning, etc.). This can differ depending on
    // whether the page has a non-default root scroller and is used to affect
    // scroll chaining from fixed elements. See discussion on
    // https://crbug.com/977954 for details.
    bool prevent_viewport_scrolling_from_inner = false;

    bool max_scroll_offset_affected_by_page_scale = false;
    CompositedScrollingPreference composited_scrolling_preference =
        CompositedScrollingPreference::kDefault;
    MainThreadScrollingReasons main_thread_repaint_reasons =
        cc::MainThreadScrollingReason::kNotScrollingOnMain;
    // The scrolling element id is stored directly on the scroll node and not
    // on the associated TransformPaintPropertyNode used for scroll offset.
    CompositorElementId compositor_element_id;
    cc::OverscrollBehavior overscroll_behavior =
        cc::OverscrollBehavior(cc::OverscrollBehavior::Type::kAuto);
    std::optional<cc::SnapContainerData> snap_container_data;

    PaintPropertyChangeType ComputeChange(const State& other) const;

    void Trace(Visitor*) const;
  };

  // This node is really a sentinel, and does not represent a real scroll.
  static const ScrollPaintPropertyNode& Root();

  static ScrollPaintPropertyNode* Create(const ScrollPaintPropertyNode& parent,
                                         State&& state) {
    return MakeGarbageCollected<ScrollPaintPropertyNode>(
        kNonParentAlias, parent, std::move(state));
  }

  void Trace(Visitor* visitor) const final {
    PaintPropertyNodeBase::Trace(visitor);
    visitor->Trace(state_);
  }

  // The empty AnimationState struct is to meet the requirement of
  // ObjectPaintProperties.
  struct AnimationState {
    STACK_ALLOCATED();
  };
  PaintPropertyChangeType Update(const ScrollPaintPropertyNode& parent,
                                 State&& state,
                                 const AnimationState& = AnimationState()) {
    auto parent_changed = SetParent(parent);
    auto state_changed = state_.ComputeChange(state);
    if (state_changed != PaintPropertyChangeType::kUnchanged) {
      state_ = std::move(state);
      Validate();
      AddChanged(state_changed);
    }
    return std::max(parent_changed, state_changed);
  }

  const ScrollPaintPropertyNode& Unalias() const = delete;

  // See PaintPropertyNode::ChangedSequenceNumber().
  void ClearChangedToRoot(int sequence_number) const;

  cc::OverscrollBehavior::Type OverscrollBehaviorX() const {
    return state_.overscroll_behavior.x;
  }

  cc::OverscrollBehavior::Type OverscrollBehaviorY() const {
    return state_.overscroll_behavior.y;
  }

  std::optional<cc::SnapContainerData> GetSnapContainerData() const {
    return state_.snap_container_data;
  }

  // Rect of the container area that the contents scrolls in, in the space of
  // the parent of the associated transform node, i.e. PaintOffsetTranslation
  // which is the parent of ScrollTranslation. It doesn't include non-overlay
  // scrollbars. Overlay scrollbars do not affect the rect.
  const gfx::Rect& ContainerRect() const { return state_.container_rect; }

  // Rect of the contents that is scrolled within the container rect, in the
  // space of the associated transform node (ScrollTranslation). It has the
  // same origin as ContainerRect().
  gfx::Rect ContentsRect() const {
    return gfx::Rect(state_.container_rect.origin(), state_.contents_size);
  }

  const ClipPaintPropertyNode* OverflowClipNode() const {
    return state_.overflow_clip_node.Get();
  }

  bool UserScrollableHorizontal() const {
    return state_.user_scrollable_horizontal;
  }
  bool UserScrollableVertical() const {
    return state_.user_scrollable_vertical;
  }
  bool UserScrollable() const {
    return UserScrollableHorizontal() || UserScrollableVertical();
  }

  bool PreventViewportScrollingFromInner() const {
    return state_.prevent_viewport_scrolling_from_inner;
  }
  bool MaxScrollOffsetAffectedByPageScale() const {
    return state_.max_scroll_offset_affected_by_page_scale;
  }
  CompositedScrollingPreference GetCompositedScrollingPreference() const {
    return state_.composited_scrolling_preference;
  }

  // Note that this doesn't include main-thread repaint reasons computed
  // after paint.
  MainThreadScrollingReasons GetMainThreadRepaintReasons() const {
    return state_.main_thread_repaint_reasons;
  }

  bool RequiresMainThreadForBackgroundAttachmentFixed() const {
    return state_.main_thread_repaint_reasons &
           cc::MainThreadScrollingReason::kHasBackgroundAttachmentFixedObjects;
  }

  const CompositorElementId& GetCompositorElementId() const {
    return state_.compositor_element_id;
  }

  std::unique_ptr<JSONObject> ToJSON() const final;

  // These are public required by MakeGarbageCollected, but the protected tags
  // prevent these from being called from outside.
  explicit ScrollPaintPropertyNode(RootTag);
  ScrollPaintPropertyNode(NonParentAliasTag,
                          const ScrollPaintPropertyNode& parent,
                          State&& state)
      : PaintPropertyNodeBase(NonParentAliasTag(), parent),
        state_(std::move(state)) {
    Validate();
  }

 private:
  void Validate() const {
#if DCHECK_IS_ON()
    DCHECK(!state_.compositor_element_id ||
           NamespaceFromCompositorElementId(state_.compositor_element_id) ==
               CompositorElementIdNamespace::kScroll);
    DCHECK(cc::MainThreadScrollingReason::AreRepaintReasons(
        state_.main_thread_repaint_reasons));
#endif
  }

  State state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCROLL_PAINT_PROPERTY_NODE_H_
