// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_SELECTOR_FILTER_PARENT_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_SELECTOR_FILTER_PARENT_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/resolver/style_resolver.h"
#include "third_party/blink/renderer/core/css/selector_filter.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

// Maintains the existence filter inside RecalcStyle.
// SelectorFilterParentScope for the parent element is added to the stack before
// recalculating style for its children.
class CORE_EXPORT SelectorFilterParentScope {
  STACK_ALLOCATED();

 public:
  // When starting the style recalc, we push an object of this class onto the
  // stack to establish a root scope for the SelectorFilter for a document to
  // make the style recalc re-entrant.
  enum class ScopeType { kParent, kRoot };

  explicit SelectorFilterParentScope(Element* parent, ScopeType scope_type) {
    if (parent) {
      DCHECK(parent->GetDocument().InStyleRecalc());
      resolver_ = &parent->GetDocument().GetStyleResolver();
      mark_ = resolver_->GetSelectorFilter().SetMark();
      if (scope_type == ScopeType::kRoot) {
        PushAncestors(*parent);
        resolver_->GetSelectorFilter().PushParent(*parent);
      } else {
        resolver_->GetSelectorFilter().PushParent(*parent);
      }
    }
  }

  ~SelectorFilterParentScope() {
    if (resolver_) {
      resolver_->GetSelectorFilter().PopTo(mark_);
    }
  }

 private:
  void PushParentIfNeeded();
  void PushAncestors(Element&);
  void PopAncestors(Element&);

  bool pushed_ = false;
  SelectorFilter::Mark mark_;
  StyleResolver* resolver_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_SELECTOR_FILTER_PARENT_SCOPE_H_
