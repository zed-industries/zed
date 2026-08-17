// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_FRAGMENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_FRAGMENT_DATA_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/layout/layout_input_node.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// COLGROUP/COL geometry information. Used for painting column backgrounds.
// Only present if column has a background.
struct TableColumnGeometry {
  DISALLOW_NEW();

 public:
  TableColumnGeometry(wtf_size_t start_column,
                      wtf_size_t span,
                      LayoutUnit inline_offset,
                      LayoutUnit inline_size,
                      LayoutInputNode node)
      : start_column(start_column),
        span(span),
        inline_offset(inline_offset),
        inline_size(inline_size),
        node(node) {}
  void Trace(Visitor* visitor) const { visitor->Trace(node); }
  wtf_size_t start_column;
  wtf_size_t span;
  LayoutUnit inline_offset;
  LayoutUnit inline_size;
  LayoutInputNode node;
};

using TableColumnGeometries = HeapVector<TableColumnGeometry>;
using GCedTableColumnGeometries = GCedHeapVector<TableColumnGeometry>;

// Column locations are used for collapsed-border painting.
struct CollapsedTableBordersGeometry {
  USING_FAST_MALLOC(CollapsedTableBordersGeometry);

 public:
  Vector<LayoutUnit> columns;

#if DCHECK_IS_ON()
  void CheckSameForSimplifiedLayout(
      const CollapsedTableBordersGeometry& other) const {
    DCHECK(columns == other.columns);
  }
#endif
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableColumnGeometry)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_FRAGMENT_DATA_H_
