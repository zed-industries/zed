// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_PHYSICAL_LINE_BOX_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_PHYSICAL_LINE_BOX_FRAGMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/platform/fonts/font_height.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class LineBoxFragmentBuilder;

class CORE_EXPORT PhysicalLineBoxFragment final : public PhysicalFragment {
 public:
  enum LineBoxType {
    kNormalLineBox,
    // This is an "empty" line box, or "certain zero-height line boxes":
    // https://drafts.csswg.org/css2/visuren.html#phantom-line-box
    // that are ignored for margin collapsing and for other purposes.
    // https://drafts.csswg.org/css2/box.html#collapsing-margins
    // Also see |InlineItem::IsEmptyItem|.
    kEmptyLineBox
  };

  static const PhysicalLineBoxFragment* Create(LineBoxFragmentBuilder* builder);

  static const PhysicalLineBoxFragment* Clone(const PhysicalLineBoxFragment&);

  using PassKey = base::PassKey<PhysicalLineBoxFragment>;
  PhysicalLineBoxFragment(PassKey, LineBoxFragmentBuilder* builder);
  PhysicalLineBoxFragment(PassKey, const PhysicalLineBoxFragment&);
  ~PhysicalLineBoxFragment();

  void TraceAfterDispatch(Visitor*) const;

  LineBoxType GetLineBoxType() const {
    return static_cast<LineBoxType>(sub_type_);
  }
  bool IsEmptyLineBox() const { return GetLineBoxType() == kEmptyLineBox; }

  // True if descendants were propagated to outside of this fragment.
  bool HasPropagatedDescendants() const { return has_propagated_descendants_; }

  // True if there is any hanging white-space or similar.
  bool HasHanging() const { return has_hanging_; }

  const FontHeight& Metrics() const { return metrics_; }

  // The base direction of this line. Also known as the paragraph direction.
  // This may be different from the direction of the container box when
  // first-line style is used, or when 'unicode-bidi: plaintext' is used.
  TextDirection BaseDirection() const {
    return static_cast<TextDirection>(base_direction_);
  }

  // Compute the baseline metrics for this linebox.
  FontHeight BaselineMetrics() const;

  // Whether the content soft-wraps to the next line.
  bool HasSoftWrapToNextLine() const;

  // Returns the |LayoutObject| of the container. |GetLayoutObject()| returns
  // |nullptr| because line boxes do not have corresponding |LayoutObject|.
  const LayoutObject* ContainerLayoutObject() const {
    return layout_object_.Get();
  }

 private:
  FontHeight metrics_;
};

template <>
struct DowncastTraits<PhysicalLineBoxFragment> {
  static bool AllowFrom(const PhysicalFragment& fragment) {
    return fragment.Type() == PhysicalFragment::kFragmentLineBox;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_PHYSICAL_LINE_BOX_FRAGMENT_H_
