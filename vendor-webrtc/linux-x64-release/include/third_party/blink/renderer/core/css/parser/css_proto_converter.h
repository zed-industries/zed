// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PROTO_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PROTO_CONVERTER_H_

#include <string>

#include "third_party/blink/renderer/core/css/parser/css.pb.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace css_proto_converter {

class Converter {
  STACK_ALLOCATED();

 public:
  Converter();
  std::string Convert(const StyleSheet&);

 private:
  std::string string_;

  static const int kAtRuleDepthLimit;
  static const int kSupportsConditionDepthLimit;
  static const std::string kPseudoLookupTable[];
  static const std::string kMediaTypeLookupTable[];
  static const std::string kMfNameLookupTable[];
  static const std::string kImportLookupTable[];
  static const std::string kEncodingLookupTable[];
  static const std::string kValueLookupTable[];
  static const std::string kPropertyLookupTable[];
  static const std::string kViewportPropertyLookupTable[];
  static const std::string kViewportValueLookupTable[];

  void Visit(const Unicode&);
  void Visit(const Escape&);
  void Visit(const Nmstart&);
  void Visit(const Nmchar&);
  void Visit(const String&);
  void Visit(const StringCharOrQuote&, bool use_single);
  void Visit(const StringChar&);
  void Visit(const Ident&);
  void Visit(const Num&);
  void Visit(const UrlChar&);
  void Visit(const W&);
  void Visit(const UnrepeatedW&);
  void Visit(const Nl&);
  void Visit(const Length&);
  void Visit(const Angle&);
  void Visit(const Time&);
  void Visit(const Freq&);
  void Visit(const Uri&);
  void Visit(const FunctionToken&);
  void Visit(const StyleSheet&);
  void Visit(const CharsetDeclaration&);
  void Visit(const NestedAtRule&, int depth = 0);
  void Visit(const Import&);
  void Visit(const Namespace&);
  void Visit(const NamespacePrefix&);
  void Visit(const Media&);
  void Visit(const Page&);
  void Visit(const DeclarationList&);
  void Visit(const FontFace&);
  void Visit(const Operator&);
  void Visit(const UnaryOperator&);
  void Visit(const Property&);
  void Visit(const Ruleset&);
  void Visit(const SelectorList&);
  void Visit(const Declaration&);
  void Visit(const PropertyAndValue&);
  void Visit(const Expr&, int declaration_value_id = 0);
  void Visit(const OperatorTerm&);
  void Visit(const Term&);
  void Visit(const TermPart&);
  void Visit(const Function&);
  void Visit(const Hexcolor&);
  void Visit(const HexcolorThree&);
  void Visit(const MediaQueryList&);
  void Visit(const MediaQuery&);
  void Visit(const MediaQueryPartTwo&);
  void Visit(const MediaType&);
  void Visit(const MediaNot&);
  void Visit(const MediaAnd&);
  void Visit(const MediaOr&);
  void Visit(const MediaInParens&);
  void Visit(const MediaFeature&);
  void Visit(const MfBool&);
  void Visit(const MfName&);
  void Visit(const MfValue&);
  void Visit(const MediaCondition&);
  void Visit(const MediaConditionWithoutOr&);
  void Visit(const Selector&, bool is_first);
  void Visit(const PseudoPage&);
  void Visit(const ViewportValue&);
  void Visit(const Viewport&);
  void Visit(const SupportsRule&, int depth);
  void Visit(const SupportsCondition&, int depth);
  void AppendBinarySupportsCondition(
      const BinarySupportsCondition& binary_condition,
      std::string binary_operator,
      int depth);
  void Visit(const AtRuleOrRulesets&, int depth);
  void Visit(const AtRuleOrRuleset&, int depth);

  void Reset();
  template <size_t EnumSize, class T, size_t TableSize>
  void AppendPropertyAndValue(T property_and_value,
                              const std::string (&lookup_table)[TableSize],
                              bool append_semicolon = true);
  template <size_t EnumSize, size_t TableSize>
  void AppendTableValue(int id, const std::string (&lookup_table)[TableSize]);
};
}  // namespace css_proto_converter

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PROTO_CONVERTER_H_
