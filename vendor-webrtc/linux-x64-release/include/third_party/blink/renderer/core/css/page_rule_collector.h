/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_RULE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_RULE_COLLECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/resolver/match_result.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CascadeLayerMap;
class StyleRulePage;

class CORE_EXPORT PageRuleCollector {
  STACK_ALLOCATED();

 public:
  PageRuleCollector(const ComputedStyle* root_element_style,
                    CSSAtRuleID at_rule_id,
                    uint32_t page_index,
                    const AtomicString& page_name,
                    MatchResult&);

  // TreeScope is required for CascadeOrigin::kAuthor,
  // and ignored for other origins.
  void MatchPageRules(RuleSet* rules,
                      CascadeOrigin,
                      TreeScope*,
                      const CascadeLayerMap* layer_map);
  const MatchResult& MatchedResult() { return result_; }

 private:
  bool IsLeftPage(const ComputedStyle* root_element_style,
                  uint32_t page_index) const;
  bool IsRightPage(const ComputedStyle* root_element_style,
                   uint32_t page_index) const {
    return !IsLeftPage(root_element_style, page_index);
  }
  bool IsFirstPage(uint32_t page_index) const;

  void MatchPageRulesForList(HeapVector<Member<StyleRulePage>>& matched_rules,
                             const HeapVector<Member<StyleRulePage>>& rules);

  const bool is_left_page_;
  const bool is_first_page_;
  const CSSAtRuleID at_rule_id_;
  const AtomicString page_name_;

  MatchResult& result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_RULE_COLLECTOR_H_
