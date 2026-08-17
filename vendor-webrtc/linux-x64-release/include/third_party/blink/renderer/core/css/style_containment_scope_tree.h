// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CONTAINMENT_SCOPE_TREE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CONTAINMENT_SCOPE_TREE_H_

#include "third_party/blink/renderer/core/css/style_containment_scope.h"
#include "third_party/blink/renderer/core/layout/layout_quote.h"

namespace blink {

// Manages the contain style scopes and quotes of the document.
// Maps as 1:1 to the StyleEngine.
class CORE_EXPORT StyleContainmentScopeTree final
    : public GarbageCollected<StyleContainmentScopeTree> {
 public:
  StyleContainmentScopeTree()
      : root_scope_(
            MakeGarbageCollected<StyleContainmentScope>(nullptr, this)) {}
  StyleContainmentScopeTree(const StyleContainmentScopeTree&) = delete;
  StyleContainmentScopeTree& operator=(const StyleContainmentScopeTree&) =
      delete;

  StyleContainmentScope* FindOrCreateEnclosingScopeForElement(const Element&);
  StyleContainmentScope* CreateScopeForElement(const Element&);
  void DestroyScopeForElement(const Element&);
  void RemoveScopeForElement(const Element&);

  // If there is a dirty scope start an update from it going down its subtree.
  // During the update we calculate the correct depth for each quote and set
  // the correct text.
  // It can change the layout tree by creating text fragments.
  void UpdateQuotes();

  void UpdateOutermostQuotesDirtyScope(StyleContainmentScope*);

  void Trace(Visitor*) const;

#if DCHECK_IS_ON()
  String ToString(StyleContainmentScope* style_scope = nullptr,
                  wtf_size_t depth = 0u) const;
#endif  // DCHECK_IS_ON()

 private:
  // The implicit top level scope for elements with no contain:style ancestors.
  Member<StyleContainmentScope> root_scope_;
  // The outermost dirty scope for the quotes update.
  Member<StyleContainmentScope> outermost_quotes_dirty_scope_;
  // The map from element with style containment to the scope it creates.
  HeapHashMap<WeakMember<const Element>, Member<StyleContainmentScope>> scopes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_CONTAINMENT_SCOPE_TREE_H_
