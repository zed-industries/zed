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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_METADATA_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_METADATA_TRACKER_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <optional>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/storage/metadata.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

// Tracks information in the metadata table.
class MetadataTracker {
 public:
  explicit MetadataTracker(TraceStorage* storage);

  // Example usage:
  // SetMetadata(metadata::benchmark_name,
  //             Variadic::String(storage->InternString("foo"));
  // Returns the id of the new entry.
  MetadataId SetMetadata(metadata::KeyId key, Variadic value);

  // Example usage:
  // AppendMetadata(metadata::benchmark_story_tags,
  //                Variadic::String(storage->InternString("bar"));
  // Returns the id of the new entry.
  MetadataId AppendMetadata(metadata::KeyId key, Variadic value);

  // Sets a metadata entry using any interned string as key.
  // Returns the id of the new entry.
  MetadataId SetDynamicMetadata(StringId key, Variadic value);

  // Reads back a set metadata value.
  // Only kSingle types are supported right now.
  std::optional<SqlValue> GetMetadata(metadata::KeyId key);

  // Tracks how many ChromeMetadata bundles have been parsed.
  uint32_t IncrementChromeMetadataBundleCount() {
    return ++chrome_metadata_bundle_count_;
  }

 private:
  static constexpr size_t kNumKeys =
      static_cast<size_t>(metadata::KeyId::kNumKeys);
  static constexpr size_t kNumKeyTypes =
      static_cast<size_t>(metadata::KeyType::kNumKeyTypes);

  void WriteValue(uint32_t row, Variadic value);

  std::array<StringId, kNumKeys> key_ids_;
  std::array<StringId, kNumKeyTypes> key_type_ids_;
  uint32_t chrome_metadata_bundle_count_ = 0;

  TraceStorage* storage_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_METADATA_TRACKER_H_
