// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CELL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CELL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/table_constants.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

class LayoutTable;
class LayoutTableRow;
class LayoutTableSection;

class CORE_EXPORT LayoutTableCell : public LayoutBlockFlow {
 public:
  explicit LayoutTableCell(Element*);

  static LayoutTableCell* CreateAnonymousWithParent(const LayoutObject&);

  // NOTE: Rowspan might overflow section boundaries.
  unsigned ComputedRowSpan() const {
    NOT_DESTROYED();
    if (!has_rowspan_)
      return 1;
    unsigned rowspan = ParseRowSpanFromDOM();
    if (rowspan == 0)  // rowspan == 0 means all rows.
      rowspan = kMaxRowSpan;
    return rowspan;
  }

  const BoxStrut& IntrinsicLogicalWidthsBorderSizes() const {
    NOT_DESTROYED();
    return intrinsical_logical_widths_border_sizes_;
  }

  void SetIntrinsicLogicalWidthsBorderSizes(const BoxStrut& border_sizes) {
    NOT_DESTROYED();
    intrinsical_logical_widths_border_sizes_ = border_sizes;
  }

  // This method is called after a new *measure* layout-result is set.
  // Tables are special in that the table accesses the table-cells directly
  // for computing the size of the table-grid (bypassing the section/row).
  // Due to this when we set a new measure layout-result, we also invalidate
  // the layout-result cache of the associated section/row (as the cell
  // fragment would be in the incorrect state).
  void InvalidateLayoutResultCacheAfterMeasure() const;

  LayoutUnit BorderTop() const override;
  LayoutUnit BorderBottom() const override;
  LayoutUnit BorderLeft() const override;
  LayoutUnit BorderRight() const override;

  LayoutTableCell* NextCell() const;
  LayoutTableCell* PreviousCell() const;
  LayoutTableRow* Row() const;
  LayoutTableSection* Section() const;
  LayoutTable* Table() const;

  // LayoutBlockFlow methods start.

  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) final;

  void WillBeRemovedFromTree() override;

  const char* GetName() const final {
    NOT_DESTROYED();
    return "LayoutTableCell";
  }

  bool CreatesNewFormattingContext() const final {
    NOT_DESTROYED();
    return true;
  }

  bool BackgroundIsKnownToBeOpaqueInRect(const PhysicalRect&) const override;

  LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject* parent) const override;

  LayoutBlock* StickyContainer() const override;

  void InvalidatePaint(const PaintInvalidatorContext&) const override;

  // LayoutBlockFlow methods end.

  void ColSpanOrRowSpanChanged();

  unsigned RowIndex() const;

  unsigned ResolvedRowSpan() const;

  unsigned AbsoluteColumnIndex() const;

  // Guaranteed to be between kMinColSpan and kMaxColSpan.
  unsigned ColSpan() const;

 protected:
  bool IsTableCell() const final {
    NOT_DESTROYED();
    return true;
  }

  // Table cell applies a special clip to its background.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }

 private:
  void UpdateColAndRowSpanFlags();

  unsigned ParseRowSpanFromDOM() const;

  unsigned ParseColSpanFromDOM() const;
  // Use ComputedRowSpan instead
  unsigned ParsedRowSpan() const {
    NOT_DESTROYED();
    if (!has_rowspan_)
      return 1;
    return ParseRowSpanFromDOM();
  }

  // Cached cell border. Used to invalidate calculation of
  // intrinsic logical width.
  BoxStrut intrinsical_logical_widths_border_sizes_;

  unsigned has_col_span_ : 1;
  unsigned has_rowspan_ : 1;
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutTableCell> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsTableCell();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CELL_H_
