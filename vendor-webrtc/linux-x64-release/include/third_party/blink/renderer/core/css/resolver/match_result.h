/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_RESULT_H_

#include "base/compiler_specific.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cascade_layer_map.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_origin.h"
#include "third_party/blink/renderer/core/css/resolver/match_flags.h"
#include "third_party/blink/renderer/core/css/rule_set.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSPropertyValueSet;

struct CORE_EXPORT MatchedProperties {
  DISALLOW_NEW();

 public:
  // NOTE: tree_order is filled by AddMatchedProperties().
  struct Data {
    // This is approximately equivalent to the 'shadow-including tree order'.
    // It can be used to evaluate the 'Shadow Tree' criteria. Note that the
    // number stored here is 'local' to each origin (user, author), and is
    // not used at all for the UA origin. Hence, it is not possible to compare
    // tree_orders from two different origins.
    //
    // https://drafts.csswg.org/css-scoping/#shadow-cascading
    uint16_t tree_order = 0;
    uint8_t link_match_type : 2 = CSSSelector::kMatchAll;
    uint8_t valid_property_filter : 4 =
        static_cast<std::underlying_type_t<ValidPropertyFilter>>(
            ValidPropertyFilter::kNoFilter);
    uint8_t is_inline_style : 1 = false;
    // Try styles come from position-try-fallbacks.
    // https://drafts.csswg.org/css-anchor-position-1/#fallback
    uint8_t is_try_style : 1 = false;
    CascadeOrigin origin = CascadeOrigin::kNone;
    // https://drafts.csswg.org/css-cascade-5/#layer-ordering
    uint16_t layer_order = CascadeLayerMap::kImplicitOuterLayerOrder;
    // Try-tactics style come from <try-tactic>.
    // https://drafts.csswg.org/css-anchor-position-1/#typedef-position-try-fallbacks-try-tactic
    bool is_try_tactics_style = false;
    // 15 free bits after this, but since the MPC hashes and compares
    // this as raw bytes, we cannot have undefined padding.
    uint8_t padding = 0;

    bool operator==(const Data& other) const {
      return UNSAFE_TODO(memcmp(this, &other, sizeof(*this))) == 0;
    }
    bool operator!=(const Data& other) const {
      return UNSAFE_TODO(memcmp(this, &other, sizeof(*this))) != 0;
    }
  };

  MatchedProperties(CSSPropertyValueSet* properties_arg, const Data& data_arg)
      : properties(properties_arg), data_(data_arg) {}

  void Trace(Visitor*) const;

  Member<CSSPropertyValueSet> properties;
  Data data_;
};

struct SameSizeAsMatchedProperties {
  Member<void*> properties;
  uint8_t data_[8];
};

ASSERT_SIZE(MatchedProperties, SameSizeAsMatchedProperties);

struct CORE_EXPORT MatchedPropertiesHash {
  // The value of ComputeHash() of the corresponding CSSPropertyValueSet.
  unsigned hash;

  // It's unfortunate that we need to duplicate Data here just to have it be
  // part of the hash, but we cannot easily move it out of MatchedProperties,
  // since CachedMatchedProperties::CorrespondsTo() needs it (and the hashes
  // are not stored in CachedMatchedProperties).
  MatchedProperties::Data data;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::MatchedProperties)

namespace blink {

using MatchedPropertiesVector = HeapVector<MatchedProperties, 64>;
using MatchedPropertiesHashVector = HeapVector<MatchedPropertiesHash, 64>;

class CORE_EXPORT MatchResult {
  STACK_ALLOCATED();

 public:
  MatchResult() = default;
  MatchResult(const MatchResult&) = delete;
  MatchResult& operator=(const MatchResult&) = delete;

  void AddMatchedProperties(const CSSPropertyValueSet* properties,
                            MatchedProperties::Data types);
  bool HasMatchedProperties() const { return matched_properties_.size(); }

  void BeginAddingAuthorRulesForTreeScope(const TreeScope&);

  void AddCustomHighlightName(const AtomicString& custom_highlight_name) {
    custom_highlight_names_.insert(custom_highlight_name);
  }
  const HashSet<AtomicString>& CustomHighlightNames() const {
    return custom_highlight_names_;
  }

  void SetIsCacheable(bool cacheable) { is_cacheable_ = cacheable; }
  bool IsCacheable() const { return is_cacheable_; }
  void SetDependsOnSizeContainerQueries() {
    depends_on_size_container_queries_ = true;
  }
  bool DependsOnSizeContainerQueries() const {
    return depends_on_size_container_queries_;
  }
  void SetDependsOnStyleContainerQueries() {
    depends_on_style_container_queries_ = true;
  }
  bool DependsOnStyleContainerQueries() const {
    return depends_on_style_container_queries_;
  }
  void SetDependsOnScrollStateContainerQueries() {
    depends_on_scroll_state_container_queries_ = true;
  }
  bool DependsOnScrollStateContainerQueries() const {
    return depends_on_scroll_state_container_queries_;
  }
  void SetFirstLineDependsOnSizeContainerQueries() {
    first_line_depends_on_size_container_queries_ = true;
  }
  bool FirstLineDependsOnSizeContainerQueries() const {
    return first_line_depends_on_size_container_queries_;
  }
  void SetDependsOnStaticViewportUnits() {
    depends_on_static_viewport_units_ = true;
  }
  void SetDependsOnDynamicViewportUnits() {
    depends_on_dynamic_viewport_units_ = true;
  }
  bool DependsOnStaticViewportUnits() const {
    return depends_on_static_viewport_units_;
  }
  bool DependsOnDynamicViewportUnits() const {
    return depends_on_dynamic_viewport_units_;
  }
  void SetDependsOnRootFontContainerQueries() {
    depends_on_root_font_container_queries_ = true;
  }
  bool DependsOnRootFontContainerQueries() const {
    return depends_on_root_font_container_queries_;
  }
  void SetConditionallyAffectsAnimations() {
    conditionally_affects_animations_ = true;
  }
  bool ConditionallyAffectsAnimations() const {
    return conditionally_affects_animations_;
  }
  void SetHasNonUniversalHighlightPseudoStyles() {
    has_non_universal_highlight_pseudo_styles_ = true;
  }
  bool HasNonUniversalHighlightPseudoStyles() const {
    return has_non_universal_highlight_pseudo_styles_;
  }
  void SetHasNonUaHighlightPseudoStyles() {
    has_non_ua_highlight_pseudo_styles_ = true;
  }
  bool HasNonUaHighlightPseudoStyles() const {
    return has_non_ua_highlight_pseudo_styles_;
  }
  void SetHighlightsDependOnSizeContainerQueries() {
    highlights_depend_on_size_container_queries_ = true;
  }
  bool HighlightsDependOnSizeContainerQueries() const {
    return highlights_depend_on_size_container_queries_;
  }

  bool HasFlag(MatchFlag flag) const {
    return flags_ & static_cast<MatchFlags>(flag);
  }
  void AddFlags(MatchFlags flags) { flags_ |= flags; }

  void SetHasPseudoElementStyle(PseudoId pseudo) {
    DCHECK(pseudo >= kFirstPublicPseudoId);
    DCHECK(pseudo <= kLastTrackedPublicPseudoId);
    pseudo_element_styles_ |= 1 << (pseudo - kFirstPublicPseudoId);
  }
  unsigned PseudoElementStyles() const { return pseudo_element_styles_; }

  const MatchedPropertiesVector& GetMatchedProperties() const {
    return matched_properties_;
  }
  const MatchedPropertiesHashVector& GetMatchedPropertiesHash() const {
    return matched_properties_hashes_;
  }

  // Reset the MatchResult to its initial state, as if no MatchedProperties
  // objects were added.
  void Reset();

  const TreeScope* CurrentTreeScope() const {
    if (tree_scopes_.empty()) {
      return nullptr;
    }
    return tree_scopes_.back().Get();
  }

  const TreeScope& ScopeFromTreeOrder(uint16_t tree_order) const {
    SECURITY_DCHECK(tree_order < tree_scopes_.size());
    return *tree_scopes_[tree_order];
  }

 private:
  MatchedPropertiesVector matched_properties_;
  // Same size as matched_properties_; kept separate so that it is
  // contiguous (so that we hash the hashes more easily).
  MatchedPropertiesHashVector matched_properties_hashes_;
  HeapVector<Member<const TreeScope>, 4> tree_scopes_;
  HashSet<AtomicString> custom_highlight_names_;
  bool is_cacheable_{true};
  bool depends_on_size_container_queries_{false};
  bool depends_on_style_container_queries_{false};
  bool depends_on_scroll_state_container_queries_{false};
  bool first_line_depends_on_size_container_queries_{false};
  bool depends_on_static_viewport_units_{false};
  bool depends_on_dynamic_viewport_units_{false};
  bool depends_on_root_font_container_queries_{false};
  bool conditionally_affects_animations_{false};
  bool has_non_universal_highlight_pseudo_styles_{false};
  bool has_non_ua_highlight_pseudo_styles_{false};
  bool highlights_depend_on_size_container_queries_{false};
  MatchFlags flags_{0};
#if DCHECK_IS_ON()
  CascadeOrigin last_origin_{CascadeOrigin::kNone};
#endif
  uint16_t current_tree_order_{0};
  uint32_t pseudo_element_styles_{kPseudoIdNone};
};

inline bool operator==(const MatchedProperties& a, const MatchedProperties& b) {
  return a.properties == b.properties &&
         a.data_.link_match_type == b.data_.link_match_type;
}

inline bool operator!=(const MatchedProperties& a, const MatchedProperties& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_RESULT_H_
