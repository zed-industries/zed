// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_QUIET_MUTATION_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_QUIET_MUTATION_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_style_sheet.h"
#include "third_party/blink/renderer/core/css/style_sheet_contents.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

// Allows "quiet" (non-invalidating) mutations of CSS rules
// for any added CSSStyleSheets. See InspectorGhostRules.
//
// Since multiple CSSStyleSheets can share the same StyleSheetContents,
// any kind of mutation requires a copy-on-write to take place. This is
// exactly what QuietMutationScope:Add does: copies the underlying
// StyleSheetContents if it's shared. When this object goes out of scope,
// the original StyleSheetContents is restored for each affected CSSStyleSheet.
class CORE_EXPORT QuietMutationScope {
  STACK_ALLOCATED();

 public:
  ~QuietMutationScope() {
    for (auto& [sheet, original_contents] : sheets_) {
      sheet->EndQuietMutation(original_contents);
    }
  }

  void Add(CSSStyleSheet& sheet) {
    StyleSheetContents* original_contents = sheet.Contents();
    sheet.BeginQuietMutation();
    if (sheet.Contents() != original_contents) {
      CHECK(!sheets_.Contains(&sheet));
      sheets_.insert(&sheet, original_contents);
    }
  }

 private:
  HeapHashMap<Member<CSSStyleSheet>, Member<StyleSheetContents>> sheets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_QUIET_MUTATION_SCOPE_H_
