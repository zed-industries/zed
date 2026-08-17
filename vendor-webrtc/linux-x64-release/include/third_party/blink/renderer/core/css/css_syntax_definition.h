// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_DEFINITION_H_

#include <algorithm>
#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_syntax_component.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token_stream.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

class CSSParserContext;
class CSSValue;

class CORE_EXPORT CSSSyntaxDefinition {
 public:
  // https://drafts.csswg.org/css-values-5/#css-syntax
  static std::optional<CSSSyntaxDefinition> Consume(CSSParserTokenStream&);
  // https://drafts.csswg.org/css-values-5/#typedef-syntax-component
  static std::optional<CSSSyntaxDefinition> ConsumeComponent(
      CSSParserTokenStream&);
  const CSSValue* Parse(StringView,
                        const CSSParserContext&,
                        bool is_animation_tainted,
                        bool is_attr_tainted = false) const;

  // https://drafts.css-houdini.org/css-properties-values-api-1/#universal-syntax-descriptor
  bool IsUniversal() const {
    return syntax_components_.size() == 1 &&
           syntax_components_[0].GetType() == CSSSyntaxType::kTokenStream;
  }
  bool ContainsUrlComponent() const {
    return std::find_if(syntax_components_.begin(), syntax_components_.end(),
                        [](const CSSSyntaxComponent& component) {
                          return component.GetType() == CSSSyntaxType::kUrl;
                        }) != syntax_components_.end();
  }
  const Vector<CSSSyntaxComponent>& Components() const {
    return syntax_components_;
  }
  bool operator==(const CSSSyntaxDefinition& a) const {
    return Components() == a.Components();
  }
  bool operator!=(const CSSSyntaxDefinition& a) const {
    return Components() != a.Components();
  }

  CSSSyntaxDefinition IsolatedCopy() const;
  String ToString() const;

  // https://drafts.css-houdini.org/css-properties-values-api-1/#universal-syntax-descriptor
  static CSSSyntaxDefinition CreateUniversal();

 private:
  friend class CSSSyntaxStringParser;
  friend class CSSSyntaxStringParserTest;
  friend class CSSSyntaxDefinitionTest;

  explicit CSSSyntaxDefinition(Vector<CSSSyntaxComponent>);

  Vector<CSSSyntaxComponent> syntax_components_;
};

}  // namespace blink

namespace WTF {

template <wtf_size_t inlineCapacity, typename Allocator>
struct CrossThreadCopier<
    Vector<blink::CSSSyntaxDefinition, inlineCapacity, Allocator>> {
  using Type = Vector<blink::CSSSyntaxDefinition, inlineCapacity, Allocator>;
  static Type Copy(const Type& value) {
    Type result;
    result.ReserveInitialCapacity(value.size());
    for (const auto& element : value) {
      result.push_back(element.IsolatedCopy());
    }
    return result;
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_DEFINITION_H_
