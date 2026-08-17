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

#ifndef FUZZTEST_GRAMMAR_CODEGEN_GRAMMAR_INFO_H_
#define FUZZTEST_GRAMMAR_CODEGEN_GRAMMAR_INFO_H_

#include <optional>
#include <string>
#include <variant>
#include <vector>

namespace fuzztest::internal::grammar {

// A symbol can be quantified by a range (?, +, *). `?` means the symbol is
// optional, `+` means at least one, and `*` means arbitrary number.
enum class Range {
  kNoRange,
  kOptional,   // [0, 1]
  kNonEmpty,   // [1, +infinity]
  kUnlimited,  // [0, +infinity]
};

// A terminal is either a string literal or a charset (regex). A lexer symbol
// (Token) doesn't have to be a terminal. For example, we treat the lexer symbol
// `LoopStatement: "break" | "continue"` as non-terminal because it expands to
// other symbols.
enum class TerminalType {
  kStringLiteral,
  kCharSet,
};

struct Terminal {
  TerminalType type;
  std::string content;
};

// A non-terminal is a symbol in the grammar file.
struct NonTerminal {
  std::string name;
};

struct ProductionRule;

// A list of production rules. It can be the expansion rules for a symbol or a
// block in one production rule. The fallback index is for finite string
// generation: if we choose the production of fallback index to expand every
// symbol, then the result string must be finite.
struct ProductionWithFallbackIndex {
  std::optional<int> fallback_index;
  std::vector<ProductionRule> production_rules;
};

// A block is the basic unit that constructs a production rule. It can be a
// terminal, non-terminal or production rules. For example, for the grammar rule
// `A : "a" | B | (C | D)*`, each production rule of `A` consists of one block.
// "a" is a terminal, `B` is a non-terminal, `(C | D)*` is a set of
// sub-production rules.
enum BlockType {
  kTerminal = 0,
  kNonTerminal = 1,
  kSubProductions = 2,
};

struct Block {
  Range range = Range::kNoRange;
  std::variant<Terminal, NonTerminal, ProductionWithFallbackIndex> element;
};

// A production is a tuple of blocks that a symbol can expand to.
struct ProductionRule {
  std::vector<Block> blocks;
};

// Decribes the rule of productions for a symbol.
struct GrammarRule {
  std::string symbol_name;
  ProductionWithFallbackIndex productions;
};

// All the information about the grammar of a language.
struct Grammar {
  std::string grammar_name;
  std::vector<GrammarRule> rules;
};
}  // namespace fuzztest::internal::grammar
#endif  // FUZZTEST_GRAMMAR_CODEGEN_GRAMMAR_INFO_H_
