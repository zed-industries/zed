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

#ifndef SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_PARSER_H_
#define SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_PARSER_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <vector>

namespace protozero {

// Loads the proto-encoded bytecode in memory and allows fast lookups for tuples
// (msg_index, field_id) to tell if a given field should be allowed or not and,
// in the case of nested fields, what is the next message index to recurse into.
// This class does two things:
// 1. Expands the array of varint from the proto into a vector<uint32_t>. This
//    is to avoid performing varint decoding on every lookup, at the cost of
//    some extra memory (2KB-4KB). Note that the expanded vector is not just a
//    1:1 copy of the proto one (more below). This is to avoid O(Fields) linear
//    lookup complexity.
// 2. Creates an index of offsets to remember the start word for each message.
//    This is so we can jump to O(1) to the N-th message when recursing into a
//    nested fields, without having to scan and find the (N-1)-th END_OF_MESSAGE
//    marker.
// Overall lookups are O(1) for field ids < 128 (kDirectlyIndexLimit) and O(N),
// with N being the number of allowed field ranges for other fields.
// See comments around |word_| below for the structure of the word vector.
class FilterBytecodeParser {
 public:
  // Result of a Query() operation
  struct QueryResult {
    bool allowed;  // Whether the field is allowed at all or no.

    // If |allowed|==true && nested_msg_field() == true, this tells the message
    // index of the nested field that should be used when recursing in the
    // parser.
    uint32_t nested_msg_index;

    // If |allowed|==true, specifies if the field is of a simple type (varint,
    // fixed32/64, string or byte).
    bool simple_field() const { return nested_msg_index == kSimpleField; }

    // If |allowed|==true, specifies if this field is a string field that needs
    // to be filtered.
    bool filter_string_field() const {
      return nested_msg_index == kFilterStringField;
    }

    // If |allowed|==true, specifies if the field is a nested field that needs
    // recursion. The caller is expected to use |nested_msg_index| for the next
    // Query() calls.
    bool nested_msg_field() const {
      static_assert(kFilterStringField < kSimpleField,
                    "kFilterStringField < kSimpleField");
      return nested_msg_index < kFilterStringField;
    }
  };

  // Loads a filter. The filter data consists of a sequence of varints which
  // contains the filter opcodes and a final checksum.
  bool Load(const void* filter_data, size_t len);

  // Checks wheter a given field is allowed or not.
  // msg_index = 0 is the index of the root message, where all queries should
  // start from (typically perfetto.protos.Trace).
  QueryResult Query(uint32_t msg_index, uint32_t field_id) const;

  void Reset();
  void set_suppress_logs_for_fuzzer(bool x) { suppress_logs_for_fuzzer_ = x; }

 private:
  static constexpr uint32_t kDirectlyIndexLimit = 128;
  static constexpr uint32_t kAllowed = 1u << 31u;
  static constexpr uint32_t kSimpleField = 0x7fffffff;
  static constexpr uint32_t kFilterStringField = 0x7ffffffe;

  bool LoadInternal(const uint8_t* filter_data, size_t len);

  // The state of all fields for all messages is stored in one contiguous array.
  // This is to avoid memory fragmentation and allocator overhead.
  // We expect a high number of messages (hundreds), but each message is small.
  // For each message we store two sets of uint32:
  // 1. A set of "directly indexed" fields, for field ids < 128.
  // 2. The remainder is a set of ranges.
  // So each message descriptor consists of a sequence of words as follows:
  //
  // [0] -> how many directly indexed fields are stored next (up to 128)
  //
  // [1..N] -> One word per field id (See "field state" below).
  //
  // [N + 1] -> Start of field id range 1
  // [N + 2] -> End of field id range 1 (exclusive, STL-style).
  // [N + 3] -> Field state for fields in range 1 (below)
  //
  // [N + 4] -> Start of field id range 2
  // [N + 5] -> End of field id range 2 (exclusive, STL-style).
  // [N + 6] -> Field state for fields in range 2 (below)

  // The "field state" word is as follows:
  // Bit 31: 1 if the field is allowed, 0 if disallowed.
  //         Only directly indexed fields can be 0 (it doesn't make sense to add
  //         a range and then say "btw it's NOT allowed".. don't add it then.
  //         0 is only used for filling gaps in the directly indexed bucket.
  // Bits [30..0] (only when MSB == allowed):
  //  0x7fffffff: The field is "simple" (varint, fixed32/64, string, bytes) and
  //      can be directly passed through in output. No recursion is needed.
  //  0x7ffffffe: The field is string field which needs to be filtered.
  //  [0, 7ffffffd]: The field is a nested submessage. The value is the index
  //     that must be passed as first argument to the next Query() calls.
  //     Note that the message index is purely a monotonic counter in the
  std::vector<uint32_t> words_;

  // One entry for each message index stored in the filter plus a sentinel at
  // the end. Maps each message index to the offset in |words_| where the
  // Nth message start.
  // message_offset_.size() - 2 == the max message id that can be parsed.
  std::vector<uint32_t> message_offset_;

  bool suppress_logs_for_fuzzer_ = false;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_FILTER_BYTECODE_PARSER_H_
