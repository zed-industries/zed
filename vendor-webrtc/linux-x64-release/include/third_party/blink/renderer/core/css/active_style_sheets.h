// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ACTIVE_STYLE_SHEETS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ACTIVE_STYLE_SHEETS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_value_change.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class CSSStyleSheet;
class RuleSet;
class RuleSetDiff;

using ActiveStyleSheet = std::pair<Member<CSSStyleSheet>, Member<RuleSet>>;
using ActiveStyleSheetVector = HeapVector<ActiveStyleSheet>;

enum ActiveSheetsChange {
  kNoActiveSheetsChanged,  // Nothing changed.
  kActiveSheetsChanged,    // Sheets were added and/or inserted.
  kActiveSheetsAppended    // Only additions, and all appended.
};

CORE_EXPORT ActiveSheetsChange
CompareActiveStyleSheets(const ActiveStyleSheetVector& old_style_sheets,
                         const ActiveStyleSheetVector& new_style_sheets,
                         const HeapVector<Member<RuleSetDiff>>& diffs,
                         HeapHashSet<Member<RuleSet>>& changed_rule_sets);

bool AffectedByMediaValueChange(const ActiveStyleSheetVector& active_sheets,
                                MediaValueChange change);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ACTIVE_STYLE_SHEETS_H_
