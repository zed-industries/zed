// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_SCRIPTS_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_SCRIPTS_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/core/mathml/mathml_scripts_element.h"

namespace blink {

class BlockNode;

// This algorithm handles msub, msup and msubsup elements.
class CORE_EXPORT MathScriptsLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit MathScriptsLayoutAlgorithm(const LayoutAlgorithmParams& params);

  struct ChildAndMetrics {
    DISALLOW_NEW();

   public:
    Member<const LayoutResult> result;
    LayoutUnit ascent;
    LayoutUnit descent;
    LayoutUnit inline_size;
    LayoutUnit base_italic_correction;
    BoxStrut margins;
    BlockNode node = nullptr;

    void Trace(Visitor* visitor) const {
      visitor->Trace(result);
      visitor->Trace(node);
    }
  };

  struct SubSupPair {
    DISALLOW_NEW();

   public:
    void Trace(Visitor* visitor) const {
      visitor->Trace(sub);
      visitor->Trace(sup);
    }

    BlockNode sub = nullptr;
    BlockNode sup = nullptr;
  };

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);
  const LayoutResult* Layout();

 private:
  void GatherChildren(BlockNode* base,
                      HeapVector<SubSupPair>*,
                      BlockNode* prescripts,
                      unsigned* first_prescript_index,
                      BoxFragmentBuilder* = nullptr) const;

  typedef HeapVector<ChildAndMetrics, 4> ChildrenAndMetrics;

  ChildAndMetrics LayoutAndGetMetrics(BlockNode child) const;

  struct VerticalMetrics {
    STACK_ALLOCATED();

   public:
    LayoutUnit sub_shift;
    LayoutUnit sup_shift;
    LayoutUnit ascent;
    LayoutUnit descent;
    BoxStrut margins;
  };
  VerticalMetrics GetVerticalMetrics(
      const ChildAndMetrics& base_metrics,
      const ChildrenAndMetrics& sub_metrics,
      const ChildrenAndMetrics& sup_metrics) const;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::MathScriptsLayoutAlgorithm::ChildAndMetrics)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::MathScriptsLayoutAlgorithm::SubSupPair)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_SCRIPTS_LAYOUT_ALGORITHM_H_
