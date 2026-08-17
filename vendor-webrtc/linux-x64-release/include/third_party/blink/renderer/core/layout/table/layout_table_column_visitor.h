// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_VISITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_VISITOR_H_

#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/table/layout_table_column.h"

namespace blink {

// Rules for computed spans of COLGROUP/COL elements are complicated.
// 1) Spans cannot protrude outside of table boundary.
// 2) Colgroup span is
//    a) sum of children COL spans if it has children.
//    b) span attribute if it has no children.
//
// This function implements a Visitor pattern for LayoutTableColumn
// traversal, and computes correct spans.
//
// class Visitor {
//   void VisitCol(const BlockNode& column,
//                 wtf_size_t start_column_index,
//                 wtf_size_t span);
//   void EnterColgroup(const BlockNode&  colgroup,
//                      wtf_size_t start_column_index);
//   void LeaveColgroup(const BlockNode& colgroup,
//                      wtf_size_t start_column_index,
//                      wtf_size_t span,
//                      bool has_children);
// }
template <typename Visitor>
void VisitLayoutTableColumn(const HeapVector<BlockNode>& columns,
                            wtf_size_t table_column_count,
                            Visitor* visitor) {
  wtf_size_t current_column_index = 0;

  auto VisitCol = [&](const BlockNode& col) {
    wtf_size_t span = col.TableColumnSpan();
    span = std::min(span, table_column_count - current_column_index);
    visitor->VisitCol(col, current_column_index, span);
    current_column_index += span;
    return span;
  };

  for (const BlockNode& table_column : columns) {
    // Col spans can cause columns to extend beyond table's edge.
    // These columns are ignored.
    if (current_column_index >= table_column_count)
      break;
    if (table_column.IsTableCol()) {  // COL element.
      VisitCol(table_column);
      continue;
    }
    DCHECK(table_column.IsTableColgroup());
    // Visit COLGROUP element.
    visitor->EnterColgroup(table_column, current_column_index);
    BlockNode col_child = To<BlockNode>(table_column.FirstChild());
    wtf_size_t colgroup_start_index = current_column_index;
    wtf_size_t colgroup_span = 0;
    bool has_children = bool(col_child);
    if (col_child) {
      while (col_child) {
        colgroup_span += VisitCol(col_child);
        if (current_column_index >= table_column_count)
          break;
        col_child = To<BlockNode>(col_child.NextSibling());
      }
    } else {
      // If COLGROUP has no children, its span is defined by the COLGROUP.
      colgroup_span = std::min(table_column.TableColumnSpan(),
                               table_column_count - current_column_index);
      current_column_index += colgroup_span;
    }
    visitor->LeaveColgroup(table_column, colgroup_start_index, colgroup_span,
                           has_children);
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_COLUMN_VISITOR_H_
