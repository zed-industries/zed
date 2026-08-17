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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_FEATURE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_FEATURE_SET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_flags.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_set.h"
#include "third_party/blink/renderer/core/css/invalidation/rule_invalidation_data.h"
#include "third_party/blink/renderer/core/css/invalidation/selector_pre_match.h"
#include "third_party/blink/renderer/core/css/resolver/media_query_result.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class CSSSelector;
class StyleScope;

// RuleFeatureSet is a container for invalidation related information collected
// from style rules (RuleInvalidationData) plus information on whether those
// style rules are contained within media queries that have environmental
// dependencies (MediaQueryResultFlags).
//
// The name may be somewhat confusing and is for historical reasons. The
// original “features” extracted from the selectors (e.g. “does any selector use
// ::first-line”) are now held in metadata flags on RuleInvalidationData. Media
// query results and invalidation sets were added later. Then invalidation sets
// and metadata were moved to RuleInvalidationData to group them together, since
// they are collected at the same time, with two goals in mind: (a) consolidate
// invalidation related data structures under css/invalidation; (b) reuse the
// logic for collecting invalidation sets and metadata as a `const` variant that
// can reconstruct the relationships between style rules and invalidation sets
// for developer tooling.
class CORE_EXPORT RuleFeatureSet {
  DISALLOW_NEW();

 public:
  RuleFeatureSet() = default;
  RuleFeatureSet(const RuleFeatureSet&) = delete;
  RuleFeatureSet& operator=(const RuleFeatureSet&) = delete;

  bool operator==(const RuleFeatureSet&) const;
  bool operator!=(const RuleFeatureSet& o) const { return !(*this == o); }

  // Merge the given RuleFeatureSet (which remains unchanged) into this one.
  void Merge(const RuleFeatureSet&);
  void Clear();

  // Creates invalidation sets for the given CSS selector. This is done as part
  // of creating the RuleSet for the style sheet, i.e., before matching or
  // mutation begins.
  SelectorPreMatch CollectFeaturesFromSelector(const CSSSelector&,
                                               const StyleScope*);

  void RevisitSelectorForInspector(const CSSSelector&) const;

  // Member functions for accessing non-invalidation-set related features.
  MediaQueryResultFlags& MutableMediaQueryResultFlags() {
    return media_query_result_flags_;
  }
  bool HasMediaQueryResults() const {
    return media_query_result_flags_.is_viewport_dependent ||
           media_query_result_flags_.is_device_dependent;
  }
  bool HasViewportDependentMediaQueries() const;
  bool HasDynamicViewportDependentMediaQueries() const;

  const RuleInvalidationData& GetRuleInvalidationData() const;

  // Format the RuleFeatureSet for debugging purposes.
  String ToString() const;

 private:
  RuleInvalidationData rule_invalidation_data_;

  MediaQueryResultFlags media_query_result_flags_;

  friend class RuleFeatureSetTest;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const RuleFeatureSet&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_FEATURE_SET_H_
