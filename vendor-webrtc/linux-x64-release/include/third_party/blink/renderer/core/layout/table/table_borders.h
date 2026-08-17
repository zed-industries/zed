// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BORDERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BORDERS_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BlockNode;
class ComputedStyle;
struct BoxStrut;

// When table has collapsed borders, computing borders for table parts is
// complex, and costly.
// TableBorders precomputes collapsed borders. It exposes the API for
// border access. If borders are not collapsed, the API returns regular
// borders.
//
// TableBorders methods often take rowspan/colspan arguments.
// Rowspan must never be taller than the section.
// Colspan must never be wider than the table.
// To enforce this, TableBorders keeps track of section dimensions,
// and table's last column.
//
// Collapsed borders are stored as edges.
// Edges are stored in a 1D array. The array does not grow if borders are
// not set.
// Each edge represents a cell border.
// Mapping between edges and cells is best understood like this:
// - each cell stores only two edges, left edge, and a top edge.
// - cell's right edge is the left edge of the next cell.
// - cell's bottom edge is the top edge of the cell in the next row.
//
// To store all last row/col edges, an extra imaginary cell is used.
//
// A grid with R rows and C columns has |2 * (R+1) * (C+1)| edges.
// Example; R=1, C=3, 2*(1+1)*(3+1) = 16 edges.
// Edges 7, 9, 11, 13, 14, 15 are unused.
//
//     1    3   5   7
//   ------------------    <= edges for 3 cols X 1 row
//   |0  |2  |4   |6
//   |   |   |    |
//   ------------------
//   | 8 | 10| 12 | 14
//   |   |   |    |
//   |9  |11 |13  |15

class TableBorders : public GarbageCollected<TableBorders> {
 public:
  static const TableBorders* ComputeTableBorders(const BlockNode&);

  TableBorders(const BoxStrut& table_border, const bool is_collapsed);

  void Trace(Visitor* visitor) const { visitor->Trace(edges_); }

#if DCHECK_IS_ON()
  String DumpEdges();
  void ShowEdges();
  bool operator==(const TableBorders& other) const;
#endif

  enum class EdgeSource { kNone, kCell, kRow, kSection, kColumn, kTable };

  // Specifies which side of CSS style defines the edge.
  // |kDoNotFill| means edge is empty, but should not be filled.
  enum class EdgeSide { kTop, kRight, kBottom, kLeft, kDoNotFill };

  // Edge is defined by a style, and box side. Box side specifies which
  // style border defines the edge.
  struct Edge {
    DISALLOW_NEW();
    Member<const ComputedStyle> style;
    EdgeSide edge_side;
    // Box order is used to compute edge painting precedence.
    // Lower box order has precedence.
    // The order value is defined as "box visited index" while computing
    // collapsed edges.
    wtf_size_t box_order;

    void Trace(Visitor* visitor) const { visitor->Trace(style); }
  };

  static LayoutUnit BorderWidth(const ComputedStyle* style,
                                EdgeSide edge_side) {
    if (!style)
      return LayoutUnit();
    switch (edge_side) {
      case EdgeSide::kLeft:
        return LayoutUnit(style->BorderLeftWidth());
      case EdgeSide::kRight:
        return LayoutUnit(style->BorderRightWidth());
      case EdgeSide::kTop:
        return LayoutUnit(style->BorderTopWidth());
      case EdgeSide::kBottom:
        return LayoutUnit(style->BorderBottomWidth());
      case EdgeSide::kDoNotFill:
        return LayoutUnit();
    }
  }

  static EBorderStyle BorderStyle(const ComputedStyle* style,
                                  EdgeSide edge_side) {
    if (!style)
      return EBorderStyle::kNone;
    EBorderStyle border_style;
    switch (edge_side) {
      case EdgeSide::kLeft:
        border_style = style->BorderLeftStyle();
        break;
      case EdgeSide::kRight:
        border_style = style->BorderRightStyle();
        break;
      case EdgeSide::kTop:
        border_style = style->BorderTopStyle();
        break;
      case EdgeSide::kBottom:
        border_style = style->BorderBottomStyle();
        break;
      case EdgeSide::kDoNotFill:
        border_style = EBorderStyle::kNone;
        break;
    }
    return ComputedStyle::CollapsedBorderStyle(border_style);
  }

  static Color BorderColor(const ComputedStyle* style, EdgeSide edge_side);

  static bool HasBorder(const ComputedStyle* style) {
    if (!style)
      return false;
    return style->BorderLeftStyle() != EBorderStyle::kNone ||
           style->BorderRightStyle() != EBorderStyle::kNone ||
           style->BorderTopStyle() != EBorderStyle::kNone ||
           style->BorderBottomStyle() != EBorderStyle::kNone;
  }

  LayoutUnit BorderWidth(wtf_size_t edge_index) const {
    return BorderWidth(edges_[edge_index].style.Get(),
                       edges_[edge_index].edge_side);
  }

  EBorderStyle BorderStyle(wtf_size_t edge_index) const {
    return BorderStyle(edges_[edge_index].style.Get(),
                       edges_[edge_index].edge_side);
  }

  Color BorderColor(wtf_size_t edge_index) const {
    return BorderColor(edges_[edge_index].style.Get(),
                       edges_[edge_index].edge_side);
  }

  wtf_size_t BoxOrder(wtf_size_t edge_index) const {
    return edges_[edge_index].box_order;
  }

  using Edges = HeapVector<Edge>;

  struct Section {
    wtf_size_t start_row;
    wtf_size_t row_count;
  };

  bool IsEmpty() const { return edges_.empty(); }

  bool IsCollapsed() const { return is_collapsed_; }

  wtf_size_t EdgesPerRow() const { return edges_per_row_; }

  const BoxStrut& TableBorder() const { return table_border_; }

  BoxStrut CellBorder(const BlockNode& cell,
                      wtf_size_t row,
                      wtf_size_t column,
                      wtf_size_t section,
                      WritingDirectionMode table_writing_direction) const;

  BoxStrut CellPaddingForMeasure(
      const ComputedStyle& cell_style,
      WritingDirectionMode table_writing_direction) const;

  void UpdateTableBorder(wtf_size_t table_row_count,
                         wtf_size_t table_column_count);

  // |section_index| is only used to clamp rowspan. Only needed for
  // cells with rowspan > 1.
  void MergeBorders(wtf_size_t start_row,
                    wtf_size_t start_column,
                    wtf_size_t rowspan,
                    wtf_size_t colspan,
                    const ComputedStyle& source_style,
                    EdgeSource source,
                    wtf_size_t box_order,
                    WritingDirectionMode table_writing_direction,
                    wtf_size_t section_index = kNotFound);

  void AddSection(wtf_size_t start_row, wtf_size_t row_count) {
    sections_.push_back(Section{start_row, row_count});
  }

  Section GetSection(wtf_size_t section_index) {
    return sections_[section_index];
  }

  void SetLastColumnIndex(wtf_size_t last_column_index) {
    last_column_index_ = last_column_index;
  }

  Edges::const_iterator begin() const { return edges_.begin(); }

  Edges::const_iterator end() const { return edges_.end(); }

  wtf_size_t EdgeCount() const { return edges_.size(); }

  bool CanPaint(wtf_size_t edge_index) const {
    if (!HasEdgeAtIndex(edge_index))
      return false;
    EBorderStyle border_style = BorderStyle(edge_index);
    if (border_style == EBorderStyle::kNone ||
        border_style == EBorderStyle::kHidden)
      return false;
    if (BorderWidth(edge_index) == 0)
      return false;
    return true;
  }

  bool HasEdgeAtIndex(wtf_size_t edge_index) const {
    return edge_index < edges_.size() && edges_[edge_index].style;
  }

  // Is there and edge at edges[edge_index + index_offset]?
  bool CanPaint(wtf_size_t edge_index, int index_offset) const {
    return (index_offset >= 0 ||
            (index_offset < 0 &&
             edge_index >= static_cast<wtf_size_t>(abs(index_offset)))) &&
           edge_index + index_offset < edges_.size() &&
           edges_[edge_index + index_offset].style;
  }

 private:
  wtf_size_t ClampColspan(wtf_size_t column, wtf_size_t colspan) const {
    DCHECK_GE(last_column_index_, column);
    return std::min(colspan, last_column_index_ - column);
  }

  // Clamps rowspan by section size.
  wtf_size_t ClampRowspan(wtf_size_t section_index,
                          wtf_size_t table_row_index,
                          wtf_size_t rowspan) const {
    if (rowspan <= 1)
      return rowspan;
    DCHECK_LT(section_index, sections_.size());
    return std::min(rowspan,
                    sections_[section_index].row_count -
                        (table_row_index - sections_[section_index].start_row));
  }

  BoxStrut GetCellBorders(wtf_size_t row,
                          wtf_size_t column,
                          wtf_size_t rowspan,
                          wtf_size_t colspan) const;

  void EnsureCellColumnFits(wtf_size_t cell_column);

  void EnsureCellRowFits(wtf_size_t cell_row);

  void MergeRowAxisBorder(wtf_size_t start_row,
                          wtf_size_t start_column,
                          wtf_size_t colspan,
                          const ComputedStyle& source_style,
                          wtf_size_t box_order,
                          EdgeSide side);

  void MergeColumnAxisBorder(wtf_size_t start_row,
                             wtf_size_t start_column,
                             wtf_size_t rowspan,
                             const ComputedStyle& source_style,
                             wtf_size_t box_order,
                             EdgeSide side);

  void MarkInnerBordersAsDoNotFill(wtf_size_t start_row,
                                   wtf_size_t start_column,
                                   wtf_size_t rowspan,
                                   wtf_size_t colspan);

  Edges edges_;
  Vector<Section> sections_;
  wtf_size_t edges_per_row_ = 0;
  BoxStrut table_border_;
  // Cells cannot extrude beyond table grid size.
  // Rowspan and colspan sizes must be clamped to enforce this.
  wtf_size_t last_column_index_ = UINT_MAX;
  bool is_collapsed_;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableBorders::Edge)
WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableBorders::Section)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_BORDERS_H_
