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

#ifndef INCLUDE_PERFETTO_PUBLIC_PB_MACROS_H_
#define INCLUDE_PERFETTO_PUBLIC_PB_MACROS_H_

#include "perfetto/public/compiler.h"   // IWYU pragma: export
#include "perfetto/public/pb_msg.h"     // IWYU pragma: export
#include "perfetto/public/pb_packed.h"  // IWYU pragma: export
#include "perfetto/public/pb_utils.h"   // IWYU pragma: export

// This header contains macros that define types and accessors for protobuf
// messages.
//
// Example usage:
//
// PERFETTO_PB_ENUM(perfetto_protos_BuiltinClock){
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_UNKNOWN) = 0,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_REALTIME) = 1,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_REALTIME_COARSE)
//         = 2,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_MONOTONIC) = 3,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_MONOTONIC_COARSE)
//         = 4,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_MONOTONIC_RAW) = 5,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_BOOTTIME) = 6,
//     PERFETTO_PB_ENUM_ENTRY(perfetto_protos_BUILTIN_CLOCK_MAX_ID) = 63,
// };
//
// PERFETTO_PB_MSG(perfetto_protos_TraceConfig_BuiltinDataSource);
// PERFETTO_PB_FIELD(perfetto_protos_TraceConfig_BuiltinDataSource,
//                   VARINT,
//                   perfetto_protos_BuiltinClock,
//                   primary_trace_clock,
//                   5);
//
// PERFETTO_PB_MSG(perfetto_protos_TraceConfig);
// PERFETTO_PB_ENUM_IN_MSG(perfetto_protos_TraceConfig, LockdownModeOperation){
//     PERFETTO_PB_ENUM_IN_MSG_ENTRY(perfetto_protos_TraceConfig,
//         LOCKDOWN_UNCHANGED) = 0,
//     PERFETTO_PB_ENUM_IN_MSG_ENTRY(perfetto_protos_TraceConfig,
//         LOCKDOWN_CLEAR) = 1,
//     PERFETTO_PB_ENUM_IN_MSG_ENTRY(perfetto_protos_TraceConfig,
//         LOCKDOWN_SET) = 2,
// };
// PERFETTO_PB_FIELD(perfetto_protos_TraceConfig,
//                   VARINT,
//                   perfetto_protos_TraceConfig_LockdownModeOperation,
//                   lockdown_mode,
//                   5);
//
// PERFETTO_PB_MSG_DECL(perfetto_protos_DebugAnnotation);
// PERFETTO_PB_MSG_DECL(perfetto_protos_TestEvent_TestPayload);
//
// PERFETTO_PB_MSG(perfetto_protos_TestEvent);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent, STRING, const char*, str, 1);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent, VARINT, uint32_t, seq_value, 2);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent, VARINT, uint64_t, counter, 3);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent,
//                   MSG,
//                   perfetto_protos_TestEvent_TestPayload,
//                   payload,
//                   5);
//
// PERFETTO_PB_MSG(perfetto_protos_TestEvent_TestPayload);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent_TestPayload,
//                   STRING,
//                   const char*,
//                   str,
//                   1);
// PERFETTO_PB_FIELD(perfetto_protos_TestEvent_TestPayload,
//                   MSG,
//                   perfetto_protos_DebugAnnotation,
//                   debug_annotations,
//                   7);

#define PERFETTO_I_PB_CONCAT_4_(A, B, C, D) A##B##C##D
#define PERFETTO_I_PB_CONCAT_4(A, B, C, D) PERFETTO_I_PB_CONCAT_4_(A, B, C, D)

#define PERFETTO_I_PB_CONCAT_3_(A, B, C) A##B##C
#define PERFETTO_I_PB_CONCAT_3(A, B, C) PERFETTO_I_PB_CONCAT_3_(A, B, C)

#define PERFETTO_I_PB_SETTER_NAME(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_3(PROTO, _set_, NAME)

#define PERFETTO_I_PB_SETTER_CSTR_NAME(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_3(PROTO, _set_cstr_, NAME)

#define PERFETTO_I_PB_SETTER_BEGIN_NAME(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_3(PROTO, _begin_, NAME)

#define PERFETTO_I_PB_SETTER_END_NAME(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_3(PROTO, _end_, NAME)

#define PERFETTO_I_PB_NUM_FIELD_NAME(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_4(PROTO, _, NAME, _field_number)

#define PERFETTO_I_PB_GET_MSG(C_TYPE) PERFETTO_I_PB_CONCAT_3(C_TYPE, _, get_msg)

#define PERFETTO_I_PB_FIELD_STRING(PREFIX, PROTO, C_TYPE, NAME, NUM)      \
  static inline void PERFETTO_I_PB_SETTER_CSTR_NAME(PREFIX, NAME)(        \
      struct PROTO * msg, const char* value) {                            \
    PerfettoPbMsgAppendCStrField(&msg->msg, NUM, value);                  \
  }                                                                       \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(             \
      struct PROTO * msg, const void* data, size_t len) {                 \
    PerfettoPbMsgAppendType2Field(                                        \
        &msg->msg, NUM, PERFETTO_STATIC_CAST(const uint8_t*, data), len); \
  }                                                                       \
  static inline void PERFETTO_I_PB_SETTER_BEGIN_NAME(PREFIX, NAME)(       \
      struct PROTO * msg, struct PerfettoPbMsg * nested) {                \
    PerfettoPbMsgBeginNested(&msg->msg, nested, NUM);                     \
  }                                                                       \
  static inline void PERFETTO_I_PB_SETTER_END_NAME(PREFIX, NAME)(         \
      struct PROTO * msg, struct PerfettoPbMsg * nested) {                \
    (void)nested;                                                         \
    PerfettoPbMsgEndNested(&msg->msg);                                    \
  }

#define PERFETTO_I_PB_FIELD_VARINT(PREFIX, PROTO, C_TYPE, NAME, NUM)      \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(             \
      struct PROTO * msg, C_TYPE value) {                                 \
    PerfettoPbMsgAppendType0Field(&msg->msg, NUM,                         \
                                  PERFETTO_STATIC_CAST(uint64_t, value)); \
  }

#define PERFETTO_I_PB_FIELD_ZIGZAG(PREFIX, PROTO, C_TYPE, NAME, NUM)    \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(           \
      struct PROTO * msg, C_TYPE value) {                               \
    uint64_t encoded =                                                  \
        PerfettoPbZigZagEncode64(PERFETTO_STATIC_CAST(int64_t, value)); \
    PerfettoPbMsgAppendType0Field(&msg->msg, NUM, encoded);             \
  }

#define PERFETTO_I_PB_FIELD_FIXED64(PREFIX, PROTO, C_TYPE, NAME, NUM) \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(         \
      struct PROTO * msg, C_TYPE value) {                             \
    uint64_t val;                                                     \
    memcpy(&val, &value, sizeof val);                                 \
    PerfettoPbMsgAppendFixed64Field(&msg->msg, NUM, val);             \
  }

#define PERFETTO_I_PB_FIELD_FIXED32(PREFIX, PROTO, C_TYPE, NAME, NUM) \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(         \
      struct PROTO * msg, C_TYPE value) {                             \
    uint32_t val;                                                     \
    memcpy(&val, &value, sizeof val);                                 \
    PerfettoPbMsgAppendFixed32Field(&msg->msg, NUM, val);             \
  }

#define PERFETTO_I_PB_FIELD_MSG(PREFIX, PROTO, C_TYPE, NAME, NUM)   \
  static inline void PERFETTO_I_PB_SETTER_BEGIN_NAME(PREFIX, NAME)( \
      struct PROTO * msg, struct C_TYPE * nested) {                 \
    struct PerfettoPbMsg* nested_msg =                              \
        PERFETTO_REINTERPRET_CAST(struct PerfettoPbMsg*, nested);   \
    PerfettoPbMsgBeginNested(&msg->msg, nested_msg, NUM);           \
  }                                                                 \
  static inline void PERFETTO_I_PB_SETTER_END_NAME(PREFIX, NAME)(   \
      struct PROTO * msg, struct C_TYPE * nested) {                 \
    (void)nested;                                                   \
    PerfettoPbMsgEndNested(&msg->msg);                              \
  }

#define PERFETTO_I_PB_FIELD_PACKED(PREFIX, PROTO, C_TYPE, NAME, NUM)      \
  static inline void PERFETTO_I_PB_SETTER_NAME(PREFIX, NAME)(             \
      struct PROTO * msg, const void* data, size_t len) {                 \
    PerfettoPbMsgAppendType2Field(                                        \
        &msg->msg, NUM, PERFETTO_STATIC_CAST(const uint8_t*, data), len); \
  }                                                                       \
  static inline void PERFETTO_I_PB_SETTER_BEGIN_NAME(PREFIX, NAME)(       \
      struct PROTO * msg, struct PerfettoPbPackedMsg##C_TYPE * nested) {  \
    struct PerfettoPbMsg* nested_msg =                                    \
        PERFETTO_REINTERPRET_CAST(struct PerfettoPbMsg*, nested);         \
    PerfettoPbMsgBeginNested(&msg->msg, nested_msg, NUM);                 \
  }                                                                       \
  static inline void PERFETTO_I_PB_SETTER_END_NAME(PREFIX, NAME)(         \
      struct PROTO * msg, struct PerfettoPbPackedMsg##C_TYPE * nested) {  \
    (void)nested;                                                         \
    PerfettoPbMsgEndNested(&msg->msg);                                    \
  }

#define PERFETTO_I_PB_NUM_FIELD(PROTO, NAME, NUM) \
  enum { PERFETTO_I_PB_NUM_FIELD_NAME(PROTO, NAME) = NUM }

// Below are public macros that can be used to define protos. All the macros
// above are just implementation details and can change at any time.

// Defines the type for a protobuf message.
// `PROTO` is the name of the message type. For nested messages, an underscore
// should be used as a separator.
#define PERFETTO_PB_MSG(PROTO) \
  struct PROTO {               \
    struct PerfettoPbMsg msg;  \
  }

// Declares the type for a protobuf message. Useful when a file references a
// type (because it is used as type for a field), but doesn't need the full
// definition.
#define PERFETTO_PB_MSG_DECL(PROTO) struct PROTO

// Defines accessors for a field of a message.
// * `PROTO`: The message that contains this field. This should be the same
//   identifier passed to PERFETTO_PB_MSG.
// * `NAME`: The name of the field. It will be concatenated with `PROTO` to
//   produce the name for the accessors.
// * `NUM`: The numeric identifier for this field.
// * `TYPE`: The protobuf type of the field. Possible options:
//   * `VARINT`: For most integer (scalar and repeated non-packed) and enum
//     field types. `CTYPE` is the corresponding C type of the field. Generates
//     a single NAMESPACE_PROTO_set_NAME(CTYPE value accessor).
//   * `ZIGZAG`: For sint* (scalar and repeated non-packed) field types. `CTYPE`
//     is the corresponding C type of the field. Generates a single
//     PROTO_set_NAME(struct PROTO*, CTYPE) value setter.
//   * `FIXED32`: For fixed32, sfixed32 and float (scalar and repeated
//     non-packed) field types. `CTYPE` can be uint32_t, int32_t or float.
//     Generates a single PROTO_set_NAME(struct PROTO*, CTYPE) value setter.
//   * `FIXED64`: For fixed64, sfixed64 or double (scalar and repeated
//     non-packed) field types. `CTYPE` can be uint64_t or int64_t or double.
//     Generates a single PROTO_set_NAME(struct PROTO*, CTYPE) value setter.
//   * `MSG`: for nested (scalar and repeated) messages field types. `CTYPE` is
//     the type of the nested message (full type, including the namespace).
//     Generates
//     `PROTO_begin_NAME(struct PROTO*, struct CTYPE* nested)` and
//     `PROTO_end_NAME(struct PROTO*, struct CTYPE* nested)` that allows to
//     begin and end a nested submessage. `*nested` doesn't need to be
//     initialized.
//   * `STRING`: for bytes and string field types. `CTYPE` should be
//     `const char *`. Generates multiple accessors:
//      * PROTO_set_cstr_NAME(struct PROTO*, const char*): Sets the value of the
//        field by copying from a null terminated string.
//      * PROTO_set_NAME(struct PROTO*, const void*, size_t): Sets the value of
//        the field by copying from a buffer at an address with the specified
//        size.
//      * PROTO_begin_NAME(struct PROTO*, struct PerfettoPbMsg* nested) and
//        PROTO_end_NAME(struct PROTO*, struct PerfettoPbMsg* nested):
//        Begins (and ends) a nested submessage to allow users to generate part
//        of the length delimited buffer piece by piece.
//   * `PACKED`: for packed repeated field types. `CTYPE` should be
//     one of `PerfettoPbPacked*`. Generates multiple accessors:
//      * PROTO_set_NAME(struct PROTO*, const void*, size_t): Sets the value of
//        the field by copying from a buffer at an address with the specified
//        size.
//      * PROTO_begin_NAME(struct PROTO*, struct PerfettoPbPackedMsgCTYPE*
//        nested) and
//        PROTO_end_NAME(struct PROTO*, struct PerfettoPbPackedMsgCTYPE*
//        nested): Begins (and ends) a packed helper nested submessage (of the
//        right type) to allow users to push repeated entries one by one
//        directly into the stream writer buffer.
#define PERFETTO_PB_FIELD(PROTO, TYPE, C_TYPE, NAME, NUM)     \
  PERFETTO_I_PB_FIELD_##TYPE(PROTO, PROTO, C_TYPE, NAME, NUM) \
      PERFETTO_I_PB_NUM_FIELD(PROTO, NAME, NUM)

// Defines accessors for a field of a message for an extension.
// * `EXTENSION`: The name of the extension. it's going to be used as a prefix.
//    There doesn't need to be a PERFETTO_PB_MSG definition for this.
// * `PROTO`: The (base) message that contains this field. This should be the
//   same identifier passed to PERFETTO_PB_MSG.
// The rest of the params are the same as the PERFETTO_PB_FIELD macro.
#define PERFETTO_PB_EXTENSION_FIELD(EXTENSION, PROTO, TYPE, C_TYPE, NAME, NUM) \
  PERFETTO_I_PB_FIELD_##TYPE(EXTENSION, PROTO, C_TYPE, NAME, NUM)              \
      PERFETTO_I_PB_NUM_FIELD(EXTENSION, NAME, NUM)

// Defines an enum type nested inside a message (PROTO).
#define PERFETTO_PB_ENUM_IN_MSG(PROTO, ENUM) \
  enum PERFETTO_I_PB_CONCAT_3(PROTO, _, ENUM)

// Defines an entry for an enum tpye nested inside a message.
#define PERFETTO_PB_ENUM_IN_MSG_ENTRY(PROTO, NAME) \
  PERFETTO_I_PB_CONCAT_3(PROTO, _, NAME)

// Defines a global enum type.
#define PERFETTO_PB_ENUM(ENUM) enum ENUM

// Defines an entry for global enum type.
#define PERFETTO_PB_ENUM_ENTRY(NAME) NAME

#endif  // INCLUDE_PERFETTO_PUBLIC_PB_MACROS_H_
