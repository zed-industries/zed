// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_UNPOSITIONED_LIST_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_UNPOSITIONED_LIST_MARKER_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BlockNode;
class BoxFragmentBuilder;
class ComputedStyle;
class ConstraintSpace;
class LayoutOutsideListMarker;
class LayoutResult;
class PhysicalFragment;

// Represents an unpositioned list marker.
//
// A list item can have either block children or inline children. Because
// BLockLayoutAlgorithm handles the former while InlineLayoutAlgorithm
// handles the latter, list marker can appear in either algorithm.
//
// To handle these two cases consistently, when list markers appear in these
// algorithm, they are set as "unpositioned", and are propagated to ancestors
// through LayoutResult until they meet the corresponding list items.
//
// In order to adjust with the other content of LI, marker will be handled
// after other children.
// First, try to find the alignment-baseline for the marker. See
// |ContentAlignmentBaseline()| for details.
// If found, layout marker, compute the content adjusted offset and float
// intuded offset. See |AddToBox()| for details.
// If not, layout marker and deal with it in |AddToBoxWithoutLineBoxes()|.
//
// In addition, marker makes LI non self-collapsing. If the BFC block-offset of
// LI isn't resolved after layout marker, we'll resolve it. See
// |BlockLayoutAlgorithm::PositionOrPropagateListMarker()| and
// |BlockLayoutAlgorithm::PositionListMarkerWithoutLineBoxes()| for details.
class CORE_EXPORT UnpositionedListMarker final {
  DISALLOW_NEW();

 public:
  UnpositionedListMarker() : marker_layout_object_(nullptr) {}
  explicit UnpositionedListMarker(LayoutOutsideListMarker*);
  explicit UnpositionedListMarker(const BlockNode&);

  explicit operator bool() const { return marker_layout_object_ != nullptr; }

  // Returns the baseline that the list-marker should place itself along.
  //
  // |std::nullopt| indicates that the child |content| does not have a baseline
  // to align to, and that caller should try next child, or use the
  // |AddToBoxWithoutLineBoxes()| method.
  std::optional<LayoutUnit> ContentAlignmentBaseline(
      const ConstraintSpace&,
      FontBaseline,
      const PhysicalFragment& content) const;
  // Add a fragment for an outside list marker.
  void AddToBox(const ConstraintSpace&,
                FontBaseline,
                const PhysicalFragment& content,
                const BoxStrut&,
                const LayoutResult& marker_layout_result,
                LayoutUnit content_baseline,
                LayoutUnit* block_offset,
                BoxFragmentBuilder*) const;

  // Add a fragment for an outside list marker when the list item has no line
  // boxes. Also adjust |intrinsic_block_size| if it was smaller than the list
  // marker.
  void AddToBoxWithoutLineBoxes(const ConstraintSpace&,
                                FontBaseline,
                                const LayoutResult& marker_layout_result,
                                BoxFragmentBuilder*,
                                LayoutUnit* intrinsic_block_size) const;
  LayoutUnit InlineOffset(const LayoutUnit marker_inline_size) const;

  bool operator==(const UnpositionedListMarker& other) const {
    return marker_layout_object_ == other.marker_layout_object_;
  }

  const LayoutResult* Layout(const ConstraintSpace& parent_space,
                             const ComputedStyle& parent_style,
                             FontBaseline) const;

#if DCHECK_IS_ON()
  void CheckMargin() const;
#endif

  void Trace(Visitor*) const;

 private:
  LayoutUnit ComputeIntrudedFloatOffset(const ConstraintSpace&,
                                        const BoxFragmentBuilder*,
                                        const BoxStrut&,
                                        LayoutUnit) const;

  Member<LayoutOutsideListMarker> marker_layout_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_UNPOSITIONED_LIST_MARKER_H_
