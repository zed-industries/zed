// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_GRID_FOCUSGROUP_STRUCTURE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_GRID_FOCUSGROUP_STRUCTURE_INFO_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/focusgroup_flags.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class LayoutObject;
class LayoutTable;
class LayoutTableCell;
class LayoutTableRow;

// This interface is used to expose the grid focusgroup navigation functions
// while hiding the type of grid we're in. A grid focusgroup can either be
// 'automatic' or 'manual', but there's no need to expose this additional level
// of complexity to the `FocusgroupController`. The interface is designed so
// that the classes that implements it are stateful, keeping a reference to the
// grid focusgroup root. This will prove useful to reduce the number of times
// helper functions need to do an ancestor chain walk to find the root grid
// focusgroup.
//
// TODO(bebeaudr): Implement ManualGridFocusgroupStructureInfo for 'manual-grid'
// focusgroups.
class GridFocusgroupStructureInfo {
 public:
  enum class NoCellFoundAtIndexBehavior {
    kReturn,
    // Same row, but index - 1.
    kFindPreviousCellInRow,
    // Same row, but index + 1.
    kFindNextCellInRow,
    // Same col index, previous row.
    kFindPreviousCellInColumn,
    // Same col index, next row.
    kFindNextCellInColumn,
  };

  virtual ~GridFocusgroupStructureInfo() = default;

  virtual Element* Root() = 0;
  virtual FocusgroupFlags Flags() = 0;
  virtual unsigned ColumnCount() = 0;

  virtual Element* PreviousCellInRow(const Element* cell) = 0;
  virtual Element* NextCellInRow(const Element* cell) = 0;
  virtual Element* FirstCellInRow(Element* row) = 0;
  virtual Element* LastCellInRow(Element* row) = 0;

  virtual unsigned ColumnIndexForCell(const Element* cell) = 0;

  virtual Element* PreviousCellInColumn(const Element* cell) = 0;
  virtual Element* NextCellInColumn(const Element* cell) = 0;
  virtual Element* FirstCellInColumn(unsigned index) = 0;
  virtual Element* LastCellInColumn(unsigned index) = 0;

  virtual Element* PreviousRow(Element* row) = 0;
  virtual Element* NextRow(Element* row) = 0;
  virtual Element* FirstRow() = 0;
  virtual Element* LastRow() = 0;
  virtual Element* RowForCell(Element* cell) = 0;

  // This function is used by most of the grid focusgroup navigation helper
  // functions. It returns the cell at the column |index| in the |row|. When
  // no cell is found at that |index|, the |behavior| parameter tells the
  // function how the caller wants to deal with this case of missing cell.
  virtual Element* CellAtIndexInRow(unsigned index,
                                    Element* row,
                                    NoCellFoundAtIndexBehavior behavior) = 0;
};

// An automatic grid focusgroup is one that is created by setting
// focusgroup='grid' on an HTML table element or an element that has
// `display: table`.
class CORE_EXPORT AutomaticGridFocusgroupStructureInfo final
    : public GarbageCollected<AutomaticGridFocusgroupStructureInfo>,
      public GridFocusgroupStructureInfo {
 public:
  explicit AutomaticGridFocusgroupStructureInfo(LayoutObject* root);

  void Trace(Visitor*) const;

  const LayoutTable* Table();

  Element* Root() override;
  FocusgroupFlags Flags() override;
  unsigned ColumnCount() override;

  Element* PreviousCellInRow(const Element* cell_element) override;
  Element* NextCellInRow(const Element* cell_element) override;
  Element* FirstCellInRow(Element* row) override;
  Element* LastCellInRow(Element* row) override;

  unsigned ColumnIndexForCell(const Element* cell_element) override;

  Element* PreviousCellInColumn(const Element* cell_element) override;
  Element* NextCellInColumn(const Element* cell_element) override;
  Element* FirstCellInColumn(unsigned index) override;
  Element* LastCellInColumn(unsigned index) override;

  Element* PreviousRow(Element* row_element) override;
  Element* NextRow(Element* row_element) override;
  Element* FirstRow() override;
  Element* LastRow() override;
  Element* RowForCell(Element* cell_element) override;

  Element* CellAtIndexInRow(unsigned index,
                            Element* row_element,
                            NoCellFoundAtIndexBehavior behavior) override;

 private:
  LayoutTableRow* PreviousRow(LayoutTableRow* current_row);
  LayoutTableRow* NextRow(LayoutTableRow* current_row);

  LayoutTableCell* TableCellAtIndexInRowRecursive(
      unsigned index,
      LayoutTableRow* row,
      std::optional<unsigned> expected_rowspan = std::nullopt);

  Member<LayoutObject> table_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_GRID_FOCUSGROUP_STRUCTURE_INFO_H_
