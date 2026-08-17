// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_BREAK_TOKEN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_BREAK_TOKEN_DATA_H_

#include "third_party/blink/renderer/core/layout/block_break_token_data.h"

namespace blink {

struct FieldsetBreakTokenData final : BlockBreakTokenData {
  explicit FieldsetBreakTokenData(const BlockBreakTokenData* other_data)
      : BlockBreakTokenData(kFieldsetBreakTokenData, other_data) {}

  LayoutUnit legend_block_size_contribution;
};

template <>
struct DowncastTraits<FieldsetBreakTokenData> {
  static bool AllowFrom(const BlockBreakTokenData& token_data) {
    return token_data.IsFieldsetType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_BREAK_TOKEN_DATA_H_
