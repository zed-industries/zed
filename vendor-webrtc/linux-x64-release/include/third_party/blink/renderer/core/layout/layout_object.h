/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 *           (C) 2004 Allan Sandfeld Jensen (kde@carewolf.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2012 Apple Inc.
 *               All rights reserved.
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_H_

#include <utility>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/notreached.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/display_lock/display_lock_context.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/document_lifecycle.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/inspector/inspector_trace_events.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/geometry/transform_state.h"
#include "third_party/blink/renderer/core/layout/hit_test_phase.h"
#include "third_party/blink/renderer/core/layout/layout_object_child_list.h"
#include "third_party/blink/renderer/core/layout/map_coordinates_flags.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/core/layout/outline_rect_collector.h"
#include "third_party/blink/renderer/core/layout/outline_type.h"
#include "third_party/blink/renderer/core/layout/selection_state.h"
#include "third_party/blink/renderer/core/layout/style_variant.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/core/paint/fragment_data.h"
#include "third_party/blink/renderer/core/paint/paint_phase.h"
#include "third_party/blink/renderer/core/paint/pre_paint_disable_side_effects_scope.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/style_difference.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_utils.h"
#include "third_party/blink/renderer/platform/graphics/compositing_reasons.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/graphics/subtree_paint_property_update_reason.h"
#include "third_party/blink/renderer/platform/graphics/visual_rect_flags.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/quad_f.h"
#include "ui/gfx/geometry/transform.h"

namespace ui {
class Cursor;
}

namespace blink {
class AccompaniedFragmentIterator;
class AffineTransform;
class HitTestLocation;
class HitTestRequest;
class HitTestResult;
class LayoutBlock;
class LayoutBlockFlow;
class LayoutFlowThread;
class LayoutMultiColumnSpannerPlaceholder;
class LayoutView;
class LocalFrameView;
class PaintLayer;
class StyleRequest;
struct PaintInfo;
struct PaintInvalidatorContext;
struct SVGLayoutInfo;
struct SVGLayoutResult;

enum CursorDirective { kSetCursorBasedOnStyle, kSetCursor, kDoNotSetCursor };

enum MarkingBehavior {
  kMarkOnlyThis,
  kMarkContainerChain,
};

enum ScheduleRelayoutBehavior { kScheduleRelayout, kDontScheduleRelayout };

enum {
  // Backgrounds paint under FragmentData::LocalBorderBoxProperties().
  kBackgroundPaintInBorderBoxSpace = 1 << 0,
  // Backgrounds paint under FragmentData::ContentsProperties().
  kBackgroundPaintInContentsSpace = 1 << 1,
  // Paint backgrounds twice.
  kBackgroundPaintInBothSpaces =
      kBackgroundPaintInBorderBoxSpace | kBackgroundPaintInContentsSpace,
};
using BackgroundPaintLocation = unsigned;

struct DraggableRegionValue {
  DISALLOW_NEW();
  bool operator==(const DraggableRegionValue& o) const {
    return draggable == o.draggable && bounds == o.bounds;
  }

  PhysicalRect bounds;
  bool draggable;
};

// The axes which overflows should be clipped. This is not just because of
// overflow clip, but other types of clip as well, such as control clips or
// contain: paint.
using OverflowClipAxes = unsigned;

enum {
  kNoOverflowClip = 0,
  kOverflowClipX = 1 << 0,
  kOverflowClipY = 1 << 1,
  kOverflowClipBothAxis = kOverflowClipX | kOverflowClipY,
};

// Expands |clip_rect| to allow infinite overflow in horizontal and/or vertical
// direction.
void ApplyVisibleOverflowToClipRect(OverflowClipAxes, PhysicalRect& clip_rect);

#if DCHECK_IS_ON()
const int kShowTreeCharacterOffset = 39;
#endif

// Usually calling LayooutObject::Destroy() is banned. This scope can be used to
// exclude certain functions like ~SVGImage() from this rule. This is allowed
// when a Persistent is guaranteeing to keep the LayoutObject alive for that GC
// cycle.
class CORE_EXPORT AllowDestroyingLayoutObjectInFinalizerScope {
  STACK_ALLOCATED();

 public:
  AllowDestroyingLayoutObjectInFinalizerScope();
  ~AllowDestroyingLayoutObjectInFinalizerScope();
};

// The result of |LayoutObject::RecalcScrollableOverflow|.
struct RecalcScrollableOverflowResult {
  STACK_ALLOCATED();

 public:
  // True if the scrollable-overflow (from the viewpoint of the parent) changed,
  // indicating that the parent should also recalculate its scrollable-overflow.
  bool scrollable_overflow_changed = false;

  // True if parents should rebuild their fragments to ensure fragment tree
  // consistency. This may be true even if |scrollable_overflow_changed| is
  // false.
  bool rebuild_fragment_tree = false;

  void Unite(const RecalcScrollableOverflowResult& other) {
    scrollable_overflow_changed |= other.scrollable_overflow_changed;
    rebuild_fragment_tree |= other.rebuild_fragment_tree;
  }
};

// LayoutObject is the base class for all layout tree objects.
//
// LayoutObjects form a tree structure that is a close mapping of the DOM tree.
// The root of the LayoutObject tree is the LayoutView, which is the
// LayoutObject associated with the Document.
//
// Some LayoutObjects don't have an associated Node and are called "anonymous"
// (see the constructor below). Anonymous LayoutObjects exist for several
// purposes but are usually required by CSS. A good example is anonymous table
// parts (see LayoutTable for the expected structure). Anonymous LayoutObjects
// are generated when a new child is added to the tree in addChild(). See the
// function for some important information on this.
//
// Also some Node don't have an associated LayoutObjects e.g. if display: none
// or display: contents is set. For more detail, see LayoutObject::createObject
// that creates the right LayoutObject based on the style.
//
// Because the SVG and CSS classes both inherit from this object, functions can
// belong to either realm and sometimes to both.
//
// The purpose of the layout tree is to do layout (aka reflow) and store its
// results for painting and hit-testing. Layout is the process of sizing and
// positioning Nodes on the page. In Blink, layouts always start from a relayout
// boundary (see ObjectIsRelayoutBoundary in layout_object.cc). As such, we
// need to mark the ancestors all the way to the enclosing relayout boundary in
// order to do a correct layout.
//
// Due to the high cost of layout, a lot of effort is done to avoid doing full
// layouts of nodes. This is why there are several types of layout available to
// bypass the complex operations. See the comments on the layout booleans in
// LayoutObjectBitfields below about the different layouts.
//
// To save memory, especially for the common child class LayoutText,
// LayoutObject doesn't provide storage for children. Descendant classes that do
// allow children have to have a LayoutObjectChildList member that stores the
// actual children and override virtualChildren().
//
// LayoutObject is an ImageResourceObserver, which means that it gets notified
// when associated images are changed. This is used for 2 main use cases:
// - reply to 'background-image' as we need to invalidate the background in this
//   case.
//   (See https://drafts.csswg.org/css-backgrounds-3/#the-background-image)
// - image (LayoutImage, LayoutSVGImage) or video (LayoutVideo) objects that are
//   placeholders for displaying them.
//
//
// ***** LIFETIME *****
//
// LayoutObjects are fully owned by their associated DOM node. In other words,
// it's the DOM node's responsibility to free its LayoutObject, this is why
// LayoutObjects are not and SHOULD NOT be RefCounted.
//
// LayoutObjects are created during the DOM attachment. This phase computes
// the style and create the LayoutObject associated with the Node (see
// Node::attachLayoutTree). LayoutObjects are destructed during detachment (see
// Node::detachLayoutTree), which can happen when the DOM node is removed from
// the
// DOM tree, during page tear down or when the style is changed to contain
// 'display: none'.
//
// Anonymous LayoutObjects are owned by their enclosing DOM node. This means
// that if the DOM node is detached, it has to destroy any anonymous
// descendants. This is done in LayoutObject::destroy().
//
// Note that for correctness, destroy() is expected to clean any anonymous
// wrappers as sequences of insertion / removal could make them visible to
// the page. This is done by LayoutObject::destroyAndCleanupAnonymousWrappers()
// which is the preferred way to destroy an object.
//
//
// ***** INTRINSIC SIZES / PREFERRED LOGICAL WIDTHS *****
// The preferred logical widths are the intrinsic sizes of this element
// (https://drafts.csswg.org/css-sizing-3/#intrinsic). Intrinsic sizes depend
// mostly on the content and a limited set of style properties (e.g. any
// font-related property for text, 'min-width'/'max-width',
// 'min-height'/'max-height').
//
// Those widths are used to determine the final layout logical width, which
// depends on the layout algorithm used and the available logical width.
//
// LayoutObject only has a getter for the widths (PreferredLogicalWidths).
// However the storage for them is in LayoutBox (see
// min_preferred_logical_width_ and max_preferred_logical_width_). This is
// because only boxes implementing the full box model have a need for them.
// Because LayoutBlockFlow's intrinsic widths rely on the underlying text
// content, LayoutBlockFlow may call LayoutText::ComputePreferredLogicalWidths.
//
// The 2 widths are computed lazily during layout when the getters are called.
// The computation is done by calling ComputePreferredLogicalWidths() behind the
// scene. The boolean used to control the lazy recomputation is
// IntrinsicLogicalWidthsDirty.
//
// See the individual getters below for more details about what each width is.
class CORE_EXPORT LayoutObject : public GarbageCollected<LayoutObject>,
                                 public ImageResourceObserver,
                                 public DisplayItemClient {
  friend class LayoutObjectChildList;
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest, MutableForPaintingClearPaintFlags);
  FRIEND_TEST_ALL_PREFIXES(
      LayoutObjectTest,
      ContainingBlockAbsoluteLayoutObjectShouldBeNonStaticallyPositionedBlockAncestor);
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest,
                           ContainingBlockFixedLayoutObjectInTransformedDiv);
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest,
                           ContainingBlockFixedLayoutObjectInTransformedDiv);
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest,
                           ContainingBlockFixedLayoutObjectInBody);
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest,
                           ContainingBlockAbsoluteLayoutObjectInBody);
  FRIEND_TEST_ALL_PREFIXES(
      LayoutObjectTest,
      ContainingBlockAbsoluteLayoutObjectShouldNotBeNonStaticallyPositionedInlineAncestor);
  FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest, VisualRect);

  friend class VisualRectMappingTest;

 public:
  // Anonymous objects should pass the document as their node, and they will
  // then automatically be marked as anonymous in the constructor.
  explicit LayoutObject(Node*);
  LayoutObject(const LayoutObject&) = delete;
  LayoutObject& operator=(const LayoutObject&) = delete;
  ~LayoutObject() override;
  void Trace(Visitor*) const override;

// Should be added at the beginning of every method to ensure we are not
// accessing a LayoutObject after the Desroy() call.
#if DCHECK_IS_ON()
  ALWAYS_INLINE void CheckIsNotDestroyed() const { DCHECK(!is_destroyed_); }
#else
  ALWAYS_INLINE void CheckIsNotDestroyed() const {}
#endif
#define NOT_DESTROYED() CheckIsNotDestroyed()

  // Returns the name of the layout object.
  virtual const char* GetName() const = 0;

  // Returns the decorated name used by run-layout-tests. The name contains the
  // name of the object along with extra information about the layout object
  // state (e.g. positioning).
  String DecoratedName() const;

  // Returns the decorated name, and DOM node info (tag name and style / class /
  // id attributes, if present).
  String ToString() const;

  // This is an inexact determination of whether the display of this objects is
  // altered or obscured by CSS effects.
  bool HasDistortingVisualEffects() const;

  // Returns false iff this object or one of its ancestors has opacity:0.
  bool HasNonZeroEffectiveOpacity() const;

  // Returns true if the offset ot the containing block depends on the point
  // being mapped.
  bool OffsetForContainerDependsOnPoint(const LayoutObject* container) const;

 protected:
  void EnsureIdForTesting() {
    NOT_DESTROYED();
    fragment_->EnsureId();
  }

 private:
  // DisplayItemClient methods.

  // Hide DisplayItemClient's methods whose names are too generic for
  // LayoutObjects. Should use LayoutObject's methods instead.
  using DisplayItemClient::GetPaintInvalidationReason;
  using DisplayItemClient::Invalidate;
  using DisplayItemClient::IsValid;

  DOMNodeId OwnerNodeId() const override;

 public:
  String DebugName() const final;

  // End of DisplayItemClient methods.

  LayoutObject* Parent() const {
    NOT_DESTROYED();
    return parent_.Get();
  }
  bool IsDescendantOf(const LayoutObject*) const;

  LayoutObject* PreviousSibling() const {
    NOT_DESTROYED();
    return previous_.Get();
  }
  LayoutObject* NextSibling() const {
    NOT_DESTROYED();
    return next_.Get();
  }

  DISABLE_CFI_PERF
  LayoutObject* SlowFirstChild() const {
    NOT_DESTROYED();
    if (const LayoutObjectChildList* children = VirtualChildren())
      return children->FirstChild();
    return nullptr;
  }
  LayoutObject* SlowLastChild() const {
    NOT_DESTROYED();
    if (const LayoutObjectChildList* children = VirtualChildren())
      return children->LastChild();
    return nullptr;
  }

  // See comment in the class description as to why there is no child.
  virtual LayoutObjectChildList* VirtualChildren() {
    NOT_DESTROYED();
    return nullptr;
  }
  virtual const LayoutObjectChildList* VirtualChildren() const {
    NOT_DESTROYED();
    return nullptr;
  }

  LayoutObject* NextInPreOrder() const;
  LayoutObject* NextInPreOrder(const LayoutObject* stay_within) const;
  LayoutObject* NextInPreOrderAfterChildren() const;
  LayoutObject* NextInPreOrderAfterChildren(
      const LayoutObject* stay_within) const;

  // Traverse in the exact reverse of the preorder traversal. In order words,
  // they traverse in the last child -> first child -> root ordering.
  LayoutObject* PreviousInPreOrder() const;
  LayoutObject* PreviousInPreOrder(const LayoutObject* stay_within) const;

  // Traverse in the exact reverse of the postorder traversal. In other words,
  // they traverse in the root -> last child -> first child ordering.
  LayoutObject* PreviousInPostOrder(const LayoutObject* stay_within) const;
  LayoutObject* PreviousInPostOrderBeforeChildren(
      const LayoutObject* stay_within) const;

  // The depth of the tree.
  wtf_size_t Depth() const;

  struct CommonAncestorData {
    STACK_ALLOCATED();

   public:
    // The last object before reaching the common ancestor from |this| and
    // |other|.
    LayoutObject* last = nullptr;
    LayoutObject* other_last = nullptr;
  };
  LayoutObject* CommonAncestor(const LayoutObject& other,
                               CommonAncestorData* data = nullptr) const;

  bool IsBeforeInPreOrder(const LayoutObject& other) const;

  LayoutObject* LastLeafChild() const;

  // The following functions are used when the layout tree hierarchy changes to
  // make sure layers get properly added and removed. Since containership can be
  // implemented by any subclass, and since a hierarchy can contain a mixture of
  // boxes and other object types, these functions need to be in the base class.
  PaintLayer* EnclosingLayer() const;
  void AddLayers(PaintLayer* parent_layer);
  void RemoveLayers(PaintLayer* parent_layer);
  void MoveLayers(PaintLayer* old_parent, PaintLayer* new_parent);
  PaintLayer* FindNextLayer(PaintLayer* parent_layer,
                            LayoutObject* start_point,
                            bool check_parent = true);

  // Returns the layer that will paint this object. During paint invalidation,
  // we should use the faster PaintInvalidatorContext::painting_layer instead.
  PaintLayer* PaintingLayer(int max_depth = -1) const;

  // Convenience function for getting to the nearest enclosing box of a
  // LayoutObject.
  LayoutBox* EnclosingBox() const;

  // Return the NG |LayoutBlockFlow| that will have any |FragmentItems| for
  // |this|, or nullptr if the containing block isn't an NG inline formatting
  // context root. |this| is required to be an object that participates in an
  // inline formatting context (i.e. something inline-level, or a float).
  LayoutBlockFlow* FragmentItemsContainer() const;

  // Return the containing NG block, if the containing block is an NG block,
  // or the LayoutMedia parent.
  // Nullptr otherwise.
  LayoutBox* ContainingNGBox() const;

  // Return the nearest fragmentation context root, if any.
  LayoutBlock* ContainingFragmentationContextRoot() const;

  // Function to return our enclosing flow thread if we are contained inside
  // one. This function follows the containing block chain.
  LayoutFlowThread* FlowThreadContainingBlock() const {
    NOT_DESTROYED();
    if (!IsInsideFlowThread())
      return nullptr;
    return LocateFlowThreadContainingBlock();
  }

#if DCHECK_IS_ON()
  void SetHasAXObject(bool flag) {
    NOT_DESTROYED();
    has_ax_object_ = flag;
  }
  bool HasAXObject() const {
    NOT_DESTROYED();
    return has_ax_object_;
  }

  // Helper class forbidding calls to setNeedsLayout() during its lifetime.
  class SetLayoutNeededForbiddenScope {
    STACK_ALLOCATED();

   public:
    explicit SetLayoutNeededForbiddenScope(LayoutObject&);
    ~SetLayoutNeededForbiddenScope();

   private:
    LayoutObject& layout_object_;
    bool preexisting_forbidden_;
  };

  void AssertLaidOut() const {
    NOT_DESTROYED();
    if (NeedsLayout() && !ChildLayoutBlockedByDisplayLock())
      ShowLayoutTreeForThis();
    DCHECK(!NeedsLayout() || ChildLayoutBlockedByDisplayLock());
  }

  void AssertSubtreeIsLaidOut() const {
    NOT_DESTROYED();
    for (const LayoutObject* layout_object = this; layout_object;
         layout_object = layout_object->ChildLayoutBlockedByDisplayLock()
                             ? layout_object->NextInPreOrderAfterChildren(this)
                             : layout_object->NextInPreOrder(this)) {
      layout_object->AssertLaidOut();
    }
  }

  // This function checks if the fragment tree is consistent with the
  // |LayoutObject| tree. This consistency is critical, as sometimes we traverse
  // the fragment tree, sometimes the |LayoutObject| tree, or mix the
  // traversals. Also we rely on the consistency to avoid using fragments whose
  // |LayoutObject| were destroyed.
  void AssertFragmentTree(bool display_locked = false) const;

  void AssertClearedPaintInvalidationFlags() const;

  void AssertSubtreeClearedPaintInvalidationFlags() const {
    NOT_DESTROYED();
    for (const LayoutObject* layout_object = this; layout_object;
         layout_object = layout_object->ChildPrePaintBlockedByDisplayLock()
                             ? layout_object->NextInPreOrderAfterChildren(this)
                             : layout_object->NextInPreOrder(this)) {
      layout_object->AssertClearedPaintInvalidationFlags();
    }
  }

#endif  // DCHECK_IS_ON()

  // LayoutObject tree manipulation
  //////////////////////////////////////////
  DISABLE_CFI_PERF virtual bool CanHaveChildren() const {
    NOT_DESTROYED();
    return VirtualChildren();
  }
  virtual bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const {
    NOT_DESTROYED();
    return true;
  }

  // This function is called whenever a child is inserted under |this|.
  //
  // The main purpose of this function is to generate a consistent layout
  // tree, which means generating the missing anonymous objects. Most of the
  // time there'll be no anonymous objects to generate.
  //
  // The following invariants are true on the input:
  // - |newChild->node()| is a child of |node()|, if |this| is not
  //   anonymous. If |this| is anonymous, the invariant holds with the
  //   enclosing non-anonymous LayoutObject.
  // - |beforeChild->node()| (if |beforeChild| is provided and not anonymous)
  //   is a sibling of |newChild->node()| (if |newChild| is not anonymous).
  //
  // The reason for these invariants is that insertions are performed on the
  // DOM tree. Because the layout tree may insert extra anonymous renderers,
  // the previous invariants are only guaranteed for the DOM tree. In
  // particular, |beforeChild| may not be a direct child when it's wrapped in
  // anonymous wrappers.
  //
  // Classes inserting anonymous LayoutObjects in the tree are expected to
  // check for the anonymous wrapper case with:
  //                    beforeChild->parent() != this
  //
  // The usage of |child/parent/sibling| in this comment actually means
  // |child/parent/sibling| in a flat tree because a layout tree is generated
  // from a structure of a flat tree if Shadow DOM is used.
  // See LayoutTreeBuilderTraversal and FlatTreeTraversal.
  //
  // See LayoutTable::AddChild and LayoutBlockFlow::AddChild.
  // TODO(jchaffraix): |newChild| cannot be nullptr and should be a reference.
  virtual void AddChild(LayoutObject* new_child,
                        LayoutObject* before_child = nullptr);
  virtual void AddChildIgnoringContinuation(
      LayoutObject* new_child,
      LayoutObject* before_child = nullptr) {
    NOT_DESTROYED();
    return AddChild(new_child, before_child);
  }
  virtual void RemoveChild(LayoutObject*);
  //////////////////////////////////////////

  UniqueObjectId UniqueId() const {
    NOT_DESTROYED();
    return fragment_->UniqueId();
  }

  // Returns true if the overflow property should be respected. Otherwise
  // HasNonVisibleOverflow() will be false and we won't create scrollable area
  // for this object even if overflow is non-visible.
  virtual bool RespectsCSSOverflow() const {
    NOT_DESTROYED();
    return false;
  }

  inline bool ShouldApplyOverflowClipMargin() const {
    NOT_DESTROYED();
    // If the object is clipped by something other than overflow:clip (i.e. it's
    // a scroll container), then we should not apply overflow-clip-margin.
    if (IsScrollContainer())
      return false;

    const auto& style = StyleRef();
    // Nothing to apply if there is no margin.
    if (!style.OverflowClipMarginHasAnEffect()) {
      return false;
    }

    // Replaced elements have a used value of 'clip' for all overflow values
    // except visible. See discussion at:
    // https://github.com/w3c/csswg-drafts/issues/7714#issuecomment-1248761712
    bool is_overflow_clip = false;
    if (IsLayoutReplaced()) {
      is_overflow_clip = style.OverflowX() != EOverflow::kVisible &&
                         style.OverflowY() != EOverflow::kVisible;
    } else {
      is_overflow_clip = style.OverflowX() == EOverflow::kClip &&
                         style.OverflowY() == EOverflow::kClip;
    }

    // In all other cases, we apply overflow-clip-margin when we clip to
    // overflow clip edge, meaning we have overflow: clip or paint containment.
    // Also only apply this if the element respects overflow css, meaning it
    // allows non-visible overflow.
    return (is_overflow_clip || ShouldApplyPaintContainment()) &&
           RespectsCSSOverflow();
  }

  inline bool IsEligibleForPaintOrLayoutContainment() const {
    NOT_DESTROYED();
    return (!IsInline() || IsAtomicInlineLevel()) &&
           (!IsTablePart() || IsLayoutBlockFlow());
  }

  inline bool ShouldApplyPaintContainment(const ComputedStyle& style) const {
    NOT_DESTROYED();
    return style.ContainsPaint() && IsEligibleForPaintOrLayoutContainment();
  }

  inline bool ShouldApplyPaintContainment() const {
    NOT_DESTROYED();
    return ShouldApplyPaintContainment(StyleRef());
  }

  inline bool ShouldApplyLayoutContainment(const ComputedStyle& style) const {
    NOT_DESTROYED();
    return style.ContainsLayout() && IsEligibleForPaintOrLayoutContainment();
  }

  inline bool ShouldApplyLayoutContainment() const {
    NOT_DESTROYED();
    return ShouldApplyLayoutContainment(StyleRef());
  }

  inline bool IsEligibleForSizeContainment() const {
    NOT_DESTROYED();
    return (!IsInline() || IsAtomicInlineLevel()) &&
           (!IsTablePart() || IsTableCaption()) && !IsTable();
  }
  inline bool ShouldApplySizeContainment() const {
    NOT_DESTROYED();
    return StyleRef().ContainsSize() && IsEligibleForSizeContainment();
  }
  inline bool ShouldApplyInlineSizeContainment() const {
    NOT_DESTROYED();
    return StyleRef().ContainsInlineSize() && IsEligibleForSizeContainment();
  }
  inline bool ShouldApplyBlockSizeContainment() const {
    NOT_DESTROYED();
    return StyleRef().ContainsBlockSize() && IsEligibleForSizeContainment();
  }
  inline bool ShouldApplyAnySizeContainment() const {
    NOT_DESTROYED();
    return StyleRef().ContainsAnySize() && IsEligibleForSizeContainment();
  }
  inline bool ShouldApplyStyleContainment() const {
    NOT_DESTROYED();
    return StyleRef().ContainsStyle();
  }
  inline bool ShouldApplyContentContainment() const {
    NOT_DESTROYED();
    return ShouldApplyStyleContainment() && ShouldApplyPaintContainment() &&
           ShouldApplyLayoutContainment();
  }
  inline bool ShouldApplyStrictContainment() const {
    NOT_DESTROYED();
    return ShouldApplyStyleContainment() && ShouldApplyPaintContainment() &&
           ShouldApplyLayoutContainment() && ShouldApplySizeContainment();
  }
  inline bool ShouldApplyAnyContainment() const {
    NOT_DESTROYED();
    return ShouldApplyPaintContainment() || ShouldApplyLayoutContainment() ||
           ShouldApplyStyleContainment() || ShouldApplyBlockSizeContainment() ||
           ShouldApplyInlineSizeContainment();
  }

  inline bool CanMatchSizeContainerQueries() const {
    NOT_DESTROYED();
    if (Element* element = DynamicTo<Element>(GetNode()))
      return StyleRef().CanMatchSizeContainerQueries(*element);
    return false;
  }

  inline bool IsStackingContext() const {
    NOT_DESTROYED();
    return IsStackingContext(StyleRef());
  }
  inline bool IsStackingContext(const ComputedStyle& style) const {
    NOT_DESTROYED();
    // This is an inlined version of the following:
    // `IsStackingContextWithoutContainment() ||
    //  ShouldApplyLayoutContainment() ||
    //  ShouldApplyPaintContainment()`
    // The reason it is inlined is that the containment checks share
    // common logic, which is extracted here to avoid repeated computation.
    return style.IsStackingContextWithoutContainment() ||
           ((style.ContainsLayout() || style.ContainsPaint()) &&
            (!IsInline() || IsAtomicInlineLevel()) &&
            (!IsTablePart() || IsLayoutBlockFlow()));
  }

  inline bool IsStacked() const {
    NOT_DESTROYED();
    return IsStacked(StyleRef());
  }
  inline bool IsStacked(const ComputedStyle& style) const {
    NOT_DESTROYED();
    return style.GetPosition() != EPosition::kStatic ||
           IsStackingContext(style);
  }

  // Returns true if the LayoutObject is rendered in the top layer or the layer
  // for view transitions. Such objects are rendered as subsequent siblings of
  // the root element box and have specific stacking requirements.
  bool IsInTopOrViewTransitionLayer() const;

  void NotifyPriorityScrollAnchorStatusChanged();

 private:
  //////////////////////////////////////////
  // Helper functions. Dangerous to use!
  void SetPreviousSibling(LayoutObject* previous) {
    NOT_DESTROYED();
    previous_ = previous;
  }
  void SetNextSibling(LayoutObject* next) {
    NOT_DESTROYED();
    next_ = next;
  }
  void SetParent(LayoutObject* parent) {
    NOT_DESTROYED();
    parent_ = parent;

    // Only update if our flow thread state is different from our new parent and
    // if we're not a LayoutFlowThread.
    // A LayoutFlowThread is always considered to be inside itself, so it never
    // has to change its state in response to parent changes.
    bool inside_flow_thread = parent && parent->IsInsideFlowThread();
    if (inside_flow_thread != IsInsideFlowThread() && !IsLayoutFlowThread())
      SetIsInsideFlowThreadIncludingDescendants(inside_flow_thread);
  }

  //////////////////////////////////////////
 private:
#if DCHECK_IS_ON()
  bool IsSetNeedsLayoutForbidden() const {
    NOT_DESTROYED();
    return set_needs_layout_forbidden_;
  }
  void SetNeedsLayoutIsForbidden(bool flag) {
    NOT_DESTROYED();
    set_needs_layout_forbidden_ = flag;
  }
#endif

  void AddAbsoluteRectForLayer(gfx::Rect& result);

 protected:
  // A helper for AddChild().
  bool RequiresAnonymousTableWrappers(const LayoutObject*) const;

 public:
#if DCHECK_IS_ON()
  // Dump this layout object to the specified string builder.
  void DumpLayoutObject(StringBuilder&,
                        bool dump_address,
                        unsigned show_tree_character_offset) const;
  void ShowTreeForThis() const;
  void ShowLayoutTreeForThis() const;
  void ShowLayoutObject() const;

  // Dump the subtree established by this layout object to the specified string
  // builder. There will be one object per line, and descendants will be
  // indented according to their tree level. The optional "marked_foo"
  // parameters can be used to mark up to two objects in the subtree with a
  // label.
  void DumpLayoutTreeAndMark(StringBuilder&,
                             const LayoutObject* marked_object1 = nullptr,
                             const char* marked_label1 = nullptr,
                             const LayoutObject* marked_object2 = nullptr,
                             const char* marked_label2 = nullptr,
                             unsigned depth = 0) const;
#endif  // DCHECK_IS_ON()

  // This function is used to create the appropriate LayoutObject based
  // on the style, in particular 'display' and 'content'.
  // "display: none" or "display: contents" are the only times this function
  // will return nullptr.
  //
  // For renderer creation, the inline-* values create the same renderer
  // as the non-inline version. The difference is that inline-* sets
  // is_inline_ during initialization. This means that
  // "display: inline-table" creates a LayoutTable, like "display: table".
  //
  // Ideally every Element::createLayoutObject would call this function to
  // respond to 'display' but there are deep rooted assumptions about
  // which LayoutObject is created on a fair number of Elements. This
  // function also doesn't handle the default association between a tag
  // and its renderer (e.g. <iframe> creates a LayoutIFrame even if the
  // initial 'display' value is inline).
  static LayoutObject* CreateObject(Element*, const ComputedStyle&);
  static LayoutBlockFlow* CreateBlockFlowOrListItem(Element* element,
                                                    const ComputedStyle& style);

  bool IsPseudoElement() const {
    NOT_DESTROYED();
    return GetNode() && GetNode()->IsPseudoElement();
  }

  virtual bool IsBoxModelObject() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsBox() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsText() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsBR() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsCanvas() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsCounter() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsEmbeddedObject() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsFieldset() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsFrame() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsFrameSet() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsFlexibleBox() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutListItem() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsInlineListItem() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutInsideListMarker() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutOutsideListMarker() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutTextCombine() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutTableCol() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsListMarkerImage() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsMathML() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsMathMLRoot() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsMedia() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsProgress() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsQuote() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutCustom() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutGrid() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutIFrame() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutImage() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutMasonry() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutMultiColumnSet() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutMultiColumnSpannerPlaceholder() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutReplaced() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutCustomScrollbarPart() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutView() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsRuby() const {
    NOT_DESTROYED();
    return false;
  }
  bool IsInlineRuby() const;
  bool IsInlineRubyText() const;
  virtual bool IsTable() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTableCaption() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTableCell() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTableRow() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTableSection() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTextArea() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsTextField() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsVideo() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsImage() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsViewTransitionContent() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsViewTransitionRoot() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutBlock() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutBlockFlow() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutFlowThread() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutInline() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutEmbeddedContent() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsLayoutNGObject() const {
    NOT_DESTROYED();
    return false;
  }

  bool IsTextControl() const {
    NOT_DESTROYED();
    return IsTextArea() || IsTextField();
  }

  bool IsDocumentElement() const {
    NOT_DESTROYED();
    return GetDocument().documentElement() == node_;
  }
  // isBody is called from LayoutBox::styleWillChange and is thus quite hot.
  bool IsBody() const {
    NOT_DESTROYED();
    return GetNode() && GetNode()->HasTagName(html_names::kBodyTag);
  }

  bool IsHR() const;
  bool IsButtonOrInputButton() const;
  bool IsInputButton() const;
  bool IsMenuList() const;
  bool IsListBox() const;

  bool IsTablePart() const {
    NOT_DESTROYED();
    return IsTableCell() || IsLayoutTableCol() || IsTableCaption() ||
           IsTableRow() || IsTableSection();
  }
  inline bool IsCheckContent() const;
  inline bool IsBeforeContent() const;
  inline bool IsAfterContent() const;
  inline bool IsPickerIconContent() const;
  inline bool IsMarkerContent() const;
  inline bool IsScrollButtonContent() const;
  inline bool IsScrollMarkerContent() const;
  inline bool IsScrollButtonOrMarkerContent() const;
  inline bool IsBeforeOrAfterContent() const;
  static inline bool IsAfterContent(const LayoutObject* obj) {
    return obj && obj->IsAfterContent();
  }
  static inline bool IsPickerIconContent(const LayoutObject* obj) {
    return obj && obj->IsPickerIconContent();
  }

  // Returns true if the text is generated (from, e.g., list marker,
  // pseudo-element, ...) instead of from a DOM text node. See
  // |TextFragmentType::kLayoutGenerated| for the other type of generated text.
  bool IsStyleGenerated() const;

  // |PhysicalAnchorQuery| is built and propagated up in the fragment tree
  // during the layout. This function indicates whether |this| may have an
  // anchor query or not before the layout. When it returns false, |this| does
  // not have an |PhysicalAnchorQuery|.
  bool MayHaveAnchorQuery() const {
    NOT_DESTROYED();
    return bitfields_.MayHaveAnchorQuery();
  }
  void SetSelfMayHaveAnchorQuery() {
    NOT_DESTROYED();
    bitfields_.SetMayHaveAnchorQuery(true);
  }
  virtual void MarkMayHaveAnchorQuery();

  void SetHasBrokenSpine() {
    NOT_DESTROYED();
    bitfields_.SetHasBrokenSpine(true);
  }
  void ClearHasBrokenSpine() {
    NOT_DESTROYED();
    bitfields_.SetHasBrokenSpine(false);
  }
  bool HasBrokenSpine() const {
    NOT_DESTROYED();
    return bitfields_.HasBrokenSpine();
  }

  bool IsTruncated() const {
    NOT_DESTROYED();
    return bitfields_.IsTruncated();
  }
  void SetIsTruncated(bool is_truncated) {
    NOT_DESTROYED();
    bitfields_.SetIsTruncated(is_truncated);
  }

  bool EverHadLayout() const {
    NOT_DESTROYED();
    return bitfields_.EverHadLayout();
  }

  bool ChildrenInline() const {
    NOT_DESTROYED();
    return bitfields_.ChildrenInline();
  }
  void SetChildrenInline(bool b) {
    NOT_DESTROYED();
    bitfields_.SetChildrenInline(b);
  }

  bool AlwaysCreateLineBoxesForLayoutInline() const {
    NOT_DESTROYED();
    DCHECK(IsLayoutInline());
    return bitfields_.AlwaysCreateLineBoxesForLayoutInline();
  }
  void SetAlwaysCreateLineBoxesForLayoutInline(bool always_create_line_boxes) {
    NOT_DESTROYED();
    DCHECK(IsLayoutInline());
    bitfields_.SetAlwaysCreateLineBoxesForLayoutInline(
        always_create_line_boxes);
  }

  void SetIsInsideFlowThreadIncludingDescendants(bool);

  bool IsInsideFlowThread() const {
    NOT_DESTROYED();
    return bitfields_.IsInsideFlowThread();
  }
  void SetIsInsideFlowThread(bool inside_flow_thread) {
    NOT_DESTROYED();
    bitfields_.SetIsInsideFlowThread(inside_flow_thread);
  }

  // Remove this object and all descendants from the containing
  // LayoutFlowThread.
  void RemoveFromLayoutFlowThread();

  // Return true if this object might be inside a fragmentation context, or
  // false if it's definitely *not* inside one.
  bool MightBeInsideFragmentationContext() const {
    NOT_DESTROYED();
    return IsInsideFlowThread() ||
           (GetDocument().Printing() && !IsLayoutView());
  }

  // FIXME: Until all SVG layoutObjects can be subclasses of
  // LayoutSVGModelObject we have to add SVG layoutObject methods to
  // LayoutObject with an NOTREACHED() default implementation.
  virtual bool IsSVG() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGRoot() const {
    NOT_DESTROYED();
    return false;
  }
  bool IsSVGChild() const {
    NOT_DESTROYED();
    return IsSVG() && !IsSVGRoot();
  }
  virtual bool IsSVGContainer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGTransformableContainer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGViewportContainer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGHiddenContainer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGShape() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGTextPath() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGTSpan() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGInline() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGInlineText() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGImage() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGForeignObject() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGResourceContainer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGFilterPrimitive() const {
    NOT_DESTROYED();
    return false;
  }
  virtual bool IsSVGText() const {
    NOT_DESTROYED();
    return false;
  }

  // FIXME: Those belong into a SVG specific base-class for all layoutObjects
  // (see above). Unfortunately we don't have such a class yet, because it's not
  // possible for all layoutObjects to inherit from LayoutSVGObject ->
  // LayoutObject (some need LayoutBlock inheritance for instance)
  virtual void SetNeedsTransformUpdate() { NOT_DESTROYED(); }
  virtual void SetNeedsBoundariesUpdate() { NOT_DESTROYED(); }

  // Per the spec, mix-blend-mode applies to all non-SVG elements, and SVG
  // elements that are container elements, graphics elements or graphics
  // referencing elements.
  // https://www.w3.org/TR/compositing-1/#propdef-mix-blend-mode
  bool IsBlendingAllowed() const {
    NOT_DESTROYED();
    return !IsSVG() || IsSVGShape() || IsSVGImage() || IsSVGInline() ||
           IsSVGRoot() || IsSVGForeignObject() || IsSVGText() ||
           // Blending does not apply to non-renderable elements such as
           // patterns (see: https://github.com/w3c/fxtf-drafts/issues/309).
           (IsSVGContainer() && !IsSVGHiddenContainer());
  }
  virtual bool HasNonIsolatedBlendingDescendants() const {
    NOT_DESTROYED();
    // This is only implemented for layout objects that containt SVG flow.
    // For HTML/CSS layout objects, use the PaintLayer version instead.
    DCHECK(IsSVG());
    return false;
  }
  enum DescendantIsolationState {
    kDescendantIsolationRequired,
    kDescendantIsolationNeedsUpdate,
  };
  virtual void DescendantIsolationRequirementsChanged(
      DescendantIsolationState) {
    NOT_DESTROYED();
  }

  // Per SVG 1.1 objectBoundingBox ignores clipping, masking, filter effects,
  // opacity and stroke-width.
  // This is used for all computation of objectBoundingBox relative units and by
  // SVGGraphicsElement::getBBox().
  // NOTE: Markers are not specifically ignored here by SVG 1.1 spec, but we
  // ignore them since stroke-width is ignored (and marker size can depend on
  // stroke-width). objectBoundingBox is returned in local coordinates and
  // always unzoomed.
  // The name objectBoundingBox is taken from the SVG 1.1 spec.
  virtual gfx::RectF ObjectBoundingBox() const;

  // Returns the smallest rectangle enclosing all of the painted content
  // respecting clipping, masking, filters, opacity, stroke-width and markers.
  // The local SVG coordinate space is the space where localSVGTransform
  // applies. For SVG objects defining viewports (e.g.
  // LayoutSVGViewportContainer and  LayoutSVGResourceMarker), the local SVG
  // coordinate space is the viewport space.
  virtual gfx::RectF VisualRectInLocalSVGCoordinates() const;

  // Compute the SVG stroke bounding box per
  // https://www.w3.org/TR/SVG2/coords.html#TermStrokeBoundingBox .
  virtual gfx::RectF StrokeBoundingBox() const;

  // Like VisualRectInLocalSVGCoordinates() but does not include visual overflow
  // (name is misleading). May be zoomed (currently only for <foreignObject>,
  // which represents this via its LocalToSVGParentTransform()).
  // It mostly corresponds to the "decorated bounding box" from the SVG spec.
  // (https://svgwg.org/svg2-draft/coords.html#BoundingBoxes)
  virtual gfx::RectF DecoratedBoundingBox() const;

  // This returns the transform applying to the local SVG coordinate space,
  // which combines the CSS transform properties and animation motion transform.
  // See SVGElement::calculateTransform().
  // Most callsites want localToSVGParentTransform() instead.
  virtual AffineTransform LocalSVGTransform() const;

  // Returns the full transform mapping from local coordinates to parent's local
  // coordinates. For most SVG objects, this is the same as localSVGTransform.
  // For SVG objects defining viewports (see visualRectInLocalSVGCoordinates),
  // this includes any viewport transforms and x/y offsets as well as
  // localSVGTransform.
  virtual AffineTransform LocalToSVGParentTransform() const {
    NOT_DESTROYED();
    return LocalSVGTransform();
  }

  // End of SVG-specific methods.

  bool IsAnonymous() const {
    NOT_DESTROYED();
    return bitfields_.IsAnonymous();
  }
  bool IsAnonymousBlock() const {
    NOT_DESTROYED();
    // This function is kept in sync with anonymous block creation conditions in
    // LayoutBlock::createAnonymousBlock(). This includes creating an anonymous
    // LayoutBlock having a BLOCK or BOX display. Other classes such as
    // LayoutTextFragment are not LayoutBlocks and will return false.
    // See https://bugs.webkit.org/show_bug.cgi?id=56709.
    return IsAnonymous() &&
           (StyleRef().Display() == EDisplay::kBlock ||
            StyleRef().Display() == EDisplay::kWebkitBox) &&
           StyleRef().StyleType() == kPseudoIdNone && IsLayoutBlock() &&
           !IsLayoutFlowThread() && !IsLayoutMultiColumnSet();
  }

  bool IsFloating() const {
    NOT_DESTROYED();
    return bitfields_.Floating();
  }

  virtual bool IsInitialLetterBox() const {
    NOT_DESTROYED();
    return false;
  }

  // absolute or fixed positioning
  bool IsOutOfFlowPositioned() const {
    NOT_DESTROYED();
    return positioned_state_ == kIsOutOfFlowPositioned;
  }
  bool IsRelPositioned() const {
    NOT_DESTROYED();
    return positioned_state_ == kIsRelativelyPositioned;
  }
  bool IsStickyPositioned() const {
    NOT_DESTROYED();
    return positioned_state_ == kIsStickyPositioned;
  }
  bool IsFixedPositioned() const {
    NOT_DESTROYED();
    return IsOutOfFlowPositioned() &&
           StyleRef().GetPosition() == EPosition::kFixed;
  }
  bool IsAbsolutePositioned() const {
    NOT_DESTROYED();
    return IsOutOfFlowPositioned() &&
           StyleRef().GetPosition() == EPosition::kAbsolute;
  }
  bool IsPositioned() const {
    NOT_DESTROYED();
    return positioned_state_ != kIsStaticallyPositioned;
  }
  bool IsInline() const {
    NOT_DESTROYED();
    return bitfields_.IsInline();
  }  // inline object
  bool IsInLayoutNGInlineFormattingContext() const {
    NOT_DESTROYED();
    return bitfields_.IsInLayoutNGInlineFormattingContext();
  }
  bool IsAtomicInlineLevel() const {
    NOT_DESTROYED();
    return bitfields_.IsAtomicInlineLevel();
  }
  bool IsBlockInInline() const {
    NOT_DESTROYED();
    return IsAnonymous() && !IsInline() && !IsFloatingOrOutOfFlowPositioned() &&
           Parent() && Parent()->IsLayoutInline();
  }
  bool IsHorizontalWritingMode() const {
    NOT_DESTROYED();
    return bitfields_.HorizontalWritingMode();
  }
  bool IsHorizontalTypographicMode() const {
    NOT_DESTROYED();
    return IsHorizontalWritingMode() ||
           StyleRef().IsHorizontalTypographicMode();
  }
  bool HasFlippedBlocksWritingMode() const {
    NOT_DESTROYED();
    return StyleRef().IsFlippedBlocksWritingMode();
  }

  bool HasLayer() const {
    NOT_DESTROYED();
    return bitfields_.HasLayer();
  }

  // This may be different from StyleRef().hasBoxDecorationBackground() because
  // some objects may have box decoration background other than from their own
  // style.
  bool HasBoxDecorationBackground() const {
    NOT_DESTROYED();
    return bitfields_.HasBoxDecorationBackground();
  }

  bool NeedsLayout() const {
    NOT_DESTROYED();
    return bitfields_.SelfNeedsFullLayout() ||
           bitfields_.ChildNeedsFullLayout() ||
           bitfields_.NeedsSimplifiedLayout();
  }

  bool NeedsSimplifiedLayoutOnly() const {
    NOT_DESTROYED();
    return bitfields_.NeedsSimplifiedLayout() &&
           !bitfields_.SelfNeedsFullLayout() &&
           !bitfields_.ChildNeedsFullLayout();
  }

  bool SelfNeedsFullLayout() const {
    NOT_DESTROYED();
    return bitfields_.SelfNeedsFullLayout();
  }
  bool ChildNeedsFullLayout() const {
    NOT_DESTROYED();
    return bitfields_.ChildNeedsFullLayout();
  }
  bool NeedsSimplifiedLayout() const {
    NOT_DESTROYED();
    return bitfields_.NeedsSimplifiedLayout();
  }
  bool NeedsCollectInlines() const {
    NOT_DESTROYED();
    return bitfields_.NeedsCollectInlines();
  }

  // Return true if the min/max intrinsic logical widths aren't up-to-date.
  // Note that for objects that *don't* need to calculate intrinsic logical
  // widths (e.g. if inline-size is a fixed value, and no other inline lengths
  // are intrinsic, and the object isn't a descendant of something that needs
  // min/max), this flag will never be cleared (since the values will never be
  // calculated).
  bool IntrinsicLogicalWidthsDirty() const {
    NOT_DESTROYED();
    return bitfields_.IntrinsicLogicalWidthsDirty();
  }

  bool IntrinsicLogicalWidthsDependsOnBlockConstraints() const {
    NOT_DESTROYED();
    return bitfields_.IntrinsicLogicalWidthsDependsOnBlockConstraints();
  }
  void SetIntrinsicLogicalWidthsDependsOnBlockConstraints(bool b) {
    NOT_DESTROYED();
    bitfields_.SetIntrinsicLogicalWidthsDependsOnBlockConstraints(b);
  }
  bool IndefiniteIntrinsicLogicalWidthsDirty() const {
    NOT_DESTROYED();
    return bitfields_.IndefiniteIntrinsicLogicalWidthsDirty();
  }
  void SetIndefiniteIntrinsicLogicalWidthsDirty(bool b) {
    NOT_DESTROYED();
    bitfields_.SetIndefiniteIntrinsicLogicalWidthsDirty(b);
  }
  bool DefiniteIntrinsicLogicalWidthsDirty() const {
    NOT_DESTROYED();
    return bitfields_.DefiniteIntrinsicLogicalWidthsDirty();
  }
  void SetDefiniteIntrinsicLogicalWidthsDirty(bool b) {
    NOT_DESTROYED();
    bitfields_.SetDefiniteIntrinsicLogicalWidthsDirty(b);
  }

  bool NeedsScrollableOverflowRecalc() const {
    NOT_DESTROYED();
    return bitfields_.SelfNeedsScrollableOverflowRecalc() ||
           bitfields_.ChildNeedsScrollableOverflowRecalc();
  }
  bool SelfNeedsScrollableOverflowRecalc() const {
    NOT_DESTROYED();
    return bitfields_.SelfNeedsScrollableOverflowRecalc();
  }
  bool ChildNeedsScrollableOverflowRecalc() const {
    NOT_DESTROYED();
    return bitfields_.ChildNeedsScrollableOverflowRecalc();
  }
  void SetSelfNeedsScrollableOverflowRecalc() {
    NOT_DESTROYED();
    bitfields_.SetSelfNeedsScrollableOverflowRecalc(true);
  }
  void SetChildNeedsScrollableOverflowRecalc() {
    NOT_DESTROYED();
    bitfields_.SetChildNeedsScrollableOverflowRecalc(true);
  }
  void ClearSelfNeedsScrollableOverflowRecalc() {
    NOT_DESTROYED();
    bitfields_.SetSelfNeedsScrollableOverflowRecalc(false);
  }
  void ClearChildNeedsScrollableOverflowRecalc() {
    NOT_DESTROYED();
    bitfields_.SetChildNeedsScrollableOverflowRecalc(false);
  }

  // CSS clip only applies when position is absolute or fixed. Prefer this check
  // over !StyleRef().HasAutoClip().
  bool HasClip() const {
    NOT_DESTROYED();
    return IsOutOfFlowPositioned() && !StyleRef().HasAutoClip();
  }
  bool HasNonVisibleOverflow() const {
    NOT_DESTROYED();
    return bitfields_.HasNonVisibleOverflow();
  }
  bool HasClipRelatedProperty() const;
  bool IsScrollContainer() const {
    NOT_DESTROYED();
    // Replaced elements don't support scrolling. If overflow is non visible,
    // the behaviour applied is equivalent to `clip`. See:
    // https://github.com/w3c/csswg-drafts/issues/7435
    if (IsLayoutReplaced()) {
      return false;
    }
    // Always check HasNonVisibleOverflow() in case the object is not allowed to
    // have non-visible overflow.
    return HasNonVisibleOverflow() && StyleRef().IsScrollContainer();
  }

  bool IsScrollContainerWithScrollMarkerGroup() const {
    NOT_DESTROYED();
    return (IsScrollContainer() || IsDocumentElement()) &&
           !Style()->ScrollMarkerGroupNone();
  }

  // Not returning StyleRef().HasTransformRelatedProperty() because some objects
  // ignore the transform-related styles (e.g., LayoutInline).
  bool HasTransformRelatedProperty() const {
    NOT_DESTROYED();
    return bitfields_.HasTransformRelatedProperty();
  }
  // Compared to StyleRef().HasTransform(), this excludes objects that ignore
  // transform-related styles (e.g. LayoutInline).
  bool HasTransform() const {
    NOT_DESTROYED();
    return HasTransformRelatedProperty() && StyleRef().HasTransform();
  }
  // Similar to the above.
  bool Preserves3D() const {
    NOT_DESTROYED();
    return HasTransformRelatedProperty() && StyleRef().Preserves3D() &&
           !IsSVGChild();
  }
  bool IsTransformApplicable() const {
    NOT_DESTROYED();
    return IsBox() || IsSVG();
  }

  bool HasMask() const {
    NOT_DESTROYED();
    return StyleRef().HasMask();
  }
  bool HasClipPath() const {
    NOT_DESTROYED();
    return StyleRef().HasClipPath();
  }
  bool HasHiddenBackface() const {
    NOT_DESTROYED();
    return StyleRef().BackfaceVisibility() == EBackfaceVisibility::kHidden;
  }
  bool HasNonInitialBackdropFilter() const {
    NOT_DESTROYED();
    return StyleRef().HasNonInitialBackdropFilter();
  }

  // Returns |true| if any property that renders using filter operations is
  // used (including, but not limited to, 'filter' and 'box-reflect').
  // Not calling StyleRef().HasFilterInducingProperty() because some objects
  // ignore reflection style (e.g. LayoutInline, LayoutSVGBlock).
  bool HasFilterInducingProperty() const {
    NOT_DESTROYED();
    return StyleRef().HasNonInitialFilter() || HasReflection();
  }

  bool HasShapeOutside() const {
    NOT_DESTROYED();
    return StyleRef().ShapeOutside();
  }

  // Return true if the given object is the effective root scroller in its
  // Document. See |effective root scroller| in page/scrolling/README.md.
  // Note: a root scroller always establishes a PaintLayer.
  // This bit is updated in
  // RootScrollerController::RecomputeEffectiveRootScroller in the LayoutClean
  // document lifecycle phase.
  bool IsEffectiveRootScroller() const {
    NOT_DESTROYED();
    return bitfields_.IsEffectiveRootScroller();
  }

  // Returns true if the given object is the global root scroller. See
  // |global root scroller| in page/scrolling/README.md.
  bool IsGlobalRootScroller() const {
    NOT_DESTROYED();
    return bitfields_.IsGlobalRootScroller();
  }

  bool IsHTMLLegendElement() const {
    NOT_DESTROYED();
    return bitfields_.IsHTMLLegendElement();
  }

  // Returns true if this can be used as a rendered legend.
  bool IsRenderedLegendCandidate() const {
    NOT_DESTROYED();
    // Note, we can't directly use LayoutObject::IsFloating() because in the
    // case where the legend is a flex/grid item, LayoutObject::IsFloating()
    // could get set to false, even if the legend's computed style indicates
    // that it is floating.
    return IsHTMLLegendElement() && !IsOutOfFlowPositioned() &&
           !Style()->IsFloating();
  }

  // Return true if this is the "rendered legend" of a fieldset. They get
  // special treatment, in that they establish a new formatting context, and
  // shrink to fit if no logical width is specified.
  //
  // This function is performance sensitive.
  inline bool IsRenderedLegend() const {
    NOT_DESTROYED();
    if (!IsRenderedLegendCandidate()) [[likely]] {
      return false;
    }

    return IsRenderedLegendInternal();
  }

  bool IsRenderedLegendInternal() const;

  bool IsScrollMarker() const;
  bool IsScrollMarkerGroup() const;
  bool IsScrollMarkerGroupBefore() const;

  // Returns true if this object represents ::marker for the first SUMMARY
  // child of a DETAILS, and list-style-type is disclosure-*.
  bool IsListMarkerForSummary() const;

  // Returns true if this object is a proper descendant of any list marker.
  bool IsInListMarker() const;

  // The pseudo element style can be cached or uncached. Use the cached method
  // if the pseudo element doesn't respect any pseudo classes (and therefore
  // has no concept of changing state). The cached pseudo style always inherits
  // from the originating element's style (because we can cache only one
  // version), while the uncached pseudo style can inherit from any style.
  const ComputedStyle* GetCachedPseudoElementStyle(PseudoId) const;
  const ComputedStyle* GetUncachedPseudoElementStyle(const StyleRequest&) const;

  // Returns the ::selection style, which may be stored in StyleCachedData (old
  // impl) or StyleHighlightData (new impl).
  // TODO(crbug.com/1024156): inline and remove on shipping HighlightInheritance
  const ComputedStyle* GetSelectionStyle() const;

  LayoutView* View() const {
    NOT_DESTROYED();
    return GetDocument().GetLayoutView();
  }
  LocalFrameView* GetFrameView() const {
    NOT_DESTROYED();
    return GetDocument().View();
  }

  bool IsRooted() const;

  Node* GetNode() const {
    NOT_DESTROYED();
    return IsAnonymous() ? nullptr : node_.Get();
  }

  Node* NonPseudoNode() const {
    NOT_DESTROYED();
    return IsPseudoElement() ? nullptr : GetNode();
  }

  void ClearNode() {
    NOT_DESTROYED();
    node_ = nullptr;
  }

  // Returns the styled node that caused the generation of this layoutObject.
  // This is the same as node() except for layoutObjects of :before, :after and
  // :first-letter pseudo elements for which their parent node is returned.
  Node* GeneratingNode() const {
    NOT_DESTROYED();
    return IsPseudoElement() ? GetNode()->ParentOrShadowHostNode() : GetNode();
  }

  // Return the Node of this object, or, if it has none (anonymous object),
  // return that of the nearest ancestor that has one.
  Node* EnclosingNode() const;

  Document& GetDocument() const {
    NOT_DESTROYED();
    DCHECK(node_ || Parent());  // crbug.com/402056
    return node_ ? node_->GetDocument() : Parent()->GetDocument();
  }
  LocalFrame* GetFrame() const {
    NOT_DESTROYED();
    return GetDocument().GetFrame();
  }

  virtual LayoutMultiColumnSpannerPlaceholder* SpannerPlaceholder() const {
    NOT_DESTROYED();
    return nullptr;
  }
  bool IsColumnSpanAll() const {
    NOT_DESTROYED();
    return StyleRef().GetColumnSpan() == EColumnSpan::kAll &&
           SpannerPlaceholder();
  }

  // We include LayoutButton in this check, because buttons are
  // implemented using flex box but should still support things like
  // first-line, first-letter and text-overflow.
  // The flex box and grid specs require that flex box and grid do not
  // support first-line|first-letter, though.
  // When LayoutObject and display do not agree, allow first-line|first-letter
  // only when both indicate it's a block container.
  // TODO(cbiesinger): Remove when buttons are implemented with align-items
  // instead of flex box. crbug.com/226252.
  bool BehavesLikeBlockContainer() const {
    NOT_DESTROYED();
    return IsLayoutBlockFlow() && StyleRef().IsDisplayBlockContainer();
  }

  // May be optionally passed to container() and various other similar methods
  // that search the ancestry for some sort of containing block. Used to
  // determine if we skipped certain objects while walking the ancestry.
  class AncestorSkipInfo {
    STACK_ALLOCATED();

   public:
    AncestorSkipInfo(const LayoutObject* ancestor,
                     bool check_for_filters = false)
        : ancestor_(ancestor), check_for_filters_(check_for_filters) {}

    // Update skip info output based on the layout object passed.
    void Update(const LayoutObject& object) {
      if (&object == ancestor_)
        ancestor_skipped_ = true;
      if (check_for_filters_ && object.HasFilterInducingProperty())
        filter_skipped_ = true;
    }

#if DCHECK_IS_ON()
    void AssertClean() {
      DCHECK(!ancestor_skipped_);
      DCHECK(!filter_skipped_);
    }
#endif

    bool AncestorSkipped() const { return ancestor_skipped_; }
    bool FilterSkipped() const {
      DCHECK(check_for_filters_);
      return filter_skipped_;
    }

   private:
    // Input: A potential ancestor to look for. If we walk past this one while
    // walking the ancestry in search of some containing block, ancestorSkipped
    // will be set to true.
    const LayoutObject* ancestor_;
    // Input: When set, we'll check if we skip objects with filter inducing
    // properties.
    bool check_for_filters_;

    // Output: Set to true if |ancestor| was walked past while walking the
    // ancestry.
    bool ancestor_skipped_ = false;
    // Output: Set to true if we walked past a filter object. This will be set
    // regardless of the value of |ancestor|.
    bool filter_skipped_ = false;
  };

  // This function returns the containing block of the object.
  // Due to CSS being inconsistent, a containing block can be a relatively
  // positioned inline, thus we can't return a LayoutBlock from this function.
  //
  // This method is extremely similar to containingBlock(), but with a few
  // notable exceptions.
  // (1) For normal flow elements, it just returns the parent.
  // (2) For absolute positioned elements, it will return a relative
  //     positioned inline. containingBlock() simply skips relpositioned inlines
  //     and lets an enclosing block handle the layout of the positioned object.
  //     This does mean that computePositionedLogicalWidth and
  //     computePositionedLogicalHeight have to use container().
  //
  // Note that floating objects don't belong to either of the above exceptions.
  //
  // This function should be used for any invalidation as it would correctly
  // walk the containing block chain. See e.g. markContainerChainForLayout.
  // It is also used for correctly sizing absolutely positioned elements
  // (point 3 above).
  LayoutObject* Container(AncestorSkipInfo* = nullptr) const;
  // Finds the container as if this object is absolute-position.
  LayoutObject* ContainerForAbsolutePosition(AncestorSkipInfo* = nullptr) const;
  // Finds the container as if this object is fixed-position.
  LayoutObject* ContainerForFixedPosition(AncestorSkipInfo* = nullptr) const;

  bool CanContainOutOfFlowPositionedElement(EPosition position) const {
    NOT_DESTROYED();
    DCHECK(position == EPosition::kAbsolute || position == EPosition::kFixed);
    return (position == EPosition::kAbsolute &&
            CanContainAbsolutePositionObjects()) ||
           (position == EPosition::kFixed && CanContainFixedPositionObjects());
  }

  // Returns true if style would make this object a fixed container.
  // This value gets cached by bitfields_.can_contain_fixed_position_objects_.
  //
  // This function doesn't work for old_style in StyleDidChange(). Use
  // CanContainFixedPositionObjects() for old_style.
  bool ComputeIsFixedContainer(const ComputedStyle& style) const;

  // Returns true if style would make this object an absolute container. This
  // value gets cached by bitfields_.can_contain_absolute_position_objects_.
  bool ComputeIsAbsoluteContainer(const ComputedStyle& style,
                                  bool is_fixed_container) const;

  // If |base| is provided, then this function will not return an Element which
  // is closed shadow hidden from |base|.
  Element* OffsetParent(const Element* base = nullptr) const;

  // Inclusive of |this|, exclusive of |below|.
  const LayoutBoxModelObject* FindFirstStickyContainer(
      const LayoutBox* below) const;

  // Mark this object needing to re-run |CollectInlines()|. Ancestors may be
  // marked too if needed.
  void SetNeedsCollectInlines();
  void SetChildNeedsCollectInlines();
  void ClearNeedsCollectInlines() {
    NOT_DESTROYED();
    SetNeedsCollectInlines(false);
  }
  void SetNeedsCollectInlines(bool b) {
    NOT_DESTROYED();
    DCHECK(!GetDocument().InvalidationDisallowed());
    bitfields_.SetNeedsCollectInlines(b);
  }

  void MarkContainerChainForLayout(bool schedule_relayout = true);
  void MarkParentForSpannerOrOutOfFlowPositionedChange();
  void SetNeedsLayout(LayoutInvalidationReasonForTracing,
                      MarkingBehavior = kMarkContainerChain);
  void SetNeedsLayoutAndFullPaintInvalidation(
      LayoutInvalidationReasonForTracing,
      MarkingBehavior = kMarkContainerChain);

  void ClearNeedsLayoutWithoutPaintInvalidation();
  // |ClearNeedsLayout()| calls |SetShouldCheckForPaintInvalidation()|.
  void ClearNeedsLayout();
  void ClearNeedsLayoutWithFullPaintInvalidation();

  void SetChildNeedsLayout(MarkingBehavior = kMarkContainerChain);
  void SetNeedsSimplifiedLayout();
  void SetIntrinsicLogicalWidthsDirty(MarkingBehavior = kMarkContainerChain);
  void ClearIntrinsicLogicalWidthsDirty();

  void SetNeedsLayoutAndIntrinsicWidthsRecalc(
      LayoutInvalidationReasonForTracing reason) {
    NOT_DESTROYED();
    SetNeedsLayout(reason);
    SetIntrinsicLogicalWidthsDirty();
  }
  void SetNeedsLayoutAndIntrinsicWidthsRecalcAndFullPaintInvalidation(
      LayoutInvalidationReasonForTracing reason) {
    NOT_DESTROYED();
    SetNeedsLayoutAndFullPaintInvalidation(reason);
    SetIntrinsicLogicalWidthsDirty();
  }

  // Returns false when certain font changes (e.g., font-face rule changes, web
  // font loaded, etc) have occurred, in which case |this| needs relayout.
  virtual bool IsFontFallbackValid() const;

  // Traverses subtree, and marks all layout objects as need relayout, repaint
  // and preferred width recalc. Also invalidates shaping on all text nodes.
  virtual void InvalidateSubtreeLayoutForFontUpdates();

  // Mark elements with a principal box and a computed position-try-fallbacks
  // different from 'none' for layout when @position-try rules are removed or
  // added. mark_style_dirty is true if the element should be marked dirty as
  // well. mark_style_dirty is typically set to false if we are inside a subtree
  // which is already marked for subtree recalc.
  void InvalidateSubtreePositionTry(bool mark_style_dirty);

 private:
  enum PositionedState {
    kIsStaticallyPositioned = 0,
    kIsRelativelyPositioned = 1,
    kIsOutOfFlowPositioned = 2,
    kIsStickyPositioned = 3,
  };

 public:
  void SetPositionState(EPosition position) {
    NOT_DESTROYED();
    DCHECK(
        (position != EPosition::kAbsolute && position != EPosition::kFixed) ||
        IsBox());
    // This maps FixedPosition and AbsolutePosition to
    // IsOutOfFlowPositioned, saving one bit.
    switch (position) {
      case EPosition::kStatic:
        positioned_state_ = kIsStaticallyPositioned;
        break;
      case EPosition::kRelative:
        positioned_state_ = kIsRelativelyPositioned;
        break;
      case EPosition::kAbsolute:
      case EPosition::kFixed:
        positioned_state_ = kIsOutOfFlowPositioned;
        break;
      case EPosition::kSticky:
        positioned_state_ = kIsStickyPositioned;
        break;
      default:
        NOTREACHED();
    }
  }
  void ClearPositionedState() {
    NOT_DESTROYED();
    positioned_state_ = kIsStaticallyPositioned;
  }

  void SetFloating(bool is_floating) {
    NOT_DESTROYED();
    bitfields_.SetFloating(is_floating);
  }
  void SetInline(bool is_inline) {
    NOT_DESTROYED();
    bitfields_.SetIsInline(is_inline);
  }

  // Return whether we can directly traverse fragments generated for this layout
  // object, when it comes to painting, hit-testing and other layout read
  // operations. If false is returned, we need to traverse the layout object
  // tree instead.
  bool CanTraversePhysicalFragments() const {
    NOT_DESTROYED();

    if (!bitfields_.MightTraversePhysicalFragments())
      return false;

    // Non-LayoutBox objects (such as LayoutInline) don't necessarily create NG
    // LayoutObjects. We'll allow traversing their fragments if they are laid
    // out by an NG container.
    if (!IsBox())
      return IsInLayoutNGInlineFormattingContext();
    return true;
  }

  // Return true if this is a LayoutBox without physical fragments.
  //
  // This may happen for certain object types in certain circumstaces [*]. Code
  // that attempts to enter fragment traversal from a LayoutObject needs to
  // check if the box actually has fragments before proceeding.
  //
  // [*] Sometimes a LayoutView is fragment-less, e.g. if the root element has
  // display:none. Frameset children may also be fragment-less, if there are
  // more children than defined in the frameset's grid. Table columns
  // (LayoutNGTableColumn) never creates fragments.
  virtual bool IsFragmentLessBox() const {
    NOT_DESTROYED();
    return false;
  }

  // Return true if |this| produces one or more inline fragments, including
  // whitespace-only text fragments.
  virtual bool HasInlineFragments() const {
    NOT_DESTROYED();
    return false;
  }

  // Paint/Physical fragments are not in sync with LayoutObject tree until it is
  // laid out. For inline, it needs to check if the containing block is
  // layout-clean. crbug.com/963103
  bool IsFirstInlineFragmentSafe() const;
  void SetIsInLayoutNGInlineFormattingContext(bool);
  virtual wtf_size_t FirstInlineFragmentItemIndex() const {
    NOT_DESTROYED();
    return 0u;
  }
  virtual void ClearFirstInlineFragmentItemIndex() { NOT_DESTROYED(); }
  virtual void SetFirstInlineFragmentItemIndex(wtf_size_t) { NOT_DESTROYED(); }

  void SetHasBoxDecorationBackground(bool);

  void SetIsAtomicInlineLevel(bool is_atomic_inline_level) {
    NOT_DESTROYED();
    bitfields_.SetIsAtomicInlineLevel(is_atomic_inline_level);
  }
  void SetHorizontalWritingMode(bool has_horizontal_writing_mode) {
    NOT_DESTROYED();
    bitfields_.SetHorizontalWritingMode(has_horizontal_writing_mode);
  }
  void SetHasNonVisibleOverflow(bool has_non_visible_overflow) {
    NOT_DESTROYED();
    bitfields_.SetHasNonVisibleOverflow(has_non_visible_overflow);
  }
  void SetOverflowClipAxes(OverflowClipAxes axes) {
    NOT_DESTROYED();
    overflow_clip_axes_ = axes;
  }
  OverflowClipAxes GetOverflowClipAxes() const {
    NOT_DESTROYED();
    return static_cast<OverflowClipAxes>(overflow_clip_axes_);
  }
  bool ShouldClipOverflowAlongEitherAxis() const {
    NOT_DESTROYED();
    return GetOverflowClipAxes() != kNoOverflowClip;
  }
  bool ShouldClipOverflowAlongBothAxis() const {
    NOT_DESTROYED();
    return GetOverflowClipAxes() == kOverflowClipBothAxis;
  }
  void SetHasLayer(bool has_layer) {
    NOT_DESTROYED();
    bitfields_.SetHasLayer(has_layer);
  }
  void SetHasTransformRelatedProperty(bool has_transform) {
    NOT_DESTROYED();
    bitfields_.SetHasTransformRelatedProperty(has_transform);
  }
  void SetHasReflection(bool has_reflection) {
    NOT_DESTROYED();
    bitfields_.SetHasReflection(has_reflection);
  }
  void SetCanContainAbsolutePositionObjects(bool can_contain) {
    NOT_DESTROYED();
    bitfields_.SetCanContainAbsolutePositionObjects(can_contain);
  }
  void SetCanContainFixedPositionObjects(bool can_contain_fixed_position) {
    NOT_DESTROYED();
    bitfields_.SetCanContainFixedPositionObjects(can_contain_fixed_position);
  }
  void SetIsEffectiveRootScroller(bool is_effective_root_scroller) {
    NOT_DESTROYED();
    bitfields_.SetIsEffectiveRootScroller(is_effective_root_scroller);
  }
  void SetIsGlobalRootScroller(bool is_global_root_scroller) {
    NOT_DESTROYED();
    bitfields_.SetIsGlobalRootScroller(is_global_root_scroller);
  }
  void SetIsHTMLLegendElement() {
    NOT_DESTROYED();
    bitfields_.SetIsHTMLLegendElement(true);
  }
  void SetWhitespaceChildrenMayChange(bool b) {
    NOT_DESTROYED();
    bitfields_.SetWhitespaceChildrenMayChange(b);
  }
  bool WhitespaceChildrenMayChange() const {
    NOT_DESTROYED();
    return bitfields_.WhitespaceChildrenMayChange();
  }
  void SetNeedsDevtoolsInfo(bool b) {
    NOT_DESTROYED();
    bitfields_.SetNeedsDevtoolsInfo(b);
  }
  bool NeedsDevtoolsInfo() const {
    NOT_DESTROYED();
    return bitfields_.NeedsDevtoolsInfo();
  }

  virtual void Paint(const PaintInfo&) const;

  virtual RecalcScrollableOverflowResult RecalcScrollableOverflow();

  // Invalidate visual overflow, using a method that varies based
  // the object type and state of layout.
  void InvalidateVisualOverflow();

  // Recalculates visual overflow for this object and non-self-painting
  // PaintLayer descendants.
  virtual void RecalcVisualOverflow();
  void RecalcNormalFlowChildVisualOverflowIfNeeded();
#if DCHECK_IS_ON()
  // Enables DCHECK to ensure that the visual overflow for |this| is computed.
  // The actual invalidation is maintained in |PaintLayer|.
  void InvalidateVisualOverflowForDCheck();
#endif

  void HandleSubtreeModifications();
  virtual void SubtreeDidChange() { NOT_DESTROYED(); }

  // Flags used to mark if an object consumes subtree change notifications.
  bool ConsumesSubtreeChangeNotification() const {
    NOT_DESTROYED();
    return bitfields_.ConsumesSubtreeChangeNotification();
  }
  void SetConsumesSubtreeChangeNotification() {
    NOT_DESTROYED();
    bitfields_.SetConsumesSubtreeChangeNotification(true);
  }

  // Flags used to mark if a descendant subtree of this object has changed.

  // Returns true if the flag did change.
  bool NotifyOfSubtreeChange();
  bool WasNotifiedOfSubtreeChange() const {
    NOT_DESTROYED();
    return bitfields_.NotifiedOfSubtreeChange();
  }

  // Flags used to signify that a layoutObject needs to be notified by its
  // descendants that they have had their child subtree changed.
  void RegisterSubtreeChangeListenerOnDescendants(bool);
  bool HasSubtreeChangeListenerRegistered() const {
    NOT_DESTROYED();
    return bitfields_.SubtreeChangeListenerRegistered();
  }

  // Update layout for an SVG object. Shouldn't be reached for non-SVG objects.
  virtual SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&);

  // Used for element state updates that cannot be fixed with a paint
  // invalidation and do not need a relayout.
  virtual void UpdateFromElement() { NOT_DESTROYED(); }

  virtual void AddDraggableRegions(Vector<DraggableRegionValue>&);

  // True for object types which override |AdditionalCompositingReasons|.
  virtual bool CanHaveAdditionalCompositingReasons() const;
  virtual CompositingReasons AdditionalCompositingReasons() const;

  // |accumulated_offset| is accumulated physical offset of this object from
  // the same origin as |hit_test_location|. The caller just ensures that
  // |hit_test_location| and |accumulated_offset| are in the same coordinate
  // space that is transform-compatible with this object (i.e. we can add 2d
  // local offset to it without considering transforms). The implementation
  // should not assume any specific coordinate space of them. The local offset
  // of |hit_test_location| in this object can be calculated by
  // |hit_test_location.Point() - accumulated_offset|.
  virtual bool HitTestAllPhases(HitTestResult&,
                                const HitTestLocation& hit_test_location,
                                const PhysicalOffset& accumulated_offset);
  // Returns the node that is ultimately added to the hit test result. Some
  // objects report a hit testing node that is not their own (such as
  // continuations and some psuedo elements) and it is important that the
  // node be consistent between point- and list-based hit test results.
  virtual Node* NodeForHitTest() const;
  virtual void UpdateHitTestResult(HitTestResult&, const PhysicalOffset&) const;
  // See HitTestAllPhases() for explanation of |hit_test_location| and
  // |accumulated_offset|.
  virtual bool NodeAtPoint(HitTestResult&,
                           const HitTestLocation& hit_test_location,
                           const PhysicalOffset& accumulated_offset,
                           HitTestPhase);

  virtual PositionWithAffinity PositionForPoint(const PhysicalOffset&) const;
  PositionWithAffinity CreatePositionWithAffinity(int offset,
                                                  TextAffinity) const;
  PositionWithAffinity CreatePositionWithAffinity(int offset) const;
  PositionWithAffinity FindPosition() const;
  PositionWithAffinity FirstPositionInOrBeforeThis() const;
  PositionWithAffinity LastPositionInOrAfterThis() const;
  PositionWithAffinity PositionAfterThis() const;
  PositionWithAffinity PositionBeforeThis() const;

  virtual void DirtyLinesFromChangedChild(LayoutObject*) { NOT_DESTROYED(); }

  // Set the style of the object and update the state of the object accordingly.
  // ApplyStyleChanges = kYes means we will apply any changes between the old
  // and new ComputedStyle like paint and size invalidations. If kNo, just set
  // the ComputedStyle member.
  enum class ApplyStyleChanges { kNo, kYes };
  void SetStyle(const ComputedStyle*,
                ApplyStyleChanges = ApplyStyleChanges::kYes);

  // Set the style of the object if it's generated content.
  void SetPseudoElementStyle(const LayoutObject& owner,
                             bool match_parent_size = false);

  // In some cases we modify the ComputedStyle after the style recalc, either
  // for updating anonymous style or doing layout hacks for special elements
  // where we update the ComputedStyle during layout.
  // If the LayoutObject has an associated node, we will SetComputedStyle on
  // that node with the new ComputedStyle. Modifying the ComputedStyle of a node
  // outside of style recalc can break invariants in the style engine, so this
  // function must not gain any new call sites.
  void SetModifiedStyleOutsideStyleRecalc(const ComputedStyle*,
                                          ApplyStyleChanges);

  // This function returns an enclosing non-anonymous LayoutBlock for this
  // element. This function is not always returning the containing block as
  // defined by CSS. In particular:
  // - if the CSS containing block is a relatively positioned inline,
  //   the function returns the inline's enclosing non-anonymous LayoutBlock.
  //   This means that a LayoutInline would be skipped (expected as it's not a
  //   LayoutBlock) but so would be an inline LayoutNGTable or LayoutBlockFlow.
  //   TODO(jchaffraix): Is that REALLY what we want here?
  // - if the CSS containing block is anonymous, we find its enclosing
  //   non-anonymous LayoutBlock.
  //   Note that in the previous examples, the returned LayoutBlock has no
  //   logical relationship to the original element.
  //
  // LayoutBlocks are the one that handle laying out positioned elements,
  // thus this function is important during layout, to insert the positioned
  // elements into the correct LayoutBlock.
  //
  // See container() for the function that returns the containing block.
  // See layout_block.h for some extra explanations on containing blocks.
  LayoutBlock* ContainingBlock(AncestorSkipInfo* = nullptr) const;

  // Returns the nearest ancestor in the layout tree that IsForElement(),
  // or null if there is none.
  LayoutObject* NearestAncestorForElement() const;

  LayoutBlock* InclusiveContainingBlock(AncestorSkipInfo* = nullptr);

  const LayoutBox* ContainingScrollContainer(
      bool ignore_layout_view_for_fixed_pos = false) const;
  const PaintLayer* ContainingScrollContainerLayer(
      bool ignore_layout_view_for_fixed_pos = false) const;

  bool CanContainAbsolutePositionObjects() const {
    NOT_DESTROYED();
    return bitfields_.CanContainAbsolutePositionObjects();
  }
  bool CanContainFixedPositionObjects() const {
    NOT_DESTROYED();
    return bitfields_.CanContainFixedPositionObjects();
  }

  // Convert a rect/quad/point in ancestor coordinates to local physical
  // coordinates, taking transforms into account unless kIgnoreTransforms (not
  // allowed in the quad versions) is specified.
  // PhysicalRect parameter/return value is preferred to Float because they
  // force physical coordinates, unless we do need quads or float precision.
  // If the LayoutBoxModelObject ancestor is non-null, the input is in the
  // space of the ancestor.
  // Otherwise:
  //   If kTraverseDocumentBoundaries is specified, the input is in the space of
  //   the local root frame.
  //   Otherwise, the input is in the space of the containing frame.
  PhysicalRect AncestorToLocalRect(const LayoutBoxModelObject* ancestor,
                                   const PhysicalRect& rect,
                                   MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return PhysicalRect::EnclosingRect(
        AncestorToLocalQuad(ancestor, gfx::QuadF(gfx::RectF(rect)), mode)
            .BoundingBox());
  }
  gfx::QuadF AncestorToLocalQuad(const LayoutBoxModelObject*,
                                 const gfx::QuadF&,
                                 MapCoordinatesFlags mode = 0) const;
  PhysicalOffset AncestorToLocalPoint(const LayoutBoxModelObject* ancestor,
                                      const PhysicalOffset& p,
                                      MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return PhysicalOffset::FromPointFRound(
        AncestorToLocalPoint(ancestor, gfx::PointF(p), mode));
  }
  gfx::PointF AncestorToLocalPoint(const LayoutBoxModelObject* ancestor,
                                   const gfx::PointF& p,
                                   MapCoordinatesFlags = 0) const;

  // Convert a rect/quad/point in local physical coordinates into ancestor
  // coordinates, taking transforms into account unless kIgnoreTransforms is
  // specified.
  // PhysicalRect parameter/return value is preferred to Float because they
  // force physical coordinates, unless we do need quads or float precision.
  // If the LayoutBoxModelObject ancestor is non-null, the result will be in the
  // space of the ancestor.
  // Otherwise:
  //   If TraverseDocumentBoundaries is specified, the result will be in the
  //   space of the outermost root frame.
  //   Otherwise, the result will be in the space of the containing frame.
  // This method supports kUseGeometryMapperMode.
  PhysicalRect LocalToAncestorRect(const PhysicalRect& rect,
                                   const LayoutBoxModelObject* ancestor,
                                   MapCoordinatesFlags mode = 0) const;
  gfx::QuadF LocalRectToAncestorQuad(const PhysicalRect& rect,
                                     const LayoutBoxModelObject* ancestor,
                                     MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorQuad(gfx::QuadF(gfx::RectF(rect)), ancestor, mode);
  }
  gfx::QuadF LocalToAncestorQuad(const gfx::QuadF&,
                                 const LayoutBoxModelObject* ancestor,
                                 MapCoordinatesFlags = 0) const;
  PhysicalOffset LocalToAncestorPoint(const PhysicalOffset& p,
                                      const LayoutBoxModelObject* ancestor,
                                      MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return PhysicalOffset::FromPointFRound(
        LocalToAncestorPoint(gfx::PointF(p), ancestor, mode));
  }
  gfx::PointF LocalToAncestorPoint(const gfx::PointF&,
                                   const LayoutBoxModelObject* ancestor,
                                   MapCoordinatesFlags = 0) const;
  void LocalToAncestorRects(Vector<PhysicalRect>&,
                            const LayoutBoxModelObject* ancestor,
                            const PhysicalOffset& pre_offset,
                            const PhysicalOffset& post_offset) const;

  // Return the transformation matrix to map points from local to the coordinate
  // system of a container, taking transforms into account (kIgnoreTransforms is
  // not allowed).
  // Passing null for |ancestor| behaves the same as LocalToAncestorRect.
  gfx::Transform LocalToAncestorTransform(const LayoutBoxModelObject* ancestor,
                                          MapCoordinatesFlags = 0) const;
  gfx::Transform LocalToAbsoluteTransform(MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorTransform(nullptr, mode);
  }

  // Shorthands of the above LocalToAncestor* and AncestorToLocal* functions,
  // with nullptr as the ancestor. See the above functions for the meaning of
  // "absolute" coordinates.
  // This method supports kUseGeometryMapperMode.
  PhysicalRect LocalToAbsoluteRect(const PhysicalRect& rect,
                                   MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorRect(rect, nullptr, mode);
  }
  gfx::QuadF LocalRectToAbsoluteQuad(const PhysicalRect& rect,
                                     MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalRectToAncestorQuad(rect, nullptr, mode);
  }
  gfx::QuadF LocalToAbsoluteQuad(const gfx::QuadF& quad,
                                 MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorQuad(quad, nullptr, mode);
  }
  PhysicalOffset LocalToAbsolutePoint(const PhysicalOffset& p,
                                      MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorPoint(p, nullptr, mode);
  }
  gfx::PointF LocalToAbsolutePoint(const gfx::PointF& p,
                                   MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return LocalToAncestorPoint(p, nullptr, mode);
  }
  PhysicalRect AbsoluteToLocalRect(const PhysicalRect& rect,
                                   MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return AncestorToLocalRect(nullptr, rect, mode);
  }
  gfx::QuadF AbsoluteToLocalQuad(const gfx::QuadF& quad,
                                 MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return AncestorToLocalQuad(nullptr, quad, mode);
  }
  PhysicalOffset AbsoluteToLocalPoint(const PhysicalOffset& p,
                                      MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return AncestorToLocalPoint(nullptr, p, mode);
  }
  gfx::PointF AbsoluteToLocalPoint(const gfx::PointF& p,
                                   MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return AncestorToLocalPoint(nullptr, p, mode);
  }

  // Return the offset from the Container() LayoutObject (excluding transforms
  // and multicol). For efficiency reasons, the container is supplied as a
  // parameter. It is however required that it be equal to Container().
  PhysicalOffset OffsetFromContainer(const LayoutObject* container,
                                     MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    return OffsetFromContainerInternal(container, mode);
  }
  // Return the offset from an object from the ancestor. The ancestor need
  // not be on the containing block chain of |this|. Note that this function
  // cannot be used when there are transforms between this object and the
  // ancestor - use |LocalToAncestorPoint| if there might be transforms.
  PhysicalOffset OffsetFromAncestor(const LayoutObject*) const;

  gfx::RectF AbsoluteBoundingBoxRectF(MapCoordinatesFlags = 0) const;
  // This returns an gfx::Rect enclosing this object. If this object has an
  // integral size and the position has fractional values, the resultant
  // gfx::Rect can be larger than the integral size.
  gfx::Rect AbsoluteBoundingBoxRect(MapCoordinatesFlags = 0) const;

  // These two functions also handle inlines without content for which the
  // location of the result rect (which may be empty) should be the absolute
  // location of the inline. This is especially useful to get the bounding
  // box of named anchors.
  // TODO(crbug.com/953479): After the bug is fixed, investigate whether we
  // can combine this with AbsoluteBoundingBoxRect().
  virtual PhysicalRect AbsoluteBoundingBoxRectHandlingEmptyInline(
      MapCoordinatesFlags flags = 0) const;
  // This returns an gfx::Rect expanded from
  // AbsoluteBoundingBoxRectHandlingEmptyInline by ScrollMargin.
  PhysicalRect AbsoluteBoundingBoxRectForScrollIntoView() const;

  // Build an array of quads relatively to `ancestor` (which may be nullptr, in
  // which case they will be in absolute coordinates).
  void QuadsInAncestor(Vector<gfx::QuadF>& quads,
                       const LayoutBoxModelObject* ancestor,
                       MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    QuadsInAncestorInternal(quads, ancestor, mode);
  }

  // Build an array of quads in absolute coords.
  void AbsoluteQuads(Vector<gfx::QuadF>& quads,
                     MapCoordinatesFlags mode = 0) const {
    NOT_DESTROYED();
    QuadsInAncestor(quads, /*ancestor=*/nullptr, mode);
  }

  // The bounding box (see: absoluteBoundingBoxRect) including all descendant
  // bounding boxes.
  gfx::Rect AbsoluteBoundingBoxRectIncludingDescendants() const;

  // For accessibility, we want the bounding box rect of this element
  // in local coordinates, which can then be converted to coordinates relative
  // to any ancestor using, e.g., localToAncestorTransform.
  virtual gfx::RectF LocalBoundingBoxRectForAccessibility() const = 0;

  const ComputedStyle* Style() const {
    NOT_DESTROYED();
    return style_.Get();
  }

  // style_ can only be nullptr before the first style is set, thus most
  // callers will never see a nullptr style and should use StyleRef().
  const ComputedStyle& StyleRef() const {
    NOT_DESTROYED();
    DCHECK(style_);
    return *style_;
  }

  /* The following methods are inlined in LayoutObjectInlines.h */
  // If first line style is requested and there is no applicable first line
  // style, the functions will return the style of this object.
  inline const ComputedStyle* FirstLineStyle() const;
  inline const ComputedStyle& FirstLineStyleRef() const;
  inline const ComputedStyle* Style(bool first_line) const;
  inline const ComputedStyle& StyleRef(bool first_line) const;

  const ComputedStyle& EffectiveStyle(StyleVariant style_variant) const {
    NOT_DESTROYED();
    return style_variant == StyleVariant::kStandard
               ? StyleRef()
               : SlowEffectiveStyle(style_variant);
  }

  static inline Color ResolveColor(const ComputedStyle& style_to_use,
                                   const Longhand& color_property) {
    return style_to_use.VisitedDependentColor(color_property);
  }

  inline Color ResolveColor(const Longhand& color_property) const {
    NOT_DESTROYED();
    return StyleRef().VisitedDependentColor(color_property);
  }

  // See ComputedStyle::VisitedDependentColorFast().
  template <class Property>
  static inline Color ResolveColorFast(const ComputedStyle& style_to_use,
                                       const Property& color_property) {
    return style_to_use.VisitedDependentColorFast(color_property);
  }

  template <class Property>
  inline Color ResolveColorFast(const Property& color_property) const {
    NOT_DESTROYED();
    return StyleRef().VisitedDependentColorFast(color_property);
  }

  virtual CursorDirective GetCursor(const PhysicalOffset&, ui::Cursor&) const;

  // Given a rect in the object's physical coordinate space, mutates the rect
  // into one representing the size of its visual painted output as if
  // |ancestor| was the root of the page: the rect is modified by any
  // intervening clips, transforms and scrolls between |this| and |ancestor|
  // (not inclusive of |ancestor|), but not any above |ancestor|.
  // The output is in the physical, painted coordinate pixel space of
  // |ancestor|.
  // Overflow clipping, CSS clipping and scrolling is *not* applied for
  // |ancestor| itself if |ancestor| scrolls overflow.
  // The output rect is suitable for purposes such as paint invalidation.
  //
  // The ancestor can be nullptr which, if |this| is not the root view, will map
  // the rect to the main frame's space which includes the root view's scroll
  // and clip. This is even true if the main frame is remote.
  //
  // If VisualRectFlags has the kEdgeInclusive bit set, clipping operations will
  // use PhysicalRect::InclusiveIntersect, and the return value of
  // InclusiveIntersect will be propagated to the return value of this method.
  // Otherwise, clipping operations will use PhysicalRect::Intersect, and the
  // return value will be true only if the clipped rect has non-zero area.
  // See the documentation for PhysicalRect::InclusiveIntersect for more
  // information.
  bool MapToVisualRectInAncestorSpace(
      const LayoutBoxModelObject* ancestor,
      PhysicalRect&,
      VisualRectFlags = kDefaultVisualRectFlags) const;

  bool MapToVisualRectInAncestorSpace(
      const LayoutBoxModelObject* ancestor,
      gfx::RectF&,
      VisualRectFlags = kDefaultVisualRectFlags) const;

  // Do not call this method directly. Call mapToVisualRectInAncestorSpace
  // instead.
  virtual bool MapToVisualRectInAncestorSpaceInternal(
      const LayoutBoxModelObject* ancestor,
      TransformState&,
      VisualRectFlags = kDefaultVisualRectFlags) const;

  // Returns the nearest ancestor in the containing block chain that
  // HasLocalBorderBoxProperties. If AncestorSkipInfo* is non-null and the
  // ancestor was skipped, returns nullptr. If PropertyTreeState* is non-null,
  // it will be populated with paint property nodes suitable for mapping upward
  // from the coordinate system of the property container.
  const LayoutObject* GetPropertyContainer(
      AncestorSkipInfo*,
      PropertyTreeStateOrAlias* = nullptr,
      VisualRectFlags = kDefaultVisualRectFlags) const;

  // Do a rect-based hit test with this object as the stop node.
  HitTestResult HitTestForOcclusion(const PhysicalRect&) const;

  // Return the offset to the column in which the specified point (in
  // flow-thread coordinates) lives. This is used to convert a flow-thread point
  // to a point in the containing coordinate space.
  virtual PhysicalOffset ColumnOffset(const PhysicalOffset&) const {
    NOT_DESTROYED();
    return PhysicalOffset();
  }

  bool IsFloatingOrOutOfFlowPositioned() const {
    NOT_DESTROYED();
    return (IsFloating() || IsOutOfFlowPositioned());
  }

  // Outside list markers are in-flow but behave kind of out-of-flowish.
  // We include them here to prevent code like '<li> <ol></ol></li>' from
  // generating an anonymous block box for the whitespace between the marker
  // and the <ol>.
  bool AffectsWhitespaceSiblings() const {
    NOT_DESTROYED();
    return !IsFloatingOrOutOfFlowPositioned() && !IsLayoutOutsideListMarker();
  }

  // Not returning StyleRef().BoxReflect() because some objects ignore the
  // reflection style (e.g. LayoutInline, LayoutSVGBlock).
  bool HasReflection() const {
    NOT_DESTROYED();
    return bitfields_.HasReflection();
  }

  // The current selection state for an object.  For blocks, the state refers to
  // the state of the leaf descendants (as described above in the SelectionState
  // enum declaration).
  SelectionState GetSelectionState() const {
    NOT_DESTROYED();
    return static_cast<SelectionState>(selection_state_);
  }
  void SetSelectionState(SelectionState state) {
    NOT_DESTROYED();
    selection_state_ = static_cast<unsigned>(state);
  }
  bool CanUpdateSelectionOnRootLineBoxes() const;

  SelectionState GetSelectionStateForPaint() const {
    NOT_DESTROYED();
    return static_cast<SelectionState>(selection_state_for_paint_);
  }
  void SetSelectionStateForPaint(SelectionState state) {
    NOT_DESTROYED();
    selection_state_for_paint_ = static_cast<unsigned>(state);
  }

  // A single rectangle that encompasses all of the selected objects within this
  // object. Used to determine the tightest possible bounding box for the
  // selection. The rect is in the object's local physical coordinate space.
  virtual PhysicalRect LocalSelectionVisualRect() const {
    NOT_DESTROYED();
    return PhysicalRect();
  }

  PhysicalRect AbsoluteSelectionRect() const;

  bool CanBeSelectionLeaf() const;
  bool IsSelected() const;
  bool IsSelectable() const;

  /**
   * Returns the local coordinates of the caret within this layout object.
   * @param caret_offset zero-based offset determining position within the
   * layout object.
   */
  virtual PhysicalRect LocalCaretRect(int caret_offset) const;

  // When performing a global document tear-down, the layoutObject of the
  // document is cleared. We use this as a hook to detect the case of document
  // destruction and don't waste time doing unnecessary work.
  bool DocumentBeingDestroyed() const;

  void DestroyAndCleanupAnonymousWrappers(bool performing_reattach);

  void Destroy();

  bool IsListItem() const {
    NOT_DESTROYED();
    return IsLayoutListItem() || IsInlineListItem();
  }

  // There 2 different types of list markers:
  // * LayoutInsideListMarker (LayoutInline): for inside markers
  // * LayoutOutsideListMarker (LayoutBlockFlow): for outside markers.

  // Any kind of list marker.
  bool IsListMarker() const {
    NOT_DESTROYED();
    return IsLayoutInsideListMarker() || IsLayoutOutsideListMarker();
  }

  // ImageResourceObserver override.
  void ImageChanged(ImageResourceContent*, CanDeferInvalidation) final;
  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override {
    NOT_DESTROYED();
  }
  void ImageNotifyFinished(ImageResourceContent*) override;
  void NotifyImageFullyRemoved(ImageResourceContent*) override;
  bool WillRenderImage() final;
  bool GetImageAnimationPolicy(mojom::blink::ImageAnimationPolicy&) final;
  InterpolationQuality GetSpeculativeDecodeQuality() const final;

  void Remove() {
    NOT_DESTROYED();
    if (Parent())
      Parent()->RemoveChild(this);
  }

  bool VisibleToHitTestRequest(const HitTestRequest& request) const {
    NOT_DESTROYED();
    return StyleRef().Visibility() == EVisibility::kVisible &&
           (request.IgnorePointerEventsNone() ||
            StyleRef().UsedPointerEvents() != EPointerEvents::kNone);
  }

  bool VisibleToHitTesting() const {
    NOT_DESTROYED();
    return StyleRef().VisibleToHitTesting();
  }

  // Map points and quads through elements, potentially via 3d transforms. You
  // should never need to call these directly; use localToAbsolute/
  // absoluteToLocal methods instead.
  virtual void MapLocalToAncestor(const LayoutBoxModelObject* ancestor,
                                  TransformState&,
                                  MapCoordinatesFlags) const;
  // If the LayoutBoxModelObject ancestor is non-null, the input quad is in the
  // space of the ancestor.
  // Otherwise:
  //   If TraverseDocumentBoundaries is specified, the input quad is in the
  //   space of the local root frame.
  //   Otherwise, the input quad is in the space of the containing frame.
  virtual void MapAncestorToLocal(const LayoutBoxModelObject*,
                                  TransformState&,
                                  MapCoordinatesFlags) const;

  bool ShouldUseTransformFromContainer(const LayoutObject* container) const;

  // The optional |size| parameter is used if the size of the object isn't
  // correct yet. If |fragment_transform| is provided, we'll use that instead of
  // using the transform stored in the PaintLayer (which is useless if a box is
  // fragmented).
  void GetTransformFromContainer(
      const LayoutObject* container,
      const PhysicalOffset& offset_in_container,
      gfx::Transform&,
      const PhysicalSize* size = nullptr,
      const gfx::Transform* fragment_transform = nullptr) const;

  bool CreatesGroup() const {
    NOT_DESTROYED();
    // See |HasReflection()| for why |StyleRef().BoxReflect()| is not used.
    return StyleRef().HasGroupingProperty(HasReflection());
  }

  // Return the outline rectangles of the current fragmentainer, as indicated by
  // |iterator|. This method will also advance |iterator| to the next
  // FragmentData (and therefore also next fragmentainer), if any.
  Vector<PhysicalRect> CollectOutlineRectsAndAdvance(
      OutlineType,
      AccompaniedFragmentIterator& iterator) const;

  struct OutlineInfo {
    int width = 0;
    int offset = 0;

    // Convenience functions to initialize outline info.
    static OutlineInfo GetFromStyle(const ComputedStyle& style) {
      return {style.OutlineWidth(), style.OutlineOffset().ToInt()};
    }

    static float getUnzoomedWidth(const ComputedStyle& style) {
      float unzoomedWidth = style.OutlineWidth() / style.EffectiveZoom();

      if (unzoomedWidth > 0.0f && unzoomedWidth <= 1.0f)
        return 1.0f;

      return std::floor(unzoomedWidth);
    }

    // Unzoomed values modifies the style values by effective zoom. This is
    // used when the outline rects are specified in a space that does not
    // include EffectiveZoom, such as SVG.
    static OutlineInfo GetUnzoomedFromStyle(const ComputedStyle& style) {
      return {static_cast<int>(getUnzoomedWidth(style)),
              static_cast<int>(
                  std::floor(style.OutlineOffset() / style.EffectiveZoom()))};
    }
  };

  // OutlineInfo, if specified, is filled in with the outline width and offset
  // in the same space as the physical rects returned.
  Vector<PhysicalRect> OutlineRects(OutlineInfo*,
                                    const PhysicalOffset& additional_offset,
                                    OutlineType) const;

  // Collects rectangles that the outline of this object would be drawing along
  // the outside of, even if the object isn't styled with a outline for now.
  // The rects also cover continuations. Note that the OutlineInfo, if
  // specified, is filled in in the same space as the rects.
  virtual void AddOutlineRects(OutlineRectCollector&,
                               OutlineInfo*,
                               const PhysicalOffset& additional_offset,
                               OutlineType) const {
    NOT_DESTROYED();
  }

  // Get the 'image-orientation' value for a (potentially null) LayoutObject.
  //
  // Returns the initial value ('from-image') if passed a nullptr, else the
  // value of the 'image-orientation' property. (If it is known at the callsite
  // that the LayoutObject* is non-null then just access its ComputedStyle
  // directly.)
  static RespectImageOrientationEnum GetImageOrientation(const LayoutObject*);

  bool IsRelayoutBoundary() const;

  PaintInvalidationReason PaintInvalidationReasonForPrePaint() const {
    NOT_DESTROYED();
    return static_cast<PaintInvalidationReason>(
        paint_invalidation_reason_for_pre_paint_);
  }
  bool ShouldDoFullPaintInvalidation() const {
    NOT_DESTROYED();
    if (ShouldDelayFullPaintInvalidation()) {
      DCHECK(!bitfields_.SubtreeShouldDoFullPaintInvalidation());
      return false;
    }
    if (IsFullPaintInvalidationReason(PaintInvalidationReasonForPrePaint())) {
      DCHECK(ShouldCheckForPaintInvalidation());
      return true;
    }
    return false;
  }
  // Indicates that the paint of the object should be fully invalidated.
  // We will repaint the object, and reraster the area on the composited layer
  // where the object shows. Note that this function doesn't automatically
  // cause invalidation of background painted on the scrolling contents layer
  // because we don't want to invalidate the whole scrolling contents layer on
  // non-background changes. It's also not safe to specially handle
  // PaintInvalidationReason::kBackground in paint invalidator because we don't
  // track paint invalidation reasons separately. To indicate that the
  // background needs full invalidation, use
  // SetBackgroundNeedsFullPaintInvalidation().
  void SetShouldDoFullPaintInvalidation(
      PaintInvalidationReason = PaintInvalidationReason::kLayout);
  void SetShouldDoFullPaintInvalidationWithoutLayoutChange(
      PaintInvalidationReason reason);

  void SetShouldInvalidatePaintForHitTest();
  bool ShouldInvalidatePaintForHitTestOnly() const {
    NOT_DESTROYED();
    return PaintInvalidationReasonForPrePaint() ==
           PaintInvalidationReason::kHitTest;
  }

  void ClearPaintInvalidationFlags();

  bool ShouldCheckForPaintInvalidation() const {
    NOT_DESTROYED();
    return bitfields_.ShouldCheckForPaintInvalidation();
  }
  // Sets both ShouldCheckForPaintInvalidation() and
  // ShouldCheckLayoutForPaintInvalidation(). Though the setter and the getter
  // are asymmetric, this prevents callers from accidentally missing the
  // layout checking flag.
  void SetShouldCheckForPaintInvalidation();
  // Sets ShouldCheckForPaintInvalidation() only. PaintInvalidator won't require
  // paint property tree update or other layout related updates.
  void SetShouldCheckForPaintInvalidationWithoutLayoutChange();

  bool SubtreeShouldCheckForPaintInvalidation() const {
    NOT_DESTROYED();
    return bitfields_.SubtreeShouldCheckForPaintInvalidation();
  }
  void SetSubtreeShouldCheckForPaintInvalidation();

  bool ShouldCheckLayoutForPaintInvalidation() const {
    NOT_DESTROYED();
    return bitfields_.ShouldCheckLayoutForPaintInvalidation();
  }
  bool DescendantShouldCheckLayoutForPaintInvalidation() const {
    NOT_DESTROYED();
    return bitfields_.DescendantShouldCheckLayoutForPaintInvalidation();
  }

  bool MayNeedPaintInvalidationAnimatedBackgroundImage() const {
    NOT_DESTROYED();
    return bitfields_.MayNeedPaintInvalidationAnimatedBackgroundImage();
  }
  void SetMayNeedPaintInvalidationAnimatedBackgroundImage();

  void SetSubtreeShouldDoFullPaintInvalidation(
      PaintInvalidationReason reason = PaintInvalidationReason::kSubtree);
  bool SubtreeShouldDoFullPaintInvalidation() const {
    NOT_DESTROYED();
    DCHECK(!bitfields_.SubtreeShouldDoFullPaintInvalidation() ||
           ShouldDoFullPaintInvalidation());
    return bitfields_.SubtreeShouldDoFullPaintInvalidation();
  }

  // If true, it means that invalidation and repainting of the object can be
  // delayed until a future frame. This can be the case for an object whose
  // content is not visible to the user.
  bool ShouldDelayFullPaintInvalidation() const {
    NOT_DESTROYED();
    return bitfields_.ShouldDelayFullPaintInvalidation();
  }
  void SetShouldDelayFullPaintInvalidation();
  void ClearShouldDelayFullPaintInvalidation();

  bool ShouldInvalidateSelection() const {
    NOT_DESTROYED();
    return bitfields_.ShouldInvalidateSelection();
  }
  void SetShouldInvalidateSelection();

  virtual PhysicalRect ViewRect() const;

  // Called by PaintInvalidator during PrePaint. Checks paint invalidation flags
  // and other changes that will cause different painting, and invalidate
  // display item clients for painting if needed.
  virtual void InvalidatePaint(const PaintInvalidatorContext&) const;

  // When this object is invalidated for paint, this method is called to
  // invalidate any DisplayItemClients owned by this object, including the
  // object itself, LayoutText/LayoutInline line boxes, etc.,
  // not including children which will be invalidated normally during
  // invalidateTreeIfNeeded() and parts which are invalidated separately (e.g.
  // scrollbars). The caller should ensure the painting layer has been
  // setNeedsRepaint before calling this function.
  virtual void InvalidateDisplayItemClients(PaintInvalidationReason) const;

  // Get the dedicated DisplayItemClient for selection. Returns nullptr if this
  // object doesn't have a dedicated DisplayItemClient.
  virtual const DisplayItemClient* GetSelectionDisplayItemClient() const {
    NOT_DESTROYED();
    return nullptr;
  }

  // Called before setting style for existing/new anonymous child. Override to
  // set custom styles for the child. For new anonymous child, |child| is null.
  virtual void UpdateAnonymousChildStyle(const LayoutObject* child,
                                         ComputedStyleBuilder&) const {
    NOT_DESTROYED();
  }

  // Returns a rect corresponding to this LayoutObject's bounds for use in
  // debugging output
  virtual PhysicalRect DebugRect() const;

  // Each LayoutObject has one or more painting fragments (exactly one
  // in the absence of multicol/pagination).
  // See ../paint/README.md for more on fragments.
  const FragmentData& FirstFragment() const {
    NOT_DESTROYED();
    return *fragment_;
  }

  const FragmentDataList& FragmentList() const {
    NOT_DESTROYED();
    return *fragment_;
  }

  bool IsFragmented() const {
    NOT_DESTROYED();
    return FragmentList().size() > 1;
  }

  enum OverflowRecalcType {
    kOnlyVisualOverflowRecalc,
    kLayoutAndVisualOverflowRecalc,
  };
  void SetNeedsOverflowRecalc(
      OverflowRecalcType = OverflowRecalcType::kLayoutAndVisualOverflowRecalc);

  void InvalidateSelectionOnStyleChange();

  // The allowed touch action is the union of the effective touch action
  // (from style) and blocking touch event handlers.
  TouchAction EffectiveAllowedTouchAction() const {
    NOT_DESTROYED();
    if (InsideBlockingTouchEventHandler())
      return TouchAction::kNone;
    return StyleRef().EffectiveTouchAction();
  }
  bool HasEffectiveAllowedTouchAction() const {
    NOT_DESTROYED();
    return EffectiveAllowedTouchAction() != TouchAction::kAuto;
  }

  // Whether this object's Node has a blocking touch event handler on itself
  // or an ancestor.
  bool InsideBlockingTouchEventHandler() const {
    NOT_DESTROYED();
    return bitfields_.InsideBlockingTouchEventHandler();
  }
  // Mark this object as having a |EffectiveAllowedTouchAction| changed, and
  // mark all ancestors as having a descendant that changed. This will cause a
  // PrePaint tree walk to update effective allowed touch action.
  void MarkEffectiveAllowedTouchActionChanged();
  void MarkDescendantEffectiveAllowedTouchActionChanged();
  bool EffectiveAllowedTouchActionChanged() const {
    NOT_DESTROYED();
    return bitfields_.EffectiveAllowedTouchActionChanged();
  }
  bool DescendantEffectiveAllowedTouchActionChanged() const {
    NOT_DESTROYED();
    return bitfields_.DescendantEffectiveAllowedTouchActionChanged();
  }
  void UpdateInsideBlockingTouchEventHandler(bool inside) {
    NOT_DESTROYED();
    bitfields_.SetInsideBlockingTouchEventHandler(inside);
  }

  // Whether this object's Node has a blocking wheel event handler on itself or
  // an ancestor.
  bool InsideBlockingWheelEventHandler() const {
    NOT_DESTROYED();
    return bitfields_.InsideBlockingWheelEventHandler();
  }
  // Mark this object as having a |InsideBlockingWheelEventHandler| changed, and
  // mark all ancestors as having a descendant that changed. This will cause a
  // PrePaint tree walk to update blocking wheel event handler state.
  void MarkBlockingWheelEventHandlerChanged();
  void MarkDescendantBlockingWheelEventHandlerChanged();
  bool BlockingWheelEventHandlerChanged() const {
    NOT_DESTROYED();
    return bitfields_.BlockingWheelEventHandlerChanged();
  }
  bool DescendantBlockingWheelEventHandlerChanged() const {
    NOT_DESTROYED();
    return bitfields_.DescendantBlockingWheelEventHandlerChanged();
  }
  void UpdateInsideBlockingWheelEventHandler(bool inside) {
    NOT_DESTROYED();
    bitfields_.SetInsideBlockingWheelEventHandler(inside);
  }

  // Painters can use const methods only, except for these explicitly declared
  // methods.
  class CORE_EXPORT MutableForPainting {
    STACK_ALLOCATED();

   public:
    // Convenience mutator that clears paint invalidation flags and this object
    // and its descendants' needs-paint-property-update flags.
    void ClearPaintFlags() { layout_object_.ClearPaintFlags(); }

    // These methods are only intended to be called when visiting this object
    // during pre-paint, and as such it should only mark itself, and not the
    // entire containing block chain.
    void SetShouldCheckForPaintInvalidation() {
      DCHECK_EQ(layout_object_.GetDocument().Lifecycle().GetState(),
                DocumentLifecycle::kInPrePaint);
      layout_object_.bitfields_.SetShouldCheckLayoutForPaintInvalidation(true);
      layout_object_.bitfields_.SetShouldCheckForPaintInvalidation(true);
    }
    void SetShouldDoFullPaintInvalidation(PaintInvalidationReason reason) {
      DCHECK_EQ(layout_object_.GetDocument().Lifecycle().GetState(),
                DocumentLifecycle::kInPrePaint);
      // This call to MutableForPainting::SetShouldCheckForPaintInvaldiation()
      // prevents LayoutObject::SetShouldDoFullPaintInvalidation() from marking
      // ancestors for paint invalidation, which is not needed when this is
      // called during PrePaint.
      SetShouldCheckForPaintInvalidation();
      layout_object_.SetShouldDoFullPaintInvalidation(reason);
    }
    void SetShouldDoFullPaintInvalidationWithoutLayoutChange(
        PaintInvalidationReason reason) {
      DCHECK_EQ(layout_object_.GetDocument().Lifecycle().GetState(),
                DocumentLifecycle::kInPrePaint);
      DCHECK(IsNonLayoutFullPaintInvalidationReason(reason));
      // This prevents LayoutObject::SetShouldDoFullPaintInvalidation...()
      // from marking ancestors for paint invalidation.
      layout_object_.bitfields_.SetShouldCheckForPaintInvalidation(true);
      layout_object_
          .SetShouldDoFullPaintInvalidationWithoutLayoutChangeInternal(reason);
    }

    void SetShouldDelayFullPaintInvalidation() {
      layout_object_.SetShouldDelayFullPaintInvalidation();
    }
    void EnsureIsReadyForPaintInvalidation() {
      layout_object_.EnsureIsReadyForPaintInvalidation();
    }
    void MarkEffectiveAllowedTouchActionChanged() {
      layout_object_.MarkEffectiveAllowedTouchActionChanged();
    }

    void SetBackgroundPaintLocation(BackgroundPaintLocation location) {
      layout_object_.SetBackgroundPaintLocation(location);
    }

    void UpdatePreviousVisibilityVisible() {
      layout_object_.bitfields_.SetPreviousVisibilityVisible(
          layout_object_.StyleRef().Visibility() == EVisibility::kVisible);
    }

    // Same as LayoutObject::SetNeedsPaintPropertyUpdate(), but does not mark
    // ancestors as having a descendant needing a paint property update.
    void SetOnlyThisNeedsPaintPropertyUpdate() {
      DCHECK(!layout_object_.GetDocument().InvalidationDisallowed());
      layout_object_.bitfields_.SetNeedsPaintPropertyUpdate(true);
    }

    void AddSubtreePaintPropertyUpdateReason(
        SubtreePaintPropertyUpdateReason reason) {
      layout_object_.AddSubtreePaintPropertyUpdateReason(reason);
    }

    void UpdateInsideBlockingTouchEventHandler(bool inside) {
      layout_object_.UpdateInsideBlockingTouchEventHandler(inside);
    }

    void UpdateInsideBlockingWheelEventHandler(bool inside) {
      layout_object_.UpdateInsideBlockingWheelEventHandler(inside);
    }

#if DCHECK_IS_ON()
    void ClearNeedsPaintPropertyUpdateForTesting() {
      layout_object_.bitfields_.SetNeedsPaintPropertyUpdate(false);
    }
#endif

    void SetShouldSkipNextLayoutShiftTracking(bool b) {
      layout_object_.SetShouldSkipNextLayoutShiftTracking(b);
    }

    void SetShouldAssumePaintOffsetTranslationForLayoutShiftTracking(bool b) {
      layout_object_
          .SetShouldAssumePaintOffsetTranslationForLayoutShiftTracking(b);
    }

    void FragmentCountChanged() {
      // Even if the fragment count has changed, the total stitched size of the
      // object may be the same as before, although the size of the individual
      // fragments may have changed. Full paint invalidation is required.
      SetShouldDoFullPaintInvalidation(PaintInvalidationReason::kLayout);
    }

    FragmentData& FirstFragment() { return *layout_object_.fragment_; }
    FragmentDataList& FragmentList() { return *layout_object_.fragment_; }

    void EnsureId() { layout_object_.fragment_->EnsureId(); }

   protected:
    friend class LayoutBoxModelObject;
    friend class CustomScrollbar;
    friend class PaintInvalidator;
    friend class PaintPropertyTreeBuilder;
    friend class PrePaintTreeWalk;
    FRIEND_TEST_ALL_PREFIXES(AnimationCompositorAnimationsTest,
                             canStartElementOnCompositorTransformCAP);
    FRIEND_TEST_ALL_PREFIXES(AnimationCompositorAnimationsTest,
                             canStartElementOnCompositorEffectCAP);
    FRIEND_TEST_ALL_PREFIXES(PrePaintTreeWalkTest, ClipRects);
    FRIEND_TEST_ALL_PREFIXES(LayoutObjectTest, VisualRect);
    FRIEND_TEST_ALL_PREFIXES(BoxPaintInvalidatorTest,
                             ComputePaintInvalidationReasonBasic);

    friend class LayoutObject;
    MutableForPainting(const LayoutObject& layout_object)
        : layout_object_(const_cast<LayoutObject&>(layout_object)) {}

    LayoutObject& layout_object_;
  };
  MutableForPainting GetMutableForPainting() const {
    NOT_DESTROYED();
    DCHECK(!PrePaintDisableSideEffectsScope::IsDisabled());
    return MutableForPainting(*this);
  }

  // Paint properties (see: |ObjectPaintProperties|) are built from an object's
  // state (location, transform, etc) as well as properties from ancestors.
  // When these inputs change, SetNeedsPaintPropertyUpdate will cause a property
  // tree update during the next document lifecycle update.
  //
  // In addition to tracking if an object needs its own paint properties
  // updated, SetNeedsPaintPropertyUpdate marks all ancestors as having a
  // descendant needing a paint property update too.
  void SetNeedsPaintPropertyUpdate();
  void SetDescendantNeedsPaintPropertyUpdate();
  bool NeedsPaintPropertyUpdate() const {
    NOT_DESTROYED();
    return bitfields_.NeedsPaintPropertyUpdate();
  }

  void AddSubtreePaintPropertyUpdateReason(
      SubtreePaintPropertyUpdateReason reason) {
    NOT_DESTROYED();
    DCHECK_LE(static_cast<unsigned>(reason),
              1u << (kSubtreePaintPropertyUpdateReasonsBitfieldWidth - 1));
    subtree_paint_property_update_reasons_ |= static_cast<unsigned>(reason);
    SetNeedsPaintPropertyUpdate();
  }
  unsigned SubtreePaintPropertyUpdateReasons() const {
    NOT_DESTROYED();
    return subtree_paint_property_update_reasons_;
  }
  bool DescendantNeedsPaintPropertyUpdate() const {
    NOT_DESTROYED();
    return bitfields_.DescendantNeedsPaintPropertyUpdate();
  }

  void SetIsScrollAnchorObject() {
    NOT_DESTROYED();
    bitfields_.SetIsScrollAnchorObject(true);
  }
  // Clears the IsScrollAnchorObject bit if and only if no ScrollAnchors still
  // reference this LayoutObject.
  void MaybeClearIsScrollAnchorObject();

  bool ScrollAnchorDisablingStyleChanged() {
    NOT_DESTROYED();
    return bitfields_.ScrollAnchorDisablingStyleChanged();
  }
  void SetScrollAnchorDisablingStyleChanged(bool changed) {
    NOT_DESTROYED();
    bitfields_.SetScrollAnchorDisablingStyleChanged(changed);
  }

  bool ShouldSkipLayoutCache() const {
    NOT_DESTROYED();
    return bitfields_.ShouldSkipLayoutCache();
  }
  void SetShouldSkipLayoutCache(bool b) {
    NOT_DESTROYED();
    bitfields_.SetShouldSkipLayoutCache(b);
  }

  bool IsBackgroundAttachmentFixedObject() const {
    NOT_DESTROYED();
    return bitfields_.IsBackgroundAttachmentFixedObject();
  }
  bool CanCompositeBackgroundAttachmentFixed() const {
    NOT_DESTROYED();
    return bitfields_.CanCompositeBackgroundAttachmentFixed();
  }

  bool BackgroundNeedsFullPaintInvalidation() const {
    NOT_DESTROYED();
    return !ShouldDelayFullPaintInvalidation() &&
           bitfields_.BackgroundNeedsFullPaintInvalidation();
  }
  void SetBackgroundNeedsFullPaintInvalidation() {
    NOT_DESTROYED();
    SetShouldDoFullPaintInvalidationWithoutLayoutChangeInternal(
        PaintInvalidationReason::kBackground);
    bitfields_.SetBackgroundNeedsFullPaintInvalidation(true);
  }

  void SetOutlineMayBeAffectedByDescendants(bool b) {
    NOT_DESTROYED();
    bitfields_.SetOutlineMayBeAffectedByDescendants(b);
  }

  inline bool ChildLayoutBlockedByDisplayLock() const {
    NOT_DESTROYED();
    auto* context = GetDisplayLockContext();
    return context && !context->ShouldLayoutChildren();
  }

  bool ChildPrePaintBlockedByDisplayLock() const {
    NOT_DESTROYED();
    auto* context = GetDisplayLockContext();
    return context && !context->ShouldPrePaintChildren();
  }

  bool ChildPaintBlockedByDisplayLock() const {
    NOT_DESTROYED();
    auto* context = GetDisplayLockContext();
    return context && !context->ShouldPaintChildren();
  }

  bool BeingDestroyed() const {
    NOT_DESTROYED();
    return bitfields_.BeingDestroyed();
  }

  bool IsTableColumnsConstraintsDirty() const {
    NOT_DESTROYED();
    return bitfields_.IsTableColumnsConstraintsDirty();
  }

  void SetTableColumnConstraintDirty(bool b) {
    NOT_DESTROYED();
    bitfields_.SetIsTableColumnsConstraintsDirty(b);
  }

  bool IsGridPlacementDirty() const {
    NOT_DESTROYED();
    return bitfields_.IsGridPlacementDirty();
  }

  void SetGridPlacementDirty(bool b) {
    NOT_DESTROYED();
    bitfields_.SetIsGridPlacementDirty(b);
  }

  bool IsSubgridMinMaxSizesCacheDirty() const {
    NOT_DESTROYED();
    return bitfields_.IsSubgridMinMaxSizesCacheDirty();
  }

  void SetSubgridMinMaxSizesCacheDirty(bool b) {
    NOT_DESTROYED();
    bitfields_.SetIsSubgridMinMaxSizesCacheDirty(b);
  }

  DisplayLockContext* GetDisplayLockContext() const {
    NOT_DESTROYED();
    auto* element = DynamicTo<Element>(GetNode());
    if (!element)
      return nullptr;
    return element->GetDisplayLockContext();
  }

  void SetDocumentForAnonymous(Document* document) {
    NOT_DESTROYED();
    DCHECK(IsAnonymous());
    node_ = document;
  }

#if DCHECK_IS_ON()
  // Return true if the layout object isn't part of the DOM tree. Such layout
  // objects either have no parent (even if it isn't a LayoutView), or is a
  // descendant of such an object, and are managed by something else than the
  // regular layout object tree builder. One example of this is @page margin
  // boxes.
  bool IsInDetachedNonDomTree() const {
    NOT_DESTROYED();
    return is_in_detached_non_dom_tree_;
  }
  void SetIsDetachedNonDomRoot(bool b) {
    NOT_DESTROYED();
    DCHECK(!Parent());
    is_in_detached_non_dom_tree_ = b;
  }
  void InheritIsInDetachedNonDomTree(const LayoutObject& parent) {
    NOT_DESTROYED();
    is_in_detached_non_dom_tree_ = parent.IsInDetachedNonDomTree();
  }
#else
  void InheritIsInDetachedNonDomTree(const LayoutObject& parent) {
    NOT_DESTROYED();
  }
  void SetIsDetachedNonDomRoot(bool) { NOT_DESTROYED(); }
#endif  // DCHECK_IS_ON()

  bool PreviousVisibilityVisible() const {
    NOT_DESTROYED();
    return bitfields_.PreviousVisibilityVisible();
  }

  // See LocalVisualRect().
  virtual bool VisualRectRespectsVisibility() const {
    NOT_DESTROYED();
    return true;
  }

  bool TransformAffectsVectorEffect() const {
    NOT_DESTROYED();
    return bitfields_.TransformAffectsVectorEffect();
  }

  bool SVGDescendantMayHaveTransformRelatedAnimation() const {
    NOT_DESTROYED();
    return bitfields_.SVGDescendantMayHaveTransformRelatedAnimation();
  }
  void SetSVGDescendantMayHaveTransformRelatedAnimation();

  bool HasViewportDependence() const {
    NOT_DESTROYED();
    return bitfields_.HasViewportDependence();
  }
  void SetHasViewportDependence(bool b) {
    NOT_DESTROYED();
    bitfields_.SetHasViewportDependence(b);
  }

  bool ShouldSkipNextLayoutShiftTracking() const {
    NOT_DESTROYED();
    return bitfields_.ShouldSkipNextLayoutShiftTracking();
  }
  void SetShouldSkipNextLayoutShiftTracking(bool b) {
    NOT_DESTROYED();
    bitfields_.SetShouldSkipNextLayoutShiftTracking(b);
  }

  bool ShouldAssumePaintOffsetTranslationForLayoutShiftTracking() const {
    NOT_DESTROYED();
    return bitfields_
        .ShouldAssumePaintOffsetTranslationForLayoutShiftTracking();
  }
  void SetShouldAssumePaintOffsetTranslationForLayoutShiftTracking(bool b) {
    NOT_DESTROYED();
    bitfields_.SetShouldAssumePaintOffsetTranslationForLayoutShiftTracking(b);
  }

  bool ScrollableAreaSizeChanged() const {
    NOT_DESTROYED();
    return bitfields_.ScrollableAreaSizeChanged();
  }
  void SetScrollableAreaSizeChanged(bool b) {
    NOT_DESTROYED();
    bitfields_.SetScrollableAreaSizeChanged(b);
  }

  bool MayBeNonContiguousIfc() const {
    NOT_DESTROYED();
    return bitfields_.MayBeNonContiguousIfc();
  }
  void SetMayBeNonContiguousIfc(bool b) {
    NOT_DESTROYED();
    bitfields_.SetMayBeNonContiguousIfc(b);
  }

  bool HasSVGTextDescendants() const {
    NOT_DESTROYED();
    return bitfields_.HasSVGTextDescendants();
  }
  void SetHasSVGTextDescendants(bool b) {
    NOT_DESTROYED();
    bitfields_.SetHasSVGTextDescendants(b);
  }

  // Returns true if this layout object is created for an element which will be
  // changing behaviour for overflow: visible.
  // See
  // https://groups.google.com/a/chromium.org/g/blink-dev/c/MuTeW_AFgxA/m/IlT4QVEfAgAJ
  // for details.
  bool BelongsToElementChangingOverflowBehaviour() const;

 protected:
  void SetDestroyedForTesting() {
    NOT_DESTROYED();
    bitfields_.SetBeingDestroyed(true);
#if DCHECK_IS_ON()
    is_destroyed_ = true;
#endif
  }

  const ComputedStyle& SlowEffectiveStyle(StyleVariant style_variant) const;

  // Updates only the local style ptr of the object.  Does not update the state
  // of the object, and so only should be called when the style is known not to
  // have changed (or from SetStyle).
  void SetStyleInternal(const ComputedStyle* style) {
    NOT_DESTROYED();
    CHECK(style);
    style_ = std::move(style);
  }

  // Set style to null. This is needed during object construction in some
  // cases. CreateObject() is expected to return a layout object with nullptr
  // style, but in some cases, during construction, we need to set style
  // temporarily (and then call this function to reset it again before
  // returning).
  void ResetStyle() {
    NOT_DESTROYED();
    style_ = nullptr;
  }

  // Overrides should call the superclass at the end. style_ will be 0 the
  // first time this function will be called.
  virtual void StyleWillChange(StyleDifference, const ComputedStyle& new_style);
  // Overrides should call the superclass at the start. |oldStyle| will be 0 the
  // first time this function is called.
  virtual void StyleDidChange(StyleDifference, const ComputedStyle* old_style);
  void PropagateStyleToAnonymousChildren();
  // Return true for objects that don't want style changes automatically
  // propagated via propagateStyleToAnonymousChildren(), but rather rely on
  // other custom mechanisms (if they need to be notified of parent style
  // changes at all).
  virtual bool AnonymousHasStylePropagationOverride() {
    NOT_DESTROYED();
    return false;
  }

  virtual void InLayoutNGInlineFormattingContextWillChange(bool) {
    NOT_DESTROYED();
  }

  // A fast path for MapToVisualRectInAncestorSpace for when GeometryMapper
  // can be used. |intersects| is set to whether the input rect intersected
  // (see documentation of return value of MapToVisualRectInAncestorSpace).
  //
  // The return value of this method is whether the fast path could be used.
  bool MapToVisualRectInAncestorSpaceInternalFastPath(
      const LayoutBoxModelObject* ancestor,
      gfx::RectF&,
      VisualRectFlags,
      bool& intersects) const;

  // This function is called before calling the destructor so that some clean-up
  // can happen regardless of whether they call a virtual function or not. As a
  // rule of thumb, this function should be preferred to the destructor. See
  // destroy() that is the one calling willBeDestroyed().
  //
  // There are 2 types of destructions: regular destructions and tree tear-down.
  // Regular destructions happen when the renderer is not needed anymore (e.g.
  // 'display' changed or the DOM Node was removed).
  // Tree tear-down is when the whole tree destroyed during navigation. It is
  // handled in the code by checking if documentBeingDestroyed() returns 'true'.
  // In this case, the code skips some unneeded expensive operations as we know
  // the tree is not reused (e.g. avoid clearing the containing block's line
  // box).
  virtual void WillBeDestroyed();

  virtual void InsertedIntoTree();
  virtual void WillBeRemovedFromTree();

#if DCHECK_IS_ON()
  virtual bool PaintInvalidationStateIsDirty() const;
#endif

  // Called before paint invalidation.
  virtual void EnsureIsReadyForPaintInvalidation();
  virtual void ClearPaintFlags();

  void SetIsBackgroundAttachmentFixedObject(bool);
  void SetCanCompositeBackgroundAttachmentFixed(bool);

  void SetEverHadLayout() {
    NOT_DESTROYED();
    bitfields_.SetEverHadLayout(true);
  }

  virtual bool CanBeSelectionLeafInternal() const {
    NOT_DESTROYED();
    return false;
  }

  virtual PhysicalOffset OffsetFromContainerInternal(
      const LayoutObject*,
      MapCoordinatesFlags mode) const;
  PhysicalOffset OffsetFromScrollableContainer(const LayoutObject*,
                                               MapCoordinatesFlags mode) const;

  virtual void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                                       const LayoutBoxModelObject* ancestor,
                                       MapCoordinatesFlags) const {
    NOT_DESTROYED();
  }

  bool BackgroundIsKnownToBeObscured() const {
    NOT_DESTROYED();
    DCHECK_GE(GetDocument().Lifecycle().GetState(),
              DocumentLifecycle::kInPrePaint);
    return bitfields_.BackgroundIsKnownToBeObscured();
  }
  void SetBackgroundIsKnownToBeObscured(bool b) {
    NOT_DESTROYED();
    DCHECK_EQ(GetDocument().Lifecycle().GetState(),
              DocumentLifecycle::kInPrePaint);
    bitfields_.SetBackgroundIsKnownToBeObscured(b);
  }

  // Returns ContainerForAbsolutePosition() if it's a LayoutBlock, or the
  // containing LayoutBlock of it.
  LayoutBlock* ContainingBlockForAbsolutePosition(
      AncestorSkipInfo* = nullptr) const;
  // Returns ContainerForFixedPosition() if it's a LayoutBlock, or the
  // containing LayoutBlock of it.
  LayoutBlock* ContainingBlockForFixedPosition(
      AncestorSkipInfo* = nullptr) const;

  // Returns the first line style declared in CSS. The style may be declared on
  // an ancestor block (see LayoutBlock::FirstLineStyleParentBlock) that applies
  // to this object. Returns nullptr if there is no applicable first line style.
  // Whether the style applies is based on CSS rules, regardless of whether this
  // object is really in the first line which is unknown before layout.
  const ComputedStyle* FirstLineStyleWithoutFallback() const;

  void SetTransformAffectsVectorEffect(bool b) {
    NOT_DESTROYED();
    DCHECK(IsSVGChild());
    bitfields_.SetTransformAffectsVectorEffect(b);
  }

  void ClearSVGDescendantMayHaveTransformRelatedAnimation() {
    NOT_DESTROYED();
    DCHECK(IsSVGChild());
    bitfields_.SetSVGDescendantMayHaveTransformRelatedAnimation(false);
  }

  void SetMightTraversePhysicalFragments(bool b) {
    NOT_DESTROYED();
    bitfields_.SetMightTraversePhysicalFragments(b);
  }

  void SetHasValidCachedGeometry(bool b) {
    NOT_DESTROYED();
    bitfields_.SetHasValidCachedGeometry(b);
  }
  bool HasValidCachedGeometry() const {
    NOT_DESTROYED();
    return bitfields_.HasValidCachedGeometry();
  }

  // For LayoutBox. They are here to use the bit fields.
  BackgroundPaintLocation GetBackgroundPaintLocation() const {
    NOT_DESTROYED();
    DCHECK(IsBox());
    return static_cast<BackgroundPaintLocation>(background_paint_location_);
  }
  void SetBackgroundPaintLocation(BackgroundPaintLocation location) {
    NOT_DESTROYED();
    DCHECK(IsBox());
    if (GetBackgroundPaintLocation() != location) {
      SetBackgroundNeedsFullPaintInvalidation();
      background_paint_location_ = static_cast<unsigned>(location);
      DCHECK_EQ(location, GetBackgroundPaintLocation());
    }
  }

 private:
  gfx::QuadF LocalToAncestorQuadInternal(const gfx::QuadF&,
                                         const LayoutBoxModelObject* ancestor,
                                         MapCoordinatesFlags = 0) const;

  void ClearLayoutRootIfNeeded() const;

  void ScheduleRelayout();

  void AddAsImageObserver(StyleImage*);
  void RemoveAsImageObserver(StyleImage*);

  void UpdateImage(StyleImage*, StyleImage*);
  void UpdateShapeImage(const ShapeValue*, const ShapeValue*);
  void UpdateFillImages(const FillLayer* old_layers,
                        const FillLayer* new_layers);
  void UpdateCursorImages(const CursorList* old_cursors,
                          const CursorList* new_cursors);

  // Walk up the parent chain and find the first scrolling block to disable
  // scroll anchoring on.
  void SetScrollAnchorDisablingStyleChangedOnAncestor();

  bool SelfPaintingLayerNeedsVisualOverflowRecalc() const;
  inline void MarkContainerChainForOverflowRecalcIfNeeded(
      bool mark_container_chain_scrollable_overflow_recalc);

  inline void InvalidateContainerIntrinsicLogicalWidths();

  // Call |SetShouldDoFullPaintInvalidation| for LayoutNG or
  // |SetShouldInvalidateSelection| on all selected children.
  void InvalidateSelectedChildrenOnStyleChange();

  LayoutFlowThread* LocateFlowThreadContainingBlock() const;
  void RemoveFromLayoutFlowThreadRecursive(LayoutFlowThread*);

  // Returns `true` if the LayoutObject is for the specified pseudo-element
  // type.
  inline bool IsPseudoElementContent(PseudoId pseudo_id) const;

  // It's unclear why Clang doesn't inline this.
  ALWAYS_INLINE
  StyleDifference AdjustStyleDifference(StyleDifference) const;

  bool IsTextOrSVGChild() const {
    NOT_DESTROYED();
    return IsText() || IsSVGChild();
  }

  static bool IsAllowedToModifyLayoutTreeStructure(Document&);

  void UpdateImageObservers(const ComputedStyle* old_style,
                            const ComputedStyle* new_style);
  void UpdateFirstLineImageObservers(const ComputedStyle* new_style);

  void ApplyPseudoElementStyleChanges(const ComputedStyle* old_style);
  void ApplyFirstLineChanges(const ComputedStyle* old_style);

  void MarkSelfPaintingLayerForVisualOverflowRecalc();

  void SetShouldDoFullPaintInvalidationWithoutLayoutChangeInternal(
      PaintInvalidationReason);

  // Additional bitfields.
  // These are not in LayoutObjectBitfields, to fill the gap between
  // the inherited DisplayItemClient data fields and bitfields_.

  // This is set by Set[Subtree]ShouldDoFullPaintInvalidation() or
  // SetShouldInvalidatePaintForHitTest(), and cleared during PrePaint in this
  // object's InvalidatePaint(). It's different from
  // DisplayItemClient::GetPaintInvalidationReason() which is set during
  // PrePaint and cleared in PaintController::FinishCycle().
  unsigned paint_invalidation_reason_for_pre_paint_ : 6;

  // This is the cached 'position' value of this object
  // (see ComputedStyle::position).
  unsigned positioned_state_ : 2;  // PositionedState

  // `selection_state_` is direct mapping of the DOM selection into the
  // respective LayoutObjects that `CanBeSelectionLeaf()`.
  // `selection_state_for_paint_` is adjusted so that the state takes into
  // account whether such a LayoutObject will be painted. If selection
  // starts/ends in an object that is not painted, we won't be able to record
  // the bounds for composited selection state that is pushed to cc.
  unsigned selection_state_ : 3;            // SelectionState
  unsigned selection_state_for_paint_ : 3;  // SelectionState

  // Reasons for the full subtree invalidation.
  unsigned subtree_paint_property_update_reasons_
      : kSubtreePaintPropertyUpdateReasonsBitfieldWidth;

  // For LayoutBox. It's updated during PrePaint.
  unsigned background_paint_location_ : 2;  // BackgroundPaintLocation.

  unsigned overflow_clip_axes_ : 2;

#if DCHECK_IS_ON()
  unsigned has_ax_object_ : 1;
  unsigned set_needs_layout_forbidden_ : 1;
  unsigned as_image_observer_count_ : 20;
  unsigned is_in_detached_non_dom_tree_ : 1 = false;
#endif

#define ADD_BOOLEAN_BITFIELD(field_name_, MethodNameBase)               \
 public:                                                                \
  bool MethodNameBase() const { return field_name_; }                   \
  void Set##MethodNameBase(bool new_value) { field_name_ = new_value; } \
                                                                        \
 private:                                                               \
  unsigned field_name_ : 1

  class LayoutObjectBitfields {
    DISALLOW_NEW();

   public:
    // LayoutObjectBitfields holds all the boolean values for LayoutObject.
    //
    // This is done to promote better packing on LayoutObject (at the expense of
    // preventing bit field packing for the subclasses). Classes concerned about
    // packing and memory use should hoist their boolean to this class. See
    // below the field from sub-classes (e.g. childrenInline).
    //
    // Some of those booleans are caches of ComputedStyle values (e.g.
    // positionState). This enables better memory locality and thus better
    // performance.
    //
    // This class is an artifact of the WebKit era where LayoutObject wasn't
    // allowed to grow and each sub-class was strictly monitored for memory
    // increase. Our measurements indicate that the size of LayoutObject and
    // subsequent classes do not impact memory or speed in a significant
    // manner. This is based on growing LayoutObject in
    // https://codereview.chromium.org/44673003 and subsequent relaxations
    // of the memory constraints on layout objects.
    explicit LayoutObjectBitfields(Node* node)
        : self_needs_full_layout_(false),
          child_needs_full_layout_(false),
          needs_simplified_layout_(false),
          self_needs_scrollable_overflow_recalc_(false),
          child_needs_scrollable_overflow_recalc_(false),
          intrinsic_logical_widths_dirty_(false),
          intrinsic_logical_widths_depends_on_block_constraints_(true),
          indefinite_intrinsic_logical_widths_dirty_(true),
          definite_intrinsic_logical_widths_dirty_(true),
          needs_collect_inlines_(false),
          should_check_for_paint_invalidation_(true),
          subtree_should_check_for_paint_invalidation_(false),
          should_delay_full_paint_invalidation_(false),
          subtree_should_do_full_paint_invalidation_(false),
          may_need_paint_invalidation_animated_background_image_(false),
          should_invalidate_selection_(false),
          should_check_layout_for_paint_invalidation_(true),
          descendant_should_check_layout_for_paint_invalidation_(true),
          needs_paint_property_update_(true),
          descendant_needs_paint_property_update_(true),
          floating_(false),
          is_anonymous(!node),
          is_inline_(true),
          is_in_layout_ng_inline_formatting_context_(false),
          is_atomic_inline_level_(false),
          horizontal_writing_mode_(true),
          has_layer_(false),
          has_non_visible_overflow_(false),
          has_transform_related_property_(false),
          has_reflection_(false),
          can_contain_absolute_position_objects_(false),
          can_contain_fixed_position_objects_(false),
          ever_had_layout_(false),
          is_inside_flow_thread_(false),
          subtree_change_listener_registered_(false),
          notified_of_subtree_change_(false),
          consumes_subtree_change_notification_(false),
          children_inline_(false),
          always_create_line_boxes_for_layout_inline_(false),
          background_is_known_to_be_obscured_(false),
          is_background_attachment_fixed_object_(false),
          can_composite_background_attachment_fixed_(false),
          is_scroll_anchor_object_(false),
          scroll_anchor_disabling_style_changed_(false),
          should_skip_layout_cache_(false),
          has_box_decoration_background_(false),
          background_needs_full_paint_invalidation_(true),
          outline_may_be_affected_by_descendants_(false),
          previous_outline_may_be_affected_by_descendants_(false),
          previous_visibility_visible_(false),
          is_truncated_(false),
          inside_blocking_touch_event_handler_(false),
          effective_allowed_touch_action_changed_(true),
          descendant_effective_allowed_touch_action_changed_(false),
          inside_blocking_wheel_event_handler_(false),
          blocking_wheel_event_handler_changed_(true),
          descendant_blocking_wheel_event_handler_changed_(false),
          is_effective_root_scroller_(false),
          is_global_root_scroller_(false),
          registered_as_first_line_image_observer_(false),
          is_html_legend_element_(false),
          being_destroyed_(false),
          is_table_column_constraints_dirty_(false),
          is_grid_placement_dirty_(true),
          is_subgrid_min_max_sizes_cache_dirty_(true),
          transform_affects_vector_effect_(false),
          svg_descendant_may_have_transform_related_animation_(false),
          should_skip_next_layout_shift_tracking_(true),
          should_assume_paint_offset_translation_for_layout_shift_tracking_(
              false),
          might_traverse_physical_fragments_(true),
          whitespace_children_may_change_(false),
          needs_devtools_info_(false),
          may_have_anchor_query_(false),
          has_broken_spine_(false),
          has_valid_cached_geometry_(false),
          may_be_non_contiguous_ifc_(false),
          has_svg_text_descendants_(false) {}

    // Typically indicates that this object has had its style changed, and
    // requires a "full" layout.
    ADD_BOOLEAN_BITFIELD(self_needs_full_layout_, SelfNeedsFullLayout);

    // Indicates that an *inflow* descendant of this object has been marked for
    // full layout. We'll typically run a full layout for these cases.
    ADD_BOOLEAN_BITFIELD(child_needs_full_layout_, ChildNeedsFullLayout);

    // Indicates that an *out-of-flow* positioned descendant requires layout.
    //
    // This will attempt to run "simplified" layout on all inflow children (as
    // they themselves may have OOF positioned children), and run the
    // out-of-flow layout part.
    //
    // This is relatively cheap compuared to "full" layout.
    ADD_BOOLEAN_BITFIELD(needs_simplified_layout_, NeedsSimplifiedLayout);

    ADD_BOOLEAN_BITFIELD(self_needs_scrollable_overflow_recalc_,
                         SelfNeedsScrollableOverflowRecalc);

    ADD_BOOLEAN_BITFIELD(child_needs_scrollable_overflow_recalc_,
                         ChildNeedsScrollableOverflowRecalc);

    // This boolean marks the intrinsic logical widths for lazy recomputation.
    //
    // See INTRINSIC SIZES / PREFERRED LOGICAL WIDTHS above about those
    // widths.
    ADD_BOOLEAN_BITFIELD(intrinsic_logical_widths_dirty_,
                         IntrinsicLogicalWidthsDirty);

    // This boolean indicates if a the result of `LayoutAlgorithm::MinMaxSizes`
    // of this node may depend on the block constraints given by the parent.
    // Used for packing a `MinMaxSizesResult`.
    ADD_BOOLEAN_BITFIELD(intrinsic_logical_widths_depends_on_block_constraints_,
                         IntrinsicLogicalWidthsDependsOnBlockConstraints);

    // Indicates if the indefinite min/max sizes cache slot is dirty.
    ADD_BOOLEAN_BITFIELD(indefinite_intrinsic_logical_widths_dirty_,
                         IndefiniteIntrinsicLogicalWidthsDirty);

    // Indicates if the definite min/max sizes cache slots are dirty.
    ADD_BOOLEAN_BITFIELD(definite_intrinsic_logical_widths_dirty_,
                         DefiniteIntrinsicLogicalWidthsDirty);

    // This flag is set on inline container boxes that need to run the
    // Pre-layout phase in LayoutNG. See InlineNode::CollectInlines().
    // Also maybe set to inline boxes to optimize the propagation.
    ADD_BOOLEAN_BITFIELD(needs_collect_inlines_, NeedsCollectInlines);

    // Paint related dirty bits.
    ADD_BOOLEAN_BITFIELD(should_check_for_paint_invalidation_,
                         ShouldCheckForPaintInvalidation);
    ADD_BOOLEAN_BITFIELD(subtree_should_check_for_paint_invalidation_,
                         SubtreeShouldCheckForPaintInvalidation);
    ADD_BOOLEAN_BITFIELD(should_delay_full_paint_invalidation_,
                         ShouldDelayFullPaintInvalidation);
    ADD_BOOLEAN_BITFIELD(subtree_should_do_full_paint_invalidation_,
                         SubtreeShouldDoFullPaintInvalidation);
    ADD_BOOLEAN_BITFIELD(may_need_paint_invalidation_animated_background_image_,
                         MayNeedPaintInvalidationAnimatedBackgroundImage);
    ADD_BOOLEAN_BITFIELD(should_invalidate_selection_,
                         ShouldInvalidateSelection);
    ADD_BOOLEAN_BITFIELD(should_check_layout_for_paint_invalidation_,
                         ShouldCheckLayoutForPaintInvalidation);
    ADD_BOOLEAN_BITFIELD(descendant_should_check_layout_for_paint_invalidation_,
                         DescendantShouldCheckLayoutForPaintInvalidation);
    // Whether the paint properties need to be updated. For more details, see
    // LayoutObject::NeedsPaintPropertyUpdate().
    ADD_BOOLEAN_BITFIELD(needs_paint_property_update_,
                         NeedsPaintPropertyUpdate);
    // Whether the paint properties of a descendant need to be updated. For more
    // details, see LayoutObject::DescendantNeedsPaintPropertyUpdate().
    ADD_BOOLEAN_BITFIELD(descendant_needs_paint_property_update_,
                         DescendantNeedsPaintPropertyUpdate);
    // End paint related dirty bits.

    // This boolean is the cached value of 'float'
    // (see ComputedStyle::isFloating).
    ADD_BOOLEAN_BITFIELD(floating_, Floating);

    ADD_BOOLEAN_BITFIELD(is_anonymous, IsAnonymous);

    // This boolean represents whether the LayoutObject is 'inline-level'
    // (a CSS concept). Inline-level boxes are laid out inside a line. If
    // unset, the box is 'block-level' and thus stack on top of its
    // siblings (think of paragraphs).
    ADD_BOOLEAN_BITFIELD(is_inline_, IsInline);

    // This boolean is set when this LayoutObject is in LayoutNG inline
    // formatting context. Note, this LayoutObject itself may be laid out by
    // legacy.
    ADD_BOOLEAN_BITFIELD(is_in_layout_ng_inline_formatting_context_,
                         IsInLayoutNGInlineFormattingContext);

    // This boolean is set if the element is an atomic inline-level box.
    //
    // In CSS, atomic inline-level boxes are laid out on a line but they
    // are opaque from the perspective of line layout. This means that they
    // can't be split across lines like normal inline boxes (LayoutInline).
    // Examples of atomic inline-level elements: inline tables, inline
    // blocks and replaced inline elements.
    // See http://www.w3.org/TR/CSS2/visuren.html#inline-boxes.
    //
    // Our code is confused about the use of this boolean and confuses it
    // with being replaced (see LayoutReplaced about this).
    // TODO(jchaffraix): We should inspect callers and clarify their use.
    // TODO(jchaffraix): We set this boolean for replaced elements that are
    // not inline but shouldn't (crbug.com/567964). This should be enforced.
    ADD_BOOLEAN_BITFIELD(is_atomic_inline_level_, IsAtomicInlineLevel);
    ADD_BOOLEAN_BITFIELD(horizontal_writing_mode_, HorizontalWritingMode);

    ADD_BOOLEAN_BITFIELD(has_layer_, HasLayer);

    // This boolean is set if overflow != 'visible'.
    // This means that this object may need an overflow clip to be applied
    // at paint time to its visual overflow (see OverflowModel for more
    // details). Only set for LayoutBoxes and descendants.
    ADD_BOOLEAN_BITFIELD(has_non_visible_overflow_, HasNonVisibleOverflow);

    // The cached value from ComputedStyle::HasTransformRelatedProperty for
    // objects that do not ignore transform-related styles (e.g. not
    // LayoutInline).
    ADD_BOOLEAN_BITFIELD(has_transform_related_property_,
                         HasTransformRelatedProperty);
    ADD_BOOLEAN_BITFIELD(has_reflection_, HasReflection);

    // This boolean is used to know if this LayoutObject is a container for
    // absolute position descendants.
    ADD_BOOLEAN_BITFIELD(can_contain_absolute_position_objects_,
                         CanContainAbsolutePositionObjects);
    // This boolean is used to know if this LayoutObject is a container for
    // fixed position descendants.
    ADD_BOOLEAN_BITFIELD(can_contain_fixed_position_objects_,
                         CanContainFixedPositionObjects);

    ADD_BOOLEAN_BITFIELD(ever_had_layout_, EverHadLayout);

    ADD_BOOLEAN_BITFIELD(is_inside_flow_thread_, IsInsideFlowThread);

    ADD_BOOLEAN_BITFIELD(subtree_change_listener_registered_,
                         SubtreeChangeListenerRegistered);
    ADD_BOOLEAN_BITFIELD(notified_of_subtree_change_, NotifiedOfSubtreeChange);
    ADD_BOOLEAN_BITFIELD(consumes_subtree_change_notification_,
                         ConsumesSubtreeChangeNotification);

    // from LayoutBlock
    ADD_BOOLEAN_BITFIELD(children_inline_, ChildrenInline);

    // from LayoutInline
    ADD_BOOLEAN_BITFIELD(always_create_line_boxes_for_layout_inline_,
                         AlwaysCreateLineBoxesForLayoutInline);

    // For LayoutBox to cache the result of LayoutBox::
    // ComputeBackgroundIsKnownToBeObscured(). It's updated during PrePaint.
    ADD_BOOLEAN_BITFIELD(background_is_known_to_be_obscured_,
                         BackgroundIsKnownToBeObscured);

    ADD_BOOLEAN_BITFIELD(is_background_attachment_fixed_object_,
                         IsBackgroundAttachmentFixedObject);
    ADD_BOOLEAN_BITFIELD(can_composite_background_attachment_fixed_,
                         CanCompositeBackgroundAttachmentFixed);
    ADD_BOOLEAN_BITFIELD(is_scroll_anchor_object_, IsScrollAnchorObject);

    // Whether changes in this LayoutObject's CSS properties since the last
    // layout should suppress any adjustments that would be made during the next
    // layout by ScrollAnchor objects for which this LayoutObject is on the path
    // from the anchor node to the scroller.
    // See http://bit.ly/sanaclap for more info.
    ADD_BOOLEAN_BITFIELD(scroll_anchor_disabling_style_changed_,
                         ScrollAnchorDisablingStyleChanged);

    ADD_BOOLEAN_BITFIELD(should_skip_layout_cache_, ShouldSkipLayoutCache);

    ADD_BOOLEAN_BITFIELD(has_box_decoration_background_,
                         HasBoxDecorationBackground);

    ADD_BOOLEAN_BITFIELD(background_needs_full_paint_invalidation_,
                         BackgroundNeedsFullPaintInvalidation);

    // Whether shape of outline may be affected by any descendants. This is
    // updated before paint invalidation, checked during paint invalidation.
    ADD_BOOLEAN_BITFIELD(outline_may_be_affected_by_descendants_,
                         OutlineMayBeAffectedByDescendants);
    // The outlineMayBeAffectedByDescendants status of the last paint
    // invalidation.
    ADD_BOOLEAN_BITFIELD(previous_outline_may_be_affected_by_descendants_,
                         PreviousOutlineMayBeAffectedByDescendants);
    // CSS visibility : visible status of the last paint invalidation.
    ADD_BOOLEAN_BITFIELD(previous_visibility_visible_,
                         PreviousVisibilityVisible);

    ADD_BOOLEAN_BITFIELD(is_truncated_, IsTruncated);

    // Whether this object's Node has a blocking touch event handler on itself
    // or an ancestor. This is updated during the PrePaint phase.
    ADD_BOOLEAN_BITFIELD(inside_blocking_touch_event_handler_,
                         InsideBlockingTouchEventHandler);

    // Set when |EffectiveAllowedTouchAction| changes (i.e., blocking touch
    // event handlers change or effective touch action style changes). This only
    // needs to be set on the object that changes as the PrePaint walk will
    // ensure descendants are updated.
    ADD_BOOLEAN_BITFIELD(effective_allowed_touch_action_changed_,
                         EffectiveAllowedTouchActionChanged);

    // Set when a descendant's |EffectiveAllowedTouchAction| changes. This
    // is used to ensure the PrePaint tree walk processes objects with
    // |effective_allowed_touch_action_changed_|.
    ADD_BOOLEAN_BITFIELD(descendant_effective_allowed_touch_action_changed_,
                         DescendantEffectiveAllowedTouchActionChanged);

    // Whether this object's Node has a blocking wheel event handler on itself
    // or an ancestor. This is updated during the PrePaint phase.
    ADD_BOOLEAN_BITFIELD(inside_blocking_wheel_event_handler_,
                         InsideBlockingWheelEventHandler);

    // Set when |InsideBlockingWheelEventHandler| changes (i.e., blocking wheel
    // event handlers change). This only needs to be set on the object that
    // changes as the PrePaint walk will ensure descendants are updated.
    ADD_BOOLEAN_BITFIELD(blocking_wheel_event_handler_changed_,
                         BlockingWheelEventHandlerChanged);

    // Set when a descendant's |InsideBlockingWheelEventHandler| changes. This
    // is used to ensure the PrePaint tree walk processes objects with
    // |blocking_wheel_event_handler_changed_|.
    ADD_BOOLEAN_BITFIELD(descendant_blocking_wheel_event_handler_changed_,
                         DescendantBlockingWheelEventHandlerChanged);

    // See page/scrolling/README.md for an explanation of root scroller and how
    // it works.
    ADD_BOOLEAN_BITFIELD(is_effective_root_scroller_, IsEffectiveRootScroller);
    ADD_BOOLEAN_BITFIELD(is_global_root_scroller_, IsGlobalRootScroller);

    // Indicates whether this object has been added as a first line image
    // observer.
    ADD_BOOLEAN_BITFIELD(registered_as_first_line_image_observer_,
                         RegisteredAsFirstLineImageObserver);

    // Whether this object's |Node| is a HTMLLegendElement. Used to increase
    // performance of |IsRenderedLegend| which is performance sensitive.
    ADD_BOOLEAN_BITFIELD(is_html_legend_element_, IsHTMLLegendElement);

    // True at start of |Destroy()| before calling |WillBeDestroyed()|.
    ADD_BOOLEAN_BITFIELD(being_destroyed_, BeingDestroyed);

    // Column constraints are cached on LayoutNGTable.
    // When this flag is set, any cached constraints are invalid.
    ADD_BOOLEAN_BITFIELD(is_table_column_constraints_dirty_,
                         IsTableColumnsConstraintsDirty);

    // Grid item placement is cached on `LayoutGrid`.
    // When this flag is set, any cached item placements are invalid.
    ADD_BOOLEAN_BITFIELD(is_grid_placement_dirty_, IsGridPlacementDirty);

    // Subgrid `MinMaxSizes` are cached on `LayoutGrid`.
    // When this flag is set, a subgrid's cached `MinMaxSizes` are invalid.
    ADD_BOOLEAN_BITFIELD(is_subgrid_min_max_sizes_cache_dirty_,
                         IsSubgridMinMaxSizesCacheDirty);

    // For transformable SVG child objects, indicates if this object or any
    // descendant has special vector effect that is affected by transform on
    // this object. For an SVG child object having special vector effect, this
    // flag is set on all transformable ancestors up to the SVG root (not
    // included).
    ADD_BOOLEAN_BITFIELD(transform_affects_vector_effect_,
                         TransformAffectsVectorEffect);

    // For SVG child objects, indicates if this object or any descendant may
    // have transform-related animation. This flag is set on all ancestors up
    // to the SVG root (not included) when an SVG child starts a
    // transform-related animation. It's cleared lazily during layout of an
    // SVG container if the container doesn't have any animating descendants.
    ADD_BOOLEAN_BITFIELD(svg_descendant_may_have_transform_related_animation_,
                         SVGDescendantMayHaveTransformRelatedAnimation);

    // For SVG objects, indicates if this object or any descendant depends on
    // the dimensions of the viewport. Updated during layout.
    ADD_BOOLEAN_BITFIELD(has_viewport_dependence_, HasViewportDependence);

    // Whether to skip layout shift tracking in the next paint invalidation.
    // See PaintInvalidator::UpdateLayoutShiftTracking().
    ADD_BOOLEAN_BITFIELD(should_skip_next_layout_shift_tracking_,
                         ShouldSkipNextLayoutShiftTracking);

    // Whether, on the next time PaintPropertyTreeBuilder builds for this
    // object, it should be assumed it had the same paint offset transform last
    // time as it has this time. This is used when layout reattach loses the
    // information from the previous frame; this bit stores that information
    // to inform the next frame for layout shift tracking.
    ADD_BOOLEAN_BITFIELD(
        should_assume_paint_offset_translation_for_layout_shift_tracking_,
        ShouldAssumePaintOffsetTranslationForLayoutShiftTracking);

    // True if there's a possibility that we can walk NG fragment children of
    // this object. False if we definitely need to walk the LayoutObject tree.
    ADD_BOOLEAN_BITFIELD(might_traverse_physical_fragments_,
                         MightTraversePhysicalFragments);

    // True if children that may affect whitespace have been removed. If true
    // during style recalc, mark ancestors for layout tree rebuild to cause a
    // re-evaluation of whitespace children.
    ADD_BOOLEAN_BITFIELD(whitespace_children_may_change_,
                         WhitespaceChildrenMayChange);

    ADD_BOOLEAN_BITFIELD(needs_devtools_info_, NeedsDevtoolsInfo);

    // See comments for |MayHaveAnchorQuery()|.
    ADD_BOOLEAN_BITFIELD(may_have_anchor_query_, MayHaveAnchorQuery);

    // Set if we stopped rebuilding the spine because this object was marked for
    // layout. We don't need to do anything if we actually end up re-laying out
    // the object, but if it turns out that we hit the cache, we need to update
    // the vertebra for this object at that point - i.e. update the associated
    // layout results, by reading out the post-layout results from the children.
    ADD_BOOLEAN_BITFIELD(has_broken_spine_, HasBrokenSpine);

    // True if LayoutBox::frame_size_ has the latest value computed from its
    // physical fragments.
    // This is set to false when LayoutBox::layout_results_ is updated.
    ADD_BOOLEAN_BITFIELD(has_valid_cached_geometry_, HasValidCachedGeometry);

    // True if the size has changed since the associated PaintLayer updated
    // its scrollable area.
    ADD_BOOLEAN_BITFIELD(scrollable_area_size_changed_,
                         ScrollableAreaSizeChanged);

    // For LayoutBlockFlow - if this is an inline formatting context root, this
    // flag is set if the inline formatting context *may* (false positives are
    // okay) be non-contiguous. Sometimes an inline formatting context may
    // start in some fragmentainer, then skip one or more fragmentainers, and
    // then resume again. This may happen for instance if a culled inline is
    // preceded by a tall float that's pushed after (due to size/breaking
    // restrictions) the contents of the culled inline.
    ADD_BOOLEAN_BITFIELD(may_be_non_contiguous_ifc_, MayBeNonContiguousIfc);

    // For LayoutBlock - true if this block has *any* SVG text descendants.
    // Used for invalidation on transform changes.
    ADD_BOOLEAN_BITFIELD(has_svg_text_descendants_, HasSVGTextDescendants);
  };

#undef ADD_BOOLEAN_BITFIELD

  LayoutObjectBitfields bitfields_;

  void SetSelfNeedsFullLayout(bool b) {
    NOT_DESTROYED();
    bitfields_.SetSelfNeedsFullLayout(b);
  }
  void SetChildNeedsFullLayout(bool b) {
    NOT_DESTROYED();
    DCHECK(!GetDocument().InvalidationDisallowed());
    bitfields_.SetChildNeedsFullLayout(b);
    if (b) {
      bitfields_.SetIsSubgridMinMaxSizesCacheDirty(true);
      bitfields_.SetIsTableColumnsConstraintsDirty(true);
    }
  }
  void SetNeedsSimplifiedLayout(bool b) {
    NOT_DESTROYED();
    DCHECK(!GetDocument().InvalidationDisallowed());
    bitfields_.SetNeedsSimplifiedLayout(b);
  }

 private:
  friend class LineLayoutItem;
  friend class LocalFrameView;

  Member<const ComputedStyle> style_;
  Member<Node> node_;

  Member<LayoutObject> parent_;
  Member<LayoutObject> previous_;
  Member<LayoutObject> next_;
  Member<FragmentDataList> fragment_;

  // Store state between styleWillChange and styleDidChange
  static bool affects_parent_block_;

#if DCHECK_IS_ON()
  friend class CachedTextInputInfo;
  bool is_destroyed_ = false;
#endif
};

template <typename T>
struct ThreadingTrait<T, std::enable_if_t<std::is_base_of_v<LayoutObject, T>>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

// Allow equality comparisons of LayoutObjects by reference or pointer,
// interchangeably.
DEFINE_COMPARISON_OPERATORS_WITH_REFERENCES(LayoutObject)

inline bool LayoutObject::DocumentBeingDestroyed() const {
  NOT_DESTROYED();
  return GetDocument().Lifecycle().GetState() >= DocumentLifecycle::kStopping;
}

inline bool LayoutObject::IsPseudoElementContent(PseudoId pseudo_id) const {
  NOT_DESTROYED();
  if (StyleRef().StyleType() != pseudo_id) {
    return false;
  }
  // Text nodes don't have their own styles, so ignore the style on a text node.
  if (IsText() && !IsBR()) {
    return false;
  }
  return true;
}

inline bool LayoutObject::IsCheckContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdCheckMark);
}

inline bool LayoutObject::IsBeforeContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdBefore);
}

inline bool LayoutObject::IsAfterContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdAfter);
}

inline bool LayoutObject::IsPickerIconContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdPickerIcon);
}

inline bool LayoutObject::IsMarkerContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdMarker);
}

inline bool LayoutObject::IsScrollButtonContent() const {
  NOT_DESTROYED();
  if (StyleRef().StyleType() != kPseudoIdScrollButton &&
      StyleRef().StyleType() != kPseudoIdScrollButtonBlockStart &&
      StyleRef().StyleType() != kPseudoIdScrollButtonInlineStart &&
      StyleRef().StyleType() != kPseudoIdScrollButtonInlineEnd &&
      StyleRef().StyleType() != kPseudoIdScrollButtonBlockEnd) {
    return false;
  }
  // Text nodes don't have their own styles, so ignore the style on a text node.
  if (IsText() && !IsBR()) {
    return false;
  }
  return true;
}

inline bool LayoutObject::IsScrollMarkerContent() const {
  NOT_DESTROYED();
  return IsPseudoElementContent(kPseudoIdScrollMarker);
}

inline bool LayoutObject::IsScrollButtonOrMarkerContent() const {
  NOT_DESTROYED();
  return IsScrollButtonContent() || IsScrollMarkerContent();
}

inline bool LayoutObject::IsBeforeOrAfterContent() const {
  NOT_DESTROYED();
  return IsBeforeContent() || IsAfterContent();
}

// setNeedsLayout() won't cause full paint invalidations as
// setNeedsLayoutAndFullPaintInvalidation() does. Otherwise the two methods are
// identical.
inline void LayoutObject::SetNeedsLayout(
    LayoutInvalidationReasonForTracing reason,
    MarkingBehavior mark_parents) {
  NOT_DESTROYED();
#if DCHECK_IS_ON()
  DCHECK(!IsSetNeedsLayoutForbidden());
#endif
  bool already_needed_layout = bitfields_.SelfNeedsFullLayout();
  SetSelfNeedsFullLayout(true);
  SetNeedsOverflowRecalc();
  SetSubgridMinMaxSizesCacheDirty(true);
  SetTableColumnConstraintDirty(true);
  if (!already_needed_layout) {
    DEVTOOLS_TIMELINE_TRACE_EVENT_INSTANT_WITH_CATEGORIES(
        TRACE_DISABLED_BY_DEFAULT("devtools.timeline.invalidationTracking"),
        "LayoutInvalidationTracking",
        inspector_layout_invalidation_tracking_event::Data, this, reason);
    if (mark_parents == kMarkContainerChain) {
      MarkContainerChainForLayout();
    }
  }
}

inline void LayoutObject::SetNeedsLayoutAndFullPaintInvalidation(
    LayoutInvalidationReasonForTracing reason,
    MarkingBehavior mark_parents) {
  NOT_DESTROYED();
  SetNeedsLayout(reason, mark_parents);
  SetShouldDoFullPaintInvalidation();
}

inline void LayoutObject::ClearNeedsLayoutWithoutPaintInvalidation() {
  NOT_DESTROYED();
  // Set flags for later stages/cycles.
  SetEverHadLayout();

  // Clear layout flags.
  SetSelfNeedsFullLayout(false);

  if (!ChildLayoutBlockedByDisplayLock()) {
    SetChildNeedsFullLayout(false);
    SetNeedsSimplifiedLayout(false);
  } else if (!ChildNeedsFullLayout() && !NeedsSimplifiedLayout()) {
    // We aren't clearing the child dirty bits because the node is locked and
    // layout for children is not done. If the children aren't dirty,  we need
    // to notify the display lock that child traversal was blocked so that when
    // the subtree gets updated/unlocked we will traverse the children.
    auto* context = GetDisplayLockContext();
    DCHECK(context);
    context->NotifyChildLayoutWasBlocked();
  }

  SetScrollAnchorDisablingStyleChanged(false);

  SetShouldSkipLayoutCache(false);
}

inline void LayoutObject::ClearNeedsLayout() {
  NOT_DESTROYED();
  ClearNeedsLayoutWithoutPaintInvalidation();
  SetShouldCheckForPaintInvalidation();
}

inline void LayoutObject::ClearNeedsLayoutWithFullPaintInvalidation() {
  NOT_DESTROYED();
  ClearNeedsLayoutWithoutPaintInvalidation();
  SetShouldDoFullPaintInvalidation();
}

inline void LayoutObject::SetChildNeedsLayout(MarkingBehavior mark_parents) {
  NOT_DESTROYED();
#if DCHECK_IS_ON()
  DCHECK(!IsSetNeedsLayoutForbidden());
#endif
  bool already_needed_layout = ChildNeedsFullLayout();
  SetNeedsOverflowRecalc();
  SetChildNeedsFullLayout(true);
  if (!already_needed_layout && mark_parents == kMarkContainerChain) {
    MarkContainerChainForLayout();
  }
}

inline void LayoutObject::SetNeedsSimplifiedLayout() {
  NOT_DESTROYED();
  bool already_needed_layout = NeedsSimplifiedLayout();
  SetNeedsSimplifiedLayout(true);
#if DCHECK_IS_ON()
  DCHECK(!IsSetNeedsLayoutForbidden());
#endif
  if (!already_needed_layout) {
    MarkContainerChainForLayout();
  }
}

// TODO(1229581): Get rid of this.
inline void LayoutObject::SetIsInLayoutNGInlineFormattingContext(
    bool new_value) {
  NOT_DESTROYED();
  DCHECK(!GetDocument().InvalidationDisallowed());
  if (IsInLayoutNGInlineFormattingContext() == new_value)
    return;
  InLayoutNGInlineFormattingContextWillChange(new_value);
  // The association cache for inline fragments is in union. Make sure the
  // cache is cleared before and after changing this flag.
  DCHECK(!HasInlineFragments());
  bitfields_.SetIsInLayoutNGInlineFormattingContext(new_value);
  DCHECK(!HasInlineFragments());
}

inline void LayoutObject::SetHasBoxDecorationBackground(bool b) {
  NOT_DESTROYED();
  DCHECK(!GetDocument().InvalidationDisallowed());
  if (b == bitfields_.HasBoxDecorationBackground())
    return;

  bitfields_.SetHasBoxDecorationBackground(b);
}

enum class LayoutObjectSide {
  kRemainingTextIfOnBoundary,
  kFirstLetterIfOnBoundary
};
CORE_EXPORT const LayoutObject* AssociatedLayoutObjectOf(
    const Node&,
    int offset_in_node,
    LayoutObjectSide = LayoutObjectSide::kRemainingTextIfOnBoundary);

CORE_EXPORT std::ostream& operator<<(std::ostream&, const LayoutObject*);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const LayoutObject&);

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
CORE_EXPORT void ShowTree(const blink::LayoutObject*);
CORE_EXPORT void ShowLayoutTree(const blink::LayoutObject* object1);
// We don't make object2 an optional parameter so that showLayoutTree
// can be called from gdb easily.
CORE_EXPORT void ShowLayoutTree(const blink::LayoutObject* object1,
                                const blink::LayoutObject* object2);

#endif

namespace cppgc {
// Assign LayoutObject to be allocated on custom LayoutObjectSpace.
template <typename T>
struct SpaceTrait<
    T,
    std::enable_if_t<std::is_base_of<blink::LayoutObject, T>::value>> {
  using Space = blink::LayoutObjectSpace;
};
}  // namespace cppgc

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_OBJECT_H_
