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

#ifndef SRC_PROTOZERO_FILTERING_MESSAGE_TOKENIZER_H_
#define SRC_PROTOZERO_FILTERING_MESSAGE_TOKENIZER_H_

#include <stdint.h>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/protozero/proto_utils.h"

namespace protozero {

// A helper class for schema-less tokenizing of protobuf messages.
// This class takes a stream of proto-encoded bytes, pushed one by one in input
// via Push(octet), and returns a stream of tokens (each Push() call can return
// 0 or 1 token).
// A "token" contains metadata about a field, specifically: its ID, its wire
// type and:
//  - For varint and fixed32/64 fields: its payload.
//  - For string and bytes fields: the length of its payload.
//    In this case the caller is supposed to "eat" those N bytes before calling
//    Push() again.
// Note that this class cannot differentiate between a string/bytes field or
// a submessage, because they are encoded in the same way. The caller is
// supposed to know whether a field can be recursed into by just keep calling
// Push() or is a string that should be skipped.
// This is inline to allow the compiler to see through the Push method and
// avoid a function call for each byte.
class MessageTokenizer {
 public:
  struct Token {
    uint32_t field_id;  // 0 == not valid.
    proto_utils::ProtoWireType type;

    // For kLengthDelimited, |value| represent the length of the payload.
    uint64_t value;

    inline bool valid() const { return field_id != 0; }
    bool operator==(const Token& o) const {
      return field_id == o.field_id && type == o.type && value == o.value;
    }
  };

  // Pushes a byte in input and returns a token, only when getting to the last
  // byte of each field. Specifically:
  // - For varint and fixed32 fields, the Token is returned after the last byte
  //   of the numeric payload is pushed.
  // - For length-delimited fields, this returns after the last byte of the
  //   length is pushed (i.e. right before the payload starts). The caller is
  //   expected to either skip the next |value| bytes (in the case of a string
  //   or bytes fields) or keep calling Push, in the case of a submessage.
  inline Token Push(uint8_t octet) {
    using protozero::proto_utils::ProtoWireType;

    // Parsing a fixed32/64 field is the only case where we don't have to do
    // any varint decoding. This is why this block is before the remaining
    // switch statement below (all the rest is a varint).
    if (PERFETTO_UNLIKELY(state_ == kFixedIntValue)) {
      PERFETTO_DCHECK(fixed_int_bits_ == 32 || fixed_int_bits_ == 64);
      fixed_int_value_ |= static_cast<uint64_t>(octet) << fixed_int_shift_;
      fixed_int_shift_ += 8;
      if (fixed_int_shift_ < fixed_int_bits_)
        return Token{};  // Intermediate byte of a fixed32/64.
      auto wire_type = fixed_int_bits_ == 32 ? ProtoWireType::kFixed32
                                             : ProtoWireType::kFixed64;
      uint64_t fixed_int_value = fixed_int_value_;
      fixed_int_value_ = fixed_int_shift_ = fixed_int_bits_ = 0;
      state_ = kFieldPreamble;
      return Token{field_id_, wire_type, fixed_int_value};
    }

    // At this point either we are: (i) parsing a field preamble; (ii) parsing a
    // varint field paylod; (iii) parsing the length of a length-delimited
    // field. In all cases, we need to decode a varint before proceeding.
    varint_ |= static_cast<uint64_t>(octet & 0x7F) << varint_shift_;
    if (octet & 0x80) {
      varint_shift_ += 7;
      if (PERFETTO_UNLIKELY(varint_shift_ >= 64)) {
        varint_shift_ = 0;
        state_ = kInvalidVarInt;
      }
      return Token{};  // Still parsing a varint.
    }

    uint64_t varint = varint_;
    varint_ = 0;
    varint_shift_ = 0;

    switch (state_) {
      case kFieldPreamble: {
        auto field_type = static_cast<uint32_t>(varint & 7u);  // 7 = 0..0111
        field_id_ = static_cast<uint32_t>(varint >> 3);

        // The field type is legit, now check it's well formed and within
        // boundaries.
        if (field_type == static_cast<uint32_t>(ProtoWireType::kVarInt)) {
          state_ = kVarIntValue;
        } else if (field_type ==
                       static_cast<uint32_t>(ProtoWireType::kFixed32) ||
                   field_type ==
                       static_cast<uint32_t>(ProtoWireType::kFixed64)) {
          state_ = kFixedIntValue;
          fixed_int_shift_ = 0;
          fixed_int_value_ = 0;
          fixed_int_bits_ =
              field_type == static_cast<uint32_t>(ProtoWireType::kFixed32) ? 32
                                                                           : 64;
        } else if (field_type ==
                   static_cast<uint32_t>(ProtoWireType::kLengthDelimited)) {
          state_ = kLenDelimited;
        } else {
          state_ = kInvalidFieldType;
        }
        return Token{};
      }

      case kVarIntValue: {
        // Return the varint field payload and go back to the next field.
        state_ = kFieldPreamble;
        return Token{field_id_, ProtoWireType::kVarInt, varint};
      }

      case kLenDelimited: {
        const auto payload_len = varint;
        if (payload_len > protozero::proto_utils::kMaxMessageLength) {
          state_ = kMessageTooBig;
          return Token{};
        }
        state_ = kFieldPreamble;
        // At this point the caller is expected to consume the next
        // |payload_len| bytes.
        return Token{field_id_, ProtoWireType::kLengthDelimited, payload_len};
      }

      case kFixedIntValue:
        // Unreachable because of the if before the switch.
        PERFETTO_DCHECK(false);
        break;

      // Unrecoverable error states.
      case kInvalidFieldType:
      case kMessageTooBig:
      case kInvalidVarInt:
        break;
    }  // switch(state_)

    return Token{};  // Keep GCC happy.
  }

  // Returns true if the tokenizer FSM has reached quiescence (i.e. if we are
  // NOT in the middle of parsing a field).
  bool idle() const {
    return state_ == kFieldPreamble && varint_shift_ == 0 &&
           fixed_int_shift_ == 0;
  }

  // Only for reporting parser errors in the trace.
  uint32_t state() const { return static_cast<uint32_t>(state_); }

 private:
  enum State {
    kFieldPreamble = 0,  // Parsing the varint for the field preamble.
    kVarIntValue = 1,    // Parsing the payload of a varint field.
    kFixedIntValue = 2,  // Parsing the payload of a fixed32/64 field.
    kLenDelimited = 3,   // Parsing the length of a length-delimited field.

    // Unrecoverable error states:
    kInvalidFieldType = 4,  // Encountered an invalid field type.
    kMessageTooBig = 5,     // Size of the length delimited message was too big.
    kInvalidVarInt = 6,     // Varint larger than 64 bits.
  };

  State state_ = kFieldPreamble;
  uint32_t field_id_ = 0;
  uint64_t varint_ = 0;
  uint32_t varint_shift_ = 0;
  uint32_t fixed_int_shift_ = 0;
  uint32_t fixed_int_bits_ = 0;
  uint64_t fixed_int_value_ = 0;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_MESSAGE_TOKENIZER_H_
