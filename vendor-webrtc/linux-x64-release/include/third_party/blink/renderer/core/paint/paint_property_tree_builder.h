// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/clip_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/effect_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/scroll_paint_property_node.h"
#include "third_party/blink/renderer/platform/graphics/paint/transform_paint_property_node.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FragmentData;
class LayoutObject;
class LocalFrameView;
class PaintLayer;
class PhysicalBoxFragment;
class VisualViewport;

// The context for PaintPropertyTreeBuilder.
// It's responsible for bookkeeping tree state in other order, for example, the
// most recent position container seen.
struct PaintPropertyTreeBuilderFragmentContext {
  STACK_ALLOCATED();

 public:
  // Initializes all property tree nodes to the roots.
  PaintPropertyTreeBuilderFragmentContext();

  // State that propagates on the containing block chain (and so is adjusted
  // when an absolute or fixed position object is encountered).
  struct ContainingBlockContext {
    STACK_ALLOCATED();

   public:
    // The combination of a transform and paint offset describes a linear space.
    // When a layout object recur to its children, the main context is expected
    // to refer the object's border box, then the callee will derive its own
    // border box by translating the space with its own layout location.
    const TransformPaintPropertyNodeOrAlias* transform = nullptr;
    // Corresponds to FragmentData::PaintOffset.
    PhysicalOffset paint_offset;

    // "Additional offset to layout shift root" is the accumulation of paint
    // offsets encoded in PaintOffsetTranslations between the local transform
    // space and the layout shift root. The layout shift root is the nearest
    // ancestor with
    // - a transform node that is one of:
    //   * the transform property tree state of the containing LayoutView
    //   * a transform that is not identity or 2d translation
    //   * a replaced contents transform
    //   * a transform isolation node
    //   * a paint offset translation for a sticky or fixed position element
    // - or an overflow clip node.
    // The offset plus paint_offset is the offset for layout shift tracking.
    // It doesn't include transforms because we need to ignore transform changes
    // for layout shift tracking, see
    //    https://github.com/WICG/layout-instability#transform-changes
    // This field is the diff between the new and the old additional offsets to
    // layout shift root.
    PhysicalOffset additional_offset_to_layout_shift_root_delta;

    // Similar to additional_offset_to_layout_shift_root_delta but for scroll
    // offsets.
    gfx::Vector2dF scroll_offset_to_layout_shift_root_delta;

    // For paint invalidation optimization for subpixel movement under
    // composited layer. It's reset to zero if subpixel can't be propagated
    // thus the optimization is not applicable (e.g. when crossing a
    // non-translation transform).
    PhysicalOffset directly_composited_container_paint_offset_subpixel_delta;

    // The PaintLayer corresponding to the origin of |paint_offset|.
    const LayoutObject* paint_offset_root;

    // True if any fixed-position children within this context are fixed to the
    // root of the FrameView (and hence above its scroll).
    bool fixed_position_children_fixed_to_root = false;

    // True if the layout shift root (see
    // additional_offset_to_layout_shift_root_delta for the definition) of this
    // object has changed.
    bool layout_shift_root_changed = false;

    bool is_in_block_fragmentation = false;

    // The clip node describes the accumulated raster clip for the current
    // subtree.  Note that the computed raster region in canvas space for a clip
    // node is independent from the transform and paint offset above. Also the
    // actual raster region may be affected by layerization and occlusion
    // tracking.
    const ClipPaintPropertyNodeOrAlias* clip = nullptr;
    // The scroll node contains information for scrolling such as the parent
    // scroll space, the extent that can be scrolled, etc. Because scroll nodes
    // reference a scroll offset transform, scroll nodes should be updated if
    // the transform tree changes.
    const ScrollPaintPropertyNode* scroll = nullptr;

    gfx::Vector2dF pending_scroll_anchor_adjustment;

    // Paint offset of the innermost fragmentainer minus accumulated offsets
    // that are baked in PaintOffsetTranslations since we entered the
    // fragmentainer.
    PhysicalOffset paint_offset_for_oof_in_fragmentainer;

    // The fragmentainer index of the nearest ancestor that participates in
    // block fragmentation. This is updated as we update properties for an
    // object that participates in block fragmentation. If we enter monolithic
    // content, the index will be kept and inherited down the tree, so that we
    // eventually set the correct "NG" fragment index in the FragmentData
    // object.
    wtf_size_t fragmentainer_idx = WTF::kNotFound;
  };

  ContainingBlockContext current;

  // Separate context for out-of-flow positioned and fixed positioned elements
  // are needed because they don't use DOM parent as their containing block.
  // These additional contexts normally pass through untouched, and are only
  // copied from the main context when the current element serves as the
  // containing block of corresponding positioned descendants.  Overflow clips
  // are also inherited by containing block tree instead of DOM tree, thus they
  // are included in the additional context too.
  ContainingBlockContext absolute_position;

  ContainingBlockContext fixed_position;

  // The effect hierarchy is applied by the stacking context tree. It is
  // guaranteed that every DOM descendant is also a stacking context descendant.
  // Therefore, we don't need extra bookkeeping for effect nodes and can
  // generate the effect tree from a DOM-order traversal.
  const EffectPaintPropertyNodeOrAlias* current_effect;
  bool this_or_ancestor_opacity_is_zero = false;

  // Set to true when we visit a view transition element and is propagated to
  // all non-alias effects.
  bool self_or_ancestor_participates_in_view_transition = false;

  // Whether newly created children should flatten their inherited transform
  // (equivalently, draw into the plane of their parent). Should generally
  // be updated whenever |transform| is; flattening only needs to happen
  // to immediate children.
  bool should_flatten_inherited_transform = false;

  // Whether newly created child Transform nodes can inherit
  // backface-visibility from the parent. Some situations (e.g. having 3d
  // transform operations) of the child can override this flag.
  bool can_inherit_backface_visibility = false;

  // Rendering context for 3D sorting. See
  // TransformPaintPropertyNode::renderingContextId.
  unsigned rendering_context_id = 0;

  PhysicalOffset old_paint_offset;

  // An additional offset that applies to the current fragment, but is detected
  // *before* the ContainingBlockContext is updated for it. Once the
  // ContainingBlockContext is set, this value should be added to
  // ContainingBlockContext::additional_offset_to_layout_shift_root_delta.
  PhysicalOffset pending_additional_offset_to_layout_shift_root_delta;

  // The delta between the old and new accumulated offsets of 2d translation
  // transforms to the layout shift root.
  gfx::Vector2dF translation_2d_to_layout_shift_root_delta;
};

struct PaintPropertyTreeBuilderContext final {
  STACK_ALLOCATED();

 public:
  // TODO(paint-dev): We should fold PaintPropertyTreeBuilderFragmentContext
  // into PaintPropertyTreeBuilderContext but we can't do it for now because
  // SVG hidden containers need the default constructor of the former to
  // initialize an independent paint property tree context.
  PaintPropertyTreeBuilderFragmentContext fragment_context;

  const LayoutObject* container_for_absolute_position = nullptr;
  const LayoutObject* container_for_fixed_position = nullptr;

#if DCHECK_IS_ON()
  // When DCHECK_IS_ON() and RuntimeEnabledFeatures::
  // PaintUnderInvalidationCheckingEnabled(), we create
  // PaintPropertyTreeBuilderContext even if not needed.
  // See PrePaintTreeWalkContext constructor.
  bool is_actually_needed = true;
#endif

  PaintLayer* painting_layer = nullptr;

  gfx::Vector2dF old_scroll_offset;

  // Specifies the reason the subtree update was forced. For simplicity, this
  // only categorizes it into two categories:
  // - Isolation piercing, meaning that the update is required for subtrees
  //   under an isolation boundary.
  // - Isolation blocked, meaning that the recursion can be blocked by
  //   isolation.
  enum SubtreeUpdateReason : unsigned {
    kSubtreeUpdateIsolationPiercing = 1 << 0,
    kSubtreeUpdateIsolationBlocked = 1 << 1
  };

  // True if a change has forced all properties in a subtree to be updated. This
  // can be set due to paint offset changes or when the structure of the
  // property tree changes (i.e., a node is added or removed).
  unsigned force_subtree_update_reasons : 2 = 0;

  // True if the current subtree is underneath a LayoutSVGHiddenContainer
  // ancestor.
  unsigned has_svg_hidden_container_ancestor : 1 = false;

  // Whether this object was a layout shift root during the previous render
  // (not this one).
  unsigned was_layout_shift_root : 1 = false;

  // This applies to all scrollers in the current LocalFrameView subtree.
  unsigned requires_main_thread_for_background_attachment_fixed : 1 = false;

  unsigned composited_scrolling_preference : 2 =
      static_cast<unsigned>(CompositedScrollingPreference::kDefault);

  // This is always recalculated in PaintPropertyTreeBuilder::UpdateForSelf()
  // which overrides the inherited value.
  CompositingReasons direct_compositing_reasons = CompositingReason::kNone;
};

class VisualViewportPaintPropertyTreeBuilder {
  STATIC_ONLY(VisualViewportPaintPropertyTreeBuilder);

 public:
  // Update the paint properties for the visual viewport and ensure the context
  // is up to date.
  static void Update(LocalFrameView& main_frame_view,
                     VisualViewport&,
                     PaintPropertyTreeBuilderContext&);
};

struct PrePaintInfo {
  STACK_ALLOCATED();

 public:
  PrePaintInfo(const PhysicalBoxFragment* box_fragment,
               PhysicalOffset paint_offset,
               wtf_size_t fragmentainer_idx,
               bool is_first_for_node,
               bool is_last_for_node,
               bool is_inside_fragment_child,
               bool fragmentainer_is_oof_containing_block)
      : box_fragment(box_fragment),
        paint_offset(paint_offset),
        fragmentainer_idx(fragmentainer_idx),
        is_first_for_node(is_first_for_node),
        is_last_for_node(is_last_for_node),
        is_inside_fragment_child(is_inside_fragment_child),
        fragmentainer_is_oof_containing_block(
            fragmentainer_is_oof_containing_block) {}

  // The fragment for the LayoutObject currently being processed, or, in the
  // case of text and non-atomic inlines: the fragment of the containing block.
  // Is nullptr if we're rebuilding the property tree for a missed descendant.
  const PhysicalBoxFragment* box_fragment;

  FragmentData* fragment_data = nullptr;
  PhysicalOffset paint_offset;
  wtf_size_t fragmentainer_idx;
  bool is_first_for_node;
  bool is_last_for_node;

  // True if |box_fragment| is the containing block of the LayoutObject
  // currently being processed. Otherwise, |box_fragment| is a fragment for the
  // LayoutObject itself.
  bool is_inside_fragment_child;

  // Due to how out-of-flow layout inside fragmentation works, if an out-of-flow
  // positioned element is contained by something that's part of a fragmentation
  // context (e.g. abspos in relpos in multicol) the containing block (as far as
  // NG layout is concerned) is a fragmentainer, not the relpos. Then this flag
  // is true. It's false if the containing block doesn't participate in block
  // fragmentation, e.g. if we're inside monolithic content.
  bool fragmentainer_is_oof_containing_block;
};

struct PaintPropertiesChangeInfo {
  STACK_ALLOCATED();

 public:
  PaintPropertyChangeType transform_changed =
      PaintPropertyChangeType::kUnchanged;
  bool transform_change_is_scroll_translation_only = true;
  PaintPropertyChangeType clip_changed = PaintPropertyChangeType::kUnchanged;
  PaintPropertyChangeType effect_changed = PaintPropertyChangeType::kUnchanged;
  PaintPropertyChangeType scroll_changed = PaintPropertyChangeType::kUnchanged;

  void Merge(const PaintPropertiesChangeInfo& other) {
    transform_changed = std::max(transform_changed, other.transform_changed);
    transform_change_is_scroll_translation_only &=
        other.transform_change_is_scroll_translation_only;
    clip_changed = std::max(clip_changed, other.clip_changed);
    effect_changed = std::max(effect_changed, other.effect_changed);
    scroll_changed = std::max(scroll_changed, other.scroll_changed);
  }

  PaintPropertyChangeType Max() const {
    return std::max(
        {transform_changed, clip_changed, effect_changed, scroll_changed});
  }
};

// Creates paint property tree nodes for non-local effects in the layout tree.
// Non-local effects include but are not limited to: overflow clip, transform,
// fixed-pos, animation, mask, filters, etc. It expects to be invoked for each
// layout tree node in DOM order during the PrePaint lifecycle phase.
class PaintPropertyTreeBuilder {
  STACK_ALLOCATED();

 public:
  static void SetupContextForFrame(LocalFrameView&,
                                   PaintPropertyTreeBuilderContext&);

  PaintPropertyTreeBuilder(const LayoutObject& object,
                           PrePaintInfo* pre_paint_info,
                           PaintPropertyTreeBuilderContext& context)
      : object_(object), pre_paint_info_(pre_paint_info), context_(context) {}

  // Update the paint properties that affect this object (e.g., properties like
  // paint offset translation) and ensure the context is up to date. Also
  // handles updating the object's paintOffset.
  void UpdateForSelf();

  // Update the paint properties that affect children of this object (e.g.,
  // scroll offset transform) and ensure the context is up to date.
  void UpdateForChildren();

  // Update paint properties for an @page border box fragment. This fragment is
  // responsible for @page borders and other decorations, in addition to the
  // document background.
  //
  // `page_container` is the parent fragment of the border box.
  void UpdateForPageBorderBox(const PhysicalBoxFragment& page_container);

  void IssueInvalidationsAfterUpdate();

  bool PropertiesChanged() const {
    return properties_changed_.Max() > PaintPropertyChangeType::kUnchanged;
  }

  static void DirectlyUpdateTransformMatrix(const LayoutObject& object);
  static void DirectlyUpdateOpacityValue(const LayoutObject& object);

  static bool ScheduleDeferredTransformNodeUpdate(LayoutObject& object);
  static bool ScheduleDeferredOpacityNodeUpdate(LayoutObject& object);

 private:
  ALWAYS_INLINE void InitPaintProperties();
  ALWAYS_INLINE bool ObjectTypeMightNeedPaintProperties() const;
  ALWAYS_INLINE FragmentData& GetFragmentData() const;
  ALWAYS_INLINE void UpdateFragmentData();
  ALWAYS_INLINE void UpdatePaintingLayer();
  ALWAYS_INLINE bool IsAffectedByOuterViewportBoundsDelta() const;

  ALWAYS_INLINE void UpdateGlobalMainThreadRepaintReasonsForScroll();

  bool IsInNGFragmentTraversal() const { return pre_paint_info_; }

  static bool CanDoDeferredTransformNodeUpdate(const LayoutObject& object);
  static bool CanDoDeferredOpacityNodeUpdate(const LayoutObject& object);

  const LayoutObject& object_;
  PrePaintInfo* pre_paint_info_;
  PaintPropertyTreeBuilderContext& context_;
  PaintPropertiesChangeInfo properties_changed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_PROPERTY_TREE_BUILDER_H_
