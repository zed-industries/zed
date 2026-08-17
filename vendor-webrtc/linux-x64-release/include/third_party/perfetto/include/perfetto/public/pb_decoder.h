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

#ifndef INCLUDE_PERFETTO_PUBLIC_PB_DECODER_H_
#define INCLUDE_PERFETTO_PUBLIC_PB_DECODER_H_

#include "perfetto/public/abi/pb_decoder_abi.h"
#include "perfetto/public/compiler.h"
#include "perfetto/public/pb_utils.h"

// Iterator for parsing protobuf messages.
//
// Example usage:
//
// const char* msg_buf = ...
// size_t msg_size = ...
//
// for (struct PerfettoPbDecoderIterator it =
//          PerfettoPbDecoderIterateBegin(msg_buf, msg_size);
//      it.field.status == PERFETTO_PB_DECODER_OK;
//      PerfettoPbDecoderIterateNext(&it)) {
//   // Do something with it.field
// }
struct PerfettoPbDecoderIterator {
  struct PerfettoPbDecoder decoder;
  struct PerfettoPbDecoderField field;
};

static inline struct PerfettoPbDecoderIterator PerfettoPbDecoderIterateBegin(
    const void* start,
    size_t size) {
  struct PerfettoPbDecoderIterator ret;
  ret.decoder.read_ptr = PERFETTO_REINTERPRET_CAST(const uint8_t*, start);
  ret.decoder.end_ptr = PERFETTO_REINTERPRET_CAST(const uint8_t*, start) + size;
  ret.field = PerfettoPbDecoderParseField(&ret.decoder);
  return ret;
}

static inline struct PerfettoPbDecoderIterator
PerfettoPbDecoderIterateNestedBegin(
    struct PerfettoPbDecoderDelimitedField val) {
  struct PerfettoPbDecoderIterator ret;
  ret.decoder.read_ptr = val.start;
  ret.decoder.end_ptr = val.start + val.len;
  ret.field = PerfettoPbDecoderParseField(&ret.decoder);
  return ret;
}

static inline void PerfettoPbDecoderIterateNext(
    struct PerfettoPbDecoderIterator* iterator) {
  iterator->field = PerfettoPbDecoderParseField(&iterator->decoder);
}

static inline bool PerfettoPbDecoderFieldGetUint32(
    const PerfettoPbDecoderField* field,
    uint32_t* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_VARINT:
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = PERFETTO_STATIC_CAST(uint32_t, field->value.integer64);
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = field->value.integer32;
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetInt32(
    const PerfettoPbDecoderField* field,
    int32_t* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_VARINT:
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = PERFETTO_STATIC_CAST(int32_t, field->value.integer64);
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = PERFETTO_STATIC_CAST(int32_t, field->value.integer32);
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetUint64(
    const PerfettoPbDecoderField* field,
    uint64_t* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_VARINT:
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = field->value.integer64;
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = field->value.integer32;
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetInt64(
    const PerfettoPbDecoderField* field,
    int64_t* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_VARINT:
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = PERFETTO_STATIC_CAST(int64_t, field->value.integer64);
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = PERFETTO_STATIC_CAST(int64_t, field->value.integer32);
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetBool(
    const PerfettoPbDecoderField* field,
    bool* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_VARINT:
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = field->value.integer64 != 0;
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = field->value.integer32 != 0;
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetFloat(
    const PerfettoPbDecoderField* field,
    float* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = PERFETTO_STATIC_CAST(float, field->value.double_val);
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = field->value.float_val;
      return true;
  }
  return false;
}

static inline bool PerfettoPbDecoderFieldGetDouble(
    const PerfettoPbDecoderField* field,
    double* out) {
  switch (field->wire_type) {
    case PERFETTO_PB_WIRE_TYPE_FIXED64:
      *out = field->value.double_val;
      return true;
    case PERFETTO_PB_WIRE_TYPE_FIXED32:
      *out = PERFETTO_STATIC_CAST(double, field->value.float_val);
      return true;
  }
  return false;
}

#endif  // INCLUDE_PERFETTO_PUBLIC_PB_DECODER_H_
