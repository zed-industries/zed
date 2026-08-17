// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ATTR_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ATTR_TYPE_H_

#include <optional>

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_syntax_definition.h"

namespace blink {

class CORE_EXPORT CSSAttrType {
  STACK_ALLOCATED();

 public:
  static std::optional<CSSAttrType> Consume(CSSParserTokenStream&);
  const CSSValue* Parse(StringView, const CSSParserContext&) const;
  static CSSAttrType GetDefaultValue();
  bool IsSyntax() const { return syntax_.has_value(); }
  bool IsString() const { return is_string_; }
  bool IsDimensionUnit() const { return dimension_unit_.has_value(); }

 private:
  CSSAttrType()
      : syntax_(std::nullopt),
        is_string_(true),
        dimension_unit_(std::nullopt) {}

  explicit CSSAttrType(CSSSyntaxDefinition syntax)
      : syntax_(syntax), is_string_(false), dimension_unit_(std::nullopt) {}

  explicit CSSAttrType(CSSPrimitiveValue::UnitType unit)
      : syntax_(std::nullopt), is_string_(false), dimension_unit_(unit) {}

  std::optional<CSSSyntaxDefinition> syntax_;
  bool is_string_;
  std::optional<CSSPrimitiveValue::UnitType> dimension_unit_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_ATTR_TYPE_H_
