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

#ifndef THIRD_PARTY_CENTIPEDE_SYMBOL_TABLE_H_
#define THIRD_PARTY_CENTIPEDE_SYMBOL_TABLE_H_

#include <cstddef>
#include <istream>
#include <ostream>
#include <string>
#include <string_view>
#include <vector>

#include "absl/container/node_hash_set.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/types/span.h"
#include "./centipede/control_flow.h"
#include "./centipede/pc_info.h"

namespace fuzztest::internal {

// Maps integer indices in [0, size) to debug symbols:
// function names, file names, line numbers, column numbers.
class SymbolTable {
 public:
  // Defines a symbol table entry.
  struct Entry {
    std::string_view func;
    std::string_view file;
    int line = -1;
    int col = -1;

    std::string file_line_col() const {
      const bool file_known = !absl::StrContains(file, '?');
      const std::string line_str =
          (file_known && line >= 0) ? absl::StrCat(":", line) : "";
      const std::string col_str =
          (file_known && col >= 0) ? absl::StrCat(":", col) : "";
      return absl::StrCat(file, line_str, col_str);
    }

    friend bool operator==(const Entry &lhs, const Entry &rhs) {
      return lhs.func == rhs.func && lhs.file == rhs.file &&
             lhs.line == rhs.line && lhs.col == rhs.col;
    }
  };

  SymbolTable() = default;
  SymbolTable(const SymbolTable &) = delete;
  SymbolTable(SymbolTable &&) = default;
  SymbolTable &operator=(SymbolTable &&) = default;

  friend bool operator==(const SymbolTable &a, const SymbolTable &b) {
    return a.entries_ == b.entries_;
  }

  // Reads the symbols from a stream produced by `llvm-symbolizer --no-inlines`.
  // https://llvm.org/docs/CommandGuide/llvm-symbolizer.html.
  // The input consists of tuples of 3 lines each:
  //   FunctionName
  //   SourceCodeLocation
  //   <empty line>
  // If called multiple times, this function will append symbols to `this`.
  void ReadFromLLVMSymbolizer(std::istream &in);

  // Writes the contents of `this` to `path` in the same format as read by
  // `ReadFromLLVMSymbolizer`.
  void WriteToLLVMSymbolizer(std::ostream &out);

  // Invokes `symbolizer_path --no-inlines` on all binaries from `dso_table`,
  // pipes through it all the PCs in `pc_table` that correspond to each of the
  // binaries, and calls `ReadFromLLVMSymbolizer()` on the output.
  // Possibly uses files `pcs_tmp_path` and `symbols_tmp_path` for temporary
  // storage.
  void GetSymbolsFromBinary(const PCTable &pc_table, const DsoTable &dso_table,
                            std::string_view symbolizer_path,
                            std::string_view tmp_dir_path);

  // Helper for `GetSymbolsFromBinary()`: symbolizes `pc_infos` for `dso_path`.
  // Possibly uses files `pcs_tmp_path` and `symbols_tmp_path` for temporary
  // storage.
  void GetSymbolsFromOneDso(absl::Span<const PCInfo> pc_infos,
                            std::string_view dso_path,
                            std::string_view symbolizer_path,
                            std::string_view tmp_dir_path);

  // Sets the table to `size` symbols all of which are unknown.
  void SetAllToUnknown(size_t size);

  // Returns the number of symbol entries.
  size_t size() const { return entries_.size(); }

  // Returns the function name for idx-th entry.
  std::string_view func(size_t idx) const { return entries_[idx].func; }

  Entry entry(size_t idx) const { return entries_[idx]; }

  // Returns source code location for idx-th entry,
  std::string location(size_t idx) const {
    return entries_[idx].file_line_col();
  }

  // Returns a full human-readable description for idx-th entry.
  std::string full_description(size_t idx) const {
    return std::string{func(idx)} + " " + location(idx);
  }

  // Add function name and file location to symbol table.
  void AddEntry(std::string_view func, std::string_view file_line_col);

  // Add all the entries from the other symbol table into this one.
  void AddEntries(const SymbolTable &other);

 private:
  void AddEntryInternal(std::string_view func, std::string_view file,
                        int line = -1, int col = -1);

  std::string_view GetOrInsert(std::string_view str);

  // Declaration order matters here, because we want `table_` to be deleted last
  // in order to avoid having dangling ptrs in `entries_`.

  // Holds the strings for files and function names of the stored symbols. This
  // avoids storing duplicate values. `node_hash_set` was chosen in order to
  // have pointer stability and not bother with storing strings in `unique_ptr`.
  absl::node_hash_set<std::string> table_;

  // Holds the the symbol entries.
  std::vector<Entry> entries_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_SYMBOL_TABLE_H_
