/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_GENERATOR_H_
#define SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_GENERATOR_H_

#include <stddef.h>
#include <stdint.h>

#include <string>
#include <vector>

namespace protozero {

// Creates a filter bytecode that can be passed to the FilterBytecodeParser.
// This class is typically only used by offline tools (e.g. the proto_filter
// cmdline tool). See go/trace-filtering for the full filtering design.
class FilterBytecodeGenerator {
 public:
  FilterBytecodeGenerator();
  ~FilterBytecodeGenerator();

  // Call at the end of every message. It implicitly starts a new message, there
  // is no corresponding BeginMessage().
  void EndMessage();

  // All the methods below must be called in monotonic field_id order or the
  // generator will CHECK() and crash.

  // Allows a simple field (varint, fixed32/64, string or bytes).
  void AddSimpleField(uint32_t field_id);

  // Allows a string field which needs to be filtered.
  void AddFilterStringField(uint32_t field_id);

  // Allows a range of simple fields. |range_start| is the id of the first field
  // in range, |range_len| the number of fields in the range.
  // AddSimpleFieldRange(N,1) is semantically equivalent to AddSimpleField(N)
  // (but it takes 2 words to encode, rather than just one).
  void AddSimpleFieldRange(uint32_t range_start, uint32_t range_len);

  // Adds a nested field. |message_index| is the index of the message that the
  // parser must recurse into. This implies that at least |message_index| calls
  // to Begin/EndMessage will be made.
  // The Serialize() method will fail if any field points to an index that is
  // out of range (e.g., if message_index = 5 but only 3 EndMessage() calls were
  // made).
  void AddNestedField(uint32_t field_id, uint32_t message_index);

  // Returns the filter bytecode, which is a buffer containing a sequence of
  // varints and a checksum. The returned string can be passed to
  // FilterBytecodeParser.Load().
  std::string Serialize();

 private:
  uint32_t num_messages_ = 0;
  uint32_t last_field_id_ = 0;
  uint32_t max_msg_index_ = 0;
  bool endmessage_called_ = false;

  std::vector<uint32_t> bytecode_;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_GENERATOR_H_
