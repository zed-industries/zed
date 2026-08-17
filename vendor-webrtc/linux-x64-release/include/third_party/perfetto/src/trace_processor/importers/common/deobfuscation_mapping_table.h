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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_DEOBFUSCATION_MAPPING_TABLE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_DEOBFUSCATION_MAPPING_TABLE_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto {
namespace trace_processor {

// Constains deobfuscation for Java class names and its members per |PackageId|.
class DeobfuscationMappingTable {
 public:
  struct PackageId {
    std::string package_name;
    int64_t version_code;

    bool operator==(const PackageId& other) const {
      return package_name == other.package_name &&
             version_code == other.version_code;
    }
  };

  // For the given |package| and |obfuscated_class_name| adds translations of
  // the class and its members.
  //
  // Returns `true` if the translation for the given class
  // was inserted, `false` if there is already a translation for the given
  // class.
  bool AddClassTranslation(
      const PackageId& package,
      StringId obfuscated_class_name,
      StringId deobfuscated_class_name,
      base::FlatHashMap<StringId, StringId> obfuscated_to_deobfuscated_members);

  // These functions return the deobfuscated class/member name from an
  // obfuscated class/member name.
  // If a package is not provided, the |default_package_id_| is used.
  // If translation is not found, returns std::nullopt.

  std::optional<StringId> TranslateClass(StringId obfuscated_class_name) const;

  std::optional<StringId> TranslateClass(const PackageId& package,
                                         StringId obfuscated_class_name) const;

  std::optional<StringId> TranslateMember(const PackageId& package,
                                          StringId obfuscated_class_name,
                                          StringId obfuscated_member) const;

 private:
  struct PackageIdHash {
    std::size_t operator()(PackageId const& p) const noexcept {
      return static_cast<std::size_t>(
          base::Hasher::Combine(p.package_name, p.version_code));
    }
  };

  using ObfuscatedToDeobfuscatedMembers = base::FlatHashMap<StringId, StringId>;
  struct ClassTranslation {
    StringId deobfuscated_class_name;
    ObfuscatedToDeobfuscatedMembers members;
  };

  using ObfuscatedClassesToMembers =
      base::FlatHashMap<StringId, ClassTranslation>;
  base::FlatHashMap<PackageId, ObfuscatedClassesToMembers, PackageIdHash>
      class_per_package_translation_;

  // To translate entities which don't have a package id, we will use
  // |default_package_id_|. |default_package_id_| is a package id of the first
  // inserted entity with a package id;
  // We need this because currently TraceProcessor doesn't use the package
  // version of the arguments.
  // TODO(b/244700870): start use the package version of arguments.
  std::optional<PackageId> default_package_id_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_DEOBFUSCATION_MAPPING_TABLE_H_
