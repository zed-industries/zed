// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_GRAMMARS_JSON_GRAMMAR_H_
#define FUZZTEST_GRAMMARS_JSON_GRAMMAR_H_

#include "./fuzztest/internal/domains/in_grammar_impl.h"

namespace fuzztest::internal::grammar::json {

enum JsonTypes {
  kJsonNode,
  kValueNode,
  kObjectNode,
  kMembersNode,
  kMemberNode,
  kArrayNode,
  kElementsNode,
  kElementNode,
  kSTRINGNode,
  kCHARACTERNode,
  kNUMBERNode,
  kINTEGERNode,
  kDIGITSNode,
  kDIGITNode,
  kONETONINENode,
  kFRACTIONNode,
  kEXPONENTNode,
  kSIGNNode,
  kWSPACENode,
  kObjectSubNode17,
  kObjectSubNode18,
  kMembersSubNode19,
  kArraySubNode20,
  kArraySubNode21,
  kElementsSubNode22,
  kElementsSubNode23,
  kSTRINGSubNode24,
  kNUMBERSubNode25,
  kNUMBERSubNode26,
  kINTEGERSubNode27,
  kINTEGERSubNode28,
  kINTEGERSubNode29,
  kDIGITSSubNode30,
  kEXPONENTSubNode31,
  kEXPONENTSubNode32,
  kWSPACESubNode33,
  kLiteral3,
  kLiteral8,
  kLiteral12,
  kLiteral9,
  kLiteral7,
  kLiteral6,
  kLiteral4,
  kLiteral13,
  kLiteral5,
  kLiteral14,
  kLiteral1,
  kLiteral2,
  kLiteral0,
  kLiteral10,
  kLiteral11,
  kCharSet3,
  kCharSet1,
  kCharSet2,
  kCharSet0,
};
class JsonNode;
class ValueNode;
class ObjectNode;
class MembersNode;
class MemberNode;
class ArrayNode;
class ElementsNode;
class ElementNode;
class STRINGNode;
class CHARACTERNode;
class NUMBERNode;
class INTEGERNode;
class DIGITSNode;
class DIGITNode;
class ONETONINENode;
class FRACTIONNode;
class EXPONENTNode;
class SIGNNode;
class WSPACENode;
class ObjectSubNode17;
class ObjectSubNode18;
class MembersSubNode19;
class ArraySubNode20;
class ArraySubNode21;
class ElementsSubNode22;
class ElementsSubNode23;
class STRINGSubNode24;
class NUMBERSubNode25;
class NUMBERSubNode26;
class INTEGERSubNode27;
class INTEGERSubNode28;
class INTEGERSubNode29;
class DIGITSSubNode30;
class EXPONENTSubNode31;
class EXPONENTSubNode32;
class WSPACESubNode33;
class Literal3;
class Literal8;
class Literal12;
class Literal9;
class Literal7;
class Literal6;
class Literal4;
class Literal13;
class Literal5;
class Literal14;
class Literal1;
class Literal2;
class Literal0;
class Literal10;
class Literal11;
class CharSet3;
class CharSet1;
class CharSet2;
class CharSet0;
inline constexpr absl::string_view kStrLiteral3 = " ";
inline constexpr absl::string_view kStrLiteral8 = "+";
inline constexpr absl::string_view kStrLiteral12 = ",";
inline constexpr absl::string_view kStrLiteral9 = "-";
inline constexpr absl::string_view kStrLiteral7 = ".";
inline constexpr absl::string_view kStrLiteral6 = "0";
inline constexpr absl::string_view kStrLiteral4 = ":";
inline constexpr absl::string_view kStrLiteral13 = "[";
inline constexpr absl::string_view kStrLiteral5 = "\"";
inline constexpr absl::string_view kStrLiteral14 = "]";
inline constexpr absl::string_view kStrLiteral1 = "false";
inline constexpr absl::string_view kStrLiteral2 = "null";
inline constexpr absl::string_view kStrLiteral0 = "true";
inline constexpr absl::string_view kStrLiteral10 = "{";
inline constexpr absl::string_view kStrLiteral11 = "}";
inline constexpr absl::string_view kStrCharSet3 = R"grammar([ \t\n\r])grammar";
inline constexpr absl::string_view kStrCharSet1 = R"grammar([1-9])grammar";
inline constexpr absl::string_view kStrCharSet2 = R"grammar([Ee])grammar";
inline constexpr absl::string_view kStrCharSet0 =
    R"grammar([a-zA-Z0-9_])grammar";
class JsonNode final : public TupleDomain<kJsonNode, ElementNode> {};
class ValueNode final
    : public VariantDomain<kValueNode, 4, ObjectNode, ArrayNode, STRINGNode,
                           NUMBERNode, Literal0, Literal1, Literal2> {};
class ObjectNode final
    : public VariantDomain<kObjectNode, 0, ObjectSubNode17, ObjectSubNode18> {};
class MembersNode final
    : public VariantDomain<kMembersNode, 0, MemberNode, MembersSubNode19> {};
class MemberNode final : public TupleDomain<kMemberNode, STRINGNode, Literal3,
                                            Literal4, Literal3, ElementNode> {};
class ArrayNode final
    : public VariantDomain<kArrayNode, 0, ArraySubNode20, ArraySubNode21> {};
class ElementsNode final : public TupleDomain<kElementsNode, ElementNode,
                                              Literal3, ElementsSubNode22> {};
class ElementNode final : public TupleDomain<kElementNode, ValueNode> {};
class STRINGNode final
    : public TupleDomain<kSTRINGNode, Literal5, STRINGSubNode24, Literal5> {};
class CHARACTERNode final : public TupleDomain<kCHARACTERNode, CharSet0> {};
class NUMBERNode final : public TupleDomain<kNUMBERNode, INTEGERNode,
                                            NUMBERSubNode25, NUMBERSubNode26> {
};
class INTEGERNode final
    : public VariantDomain<kINTEGERNode, 0, DIGITNode, INTEGERSubNode27,
                           INTEGERSubNode28, INTEGERSubNode29> {};
class DIGITSNode final : public TupleDomain<kDIGITSNode, DIGITSSubNode30> {};
class DIGITNode final
    : public VariantDomain<kDIGITNode, 0, Literal6, ONETONINENode> {};
class ONETONINENode final : public TupleDomain<kONETONINENode, CharSet1> {};
class FRACTIONNode final
    : public TupleDomain<kFRACTIONNode, Literal7, DIGITSNode> {};
class EXPONENTNode final
    : public TupleDomain<kEXPONENTNode, CharSet2, EXPONENTSubNode31,
                         ONETONINENode, EXPONENTSubNode32> {};
class SIGNNode final : public VariantDomain<kSIGNNode, 0, Literal8, Literal9> {
};
class WSPACENode final : public TupleDomain<kWSPACENode, WSPACESubNode33> {};
class ObjectSubNode17 final
    : public TupleDomain<kObjectSubNode17, Literal10, Literal3, Literal11> {};
class ObjectSubNode18 final
    : public TupleDomain<kObjectSubNode18, Literal10, Literal3, MembersNode,
                         Literal3, Literal11> {};
class MembersSubNode19 final
    : public TupleDomain<kMembersSubNode19, MemberNode, Literal3, Literal12,
                         Literal3, MembersNode> {};
class ArraySubNode20 final
    : public TupleDomain<kArraySubNode20, Literal13, Literal3, Literal14> {};
class ArraySubNode21 final
    : public TupleDomain<kArraySubNode21, Literal13, Literal3, ElementsNode,
                         Literal3, Literal14> {};
class ElementsSubNode22 final
    : public Vector<kElementsSubNode22, ElementsSubNode23> {};
class ElementsSubNode23 final
    : public TupleDomain<kElementsSubNode23, Literal12, Literal3, ElementNode> {
};
class STRINGSubNode24 final : public Vector<kSTRINGSubNode24, CHARACTERNode> {};
class NUMBERSubNode25 final : public Optional<kNUMBERSubNode25, FRACTIONNode> {
};
class NUMBERSubNode26 final : public Optional<kNUMBERSubNode26, EXPONENTNode> {
};
class INTEGERSubNode27 final
    : public TupleDomain<kINTEGERSubNode27, ONETONINENode, DIGITSNode> {};
class INTEGERSubNode28 final
    : public TupleDomain<kINTEGERSubNode28, Literal9, DIGITNode> {};
class INTEGERSubNode29 final : public TupleDomain<kINTEGERSubNode29, Literal9,
                                                  ONETONINENode, DIGITSNode> {};
class DIGITSSubNode30 final
    : public NonEmptyVector<kDIGITSSubNode30, DIGITNode> {};
class EXPONENTSubNode31 final : public Optional<kEXPONENTSubNode31, SIGNNode> {
};
class EXPONENTSubNode32 final : public Optional<kEXPONENTSubNode32, DIGITNode> {
};
class WSPACESubNode33 final
    : public NonEmptyVector<kWSPACESubNode33, CharSet3> {};
class Literal3 final : public StringLiteralDomain<kLiteral3, kStrLiteral3> {};
class Literal8 final : public StringLiteralDomain<kLiteral8, kStrLiteral8> {};
class Literal12 final : public StringLiteralDomain<kLiteral12, kStrLiteral12> {
};
class Literal9 final : public StringLiteralDomain<kLiteral9, kStrLiteral9> {};
class Literal7 final : public StringLiteralDomain<kLiteral7, kStrLiteral7> {};
class Literal6 final : public StringLiteralDomain<kLiteral6, kStrLiteral6> {};
class Literal4 final : public StringLiteralDomain<kLiteral4, kStrLiteral4> {};
class Literal13 final : public StringLiteralDomain<kLiteral13, kStrLiteral13> {
};
class Literal5 final : public StringLiteralDomain<kLiteral5, kStrLiteral5> {};
class Literal14 final : public StringLiteralDomain<kLiteral14, kStrLiteral14> {
};
class Literal1 final : public StringLiteralDomain<kLiteral1, kStrLiteral1> {};
class Literal2 final : public StringLiteralDomain<kLiteral2, kStrLiteral2> {};
class Literal0 final : public StringLiteralDomain<kLiteral0, kStrLiteral0> {};
class Literal10 final : public StringLiteralDomain<kLiteral10, kStrLiteral10> {
};
class Literal11 final : public StringLiteralDomain<kLiteral11, kStrLiteral11> {
};
class CharSet3 final : public RegexLiteralDomain<kCharSet3, kStrCharSet3> {};
class CharSet1 final : public RegexLiteralDomain<kCharSet1, kStrCharSet1> {};
class CharSet2 final : public RegexLiteralDomain<kCharSet2, kStrCharSet2> {};
class CharSet0 final : public RegexLiteralDomain<kCharSet0, kStrCharSet0> {};
}  // namespace fuzztest::internal::grammar::json

namespace fuzztest::internal_no_adl {
inline auto InJsonGrammar() {
  return internal::grammar::InGrammarImpl<internal::grammar::json::JsonNode>();
}
}  // namespace fuzztest::internal_no_adl
#endif  // FUZZTEST_GRAMMARS_JSON_GRAMMAR_H_
