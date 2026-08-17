// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"

namespace blink {

class LayoutTable;

// Represents <col> and <colgroup> elements.
class CORE_EXPORT LayoutTableColumn : public LayoutBox {
 public:
  explicit LayoutTableColumn(Element*);
  void Trace(Visitor*) const override;

  LayoutTable* Table() const;

  bool IsColumn() const {
    NOT_DESTROYED();
    return StyleRef().Display() == EDisplay::kTableColumn;
  }

  bool IsColumnGroup() const {
    NOT_DESTROYED();
    return StyleRef().Display() == EDisplay::kTableColumnGroup;
  }

  unsigned Span() const {
    NOT_DESTROYED();
    return span_;
  }

  // Clears needs-layout for child columns too.
  void ClearNeedsLayoutForChildren() const;

  PhysicalSize Size() const override;

  DeprecatedLayoutPoint LocationInternal() const override;

  // LayoutObject methods start.

  const char* GetName() const override {
    NOT_DESTROYED();
    if (IsColumn())
      return "LayoutTableCol";
    else
      return "LayoutTableColGroup";
  }

  bool IsLayoutNGObject() const final {
    NOT_DESTROYED();
    return true;
  }

  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) final;

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) final;

  void InsertedIntoTree() override;

  void WillBeRemovedFromTree() override;

  bool VisualRectRespectsVisibility() const final {
    NOT_DESTROYED();
    return false;
  }

  void SetColumnIndex(wtf_size_t column_idx) {
    NOT_DESTROYED();
    column_idx_ = column_idx;
  }

 protected:
  bool IsLayoutTableCol() const final {
    NOT_DESTROYED();
    return true;
  }

  // Table row doesn't paint background by itself.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }

 private:
  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const override;

  bool CanHaveChildren() const override;

  void UpdateFromElement() override;

  PaintLayerType LayerTypeRequired() const override {
    NOT_DESTROYED();
    return kNoPaintLayer;
  }

  LayoutObjectChildList* VirtualChildren() override {
    NOT_DESTROYED();
    return &children_;
  }

  const LayoutObjectChildList* VirtualChildren() const override {
    NOT_DESTROYED();
    return &children_;
  }

  // LayoutObject methods end

 private:
  unsigned span_ = 1;
  LayoutObjectChildList children_;
  wtf_size_t column_idx_;
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutTableColumn> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutTableCol();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_H_
