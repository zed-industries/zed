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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_IN_GRAMMAR_IMPL_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_IN_GRAMMAR_IMPL_H_

#include <cstddef>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "./fuzztest/internal/domains/container_of_impl.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/domains/in_regexp_impl.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/serialization.h"
#include "./fuzztest/internal/type_support.h"

namespace fuzztest::internal::grammar {

// We use a generic and simple data structure to represent the AST tree. The key
// member in the data structure is the type id, from which we can tell the
// structure information about the ASTNode. We then provide three generic Domain
// types for Vector (a list of AST nodes of the same types), Variant (a variant
// of several AST nodes), and Tuple (a tuple of AST nodes).  In the code
// generation, we generate an InGrammar Domain by aggregating the specific
// version of the these generic Domain types.
using ASTTypeId = int;
struct ASTNode {
  ASTTypeId type_id;
  std::variant<std::monostate,        // If the node is a string terminal.
               DFAPath,               // If the node is a regex terminal.
               std::vector<ASTNode>>  // If the node is a non-terminal.
      children;

  size_t NodeCount() const;
};

template <typename T>
bool CheckASTNodeTypeIdAndChildType(const ASTNode& astnode, ASTTypeId type_id) {
  return astnode.type_id == type_id &&
         std::holds_alternative<T>(astnode.children);
}

template <typename T>
bool CheckASTCorpusStructure(const IRObject& obj) {
  auto subs = obj.Subs();
  // An valid sub should contain a type id, an index for the variant, and an
  // IRObject for the children. So the size should be 3.
  if (!subs || subs->size() != 3) {
    return false;
  }

  if (!(*subs)[0].ToCorpus<ASTTypeId>().has_value()) {
    return false;
  }

  auto children_type = (*subs)[1].ToCorpus<size_t>();
  if (!children_type.has_value()) {
    return false;
  }

  constexpr size_t kNumChildASTType =
      std::variant_size_v<decltype(ASTNode::children)>;
  return Switch<kNumChildASTType>(*children_type, [](auto I) {
    return std::is_same_v<
        T, std::variant_alternative_t<I, decltype(ASTNode::children)>>;
  });
}

IRObject WrapASTIntoIRObject(const ASTNode& astnode, IRObject parsed_child);

template <ASTTypeId id, const absl::string_view& value>
class StringLiteralDomain {
 public:
  static ASTNode Init(absl::BitGenRef /*prng*/) {
    return ASTNode{id, std::monostate()};
  }

  static ASTNode InitWithBudget(absl::BitGenRef /*prng*/,
                                int /*generation_budget*/) {
    return ASTNode{id, std::monostate()};
  }

  static void Mutate(ASTNode& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata&, bool) {}

  static ASTTypeId TypeId() { return id; }

  static void ToString(std::string& output, const ASTNode& /*val*/) {
    absl::StrAppend(&output, value);
  }

  static bool IsMutable(const ASTNode& /*val*/) { return false; }

  static IRObject SerializeCorpus(const ASTNode& astnode) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::monostate>(astnode, id),
        "Invalid node!");
    return WrapASTIntoIRObject(astnode, {});
  }

  static std::optional<ASTNode> ParseCorpus(const IRObject& obj) {
    if (!CheckASTCorpusStructure<std::monostate>(obj)) {
      return std::nullopt;
    }
    auto subs = obj.Subs();
    auto type_id = (*subs)[0].ToCorpus<ASTTypeId>();

    ASTNode result;
    result.type_id = *type_id;
    result.children.emplace<std::monostate>();
    return result;
  }

  static absl::Status ValidateCorpusValue(const ASTNode& astnode) {
    if (!CheckASTNodeTypeIdAndChildType<std::monostate>(astnode, id)) {
      return absl::InvalidArgumentError("Invalid node type!");
    }
    return absl::OkStatus();
  }
};

template <ASTTypeId id, const absl::string_view& value>
class RegexLiteralDomain {
 public:
  static ASTNode Init(absl::BitGenRef prng) {
    return ASTNode{id, GetInnerRegexpDomain().Init(prng)};
  }

  static ASTNode InitWithBudget(absl::BitGenRef prng,
                                int /*generation_budget*/) {
    return ASTNode{id, GetInnerRegexpDomain().Init(prng)};
  }

  static void Mutate(ASTNode& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink) {
    GetInnerRegexpDomain().Mutate(std::get<DFAPath>(val.children), prng,
                                  metadata, only_shrink);
  }

  static ASTTypeId TypeId() { return id; }

  static void ToString(std::string& output, const ASTNode& val) {
    FUZZTEST_INTERNAL_CHECK(CheckASTNodeTypeIdAndChildType<DFAPath>(val, id),
                            "Not a regex literal!");
    absl::StrAppend(&output, GetInnerRegexpDomain().GetValue(
                                 std::get<DFAPath>(val.children)));
  }

  static bool IsMutable(const ASTNode& /*val*/) { return true; }

  static IRObject SerializeCorpus(const ASTNode& astnode) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<DFAPath>(astnode, id),
        "Not a regex literal!");
    return WrapASTIntoIRObject(astnode,
                               GetInnerRegexpDomain().SerializeCorpus(
                                   std::get<DFAPath>(astnode.children)));
  }

  static std::optional<ASTNode> ParseCorpus(const IRObject& obj) {
    if (!CheckASTCorpusStructure<DFAPath>(obj)) {
      return std::nullopt;
    }
    auto subs = obj.Subs();
    auto type_id = (*subs)[0].ToCorpus<ASTTypeId>();

    ASTNode result;
    result.type_id = *type_id;
    auto path = GetInnerRegexpDomain().ParseCorpus((*subs)[2]);
    if (!path) {
      return std::nullopt;
    }
    result.children.emplace<DFAPath>(*path);
    return result;
  }

  static absl::Status ValidateCorpusValue(const ASTNode& astnode) {
    if (!CheckASTNodeTypeIdAndChildType<DFAPath>(astnode, id)) {
      return absl::InvalidArgumentError("Not a regex literal!");
    }
    return absl::OkStatus();
  }

 private:
  static internal::InRegexpImpl& GetInnerRegexpDomain() {
    static internal::InRegexpImpl* inner_domain =
        new internal::InRegexpImpl(value.data());
    return *inner_domain;
  }
};

// Use for random AST generation. To avoid inifite recursive generation (i.e.,
// for grammar rules like `expr: expr '+' expr | literal`), we limit the
// generation with budget. We first assign the budget to be `kMaxGenerationNum`.
// Every time we generate a AST node, we decrement the budget by one. When we
// are running out of the budget (the budget is less or equal than 0), the
// generation will be in FallBack mode. The FallBack Mode ensures the generation
// ends. Specifically, every grammar rule that has more than 1 production rules
// has a fallback index. When every grammar rule chooses the fallback index
// during generation (aka, the FallBack Mode is on), generation will guarantee
// to end. The fallback index is precalculated by the code generator. The idea
// is simple: We define a symbol (terminal or non-terminal) as safe if it
// generates a finite string in the fallback mode. First we mark all terminals
// as safe. If a non-terminal has a production rule that consists of only safe
// symbols, we use the index of production rule as the fallback index and mark
// the non-terminal is safe. We repeat the process until every grammar rule has
// a fallback index.
inline constexpr int kMaxGenerationNum = 200;

template <ASTTypeId id, typename ElementT, int min = 0, int max = 10000>
class VectorDomain {
 public:
  static ASTNode Init(absl::BitGenRef prng) {
    return InitWithBudget(prng, kMaxGenerationNum);
  }
  static ASTNode InitWithBudget(absl::BitGenRef prng, int generation_budget) {
    std::vector<ASTNode> children;
    int element_size =
        generation_budget <= 0 ? min : min + absl::Uniform<int>(prng, 0, 2);
    children.reserve(element_size);
    for (int i = 0; i < element_size; ++i) {
      // Distribute the budget evenly between the subtrees.
      children.emplace_back(
          ElementT::InitWithBudget(prng, generation_budget / element_size));
    }
    return ASTNode{id, children};
  }

  static ASTTypeId TypeId() { return id; }

  static void Mutate(ASTNode& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(val, id),
        "Not a vector!");
    std::vector<ASTNode>& elements =
        std::get<std::vector<ASTNode>>(val.children);
    if (only_shrink) {
      ShrinkElements(elements, prng);
      return;
    }

    bool can_mutate_element =
        !elements.empty() && ElementT::IsMutable(elements.back());
    constexpr bool can_change_element_num = max > min;
    if (!can_mutate_element && !can_change_element_num) {
      FUZZTEST_INTERNAL_CHECK(false, "We shouldn't pick an unmutable node.");
      return;
    }
    if (!can_mutate_element) {
      ChangeElementNum(elements, prng);
    } else if (!can_change_element_num) {
      ElementT::Mutate(
          elements[absl::Uniform<size_t>(prng, 0, elements.size())], prng,
          metadata, only_shrink);
    } else {
      if (absl::Bernoulli(prng, 0.5)) {
        ChangeElementNum(elements, prng);
      } else {
        ElementT::Mutate(
            elements[absl::Uniform<size_t>(prng, 0, elements.size())], prng,
            metadata, only_shrink);
      }
    }
  }

  static void ToString(std::string& output, const ASTNode& val) {
    for (const auto& child : std::get<std::vector<ASTNode>>(val.children)) {
      ElementT::ToString(output, child);
    }
  }

  static bool IsMutable(const ASTNode& /*val*/) { return true; }

  static IRObject SerializeCorpus(const ASTNode& astnode) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id),
        "Not a vector!");
    IRObject expansion_obj;
    auto& inner_subs = expansion_obj.MutableSubs();
    for (auto& node : std::get<std::vector<ASTNode>>(astnode.children)) {
      inner_subs.push_back(ElementT::SerializeCorpus(node));
    }
    return WrapASTIntoIRObject(astnode, expansion_obj);
  }

  static std::optional<ASTNode> ParseCorpus(const IRObject& obj) {
    if (!CheckASTCorpusStructure<std::vector<ASTNode>>(obj)) {
      return std::nullopt;
    }
    auto subs = obj.Subs();
    auto type_id = (*subs)[0].ToCorpus<ASTTypeId>();

    auto children = (*subs)[2].Subs();
    if (!children || children->size() < min || children->size() > max) {
      return std::nullopt;
    }

    ASTNode result;
    result.type_id = *type_id;

    std::vector<ASTNode>& child_nodes =
        result.children.emplace<std::vector<ASTNode>>();

    for (const auto& child : *children) {
      auto child_node = ElementT::ParseCorpus(child);
      if (!child_node) {
        return std::nullopt;
      }
      child_nodes.push_back(*child_node);
    }
    return result;
  }

  static absl::Status ValidateCorpusValue(const ASTNode& astnode) {
    if (!CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id)) {
      return absl::InvalidArgumentError("Not a vector!");
    }
    absl::Status status = absl::OkStatus();
    for (const auto& child : std::get<std::vector<ASTNode>>(astnode.children)) {
      status.Update(ElementT::ValidateCorpusValue(child));
      if (!status.ok()) return status;
    }
    return status;
  }

 private:
  static void ShrinkElements(std::vector<ASTNode>& elements,
                             absl::BitGenRef prng) {
    if (elements.empty()) return;
    bool can_remove_element = elements.size() > min;
    bool can_shrink_element = ElementT::IsMutable(elements.back());
    if (!can_remove_element && !can_shrink_element) return;
    if (!can_remove_element) {
      // Cannot remove elements, let's shrink them.
      ElementT::Mutate(*ChoosePosition(elements, IncludeEnd::kNo, prng), prng,
                       {}, /*only_shrink=*/true);
    } else if (!can_shrink_element) {
      // Cannot shrink elements, let's remove one.
      elements.erase(ChoosePosition(elements, IncludeEnd::kNo, prng));
    } else {
      // We can do both. So let's toss a coin to decide.
      if (absl::Bernoulli(prng, 0.5)) {
        elements.erase(ChoosePosition(elements, IncludeEnd::kNo, prng));

      } else {
        ElementT::Mutate(*ChoosePosition(elements, IncludeEnd::kNo, prng), prng,
                         {}, /*only_shrink=*/true);
      }
    }
  }

  static void ChangeElementNum(std::vector<ASTNode>& elements,
                               absl::BitGenRef prng) {
    if (elements.size() == min) {
      elements.emplace_back(ElementT::InitWithBudget(prng, kMaxGenerationNum));
    } else if (elements.size() == max) {
      elements.erase(ChoosePosition(elements, IncludeEnd::kNo, prng));
    } else {
      if (absl::Bernoulli(prng, 0.5)) {
        elements.emplace_back(
            ElementT::InitWithBudget(prng, kMaxGenerationNum));
      } else {
        elements.erase(ChoosePosition(elements, IncludeEnd::kNo, prng));
      }
    }
  }
};

// Maximum number of elements allowed in a vector.
inline constexpr int kMaxElementNum = 1000;
template <ASTTypeId id, typename ElementT>
using Vector = VectorDomain<id, ElementT, 0, kMaxElementNum>;

template <ASTTypeId id, typename ElementT>
using Optional = VectorDomain<id, ElementT, 0, 1>;

template <ASTTypeId id, typename ElementT>
using NonEmptyVector = VectorDomain<id, ElementT, 1, kMaxElementNum>;

template <ASTTypeId id, typename... ElementT>
class TupleDomain {
 public:
  static ASTTypeId TypeId() { return id; }

  static ASTNode Init(absl::BitGenRef prng) {
    return InitWithBudget(prng, kMaxGenerationNum);
  }

  static ASTNode InitWithBudget(absl::BitGenRef prng, int generation_budget) {
    return ASTNode{
        id, std::vector<ASTNode>{ElementT::InitWithBudget(
                prng,
                generation_budget / static_cast<int>(sizeof...(ElementT)))...}};
  }

  static void Mutate(ASTNode& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(val, id) &&
            std::get<std::vector<ASTNode>>(val.children).size() ==
                sizeof...(ElementT),
        "Tuple elements number doesn't match!");

    std::vector<int> mutables;
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      ((ElementT::IsMutable(std::get<std::vector<ASTNode>>(val.children)[I])
            ? mutables.push_back(I)
            : (void)0),
       ...);
    });

    FUZZTEST_INTERNAL_CHECK(
        !mutables.empty(),
        "If the tuple is immutable it shouldn't be picked for mutation.");

    int choice = mutables[absl::Uniform<int>(prng, 0, mutables.size())];
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      ((choice == I
            ? (ElementT::Mutate(std::get<std::vector<ASTNode>>(val.children)[I],
                                prng, metadata, only_shrink))
            : (void)0),
       ...);
    });
  }

  static void ToString(std::string& output, const ASTNode& val) {
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      (ElementT::ToString(output,
                          std::get<std::vector<ASTNode>>(val.children)[I]),
       ...);
    });
  }

  static bool IsMutable(const ASTNode& val) {
    bool result = false;
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      result = (ElementT::IsMutable(
                    std::get<std::vector<ASTNode>>(val.children)[I]) ||
                ...);
    });
    return result;
  }

  static IRObject SerializeCorpus(const ASTNode& astnode) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id),
        "Invalid node!");
    IRObject expansion_obj;
    auto& inner_subs = expansion_obj.MutableSubs();
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      (inner_subs.push_back(ElementT::SerializeCorpus(
           std::get<std::vector<ASTNode>>(astnode.children)[I])),
       ...);
    });
    return WrapASTIntoIRObject(astnode, expansion_obj);
  }

  static std::optional<ASTNode> ParseCorpus(const IRObject& obj) {
    if (!CheckASTCorpusStructure<std::vector<ASTNode>>(obj)) {
      return std::nullopt;
    }
    auto subs = obj.Subs();
    auto type_id = (*subs)[0].ToCorpus<ASTTypeId>();

    auto children = (*subs)[2].Subs();
    if (!children || children->size() != sizeof...(ElementT)) {
      return std::nullopt;
    }
    ASTNode result;
    result.type_id = *type_id;

    std::vector<std::optional<ASTNode>> parsed_child_nodes;
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      (parsed_child_nodes.push_back(ElementT::ParseCorpus((*children)[I])),
       ...);
    });

    std::vector<ASTNode>& child_nodes =
        result.children.emplace<std::vector<ASTNode>>();
    for (auto& child : parsed_child_nodes) {
      if (!child.has_value()) {
        return std::nullopt;
      }
      child_nodes.push_back(*child);
    }
    return result;
  }

  static absl::Status ValidateCorpusValue(const ASTNode& astnode) {
    if (!CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id)) {
      return absl::InvalidArgumentError("Not a vector!");
    }
    if (std::get<std::vector<ASTNode>>(astnode.children).size() ==
        sizeof...(ElementT)) {
      return absl::InvalidArgumentError("Tuple elements number doesn't match!");
    }
    absl::Status status = absl::OkStatus();
    ApplyIndex<sizeof...(ElementT)>([&](auto... I) {
      (void)((status.Update(ElementT::ValidateCorpusValue(
                  std::get<std::vector<ASTNode>>(astnode.children)[I])),
              status.ok()) &&
             ...);
    });
    return status;
  }
};

template <ASTTypeId id, int fallback_index, typename... ElementT>
class VariantDomain {
 public:
  static ASTTypeId TypeId() { return id; }

  static ASTNode Init(absl::BitGenRef prng) {
    return InitWithBudget(prng, kMaxGenerationNum);
  }

  static ASTNode InitWithBudget(absl::BitGenRef prng, int generation_budget) {
    int choice = generation_budget <= 0
                     ? fallback_index
                     : absl::Uniform<int>(prng, 0, (sizeof...(ElementT)));
    ASTNode result{id, std::vector<ASTNode>()};
    Switch<sizeof...(ElementT)>(choice, [&](auto I) {
      std::get<std::vector<ASTNode>>(result.children)
          .push_back(std::tuple_element<I, std::tuple<ElementT...>>::type::
                         InitWithBudget(prng, generation_budget));
    });
    return result;
  }

  static void Mutate(ASTNode& val, absl::BitGenRef prng,
                     const domain_implementor::MutationMetadata& metadata,
                     bool only_shrink) {
    constexpr bool has_alternative = sizeof...(ElementT) > 1;
    ASTNode current_value =
        std::get<std::vector<ASTNode>>(val.children).front();
    ASTTypeId current_value_id = current_value.type_id;
    bool is_current_value_mutable;
    ((ElementT::TypeId() == current_value_id
          ? (is_current_value_mutable = ElementT::IsMutable(current_value),
             (void)0)
          : (void)0),
     ...);

    FUZZTEST_INTERNAL_CHECK(is_current_value_mutable || has_alternative,
                            "Impossible at" + std::to_string(id));
    if (only_shrink) {
      if (is_current_value_mutable) {
        MutateCurrentValue(val, prng, metadata, only_shrink);
      }
      return;
    }

    if (!is_current_value_mutable) {
      SwitchToAlternative(val, prng);
    } else if (!has_alternative) {
      MutateCurrentValue(val, prng, metadata, only_shrink);
    } else {
      if (absl::Bernoulli(prng, 0.5)) {
        MutateCurrentValue(val, prng, metadata, only_shrink);
      } else {
        SwitchToAlternative(val, prng);
      }
    }
  }

  static void ToString(std::string& output, const ASTNode& val) {
    FUZZTEST_INTERNAL_CHECK(
        std::get<std::vector<ASTNode>>(val.children).size() == 1,
        "This is not a variant ast node.");
    auto child = std::get<std::vector<ASTNode>>(val.children).front();
    ((ElementT::TypeId() == child.type_id ? (ElementT::ToString(output, child))
                                          : (void)0),
     ...);
  }

  static bool IsMutable(const ASTNode& val) {
    // If the variant has at least two choices, it is always mutable.
    if (sizeof...(ElementT) > 1) return true;

    // Otherwise, we check whether the only choice is mutable.
    bool result = false;
    auto child = std::get<std::vector<ASTNode>>(val.children).front();
    ((ElementT::TypeId() == child.type_id
          ? (result = ElementT::IsMutable(child), (void)0)
          : (void)0),
     ...);
    return result;
  }

  static IRObject SerializeCorpus(const ASTNode& astnode) {
    FUZZTEST_INTERNAL_CHECK(
        CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id),
        "Invalid node!");

    ASTTypeId child_id =
        std::get<std::vector<ASTNode>>(astnode.children).front().type_id;
    IRObject expansion_obj;
    auto& inner_subs = expansion_obj.MutableSubs();
    ((ElementT::TypeId() == child_id
          ? inner_subs.push_back(ElementT::SerializeCorpus(
                std::get<std::vector<ASTNode>>(astnode.children).front()))
          : (void)0),
     ...);
    return WrapASTIntoIRObject(astnode, expansion_obj);
  }

  static std::optional<ASTNode> ParseCorpus(const IRObject& obj) {
    if (!CheckASTCorpusStructure<std::vector<ASTNode>>(obj)) {
      return std::nullopt;
    }
    auto subs = obj.Subs();
    auto type_id = (*subs)[0].ToCorpus<ASTTypeId>();

    auto children = (*subs)[2].Subs();
    if (!children || children->size() != 1) {
      return std::nullopt;
    }
    ASTNode result;
    result.type_id = *type_id;

    std::optional<ASTNode> child_node;
    ((child_node.has_value()
          ? (void)0
          : (child_node = ElementT::ParseCorpus(children->front()), (void)0)),
     ...);
    if (!child_node.has_value()) {
      return std::nullopt;
    }
    result.children.emplace<std::vector<ASTNode>>({std::move(*child_node)});
    return result;
  }

  static absl::Status ValidateCorpusValue(const ASTNode& astnode) {
    if (!CheckASTNodeTypeIdAndChildType<std::vector<ASTNode>>(astnode, id)) {
      return absl::InvalidArgumentError("Invalid node type!");
    }
    if (std::get<std::vector<ASTNode>>(astnode.children).size() != 1) {
      return absl::InvalidArgumentError("This is not a variant ast node.");
    }
    absl::Status status = absl::OkStatus();
    auto child = std::get<std::vector<ASTNode>>(astnode.children).front();
    (void)((ElementT::TypeId() == child.type_id
                ? (status.Update(ElementT::ValidateCorpusValue(child)),
                   status.ok())
                : true) &&
           ...);
    return status;
  }

 private:
  static void MutateCurrentValue(
      ASTNode& val, absl::BitGenRef prng,
      const domain_implementor::MutationMetadata& metadata, bool only_shrink) {
    ASTNode& current_value =
        std::get<std::vector<ASTNode>>(val.children).front();
    ((ElementT::TypeId() == current_value.type_id
          ? (ElementT::Mutate(current_value, prng, metadata, only_shrink))
          : (void)0),
     ...);
  }
  static void SwitchToAlternative(ASTNode& val, absl::BitGenRef prng) {
    constexpr int n_alternative = sizeof...(ElementT);
    FUZZTEST_INTERNAL_CHECK(n_alternative > 1, "No alternative to switch!");
    int child_type_id =
        std::get<std::vector<ASTNode>>(val.children).front().type_id;
    int current_choice = 0;
    ApplyIndex<n_alternative>([&](auto... I) {
      ((ElementT::TypeId() == child_type_id ? (current_choice = I, (void)0)
                                            : (void)0),
       ...);
    });
    int choice = current_choice;
    while (choice == current_choice) {
      choice = absl::Uniform<int>(prng, 0, n_alternative);
    }

    // Switch to an alternative value.
    ApplyIndex<n_alternative>([&](auto... I) {
      ((choice == I ? (std::get<std::vector<ASTNode>>(val.children).front() =
                           ElementT::InitWithBudget(prng, kMaxGenerationNum),
                       (void)0)
                    : (void)0),
       ...);
    });
  }
};

void GroupElementByASTType(
    ASTNode& astnode,
    absl::flat_hash_map<ASTTypeId, std::vector<ASTNode*>>& groups);

template <typename TopDomain>
class InGrammarImpl
    : public domain_implementor::DomainBase<InGrammarImpl<TopDomain>,
                                            std::string, ASTNode> {
 public:
  using typename InGrammarImpl::DomainBase::corpus_type;
  using typename InGrammarImpl::DomainBase::value_type;

  ASTNode Init(absl::BitGenRef prng) {
    if (auto seed = this->MaybeGetRandomSeed(prng)) return *seed;
    return TopDomain::Init(prng);
  }

  void Mutate(ASTNode& val, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    if (only_shrink && absl::Bernoulli(prng, 0.5) &&
        ShrinkByReplaceWithSubElementOfSameType(val, prng)) {
      return;
    }
    TopDomain::Mutate(val, prng, metadata, only_shrink);
  }

  auto GetPrinter() const { return StringPrinter{}; }

  value_type GetValue(const corpus_type& v) const {
    std::string result;
    TopDomain::ToString(result, v);
    return result;
  }

  std::optional<corpus_type> FromValue(const value_type& /*v*/) const {
    FUZZTEST_INTERNAL_CHECK(false, "Parsing is not implemented yet!");
    return std::nullopt;
  }

  IRObject SerializeCorpus(const corpus_type& astnode) const {
    return TopDomain::SerializeCorpus(astnode);
  }

  std::optional<corpus_type> ParseCorpus(const IRObject& obj) const {
    return TopDomain::ParseCorpus(obj);
  }

  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    // Validation is currently done during Parsing, and UserToCorpusValue() is
    // not supported yet.
    return TopDomain::ValidateCorpusValue(corpus_value);
  }

 private:
  bool ShrinkByReplaceWithSubElementOfSameType(ASTNode& astnode,
                                               absl::BitGenRef prng) {
    absl::flat_hash_map<ASTTypeId, std::vector<ASTNode*>> groups;
    GroupElementByASTType(astnode, groups);
    std::vector<ASTTypeId> candidate_types;
    for (const auto& iter : groups) {
      if (iter.second.size() > 1) {
        candidate_types.push_back(iter.first);
      }
    }
    if (candidate_types.empty()) {
      return false;
    }

    std::vector<ASTNode*>& candidates =
        groups[*ChoosePosition(candidate_types, IncludeEnd::kNo, prng)];
    size_t dst_index = absl::Uniform<size_t>(prng, 0, candidates.size() - 1);
    size_t src_index =
        absl::Uniform<size_t>(prng, dst_index + 1, candidates.size());

    FUZZTEST_INTERNAL_CHECK(src_index < candidates.size(), "Out of bound!");
    FUZZTEST_INTERNAL_CHECK(dst_index < candidates.size(), "Out of bound!");
    if (candidates[dst_index]->NodeCount() <
        candidates[src_index]->NodeCount()) {
      std::swap(dst_index, src_index);
    }

    // This copy is necessary to avoid `self assignment`, since the src ast node
    // might be a sub node within the dst ast node. If we write it as
    // `*candidates[dst_index] = *candidates[src_index]` and the src is a sub
    // node in the dst, according to
    // (https://en.cppreference.com/w/cpp/utility/variant/operator%3D), the
    // variant in the dst will be destroyed first. This also invalidates the src
    // before its value is used, resulting a crash when the value is used later.
    auto val = *candidates[src_index];
    *candidates[dst_index] = val;
    return true;
  }
};
}  // namespace fuzztest::internal::grammar

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_IN_GRAMMAR_IMPL_H_
