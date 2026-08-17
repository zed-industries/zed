// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_LAYOUT_ALGORITHM_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/inline/fragment_items_builder.h"

namespace blink {

struct SvgTextContentRange;

class SvgTextLayoutAlgorithm {
  STACK_ALLOCATED();

 public:
  SvgTextLayoutAlgorithm(InlineNode node, WritingMode writing_mode);

  // Apply SVG specific text layout algorithm to |items|.
  // Text items in |items| will be converted to kSVGText type.
  PhysicalSize Layout(const String& ifc_text_content,
                      FragmentItemsBuilder::ItemWithOffsetList& items);

 private:
  // Returns false if we should skip the following steps.
  bool Setup(wtf_size_t approximate_count);
  void SetFlags(const String& ifc_text_content,
                const FragmentItemsBuilder::ItemWithOffsetList& items);
  void AdjustPositionsDxDy(
      const FragmentItemsBuilder::ItemWithOffsetList& items);
  void ApplyTextLengthAttribute(
      const FragmentItemsBuilder::ItemWithOffsetList& items);
  void ResolveTextLength(const FragmentItemsBuilder::ItemWithOffsetList& items,
                         const SvgTextContentRange& range,
                         Vector<wtf_size_t>& resolved_descendant_node_starts);
  void AdjustPositionsXY(const FragmentItemsBuilder::ItemWithOffsetList& items);
  void ApplyAnchoring(const FragmentItemsBuilder::ItemWithOffsetList& items);
  void PositionOnPath(const FragmentItemsBuilder::ItemWithOffsetList& items);

  PhysicalSize WriteBackToFragmentItems(
      FragmentItemsBuilder::ItemWithOffsetList& items);

  bool IsHorizontal() const {
    return inline_direction_ == PhysicalDirection::kRight;
  }
  bool IsVerticalDownward() const {
    return inline_direction_ == PhysicalDirection::kDown;
  }
  bool IsVerticalUpward() const {
    return inline_direction_ == PhysicalDirection::kUp;
  }
  float ScalingFactorAt(const FragmentItemsBuilder::ItemWithOffsetList& items,
                        wtf_size_t addressable_index) const;
  bool IsFirstCharacterInTextPath(wtf_size_t index) const;

  InlineNode inline_node_;

  // This data member represents the number of addressable characters in the
  // target IFC. It's similar to "count" defined in the specification.
  wtf_size_t addressable_count_;

  // "horizontal" flag defined in the specification.
  // This should be replaced with `inline_direction_`.
  bool horizontal_;
  // A replacement of the "horizontal" flag above.
  const PhysicalDirection inline_direction_;

  struct SvgPerCharacterInfo {
    std::optional<float> x;
    std::optional<float> y;
    std::optional<float> rotate;
    bool hidden = false;
    bool middle = false;
    bool anchored_chunk = false;

    bool in_text_path = false;
    bool text_length_resolved = false;
    float baseline_shift = 0.0f;
    float inline_size = 0.0f;
    float length_adjust_scale = 1.0f;
    float text_length_shift_x = 0.0f;
    float text_length_shift_y = 0.0f;
    wtf_size_t item_index = WTF::kNotFound;
  };
  // This data member represents "result" defined in the specification, but it
  // contains only addressable characters.
  //
  // This is built from FragmentItem text sequence. For example, if the input
  // is two FragmentItems like:
  //  0: <code point 1>, <a lead surrogate>
  //  1: <a trail surrogate>, <code point 2>
  // it produces four entries for <code point 1>, <a lead surroagte>,
  // <a trail surrogate>, and <code point 2>.
  Vector<SvgPerCharacterInfo> result_;

  // This data member represents "CSS_positions" defined in the specification,
  // but it contains only addressable characters.
  Vector<gfx::PointF> css_positions_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_LAYOUT_ALGORITHM_H_
