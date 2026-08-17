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

#ifndef SRC_TRACE_PROCESSOR_UTIL_PROTO_PROFILER_H_
#define SRC_TRACE_PROCESSOR_UTIL_PROTO_PROFILER_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/protozero/field.h"
#include "perfetto/protozero/proto_decoder.h"
#include "src/trace_processor/util/descriptors.h"

namespace perfetto::trace_processor::util {

class SizeProfileComputer {
 public:
  struct Field {
    Field(uint32_t field_idx_in,
          const FieldDescriptor* field_descriptor_in,
          uint32_t type_in,
          const ProtoDescriptor* proto_descriptor_in);

    bool has_field_name() const {
      return field_descriptor || field_idx == static_cast<uint32_t>(-1);
    }

    std::string field_name() const;
    std::string type_name() const;

    bool operator==(const Field& other) const {
      return field_idx == other.field_idx && type == other.type;
    }

    uint32_t field_idx;
    uint32_t type;
    const FieldDescriptor* field_descriptor;
    const ProtoDescriptor* proto_descriptor;
  };

  using FieldPath = std::vector<Field>;
  struct FieldPathHasher {
    using argument_type = FieldPath;
    using result_type = size_t;

    result_type operator()(const argument_type& p) const {
      size_t h = 0u;
      for (auto v : p) {
        h += (std::hash<uint32_t>{}(v.field_idx) +
              std::hash<uint32_t>{}(v.type));
        h = (h << 5) - h;
      }
      return h;
    }
  };

  explicit SizeProfileComputer(DescriptorPool* pool,
                               const std::string& message_type);

  // Re-initializes the computer to iterate over samples (i.e. all encountered
  // field sizes) for each field path in trace proto contained in the given
  // range.
  // TODO(kraskevich): consider switching to internal DescriptorPool.
  void Reset(const uint8_t* ptr, size_t size);

  // Returns the next sample size, or std::nullopt if data is exhausted. The
  // associated path can be queried with GetPath().
  std::optional<size_t> GetNext();

  // Returns the field path associated with the last sample returned by
  // GetNext().
  const FieldPath& GetPath() const { return field_path_; }

  operator bool() const;

 private:
  static size_t GetFieldSize(const protozero::Field& f);

  DescriptorPool* pool_;
  uint32_t root_message_idx_;
  // The current 'stack' we're considering as we parse the protobuf.
  // For example if we're currently looking at the varint field baz which is
  // nested inside message Bar which is in turn a field named bar on the message
  // Foo. Then the stack would be: Foo, #bar, Bar, #baz, int
  // We keep track of both the field names (#bar, #baz) and the field types
  // (Foo, Bar, int) as sometimes we are interested in which fields are big
  // and sometimes which types are big.
  FieldPath field_path_;

  // Internal state used to iterate over field path.
  struct State {
    const ProtoDescriptor* descriptor;
    protozero::ProtoDecoder decoder;
    size_t overhead;
    size_t unknown;
  };
  std::vector<State> state_stack_;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_PROTO_PROFILER_H_
