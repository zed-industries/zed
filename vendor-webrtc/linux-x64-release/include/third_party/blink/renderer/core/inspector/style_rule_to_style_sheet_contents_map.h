// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_STYLE_RULE_TO_STYLE_SHEET_CONTENTS_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_STYLE_RULE_TO_STYLE_SHEET_CONTENTS_MAP_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class StyleRule;
class StyleSheetContents;

// Implements a back-mapping from StyleRules to their containing
// StyleSheetContentses.
class StyleRuleToStyleSheetContentsMap
    : public GarbageCollected<StyleRuleToStyleSheetContentsMap> {
 public:
  void Add(const StyleRule* rule, const StyleSheetContents* contents);
  const StyleSheetContents* Lookup(const StyleRule* rule) const;

  void Trace(Visitor*) const;

 private:
  using RuleToSheet =
      HeapHashMap<Member<const StyleRule>, Member<const StyleSheetContents>>;
  RuleToSheet rule_to_sheet_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_STYLE_RULE_TO_STYLE_SHEET_CONTENTS_MAP_H_
