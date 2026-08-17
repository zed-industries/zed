/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_PROFILING_DEOBFUSCATOR_H_
#define SRC_PROFILING_DEOBFUSCATOR_H_

#include <functional>
#include <map>
#include <string>
#include <utility>
#include <vector>
#include "perfetto/base/status.h"

namespace perfetto {
namespace profiling {

std::string FlattenClasses(
    const std::map<std::string, std::vector<std::string>>& m);

class ObfuscatedClass {
 public:
  explicit ObfuscatedClass(std::string d) : deobfuscated_name_(std::move(d)) {}
  ObfuscatedClass(
      std::string d,
      std::map<std::string, std::string> f,
      std::map<std::string, std::map<std::string, std::vector<std::string>>> m)
      : deobfuscated_name_(std::move(d)),
        deobfuscated_fields_(std::move(f)),
        deobfuscated_methods_(std::move(m)) {}

  const std::string& deobfuscated_name() const { return deobfuscated_name_; }

  const std::map<std::string, std::string>& deobfuscated_fields() const {
    return deobfuscated_fields_;
  }

  std::map<std::string, std::string> deobfuscated_methods() const {
    std::map<std::string, std::string> result;
    for (const auto& p : deobfuscated_methods_) {
      result.emplace(p.first, FlattenClasses(p.second));
    }
    return result;
  }

  bool redefined_methods() const { return redefined_methods_; }

  bool AddField(std::string obfuscated_name, std::string deobfuscated_name) {
    auto p = deobfuscated_fields_.emplace(std::move(obfuscated_name),
                                          deobfuscated_name);
    return p.second || p.first->second == deobfuscated_name;
  }

  void AddMethod(std::string obfuscated_name, std::string deobfuscated_name) {
    std::string cls = deobfuscated_name_;
    auto dot = deobfuscated_name.rfind('.');
    if (dot != std::string::npos) {
      cls = deobfuscated_name.substr(0, dot);
      deobfuscated_name = deobfuscated_name.substr(dot + 1);
    }
    auto& deobfuscated_names_for_cls =
        deobfuscated_methods_[std::move(obfuscated_name)][std::move(cls)];
    deobfuscated_names_for_cls.push_back(std::move(deobfuscated_name));
    if (deobfuscated_names_for_cls.size() > 1 ||
        deobfuscated_methods_.size() > 1) {
      redefined_methods_ = true;
    }
  }

 private:
  std::string deobfuscated_name_;
  std::map<std::string, std::string> deobfuscated_fields_;
  std::map<std::string, std::map<std::string, std::vector<std::string>>>
      deobfuscated_methods_;
  bool redefined_methods_ = false;
};

class ProguardParser {
 public:
  // A return value of false means this line failed to parse. This leaves the
  // parser in an undefined state and it should no longer be used.
  base::Status AddLine(std::string line);
  bool AddLines(std::string contents);

  std::map<std::string, ObfuscatedClass> ConsumeMapping() {
    return std::move(mapping_);
  }

 private:
  std::map<std::string, ObfuscatedClass> mapping_;
  ObfuscatedClass* current_class_ = nullptr;
};

struct ProguardMap {
  std::string package;
  std::string filename;
};

void MakeDeobfuscationPackets(
    const std::string& package_name,
    const std::map<std::string, profiling::ObfuscatedClass>& mapping,
    std::function<void(const std::string&)> callback);

std::vector<ProguardMap> GetPerfettoProguardMapPath();

bool ReadProguardMapsToDeobfuscationPackets(
    const std::vector<ProguardMap>& maps,
    std::function<void(std::string)> fn);

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_DEOBFUSCATOR_H_
