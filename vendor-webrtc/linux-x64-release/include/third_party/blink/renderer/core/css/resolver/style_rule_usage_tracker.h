// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RULE_USAGE_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RULE_USAGE_TRACKER_H_

#include "third_party/blink/renderer/core/css/css_style_rule.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

class StyleRule;

class StyleRuleUsageTracker : public GarbageCollected<StyleRuleUsageTracker> {
 public:
  using RuleListByStyleSheet =
      HeapHashMap<Member<const CSSStyleSheet>,
                  Member<GCedHeapVector<Member<const StyleRule>>>>;

  void Track(const CSSStyleSheet*, const StyleRule*);
  RuleListByStyleSheet TakeDelta();

  void Trace(Visitor*) const;

 private:
  bool InsertToUsedRulesMap(const CSSStyleSheet*, const StyleRule*);

  HeapHashMap<Member<const CSSStyleSheet>,
              Member<GCedHeapHashSet<Member<const StyleRule>>>>
      used_rules_;
  RuleListByStyleSheet used_rules_delta_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_RULE_USAGE_TRACKER_H_
