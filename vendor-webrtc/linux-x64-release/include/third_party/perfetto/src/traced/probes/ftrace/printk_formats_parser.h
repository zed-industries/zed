/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACED_PROBES_FTRACE_PRINTK_FORMATS_PARSER_H_
#define SRC_TRACED_PROBES_FTRACE_PRINTK_FORMATS_PARSER_H_

#include <string>

#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/string_view.h"

namespace perfetto {

struct PrintkEntry {
  uint64_t address;
  std::string name;

  PrintkEntry(uint64_t _address) : PrintkEntry(_address, "") {}

  PrintkEntry(uint64_t _address, std::string _name)
      : address(_address), name(_name) {}

  bool operator<(const PrintkEntry& other) const {
    return address < other.address;
  }

  bool operator==(const PrintkEntry& other) const {
    return address == other.address;
  }
};

class PrintkMap {
 public:
  void insert(uint64_t address, std::string name) {
    set_.insert(PrintkEntry(address, name));
  }

  base::StringView at(uint64_t address) const {
    auto it = set_.find(address);
    if (it == set_.end()) {
      return base::StringView();
    }
    return base::StringView(it->name);
  }

  size_t size() const { return set_.size(); }

  size_t empty() const { return set_.empty(); }

  base::FlatSet<PrintkEntry> set_;
};

PrintkMap ParsePrintkFormats(const std::string& format);

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_FTRACE_PRINTK_FORMATS_PARSER_H_
