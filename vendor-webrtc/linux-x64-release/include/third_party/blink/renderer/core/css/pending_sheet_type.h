// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PENDING_SHEET_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PENDING_SHEET_TYPE_H_

#include <utility>

#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"

namespace blink {

class Element;

// TODO(xiaochengh): This enum is almost identical to RenderBlockingBehavior.
// Try to merge them.
enum class PendingSheetType {
  // Not a pending sheet, hasn't started or already finished
  kNone,
  // Pending but does not block anything
  kNonBlocking,
  // Dynamically inserted render-blocking but not script-blocking sheet
  kDynamicRenderBlocking,
  // Parser-inserted sheet that by default blocks scripts. Also blocks rendering
  // if in head, or blocks parser if in body.
  kBlocking
};

std::pair<PendingSheetType, RenderBlockingBehavior>
ComputePendingSheetTypeAndRenderBlockingBehavior(Element& sheet_owner,
                                                 bool is_critical_sheet,
                                                 bool is_created_by_parser);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PENDING_SHEET_TYPE_H_
