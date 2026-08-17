// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_ROW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_ROW_H_

#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class LayoutTableCell;
class LayoutTableSection;
class LayoutTable;

// Every child of LayoutTableRow must be LayoutTableCell.
class CORE_EXPORT LayoutTableRow : public LayoutBlock {
 public:
  explicit LayoutTableRow(Element*);

  static LayoutTableRow* CreateAnonymousWithParent(const LayoutObject&);

  LayoutTableCell* FirstCell() const;
  LayoutTableCell* LastCell() const;
  LayoutTableRow* NextRow() const;
  LayoutTableRow* PreviousRow() const;
  LayoutTableSection* Section() const;
  LayoutTable* Table() const;

  // LayoutBlock methods start.

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTableRow";
  }

  void AddChild(LayoutObject* child,
                LayoutObject* before_child = nullptr) override;

  void RemoveChild(LayoutObject*) override;

  void WillBeRemovedFromTree() override;

  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) override;

  LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject* parent) const override;

  LayoutBlock* StickyContainer() const override;

  // Whether a row has opaque background depends on many factors, e.g. border
  // spacing, border collapsing, missing cells, etc.
  // For simplicity, just conservatively assume all table rows are not opaque.
  bool ForegroundIsKnownToBeOpaqueInRect(const PhysicalRect&,
                                         unsigned) const override {
    NOT_DESTROYED();
    return false;
  }

  bool BackgroundIsKnownToBeOpaqueInRect(const PhysicalRect&) const override {
    NOT_DESTROYED();
    return false;
  }

  bool RespectsCSSOverflow() const override {
    NOT_DESTROYED();
    return false;
  }

  bool VisualRectRespectsVisibility() const final {
    NOT_DESTROYED();
    return false;
  }

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  // LayoutBlock methods end.

  unsigned RowIndex() const;

 protected:
  bool IsTableRow() const final {
    NOT_DESTROYED();
    return true;
  }

  // Table section paints background specially.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutTableRow> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsTableRow();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_ROW_H_
