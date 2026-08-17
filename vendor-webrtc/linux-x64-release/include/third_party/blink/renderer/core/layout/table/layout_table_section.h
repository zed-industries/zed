// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_SECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_SECTION_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class LayoutTable;
class LayoutTableRow;

// NOTE:
// Every child of LayoutTableSection must be LayoutTableRow.
class CORE_EXPORT LayoutTableSection : public LayoutBlock {
 public:
  explicit LayoutTableSection(Element*);

  static LayoutTableSection* CreateAnonymousWithParent(const LayoutObject&);

  bool IsEmpty() const;

  LayoutTableRow* FirstRow() const;
  LayoutTableRow* LastRow() const;
  LayoutTable* Table() const;

  // LayoutBlock methods start.

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTableSection";
  }

  void AddChild(LayoutObject* child,
                LayoutObject* before_child = nullptr) override;

  void RemoveChild(LayoutObject*) override;

  void WillBeRemovedFromTree() override;

  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) override;

  LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject* parent) const override;

  bool RespectsCSSOverflow() const override {
    NOT_DESTROYED();
    return false;
  }

  // Whether a section has opaque background depends on many factors, e.g.
  // border spacing, border collapsing, missing cells, etc. For simplicity,
  // just conservatively assume all table sections are not opaque.
  bool ForegroundIsKnownToBeOpaqueInRect(const PhysicalRect&,
                                         unsigned) const override {
    NOT_DESTROYED();
    return false;
  }

  bool BackgroundIsKnownToBeOpaqueInRect(const PhysicalRect&) const override {
    NOT_DESTROYED();
    return false;
  }

  bool VisualRectRespectsVisibility() const final {
    NOT_DESTROYED();
    return false;
  }

  // LayoutBlock methods end.

  unsigned NumRows() const;

 protected:
  bool IsTableSection() const final {
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
struct DowncastTraits<LayoutTableSection> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsTableSection();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_SECTION_H_
