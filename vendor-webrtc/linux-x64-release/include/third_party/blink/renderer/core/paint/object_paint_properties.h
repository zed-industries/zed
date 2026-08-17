// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_PROPERTIES_H_

#include <array>
#include <memory>
#include <utility>

#include "base/dcheck_is_on.h"
#include "base/functional/function_ref.h"
#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/paint/clip_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/scroll_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/transform_paint_property_node.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/sparse_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class is for storing the paint property nodes created by a
// LayoutObject. The object owns each of the property nodes directly and RefPtrs
// are only used to harden against use-after-free bugs. These paint properties
// are built/updated by PaintPropertyTreeBuilder during the PrePaint lifecycle
// step.
//
// [update & clear implementation note] This class has Update[property](...) and
// Clear[property]() helper functions for efficiently creating and updating
// properties. The update functions returns a 3-state result to indicate whether
// the value or the existence of the node has changed. They use a create-or-
// update pattern of re-using existing properties for efficiency:
// 1. It avoids extra allocations.
// 2. It preserves existing child->parent pointers.
// The clear functions return true if an existing node is removed. Property
// nodes store parent pointers but not child pointers and these return values
// are important for catching property tree structure changes which require
// updating descendant's parent pointers.
class CORE_EXPORT ObjectPaintProperties
    : public GarbageCollected<ObjectPaintProperties> {
 public:
  ObjectPaintProperties() = default;
#if DCHECK_IS_ON()
  ~ObjectPaintProperties() { DCHECK(!is_immutable_); }
#endif

  void Trace(Visitor* visitor) const { visitor->Trace(nodes_); }

 private:
// Preprocessor macro declarations.
//
// The following defines 3 functions and one variable:
// - Foo(): a getter for the property.
// - UpdateFoo(): an update function.
// - ClearFoo(): a clear function
// - foo_: the variable itself.
//
// Note that Clear* functions return true if the property tree structure
// changes (an existing node was deleted), and false otherwise. See the
// class-level comment ("update & clear implementation note") for details
// about why this is needed for efficient updates.
#define ADD_NODE(type, function, field_id)                                  \
 public:                                                                    \
  static_assert(field_id >= NodeId::kFirst##type);                          \
  static_assert(field_id <= NodeId::kLast##type);                           \
                                                                            \
  const type##PaintPropertyNode* function() const {                         \
    return GetNode<type##PaintPropertyNode>(field_id);                      \
  }                                                                         \
                                                                            \
  PaintPropertyChangeType Update##function(                                 \
      const type##PaintPropertyNodeOrAlias& parent,                         \
      type##PaintPropertyNode::State&& state,                               \
      const type##PaintPropertyNode::AnimationState& animation_state =      \
          type##PaintPropertyNode::AnimationState()) {                      \
    return Update<type##PaintPropertyNode, type##PaintPropertyNodeOrAlias>( \
        field_id, parent, std::move(state), animation_state);               \
  }                                                                         \
                                                                            \
  bool Clear##function() { return nodes_.EraseField(field_id); }            \
  // (End of ADD_NODE definition)

#define ADD_ALIAS_NODE(type, function, field_id)                        \
 public:                                                                \
  static_assert(field_id == NodeId::k##type##Alias);                    \
                                                                        \
  const type##PaintPropertyNodeOrAlias* function() const {              \
    return GetNode<type##PaintPropertyNodeAlias>(field_id);             \
  }                                                                     \
                                                                        \
  PaintPropertyChangeType Update##function(                             \
      const type##PaintPropertyNodeOrAlias& parent) {                   \
    return UpdateAlias<type##PaintPropertyNodeAlias>(field_id, parent); \
  }                                                                     \
                                                                        \
  bool Clear##function() { return nodes_.EraseField(field_id); }        \
  // (End of ADD_ALIAS_NODE definition)

#define ADD_TRANSFORM(function, field_id) \
  ADD_NODE(Transform, function, field_id)
#define ADD_EFFECT(function, field_id) ADD_NODE(Effect, function, field_id)
#define ADD_CLIP(function, field_id) ADD_NODE(Clip, function, field_id)

  // Identifier used for indexing into the sparse vector of nodes. NOTE: when
  // adding a new node to this list, make sure to do the following. Update
  // the kLast<NodeType> value to reflect the value you added, and renumber all
  // higher value enums. All nodes of NodeType should be bounded between
  // kFirst<NodeType>() and kLast<NodeType> (both inclusive), and the alias
  // node id should be after kLast<NodeType>.
  enum class NodeId : unsigned {
    // Transforms
    kFirstTransform = 0,
    kPaintOffsetTranslation = kFirstTransform,
    kStickyTranslation = 1,
    kAnchorPositionScrollTranslation = 2,
    // Transform nodes for CSS transform operations.
    kFirstCSSTransform = 3,
    kTranslate = kFirstCSSTransform,
    kRotate = 4,
    kScale = 5,
    kOffset = 6,
    kTransform = 7,
    kLastCSSTransform = kTransform,
    // End of Transform nodes for CSS transform operations.
    kPerspective = 8,
    kReplacedContentTransform = 9,
    kScrollTranslation = 10,
    kLastTransform = kScrollTranslation,
    kTransformAlias = 11,

    kScroll = 12,
    kFirstScroll = kScroll,
    kLastScroll = kScroll,

    // Effects
    kFirstEffect = 13,
    kElementCaptureEffect = kFirstEffect,
    kViewTransitionSubframeRootEffect = 14,
    kViewTransitionEffect = 15,
    kEffect = 16,
    kFilter = 17,
    kMask = 18,
    kClipPathMask = 19,
    kVerticalScrollbarEffect = 20,
    kHorizontalScrollbarEffect = 21,
    kScrollCornerEffect = 22,
    kLastEffect = kScrollCornerEffect,
    kEffectAlias = 23,

    // Clips
    kFirstClip = 24,
    kClipPathClip = kFirstClip,
    kMaskClip = 25,
    kCssClip = 26,
    kOverflowControlsClip = 27,
    kBackgroundClip = 28,
    kPixelMovingFilterClipExpander = 29,
    kInnerBorderRadiusClip = 30,
    kOverflowClip = 31,
    kCssClipFixedPosition = 32,
    kLastClip = kCssClipFixedPosition,
    kClipAlias = 33,

    // Should be updated whenever a higher value NodeType is added.
    kNumFields = kClipAlias + 1,
  };

  template <typename NodeType>
  struct NodeIdRange {};

  template <>
  struct NodeIdRange<PaintPropertyNode> {
    static constexpr NodeId kFirst = NodeId::kFirstTransform;
    static constexpr NodeId kLast = NodeId::kClipAlias;
  };
  template <>
  struct NodeIdRange<TransformPaintPropertyNodeOrAlias> {
    static constexpr NodeId kFirst = NodeId::kFirstTransform;
    static constexpr NodeId kLast = NodeId::kTransformAlias;
  };
  template <>
  struct NodeIdRange<TransformPaintPropertyNode> {
    static constexpr NodeId kFirst = NodeId::kFirstTransform;
    static constexpr NodeId kLast = NodeId::kLastTransform;
  };
  template <>
  struct NodeIdRange<ScrollPaintPropertyNode> {
    static constexpr NodeId kFirst = NodeId::kFirstScroll;
    static constexpr NodeId kLast = NodeId::kLastScroll;
  };
  template <>
  struct NodeIdRange<EffectPaintPropertyNodeOrAlias> {
    static constexpr NodeId kFirst = NodeId::kFirstEffect;
    static constexpr NodeId kLast = NodeId::kEffectAlias;
  };
  template <>
  struct NodeIdRange<EffectPaintPropertyNode> {
    static constexpr NodeId kFirst = NodeId::kFirstEffect;
    static constexpr NodeId kLast = NodeId::kLastEffect;
  };
  template <>
  struct NodeIdRange<ClipPaintPropertyNodeOrAlias> {
    static constexpr NodeId kFirst = NodeId::kFirstClip;
    static constexpr NodeId kLast = NodeId::kClipAlias;
  };
  template <>
  struct NodeIdRange<ClipPaintPropertyNode> {
    static constexpr NodeId kFirst = NodeId::kFirstClip;
    static constexpr NodeId kLast = NodeId::kLastClip;
  };

 public:
  template <typename NodeType>
  bool HasNode() const {
    return nodes_.HasFieldInRange(NodeIdRange<NodeType>::kFirst,
                                  NodeIdRange<NodeType>::kLast);
  }

  // Transform node method declarations.
  //
  // The hierarchy of the transform subtree created by a LayoutObject is as
  // follows:
  // [ PaintOffsetTranslation ]
  // |   Normally paint offset is accumulated without creating a node until
  // |   we see, for example, transform or position:fixed.
  // |
  // +-[ StickyTranslation ]
  //  /    This applies the sticky offset induced by position:sticky.
  // |
  // +-[ AnchorPositionScrollTranslation ]
  //  /    This applies the scrolling offset induced by CSS anchor positioning.
  // |
  // +-[ Translate ]
  //   |   The transform from CSS 'translate' (including the effects of
  //  /    'transform-origin').
  // |
  // +-[ Rotate ]
  //   |   The transform from CSS 'rotate' (including the effects of
  //  /    'transform-origin').
  // |
  // +-[ Scale ]
  //   |   The transform from CSS 'scale' (including the effects of
  //  /    'transform-origin').
  // |
  // +-[ Offset ]
  //   |   The transform from the longhand properties that comprise the CSS
  //  /    'offset' shorthand (including the effects of 'transform-origin').
  // |
  // +-[ Transform ]
  //   |   The transform from CSS 'transform' (including the effects of
  //   |   'transform-origin').
  //   |
  //   |   For SVG, this also includes 'translate', 'rotate', 'scale',
  //   |   'offset-*' (instead of the nodes above) and the effects of
  //   |   some characteristics of the SVG viewport and the "SVG
  //   |   additional translation" (for the x and y attributes on
  //   |   svg:use).
  //   |
  //   |   This is the local border box space (see
  //   |   FragmentData::LocalBorderBoxProperties); the nodes below influence
  //   |   the transform for the children but not the LayoutObject itself.
  //   |
  //   +-[ Perspective ]
  //     |   The space created by CSS perspective.
  //     +-[ ReplacedContentTransform ]
  //         Additional transform for replaced elements to implement object-fit.
  //         (Replaced elements don't scroll.)
  //     OR
  //     +-[ ScrollTranslation ]
  //         The space created by overflow clip. The translation equals the
  //         offset between the scrolling contents and the scrollable area of
  //         the container, both originated from the top-left corner, so it is
  //         the scroll position (instead of scroll offset) of the
  //         ScrollableArea.
  //
  // ... +-[ TransformIsolationNode ]
  //         This serves as a parent to subtree transforms on an element with
  //         paint containment. It induces a PaintOffsetTranslation node and
  //         is the deepest child of any transform tree on the contain: paint
  //         element.
  //
  // This hierarchy is related to the order of transform operations in
  // https://drafts.csswg.org/css-transforms-2/#accumulated-3d-transformation-matrix-computation
  bool HasCSSTransformPropertyNode() const {
    return nodes_.HasFieldInRange(NodeId::kFirstCSSTransform,
                                  NodeId::kLastCSSTransform);
  }
  std::array<const TransformPaintPropertyNode*, 5>
  AllCSSTransformPropertiesOutsideToInside() const {
    return {Translate(), Rotate(), Scale(), Offset(), Transform()};
  }

  ADD_TRANSFORM(PaintOffsetTranslation, NodeId::kPaintOffsetTranslation)
  ADD_TRANSFORM(StickyTranslation, NodeId::kStickyTranslation)
  ADD_TRANSFORM(AnchorPositionScrollTranslation,
                NodeId::kAnchorPositionScrollTranslation)
  ADD_TRANSFORM(Translate, NodeId::kTranslate)
  ADD_TRANSFORM(Rotate, NodeId::kRotate)
  ADD_TRANSFORM(Scale, NodeId::kScale)
  ADD_TRANSFORM(Offset, NodeId::kOffset)
  ADD_TRANSFORM(Transform, NodeId::kTransform)
  ADD_TRANSFORM(Perspective, NodeId::kPerspective)
  ADD_TRANSFORM(ReplacedContentTransform, NodeId::kReplacedContentTransform)
  ADD_TRANSFORM(ScrollTranslation, NodeId::kScrollTranslation)
  using ScrollPaintPropertyNodeOrAlias = ScrollPaintPropertyNode;
  ADD_ALIAS_NODE(Transform, TransformIsolationNode, NodeId::kTransformAlias)

  ADD_NODE(Scroll, Scroll, NodeId::kScroll)

  // Effect node method declarations.
  //
  // The hierarchy of the effect subtree created by a LayoutObject is as
  // follows:
  // [ ElementCaptureEffect ]
  // |     Isolated group to force an element to be painted separately.
  // +-[ ViewTransitionSubframeRoot ]
  //   |   Provides the root stacking context for a local subframe with an
  //   |   active ViewTransition. This is used to implement the view transition
  //  /    layer stacking context:
  // | https://drafts.csswg.org/css-view-transitions-1/#view-transition-layer
  // +-[ ViewTransitionEffect ]
  //   |   Provides the stacking context to paint all content for a Document,
  //   |   including top layer elements, into an image used for ViewTransition.
  //  /    This implements the capturing the image for the document element at:
  // |
  // https://drafts.csswg.org/css-view-transitions-1/#capture-the-image-algorithm
  // +-[ Effect ]
  //   |   Isolated group to apply various CSS effects, including opacity,
  //   |   mix-blend-mode, backdrop-filter, and for isolation if a mask needs
  //  /    to be applied or backdrop-dependent children are present, or to
  // |     induce a render surface for cc 2D transform quality optimizations.
  // +-[ Mask ]
  //   |   Isolated group for painting the CSS mask or the mask-based CSS
  //  /    clip-path. This node will have SkBlendMode::kDstIn and shall paint
  // |     last, i.e. after masked contents.
  // +-[ ClipPathMask ]
  //   |   Isolated group for painting the mask-based CSS clip-path. This node
  //   |   will have SkBlendMode::kDstIn and shall paint last, i.e. after
  //  /    clipped contents. If there is no Mask node, then this node is a
  // |     direct child of the Effect node.
  // +-[ Filter ]
  //   |   Isolated group for CSS filter. This is separate from Effect in case
  //  /    there are masks which should be applied to the output of the filter
  // |     instead of the input.
  // +-[ VerticalScrollbarEffect / HorizontalScrollbarEffect / ScrollCorner ]
  // |     Overlay Scrollbars on Aura and Android need effect node for fade
  // |     animation. Also used in ViewTransitions to separate out scrollbars
  // |     from the root snapshot.
  //
  // ... +-[ EffectIsolationNode ]
  //       This serves as a parent to subtree effects on an element with paint
  //       containment, It is the deepest child of any effect tree on the
  //       contain: paint element.
  ADD_EFFECT(ElementCaptureEffect, NodeId::kElementCaptureEffect)
  ADD_EFFECT(ViewTransitionSubframeRootEffect,
             NodeId::kViewTransitionSubframeRootEffect)
  ADD_EFFECT(ViewTransitionEffect, NodeId::kViewTransitionEffect)
  ADD_EFFECT(Effect, NodeId::kEffect)
  ADD_EFFECT(Filter, NodeId::kFilter)
  ADD_EFFECT(Mask, NodeId::kMask)
  ADD_EFFECT(ClipPathMask, NodeId::kClipPathMask)
  ADD_EFFECT(VerticalScrollbarEffect, NodeId::kVerticalScrollbarEffect)
  ADD_EFFECT(HorizontalScrollbarEffect, NodeId::kHorizontalScrollbarEffect)
  ADD_EFFECT(ScrollCornerEffect, NodeId::kScrollCornerEffect)
  ADD_ALIAS_NODE(Effect, EffectIsolationNode, NodeId::kEffectAlias)

  // Clip node declarations.
  //
  // The hierarchy of the clip subtree created by a LayoutObject is as follows:
  // [ ViewTransitionClip ]
  // |   Clip created only when there is an active ViewTransition. This is used
  // |   to clip the element's painting to a subset close to the viewport.
  // |   See https://drafts.csswg.org/css-view-transitions-1/
  // |       #compute-the-interest-rectangle-algorithm for details.
  // +-[ ClipPathClip ]
  //   |  Clip created by path-based CSS clip-path. Only exists if the
  //  /   clip-path is "simple" that can be applied geometrically. This and
  // |    the ClipPathMask effect node are mutually exclusive.
  // +-[ MaskClip ]
  //   |   Clip created by CSS mask or mask-based CSS clip-path.
  //   |   It serves two purposes:
  //   |   1. Cull painting of the masked subtree. Because anything outside of
  //   |      the mask is never visible, it is pointless to paint them.
  //   |   2. Raster clip of the masked subtree. Because the mask implemented
  //   |      as SkBlendMode::kDstIn, pixels outside of mask's bound will be
  //   |      intact when they shall be masked out. This clip ensures no pixels
  //   |      leak out.
  //   +-[ CssClip ]
  //     |   Clip created by CSS clip. CSS clip applies to all descendants, this
  //     |   node only applies to containing block descendants. For descendants
  //     |   not contained by this object, use [ css clip fixed position ].
  //     +-[ OverflowControlsClip ]
  //     |   Clip created by overflow clip to clip overflow controls
  //     |   (scrollbars, resizer, scroll corner) that would overflow the box.
  //     +-[ BackgroundClip ]
  //     |   Clip created for CompositeBackgroundAttachmentFixed background
  //     |   according to CSS background-clip.
  //     +-[ PixelMovingFilterClipExpander ]
  //       | Clip created by pixel-moving filter. Instead of intersecting with
  //       | the current clip, this clip expands the current clip to include all
  //      /  pixels in the filtered content that may affect the pixels in the
  //     |   current clip.
  //     +-[ InnerBorderRadiusClip ]
  //       |   Clip created by a rounded border with overflow clip. This clip is
  //       |   not inset by scrollbars.
  //       +-[ OverflowClip ]
  //             Clip created by overflow clip and is inset by the scrollbar.
  //   [ CssClipFixedPosition ]
  //       Clip created by CSS clip. Only exists if the current clip includes
  //       some clip that doesn't apply to our fixed position descendants.
  //
  //  ... +-[ ClipIsolationNode ]
  //       This serves as a parent to subtree clips on an element with paint
  //       containment. It is the deepest child of any clip tree on the contain:
  //       paint element.
  ADD_CLIP(ClipPathClip, NodeId::kClipPathClip)
  ADD_CLIP(MaskClip, NodeId::kMaskClip)
  ADD_CLIP(CssClip, NodeId::kCssClip)
  ADD_CLIP(OverflowControlsClip, NodeId::kOverflowControlsClip)
  ADD_CLIP(BackgroundClip, NodeId::kBackgroundClip)
  ADD_CLIP(PixelMovingFilterClipExpander,
           NodeId::kPixelMovingFilterClipExpander)
  ADD_CLIP(InnerBorderRadiusClip, NodeId::kInnerBorderRadiusClip)
  ADD_CLIP(OverflowClip, NodeId::kOverflowClip)
  ADD_CLIP(CssClipFixedPosition, NodeId::kCssClipFixedPosition)
  ADD_ALIAS_NODE(Clip, ClipIsolationNode, NodeId::kClipAlias)

#undef ADD_CLIP
#undef ADD_EFFECT
#undef ADD_TRANSFORM
#undef ADD_NODE
#undef ADD_ALIAS_NODE

  // For each node of `NodeType`, runs `action`, and returns true immediately
  // if `action` returns true, or returns false after iterating the nodes.
  template <typename NodeType>
  bool ForNodes(base::FunctionRef<bool(const NodeType&)> action) const {
    for (NodeId i = NodeIdRange<NodeType>::kFirst;
         i <= NodeIdRange<NodeType>::kLast;
         i = static_cast<NodeId>(static_cast<int>(i) + 1)) {
      if (const auto* node = GetNode<NodeType>(i)) {
        if (action(*node)) {
          return true;
        }
      }
    }
    return false;
  }

#if DCHECK_IS_ON()
  // Debug-only state change validation method implementations.
  // Used by find_properties_needing_update.h for verifying state doesn't
  // change.
  void SetImmutable() const { is_immutable_ = true; }
  bool IsImmutable() const { return is_immutable_; }
  void SetMutable() const { is_immutable_ = false; }

  void Validate() {
    DCHECK(!ScrollTranslation() || !ReplacedContentTransform())
        << "Replaced elements don't scroll so there should never be both a "
           "scroll translation and a replaced content transform.";
    DCHECK(!ClipPathClip() || !ClipPathMask())
        << "ClipPathClip and ClipPathshould be mutually exclusive.";
    DCHECK((!TransformIsolationNode() && !ClipIsolationNode() &&
            !EffectIsolationNode()) ||
           (TransformIsolationNode() && ClipIsolationNode() &&
            EffectIsolationNode()))
        << "Isolation nodes have to be created for all of transform, clip, and "
           "effect trees.";
  }

  template <typename NodeType>
  void AddNodesToPrinter(PropertyTreePrinter& printer) const {
    ForNodes<NodeType>([&printer](const NodeType& node) {
      printer.AddNode(&node);
      return false;
    });
  }
#endif

  // Direct update method implementations.
  PaintPropertyChangeType DirectlyUpdateTransformAndOrigin(
      TransformPaintPropertyNode::TransformAndOrigin&& transform_and_origin,
      const TransformPaintPropertyNode::AnimationState& animation_state) {
    CHECK(nodes_.HasField(NodeId::kTransform));
    return GetNode<TransformPaintPropertyNode>(NodeId::kTransform)
        ->DirectlyUpdateTransformAndOrigin(std::move(transform_and_origin),
                                           animation_state);
  }
  PaintPropertyChangeType DirectlyUpdateOpacity(
      float opacity,
      const EffectPaintPropertyNode::AnimationState& animation_state) {
    const bool has_effect = nodes_.HasField(NodeId::kEffect);
    // TODO(yotha): Remove this check once we make sure crbug.com/1370268
    // is fixed.
    DCHECK(has_effect);
    if (!has_effect) {
      return PaintPropertyChangeType::kNodeAddedOrRemoved;
    }
    return GetNode<EffectPaintPropertyNode>(NodeId::kEffect)
        ->DirectlyUpdateOpacity(opacity, animation_state);
  }

 private:
  using NodeList = SparseVector<NodeId, Member<PaintPropertyNode>, 2>;

  template <typename NodeType, typename ParentType>
  PaintPropertyChangeType Update(
      NodeId node_id,
      const ParentType& parent,
      NodeType::State&& state,
      const NodeType::AnimationState& animation_state) {
    // First, check if we need to add a new node.
    if (!nodes_.HasField(node_id)) {
      nodes_.SetField(node_id, NodeType::Create(parent, std::move(state)));
#if DCHECK_IS_ON()
      DCHECK(!is_immutable_) << "Sparse node added while immutable.";
#endif
      return PaintPropertyChangeType::kNodeAddedOrRemoved;
    }
    // If not, we just need to update the existing node.
    auto* node = GetNode<NodeType>(node_id);
    const PaintPropertyChangeType changed =
        node->Update(parent, std::move(state), animation_state);
#if DCHECK_IS_ON()
    DCHECK(!is_immutable_ || changed == PaintPropertyChangeType::kUnchanged)
        << "Value changed while immutable.";
#endif
    return changed;
  }

  template <typename AliasType, typename ParentType>
  PaintPropertyChangeType UpdateAlias(NodeId node_id,
                                      const ParentType& parent) {
    // First, check if we need to add a new alias.
    if (!nodes_.HasField(node_id)) {
      nodes_.SetField(node_id, AliasType::Create(parent));
#if DCHECK_IS_ON()
      DCHECK(!is_immutable_) << "Sparse node added while immutable.";
#endif
      return PaintPropertyChangeType::kNodeAddedOrRemoved;
    }
    // If not, we just need to update the existing alias.
    auto* node = GetNode<AliasType>(node_id);
    DCHECK(node->IsParentAlias());
    const PaintPropertyChangeType changed = node->SetParent(parent);
#if DCHECK_IS_ON()
    DCHECK(!is_immutable_ || changed == PaintPropertyChangeType::kUnchanged)
        << "Parent changed while immutable. New state:\n"
        << *node;
#endif
    return changed;
  }

  template <typename NodeType>
  const NodeType* GetNode(NodeId node_id) const {
    if (nodes_.HasField(node_id)) {
      return static_cast<const NodeType*>(nodes_.GetField(node_id).Get());
    }
    return nullptr;
  }

  template <typename NodeType>
  NodeType* GetNode(NodeId node_id) {
    if (nodes_.HasField(node_id)) {
      return static_cast<NodeType*>(nodes_.GetField(node_id).Get());
    }
    return nullptr;
  }

  NodeList nodes_;

#if DCHECK_IS_ON()
  mutable bool is_immutable_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_PROPERTIES_H_
