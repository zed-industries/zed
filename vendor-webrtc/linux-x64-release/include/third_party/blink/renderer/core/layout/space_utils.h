// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SPACE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SPACE_UTILS_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_input_node.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class BoxFragmentBuilder;
class ConstraintSpaceBuilder;
struct BfcOffset;

// Adjusts {@code offset} to the clearance line.
CORE_EXPORT bool AdjustToClearance(LayoutUnit clearance_offset,
                                   BfcOffset* offset);

// Calculate and set the available inline fallback size for orthogonal flow
// children. This size will be used if it's not resolvable via other means [1].
//
// TODO(mstensho): The spec [1] says to use the size of the nearest scrollport
// as constraint, if that's smaller than the initial containing block, but we
// haven't implemented that yet; we always just use the initial containing
// block size.
//
// [1] https://www.w3.org/TR/css-writing-modes-3/#orthogonal-auto
void SetOrthogonalFallbackInlineSize(const ComputedStyle& parent_style,
                                     const LayoutInputNode child,
                                     ConstraintSpaceBuilder* builder);

inline void SetOrthogonalFallbackInlineSizeIfNeeded(
    const ComputedStyle& parent_style,
    const LayoutInputNode child,
    ConstraintSpaceBuilder* builder) {
  if (IsParallelWritingMode(parent_style.GetWritingMode(),
                            child.Style().GetWritingMode())) [[likely]] {
    return;
  }
  SetOrthogonalFallbackInlineSize(parent_style, child, builder);
}

// Only to be called if the child is in a writing-mode parallel with its
// container. Return true if an auto inline-size means that the child should be
// stretched (rather than being shrink-to-fit).
bool ShouldBlockContainerChildStretchAutoInlineSize(const BlockNode&);

// Set up box trimming state on a ConstraintSpaceBuilder for a child of the box
// fragment builder. `known_to_have_successive_content` may have false
// negatives.
void SetTextBoxTrimOnChildSpaceBuilder(const BoxFragmentBuilder&,
                                       bool known_to_have_successive_content,
                                       ConstraintSpaceBuilder*);

inline void SetTextBoxTrimOnChildSpaceBuilder(
    const BoxFragmentBuilder& fragment_builder,
    ConstraintSpaceBuilder* space_builder) {
  SetTextBoxTrimOnChildSpaceBuilder(fragment_builder, false, space_builder);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SPACE_UTILS_H_
