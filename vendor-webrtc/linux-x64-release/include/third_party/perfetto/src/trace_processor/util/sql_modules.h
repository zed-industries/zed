/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_SQL_MODULES_H_
#define SRC_TRACE_PROCESSOR_UTIL_SQL_MODULES_H_

#include <cstddef>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"

namespace perfetto ::trace_processor::sql_modules {

using NameToPackage =
    base::FlatHashMap<std::string,
                      std::vector<std::pair<std::string, std::string>>>;

// Map from include key to sql file. Include key is the string used in INCLUDE
// function.
struct RegisteredPackage {
  struct ModuleFile {
    std::string sql;
    bool included;
  };
  base::FlatHashMap<std::string, ModuleFile> modules;
};

inline std::string ReplaceSlashWithDot(std::string str) {
  size_t found = str.find('/');
  while (found != std::string::npos) {
    str.replace(found, 1, ".");
    found = str.find('/');
  }
  return str;
}

inline std::string GetIncludeKey(const std::string& path) {
  base::StringView path_view(path);
  auto path_no_extension = path_view.substr(0, path_view.rfind('.'));
  return ReplaceSlashWithDot(path_no_extension.ToStdString());
}

inline std::string GetPackageName(const std::string& str) {
  size_t found = str.find('.');
  if (found == std::string::npos) {
    return str;
  }
  return str.substr(0, found);
}

}  // namespace perfetto::trace_processor::sql_modules
#endif  // SRC_TRACE_PROCESSOR_UTIL_SQL_MODULES_H_
