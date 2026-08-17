// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SCOPED_KEYWORD_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SCOPED_KEYWORD_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {
namespace cssvalue {

// Similar CSSIdentifierValue, but for tree-scoped idents.
//
// An example of a tree-scope keyword is the 'all' keyword for anchor-scope.
// https://github.com/w3c/csswg-drafts/issues/10525
class CORE_EXPORT CSSScopedKeywordValue : public CSSValue {
 public:
  explicit CSSScopedKeywordValue(CSSValueID value_id)
      : CSSValue(kScopedKeywordClass), value_id_(value_id) {
    needs_tree_scope_population_ = true;
  }

  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }
  CSSValueID GetValueID() const { return value_id_; }

  WTF::String CustomCSSText() const;

  const CSSScopedKeywordValue& PopulateWithTreeScope(
      const TreeScope* tree_scope) const;

  bool Equals(const CSSScopedKeywordValue& other) const {
    return IsScopedValue() == other.IsScopedValue() &&
           tree_scope_ == other.tree_scope_ && value_id_ == other.value_id_;
  }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const TreeScope> tree_scope_;
  CSSValueID value_id_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSScopedKeywordValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsScopedKeywordValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SCOPED_KEYWORD_VALUE_H_
