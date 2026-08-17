// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_DATA_H_

#include <optional>

#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class StyleScope;

// Implicit @scope rules are scoped to the parent element of the owner node of
// the stylesheet that defined the @scope rule. Each such parent element holds
// a StyleScopeData instance, with references back to the StyleScopes that
// are "triggered" by that element.
//
// This can be used to quickly determine if a given StyleScope is triggered
// by an Element (a check that would otherwise potentially be expensive, due
// to a single StyleSheetContents/StyleScope being shared by multiple
// CSSStyleSheets).
class StyleScopeData final : public GarbageCollected<StyleScopeData>,
                             public ElementRareDataField {
 public:
  void Trace(Visitor*) const override;

  void AddTriggeredImplicitScope(const StyleScope&);
  void RemoveTriggeredImplicitScope(const StyleScope&);
  bool TriggersScope(const StyleScope&) const;

  const HeapVector<Member<const StyleScope>, 1>& GetTriggeredScopes() const {
    return triggered_implicit_scopes_;
  }

 private:
  friend class StyleScopeDataTest;

  // An element is assumed to trigger a single StyleScope in the common case
  // (i.e. only have one <style> element beneath it).
  //
  // It's possible however to have trigger more than one StyleScope,
  // for example:
  //
  // - When there's more than one <style> child.
  // - When the element is a shadow host, and there's more than one
  //   adopted stylesheet.
  // - Or when there's a combination of <style> elements and adopted
  //   stylesheets.
  HeapVector<Member<const StyleScope>, 1> triggered_implicit_scopes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_DATA_H_
