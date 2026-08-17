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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_MAPPING_VERSION_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_MAPPING_VERSION_H_

#include <cstddef>
#include <cstdint>

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor::etm {

class MappingVersion {
 public:
  MappingVersion(MappingId id,
                 int64_t create_ts,
                 AddressRange range,
                 std::optional<TraceBlobView> content);

  bool Contains(uint64_t address) const { return range_.Contains(address); }
  bool Contains(const AddressRange& range) const {
    return range_.Contains(range);
  }
  uint64_t start() const { return range_.start(); }
  uint64_t end() const { return range_.end(); }
  uint64_t length() const { return range_.length(); }
  int64_t create_ts() const { return create_ts_; }
  MappingId id() const { return id_; }
  // Returns a valid pointer if data is available or nullptr otherwise
  const uint8_t* data() const {
    // We make sure in the constructor that if content_ length is 0 the pointer
    // will be null.
    return content_.data();
  }

  MappingVersion SplitFront(uint64_t mid);

 private:
  MappingId id_;
  int64_t create_ts_;
  AddressRange range_;
  // Content is either empty and points to nullptr, or has the same length as
  // `range_`. This is CHECKED in the constructor.
  TraceBlobView content_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_MAPPING_VERSION_H_
