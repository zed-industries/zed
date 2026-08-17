// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_TABLE_CELL_WITH_ANONYMOUS_MROW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_TABLE_CELL_WITH_ANONYMOUS_MROW_H_

#include "third_party/blink/renderer/core/layout/table/layout_table_cell.h"

namespace blink {

class LayoutTableCellWithAnonymousMrow : public LayoutTableCell {
 public:
  explicit LayoutTableCellWithAnonymousMrow(Element*);

  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;

  // The multicol flow thread expects to be a direct child of the multicol
  // container, but the AddChild() override above prevents that.
  bool AllowsColumns() const override {
    NOT_DESTROYED();
    return false;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_TABLE_CELL_WITH_ANONYMOUS_MROW_H_
