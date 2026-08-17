// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TABLE_PAINTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TABLE_PAINTERS_H_

#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/core/layout/table/table_fragment_data.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BoxDecorationData;
class LayoutBox;
class PhysicalBoxFragment;
struct PaintInfo;
struct PhysicalRect;

class TablePainter {
  STACK_ALLOCATED();

 public:
  explicit TablePainter(const PhysicalBoxFragment& table_wrapper_fragment)
      : fragment_(table_wrapper_fragment) {
    DCHECK(fragment_.IsTable());
  }

  bool WillCheckColumnBackgrounds();

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalRect&,
                                    const BoxDecorationData&);

  void PaintCollapsedBorders(const PaintInfo&,
                             const PhysicalOffset&,
                             const gfx::Rect& visual_rect);

 private:
  const PhysicalBoxFragment& fragment_;
};

class TableSectionPainter {
  STACK_ALLOCATED();

 public:
  explicit TableSectionPainter(
      const PhysicalBoxFragment& table_section_fragment)
      : fragment_(table_section_fragment) {
    DCHECK(fragment_.IsTableSection());
  }

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalRect&,
                                    const BoxDecorationData&);

  void PaintColumnsBackground(const PaintInfo&,
                              const PhysicalOffset& section_paint_offset,
                              const PhysicalRect& columns_paint_rect,
                              const TableColumnGeometries&);

 private:
  const PhysicalBoxFragment& fragment_;
};

class TableRowPainter {
  STACK_ALLOCATED();

 public:
  explicit TableRowPainter(const PhysicalBoxFragment& table_row_fragment)
      : fragment_(table_row_fragment) {
    DCHECK(fragment_.IsTableRow());
  }

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalRect&,
                                    const BoxDecorationData&);

  void PaintTablePartBackgroundIntoCells(
      const PaintInfo& paint_info,
      const LayoutBox& table_part,
      const PhysicalRect& table_part_paint_rect,
      const PhysicalOffset& row_paint_offset);

  void PaintColumnsBackground(const PaintInfo&,
                              const PhysicalOffset& row_paint_offset,
                              const PhysicalRect& columns_paint_rect,
                              const TableColumnGeometries&);

 private:
  const PhysicalBoxFragment& fragment_;
};

class TableCellPainter {
  STACK_ALLOCATED();

 public:
  explicit TableCellPainter(const PhysicalBoxFragment& table_cell_fragment)
      : fragment_(table_cell_fragment) {}

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalRect&,
                                    const BoxDecorationData&);

  void PaintBackgroundForTablePart(
      const PaintInfo& paint_info,
      const LayoutBox& table_part,
      const PhysicalRect& table_part_paint_rect,
      const PhysicalOffset& table_cell_paint_offset);

 private:
  const PhysicalBoxFragment& fragment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TABLE_PAINTERS_H_
