// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_COLUMN_LOCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_COLUMN_LOCATION_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

struct TableColumnLocation {
  DISALLOW_NEW();
  LayoutUnit offset;
  LayoutUnit size;
  bool is_collapsed;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::TableColumnLocation)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_TABLE_COLUMN_LOCATION_H_
