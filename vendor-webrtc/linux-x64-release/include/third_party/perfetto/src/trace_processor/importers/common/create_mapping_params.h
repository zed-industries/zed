/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CREATE_MAPPING_PARAMS_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CREATE_MAPPING_PARAMS_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <tuple>

#include "perfetto/ext/base/hash.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto {
namespace trace_processor {

struct CreateMappingParams {
  AddressRange memory_range;
  // This is the offset into the file that has been mapped at
  // memory_range.start()
  uint64_t exact_offset = 0;
  // This is the offset into the file where the ELF header starts. We assume
  // all file mappings are ELF files an thus this offset is 0.
  uint64_t start_offset = 0;
  // This can only be read out of the actual ELF file.
  uint64_t load_bias = 0;
  std::string name;
  std::optional<BuildId> build_id;

  auto ToTuple() const {
    return std::tie(memory_range, exact_offset, start_offset, load_bias, name,
                    build_id);
  }

  bool operator==(const CreateMappingParams& o) const {
    return ToTuple() == o.ToTuple();
  }

  struct Hasher {
    size_t operator()(const CreateMappingParams& p) const {
      base::Hasher h;
      h.UpdateAll(p.memory_range.start(), p.memory_range.end(), p.exact_offset,
                  p.start_offset, p.load_bias, p.name);
      if (p.build_id) {
        h.Update(*p.build_id);
      }
      return static_cast<size_t>(h.digest());
    }
  };
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CREATE_MAPPING_PARAMS_H_
