// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_OOF_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_OOF_LAYOUT_ALGORITHM_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

// Simplified fragmentainer layout algorithm, for OOF descendants. When regular
// layout hasn't created enough fragmentainers (because the OOFs were not known
// at that point), this algorithm will help build additional fragmentainers.
// Additionally, it is used to add additional OOF children that belong in an
// existing fragmentainer, in which case the resulting fragment returned from
// Layout() will just be used to merge the new children into the existing
// existing fragmentainer, by mutating it.
class CORE_EXPORT SimplifiedOofLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  // ``last_fragmentainer`` is the last previously generated fragmentainer,
  // which this algorithm will use as a basis in order to fill out some fields
  // in the builder.
  SimplifiedOofLayoutAlgorithm(const LayoutAlgorithmParams&,
                               const PhysicalBoxFragment& last_fragmentainer);

  const LayoutResult* Layout();
  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&) {
    NOTREACHED();
  }

  // To be called when creating a new column based on an existing one. The break
  // token passed is the outgoing break token from the last column created so
  // far.
  void ResumeColumnLayout(const BlockBreakToken* old_fragment_break_token);

  void SetHasSubsequentChildren() {
    // There will be more fragmentainers after this one. Make sure that an
    // outgoing break token is created, regardless of whether any OOFs in this
    // fragmentainer break or not.
    container_builder_.SetHasSubsequentChildren();
  }

  void AppendOutOfFlowResult(const LayoutResult*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_OOF_LAYOUT_ALGORITHM_H_
