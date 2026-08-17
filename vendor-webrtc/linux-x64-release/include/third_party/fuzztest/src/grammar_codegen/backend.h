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

#ifndef FUZZTEST_GRAMMAR_CODEGEN_BACKEND_H_
#define FUZZTEST_GRAMMAR_CODEGEN_BACKEND_H_

#include <string>
#include <vector>

#include "absl/container/btree_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "./grammar_codegen/grammar_info.h"

namespace fuzztest::internal::grammar {

class CodeGenerator {
 public:
  // The constructor accepts the information collected by the parser.
  CodeGenerator(Grammar grammar) : grammar_(std::move(grammar)) {}
  // Return the generated code as a string. The generated code works as a header
  // and contains the definition of the ast nodes for a language.
  std::string Generate();

  // Simplify the grammar so that the elements of Vector/Tuple/Variant are
  // either terminals or non-terminals.
  // For example, A: (C D)* will be converted to: A: N*, N: C D. So that we can
  // say A is a vector of N, and N is a tuple of C, D.
  void Preprocess(Grammar& grammar);

 private:
  std::string BuildClassDefinitionForSymbol(GrammarRule& rule);
  std::string BuilldClassDefinitionForCharSet(absl::string_view class_name);
  std::string BuildClassDefinitionForLiteral(absl::string_view class_name);
  std::string BuildBaseTypeForGrammarRule(const GrammarRule& rule);

  // Caculate the fallback indexes for all the symbols (including
  // sub-productions). During generation, if every grammar rule chooses the
  // fallback index during generation, generation will guarantee to end.
  void CalculateFallBackIndex(std::vector<GrammarRule>& rules);

  bool IsSymbolSafe(absl::string_view symbol);
  void MarkSymbolAsSafe(absl::string_view symbol);

  bool HasSafeRange(const Block& block);

  bool TryMarkProductionRuleVecAsSafe(ProductionWithFallbackIndex& productions);
  bool TryMarkBlockAsSafe(Block& block);
  bool TryMarkProductionRuleAsSafe(ProductionRule& prod_rule);
  bool TryMarkGrammarRuleAsSafe(GrammarRule& rule);

  // Get the name of the generated class for the block.
  std::string GetClassName(const Block& block);
  std::string GetClassNameForSymbol(const std::string id);
  std::string GetClassNameForLiteral(absl::string_view s);
  std::string GetClassNameForCharSet(absl::string_view s);

  Grammar grammar_;
  absl::flat_hash_set<std::string> safe_rules_;

  // Use ordered map so that the generated code has a stable order.
  absl::btree_map<std::string, std::string> literal_node_ids_;
  absl::btree_map<std::string, std::string> charset_node_ids_;
};
}  // namespace fuzztest::internal::grammar

#endif  // FUZZTEST_GRAMMAR_CODEGEN_BACKEND_H_
