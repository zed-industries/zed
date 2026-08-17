// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_BINARY_INFO_H_
#define THIRD_PARTY_CENTIPEDE_BINARY_INFO_H_

#include <string_view>

#include "./centipede/call_graph.h"
#include "./centipede/control_flow.h"
#include "./centipede/pc_info.h"
#include "./centipede/symbol_table.h"

namespace fuzztest::internal {

// Information about the binary being fuzzed. Created once at program startup
// and doesn't change (other than for lazily initialized fields).
struct BinaryInfo {
  PCTable pc_table;
  SymbolTable symbols;
  CFTable cf_table;
  DsoTable dso_table;
  ControlFlowGraph control_flow_graph;
  CallGraph call_graph;
  bool uses_legacy_trace_pc_instrumentation = false;

  // Initializes `pc_table`, `symbols`, `cf_table` and
  // `uses_legacy_trace_pc_instrumentation` based on `binary_path_with_args`.
  // * `binary_path_with_args` is the path to the instrumented binary,
  // possibly with space-separated arguments.
  // * `objdump_path` and `symbolizer_path` are paths to respective tools.
  // * `tmp_dir_path` is a path to a temp dir, that must exist.
  void InitializeFromSanCovBinary(std::string_view binary_path_with_args,
                                  std::string_view objdump_path,
                                  std::string_view symbolizer_path,
                                  std::string_view tmp_dir_path);

  // Serialize `this` within the given `dir`.
  void Write(std::string_view dir);

  // Initialize `this` with the serialized contents in `dir`. Assumes the same
  // format as `Write`.
  void Read(std::string_view dir);
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_BINARY_INFO_H_
