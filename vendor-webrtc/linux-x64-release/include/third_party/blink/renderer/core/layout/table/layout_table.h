// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_H_

#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"
#include "third_party/blink/renderer/core/layout/table/table_layout_algorithm_types.h"

namespace blink {

class LayoutTableSection;
class LayoutTableCell;
class TableBorders;

// LayoutTable is the LayoutObject associated with display: table or
// inline-table.
//
// LayoutTable is the coordinator for determining the overall table structure.
// The reason is that LayoutTableSection children have a local view over what
// their structure is but don't account for other LayoutTableSection. Thus
// LayoutTable helps keep consistency across LayoutTableSection.
//
// LayoutTable expects only 3 types of children:
// - zero or more LayoutTableColumn
// - zero or more LayoutTableCaption
// - zero or more LayoutTableSection
// This is aligned with what HTML5 expects:
// https://html.spec.whatwg.org/C/#the-table-element
// with one difference: we allow more than one caption as we follow what
// CSS expects (https://bugs.webkit.org/show_bug.cgi?id=69773).
// Those expectations are enforced by LayoutTable::AddChild, that wraps
// unknown children into an anonymous LayoutTableSection. This is what the
// "generate missing child wrapper" step in CSS mandates in
// http://www.w3.org/TR/CSS21/tables.html#anonymous-boxes.
//
// LayoutTable assumes a pretty strict structure that is mandated by CSS:
// (note that this structure in HTML is enforced by the HTML5 Parser).
//
//                 LayoutTable
//                 |          |
//    LayoutTableSection    LayoutTableCaption
//                 |
//        LayoutTableRow
//                 |
//       LayoutTableCell
//
// This means that we have to generate some anonymous table wrappers in order to
// satisfy the structure. See again
// http://www.w3.org/TR/CSS21/tables.html#anonymous-boxes.
// The anonymous table wrappers are inserted in LayoutTable::AddChild,
// LayoutTableSection::AddChild, LayoutTableRow::AddChild and
// LayoutObject::AddChild.
//
// Note that this yields to interesting issues in the insertion code. The DOM
// code is unaware of the anonymous LayoutObjects and thus can insert
// LayoutObjects into a different part of the layout tree. An example is:
//
// <!DOCTYPE html>
// <style>
// tablerow { display: table-row; }
// tablecell { display: table-cell; border: 5px solid purple; }
// </style>
// <tablerow id="firstRow">
//     <tablecell>Short first row.</tablecell>
// </tablerow>
// <tablecell id="cell">Long second row, shows the table structure.</tablecell>
//
// The page generates a single anonymous table (LayoutTable) and table row
// group (LayoutTableSection) to wrap the <tablerow> (#firstRow) and an
// anonymous table row (LayoutTableRow) for the second <tablecell>. It is
// possible for JavaScript to insert a new element between these 2 <tablecell>
// (using Node.insertBefore), requiring us to split the anonymous table (or the
// anonymous table row group) in 2. Also note that even though the second
// <tablecell> and <tablerow> are siblings in the DOM tree, they are not in the
// layout tree.
//
// Invalidation: LayoutTable differences from block invalidation:
//
// Cached collapsed borders:
// Table caches collapsed borders as TableBorders.
// TableBorders are stored on Table fragment for painting.
// If TableBorders change, table fragment must be regenerated.
// Any table part that contributes to collapsed borders must invalidate
// cached borders on border change by calling GridBordersChanged.
//
// Cached column constraints:
// Column constraints are used in layout. They must be regenerated
// whenever table geometry changes.
// The validation state is a IsTableColumnsConstraintsDirty flag
// on LayoutObject. They are invalidated inside
// LayoutObject::SetNeeds*Layout.
class CORE_EXPORT LayoutTable : public LayoutBlock {
 public:
  explicit LayoutTable(Element*);
  ~LayoutTable() override;

  void Trace(Visitor*) const override;

  static bool ShouldCreateInlineAnonymous(const LayoutObject& parent);
  static LayoutTable* CreateAnonymousWithParent(const LayoutObject&);

  bool IsFirstCell(const LayoutTableCell&) const;

  // All the section methods will return non-empty sections.
  LayoutTableSection* FirstSection() const;
  LayoutTableSection* LastSection() const;
  LayoutTableSection* NextSection(const LayoutTableSection*) const;
  LayoutTableSection* PreviousSection(const LayoutTableSection*) const;

  wtf_size_t ColumnCount() const;

  const TableBorders* GetCachedTableBorders() const {
    NOT_DESTROYED();
    return cached_table_borders_.Get();
  }

  void SetCachedTableBorders(const TableBorders*);

  const TableTypes::Columns* GetCachedTableColumnConstraints();

  void SetCachedTableColumnConstraints(
      scoped_refptr<const TableTypes::Columns>);

  // Any borders in table grid have changed.
  void GridBordersChanged();

  // Table descendants have been added/removed, and number of rows/columns
  // might have changed.
  void TableGridStructureChanged();

  // Table paints column backgrounds.
  // Returns true if table, or columns' style.HasBackground().
  bool HasBackgroundForPaint() const;

  // LayoutBlock methods start.

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTable";
  }

  void AddChild(LayoutObject* child,
                LayoutObject* before_child = nullptr) override;

  void RemoveChild(LayoutObject*) override;

  void StyleDidChange(StyleDifference diff,
                      const ComputedStyle* old_style) override;

  LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject* parent) const override;

  LayoutUnit BorderTop() const override;
  LayoutUnit BorderBottom() const override;
  LayoutUnit BorderLeft() const override;
  LayoutUnit BorderRight() const override;

  // The collapsing border model disallows paddings on table.
  // See http://www.w3.org/TR/CSS2/tables.html#collapsing-borders.
  LayoutUnit PaddingTop() const override;
  LayoutUnit PaddingBottom() const override;
  LayoutUnit PaddingLeft() const override;
  LayoutUnit PaddingRight() const override;

  // TODO(1151101)
  // ClientLeft/Top are incorrect for tables, but cannot be fixed
  // by subclassing ClientLeft/Top.

  PhysicalRect OverflowClipRect(const PhysicalOffset&,
                                OverlayScrollbarClipBehavior) const override;

  bool VisualRectRespectsVisibility() const override {
    NOT_DESTROYED();
    return false;
  }

  // Whether a table has opaque foreground depends on many factors, e.g. border
  // spacing, missing cells, etc. For simplicity, just conservatively assume
  // foreground of all tables are not opaque.
  bool ForegroundIsKnownToBeOpaqueInRect(
      const PhysicalRect& local_rect,
      unsigned max_depth_to_test) const override {
    NOT_DESTROYED();
    return false;
  }

  // LayoutBlock methods end.

  bool HasCollapsedBorders() const {
    NOT_DESTROYED();
    return StyleRef().BorderCollapse() == EBorderCollapse::kCollapse;
  }

  unsigned AbsoluteColumnToEffectiveColumn(
      unsigned absolute_column_index) const;

  unsigned EffectiveColumnCount() const;

 protected:
  bool IsTable() const final {
    NOT_DESTROYED();
    return true;
  }

  // Table paints background specially.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }

 private:
  void InvalidateCachedTableBorders();

  // Table borders are cached because computing collapsed borders is expensive.
  Member<const TableBorders> cached_table_borders_;

  // Table columns do not depend on any outside data (e.g. ConstraintSpace).
  // They are cached because computing them is expensive.
  scoped_refptr<const TableTypes::Columns> cached_table_columns_;
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutTable> {
  static bool AllowFrom(const LayoutObject& object) { return object.IsTable(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_H_
