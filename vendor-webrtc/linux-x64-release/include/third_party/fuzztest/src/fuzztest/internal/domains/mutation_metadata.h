// Copyright 2024 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_MUTATION_METADATA_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_MUTATION_METADATA_H_

#include "./fuzztest/internal/table_of_recent_compares.h"

namespace fuzztest::domain_implementor {

// Opaque pointer to comparison tables used by builtin domains, imported here
// for domain implementors who do not need the internal header.
using ConstCmpTablesPtr = const internal::TablesOfRecentCompares*;

// A struct holding metadata for domain mutation.
struct MutationMetadata {
  ConstCmpTablesPtr cmp_tables = nullptr;
};

}  //  namespace fuzztest::domain_implementor

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_MUTATION_METADATA_H_
