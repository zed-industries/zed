// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INVALIDATOR_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_shift_tracker.h"
#include "third_party/blink/renderer/core/paint/paint_property_tree_builder.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LayoutObject;

struct CORE_EXPORT PaintInvalidatorContext {
  STACK_ALLOCATED();

 public:
  PaintInvalidatorContext() = default;

  explicit PaintInvalidatorContext(const PaintInvalidatorContext& parent)
      : parent_context(&parent),
        subtree_flags(parent.subtree_flags),
        painting_layer(parent.painting_layer),
        inside_opaque_layout_shift_root(
            parent.inside_opaque_layout_shift_root) {}

  bool NeedsSubtreeWalk() const {
    return subtree_flags &
           (kSubtreeInvalidationChecking | kSubtreeFullInvalidation |
            kSubtreeFullInvalidationForStackedContents);
  }

  // TODO(pdr): Remove this accessor.
  const PaintInvalidatorContext* ParentContext() const {
    return parent_context;
  }
  const PaintInvalidatorContext* parent_context = nullptr;

  // When adding new subtree flags, ensure |NeedsSubtreeWalk| is updated.
  enum SubtreeFlag {
    kSubtreeInvalidationChecking = 1 << 0,
    kSubtreeFullInvalidation = 1 << 1,
    kSubtreeFullInvalidationForStackedContents = 1 << 2,

    // When this flag is set, no paint or raster invalidation will be issued
    // for the subtree.
    //
    // Context: some objects in this paint walk, for example SVG resource
    // container subtrees, always paint onto temporary PaintControllers which
    // don't have cache, and don't actually have any raster regions, so they
    // don't need any invalidation. They are used as "painting subroutines"
    // for one or more other locations in SVG.
    kSubtreeNoInvalidation = 1 << 6,
  };
  unsigned subtree_flags = 0;

  // The following fields can be null only before
  // PaintInvalidator::updateContext().

  PaintLayer* painting_layer = nullptr;

  // The previous PaintOffset of FragmentData.
  PhysicalOffset old_paint_offset;

  const FragmentData* fragment_data = nullptr;

  // Set when we have entered something that shouldn't track layout shift
  // inside (multicol container).
  bool inside_opaque_layout_shift_root = false;

 private:
  friend class PaintInvalidator;

  std::optional<LayoutShiftTracker::ContainingBlockScope>
      containing_block_scope_;
  const TransformPaintPropertyNodeOrAlias* transform_ = nullptr;
};

class PaintInvalidator final {
  STACK_ALLOCATED();

 public:
  // Returns true if the object is invalidated.
  bool InvalidatePaint(const LayoutObject&,
                       const PrePaintInfo*,
                       const PaintPropertyTreeBuilderContext*,
                       PaintInvalidatorContext&);

  // Process objects needing paint invalidation on the next frame.
  // See the definition of PaintInvalidationDelayedFull for more details.
  void ProcessPendingDelayedPaintInvalidations();

 private:
  friend struct PaintInvalidatorContext;

  ALWAYS_INLINE void UpdatePaintingLayer(const LayoutObject&,
                                         PaintInvalidatorContext&);
  ALWAYS_INLINE void UpdateFromTreeBuilderContext(
      const PaintPropertyTreeBuilderFragmentContext&,
      PaintInvalidatorContext&);
  ALWAYS_INLINE void UpdateLayoutShiftTracking(
      const LayoutObject&,
      const PaintPropertyTreeBuilderFragmentContext&,
      PaintInvalidatorContext&);

  HeapVector<Member<const LayoutObject>> pending_delayed_paint_invalidations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INVALIDATOR_H_
