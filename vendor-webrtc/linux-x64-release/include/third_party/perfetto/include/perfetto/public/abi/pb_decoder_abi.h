/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_PB_DECODER_ABI_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_PB_DECODER_ABI_H_

#include <stddef.h>
#include <stdint.h>

#include "perfetto/public/abi/export.h"

#ifdef __cplusplus
extern "C" {
#endif

// Stores the state required to decode a protobuf message (from a continuous
// memory buffer).
struct PerfettoPbDecoder {
  // Pointer to the beginning of the next field that should be decoded.
  const uint8_t* read_ptr;
  // Pointer to one past the end of the buffer.
  const uint8_t* end_ptr;
};

enum PerfettoPbDecoderStatus {
  // A field has been decoded correctly. There is more data into the buffer,
  // starting from an updated `read_ptr`.
  PERFETTO_PB_DECODER_OK = 0,
  // The last field has been decoded correctly until the end. There is no more
  // data into the buffer.
  PERFETTO_PB_DECODER_DONE = 1,
  // The data starting at `read_ptr` cannot be fully decoded as a protobuf
  // field. `read_ptr` has not been updated.
  PERFETTO_PB_DECODER_ERROR = 2,
};

// The content of a length-delimited field (wire type 2)
struct PerfettoPbDecoderDelimitedField {
  const uint8_t* start;
  size_t len;
};

// A field parsed by the decoder.
struct PerfettoPbDecoderField {
  // PerfettoPbDecoderStatus
  uint32_t status;
  // PerfettoPbWireType
  uint32_t wire_type;
  // The protobuf field id.
  uint32_t id;
  // The value of this field.
  union {
    // For wire type 2.
    struct PerfettoPbDecoderDelimitedField delimited;
    // For wire type 0 and 1.
    uint64_t integer64;
    // For wire type 5.
    uint32_t integer32;
    // For wire type 1.
    double double_val;
    // For wire type 5.
    float float_val;
  } value;
};

// Parses a field and returns it. Advances `*decoder->read_ptr` to point after
// the field.
PERFETTO_SDK_EXPORT struct PerfettoPbDecoderField PerfettoPbDecoderParseField(
    struct PerfettoPbDecoder* decoder);

// Advances `*decoder->read_ptr` to point after the current field.
// Returns a `PerfettoPbDecoderStatus`.
PERFETTO_SDK_EXPORT uint32_t
PerfettoPbDecoderSkipField(struct PerfettoPbDecoder* decoder);

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_PB_DECODER_ABI_H_
