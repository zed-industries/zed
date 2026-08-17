// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_LAYOUT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_LAYOUT_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/fonts/opentype/open_type_math_support.h"

namespace blink {

class BlockNode;
class ConstraintSpace;
class LayoutInputNode;
class SimpleFontData;
struct LogicalSize;
struct MinMaxSizes;
struct MinMaxSizesResult;

// Creates a new constraint space for the current child.
ConstraintSpace CreateConstraintSpaceForMathChild(
    const BlockNode& parent_node,
    const LogicalSize& child_available_size,
    const ConstraintSpace& parent_constraint_space,
    const LayoutInputNode&,
    const LayoutResultCacheSlot = LayoutResultCacheSlot::kLayout,
    const std::optional<ConstraintSpace::MathTargetStretchBlockSizes>
        target_stretch_block_sizes = std::nullopt,
    const std::optional<LayoutUnit> target_stretch_inline_size = std::nullopt);

MinMaxSizesResult ComputeMinAndMaxContentContributionForMathChild(
    const ComputedStyle& parent_style,
    const ConstraintSpace& parent_constraint_space,
    const BlockNode& child,
    LayoutUnit child_available_block_size);

LayoutInputNode FirstChildInFlow(const BlockNode&);
LayoutInputNode NextSiblingInFlow(const BlockNode&);

bool IsValidMathMLFraction(const BlockNode&);
bool IsValidMathMLScript(const BlockNode&);
bool IsValidMathMLRadical(const BlockNode&);

// https://w3c.github.io/mathml-core/#dfn-default-rule-thickness
inline float RuleThicknessFallback(const ComputedStyle& style) {
  const SimpleFontData* font_data = style.GetFont()->PrimaryFont();
  if (!font_data)
    return 0;
  return font_data->GetFontMetrics().UnderlineThickness().value_or(0);
}

LayoutUnit MathAxisHeight(const ComputedStyle& style);

inline std::optional<float> MathConstant(
    const ComputedStyle& style,
    OpenTypeMathSupport::MathConstants constant) {
  const SimpleFontData* font_data = style.GetFont()->PrimaryFont();
  return font_data ? OpenTypeMathSupport::MathConstant(
                         font_data->PlatformData().GetHarfBuzzFace(), constant)
                   : std::nullopt;
}

LayoutUnit FractionLineThickness(const ComputedStyle&);

inline bool HasDisplayStyle(const ComputedStyle& style) {
  return style.MathStyle() == EMathStyle::kNormal;
}

// Get parameters for horizontal positioning of mroot.
// The parameters are defined here:
// https://w3c.github.io/mathml-core/#layout-constants-mathconstants
struct RadicalHorizontalParameters {
  LayoutUnit kern_before_degree;
  LayoutUnit kern_after_degree;
};
RadicalHorizontalParameters GetRadicalHorizontalParameters(
    const ComputedStyle&);

// Get parameters for vertical positioning of msqrt/mroot.
// The parameters are defined here:
// https://w3c.github.io/mathml-core/#layout-constants-mathconstants
struct RadicalVerticalParameters {
  LayoutUnit vertical_gap;
  LayoutUnit rule_thickness;
  LayoutUnit extra_ascender;
  float degree_bottom_raise_percent;
};
RadicalVerticalParameters GetRadicalVerticalParameters(const ComputedStyle&,
                                                       bool has_index);

// https://w3c.github.io/mathml-core/#dfn-preferred-inline-size-of-a-glyph-stretched-along-the-block-axis
MinMaxSizes GetMinMaxSizesForVerticalStretchyOperator(const ComputedStyle&,
                                                      UChar character);

bool IsUnderOverLaidOutAsSubSup(const BlockNode& node);
bool IsTextOnlyToken(const BlockNode& node);
bool IsOperatorWithSpecialShaping(const BlockNode& node);

LayoutUnit MathTableBaseline(const ComputedStyle&, LayoutUnit block_offset);

// For nodes corresponding to embellished operators, this function returns the
// properties of its core operator. Otherwise, it returns a null optional.
// See https://mathml-refresh.github.io/mathml-core/#embellished-operators
struct MathMLEmbellishedOperatorProperties {
  bool has_movablelimits;
  bool is_large_op;
  bool is_stretchy;
  bool is_vertical;
  LayoutUnit lspace;
  LayoutUnit rspace;
};
std::optional<MathMLEmbellishedOperatorProperties>
GetMathMLEmbellishedOperatorProperties(const BlockNode&);

bool IsStretchyOperator(const BlockNode& node, bool stretch_axis_is_vertical);
inline bool IsBlockAxisStretchyOperator(const BlockNode& node) {
  return IsStretchyOperator(node, true);
}
inline bool IsInlineAxisStretchyOperator(const BlockNode& node) {
  return IsStretchyOperator(node, false);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_LAYOUT_UTILS_H_
