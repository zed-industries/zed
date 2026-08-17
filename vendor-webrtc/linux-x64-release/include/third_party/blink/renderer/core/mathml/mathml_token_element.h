// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TOKEN_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TOKEN_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_element.h"
#include "third_party/blink/renderer/platform/wtf/text/character_names.h"

namespace blink {

class Document;

class CORE_EXPORT MathMLTokenElement : public MathMLElement {
 public:
  explicit MathMLTokenElement(const QualifiedName&, Document&);

  struct TokenContent {
    String characters;
    UChar32 code_point = kNonCharacter;
  };
  const TokenContent& GetTokenContent();

 protected:
  void ChildrenChanged(const ChildrenChange&) override;

 private:
  bool IsPresentationAttribute(const QualifiedName&) const final;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) final;
  TokenContent ParseTokenContent();
  std::optional<TokenContent> token_content_;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) final;
};

template <>
struct DowncastTraits<MathMLTokenElement> {
  static bool AllowFrom(const Node& node) {
    auto* mathml_element = DynamicTo<MathMLElement>(node);
    return mathml_element && AllowFrom(*mathml_element);
  }
  static bool AllowFrom(const MathMLElement& mathml_element) {
    return mathml_element.HasTagName(mathml_names::kMiTag) ||
           mathml_element.HasTagName(mathml_names::kMoTag) ||
           mathml_element.HasTagName(mathml_names::kMnTag) ||
           mathml_element.HasTagName(mathml_names::kMtextTag) ||
           mathml_element.HasTagName(mathml_names::kMsTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TOKEN_ELEMENT_H_
