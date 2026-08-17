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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ELF_BINARY_INFO_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ELF_BINARY_INFO_H_

#include <cstdint>
#include <optional>
#include <string>

namespace perfetto::trace_processor::elf {

enum BinaryType {
  kElf,
  kMachO,
  kMachODsym,
};

struct BinaryInfo {
  std::optional<std::string> build_id;
  uint64_t load_bias;
  BinaryType type;
};

bool IsElf(const uint8_t* mem, size_t size);

std::optional<BinaryInfo> GetBinaryInfo(const uint8_t* mem, size_t size);

}  // namespace perfetto::trace_processor::elf

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ELF_BINARY_INFO_H_
