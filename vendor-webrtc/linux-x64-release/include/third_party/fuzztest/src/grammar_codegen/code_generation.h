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

#ifndef FUZZTEST_GRAMMAR_CODEGEN_CODE_GENERATION_H_
#define FUZZTEST_GRAMMAR_CODEGEN_CODE_GENERATION_H_

#include <optional>
#include <string>
#include <vector>

#include "./grammar_codegen/antlr_frontend.h"
#include "./grammar_codegen/backend.h"

namespace fuzztest::internal::grammar {

// Generate the code given the input grammar files. This simply combine the
// process of building grammar information from grammar files and generating
// code from grammar information.
std::string GenerateGrammarHeader(
    const std::vector<std::string>& input_grammar_specs,
    std::optional<std::string> grammar_name = std::nullopt,
    bool insert_space_between_blocks = false);

}  // namespace fuzztest::internal::grammar

#endif  // FUZZTEST_GRAMMAR_CODEGEN_CODE_GENERATION_H_
