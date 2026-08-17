// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_EXTENSION_SET_INL_H__
#define GOOGLE_PROTOBUF_EXTENSION_SET_INL_H__

#include "google/protobuf/extension_set.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/parse_context.h"

namespace google {
namespace protobuf {
namespace internal {

template <typename T>
const char* ExtensionSet::ParseFieldWithExtensionInfo(
    int number, bool was_packed_on_wire, const ExtensionInfo& extension,
    InternalMetadata* metadata, const char* ptr, internal::ParseContext* ctx) {
  if (was_packed_on_wire) {
    switch (extension.type) {
#define HANDLE_TYPE(UPPERCASE, CPP_CAMELCASE)                                \
  case WireFormatLite::TYPE_##UPPERCASE:                                     \
    return internal::Packed##CPP_CAMELCASE##Parser(                          \
        MutableRawRepeatedField(number, extension.type, extension.is_packed, \
                                extension.descriptor),                       \
        ptr, ctx);
      HANDLE_TYPE(INT32, Int32);
      HANDLE_TYPE(INT64, Int64);
      HANDLE_TYPE(UINT32, UInt32);
      HANDLE_TYPE(UINT64, UInt64);
      HANDLE_TYPE(SINT32, SInt32);
      HANDLE_TYPE(SINT64, SInt64);
      HANDLE_TYPE(FIXED32, Fixed32);
      HANDLE_TYPE(FIXED64, Fixed64);
      HANDLE_TYPE(SFIXED32, SFixed32);
      HANDLE_TYPE(SFIXED64, SFixed64);
      HANDLE_TYPE(FLOAT, Float);
      HANDLE_TYPE(DOUBLE, Double);
      HANDLE_TYPE(BOOL, Bool);
#undef HANDLE_TYPE

      case WireFormatLite::TYPE_ENUM:
        return internal::PackedEnumParserArg<T>(
            MutableRawRepeatedField(number, extension.type, extension.is_packed,
                                    extension.descriptor),
            ptr, ctx, extension.enum_validity_check, metadata, number);
      case WireFormatLite::TYPE_STRING:
      case WireFormatLite::TYPE_BYTES:
      case WireFormatLite::TYPE_GROUP:
      case WireFormatLite::TYPE_MESSAGE:
        ABSL_LOG(FATAL) << "Non-primitive types can't be packed.";
        break;
    }
  } else {
    switch (extension.type) {
#define HANDLE_VARINT_TYPE(UPPERCASE, CPP_CAMELCASE)                        \
  case WireFormatLite::TYPE_##UPPERCASE: {                                  \
    uint64_t value;                                                         \
    ptr = VarintParse(ptr, &value);                                         \
    GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);                                    \
    if (extension.is_repeated) {                                            \
      Add##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE,          \
                         extension.is_packed, value, extension.descriptor); \
    } else {                                                                \
      Set##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE, value,   \
                         extension.descriptor);                             \
    }                                                                       \
  } break

      HANDLE_VARINT_TYPE(INT32, Int32);
      HANDLE_VARINT_TYPE(INT64, Int64);
      HANDLE_VARINT_TYPE(UINT32, UInt32);
      HANDLE_VARINT_TYPE(UINT64, UInt64);
      HANDLE_VARINT_TYPE(BOOL, Bool);
#undef HANDLE_VARINT_TYPE
#define HANDLE_SVARINT_TYPE(UPPERCASE, CPP_CAMELCASE, SIZE)                 \
  case WireFormatLite::TYPE_##UPPERCASE: {                                  \
    uint64_t val;                                                           \
    ptr = VarintParse(ptr, &val);                                           \
    GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);                                    \
    auto value = WireFormatLite::ZigZagDecode##SIZE(val);                   \
    if (extension.is_repeated) {                                            \
      Add##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE,          \
                         extension.is_packed, value, extension.descriptor); \
    } else {                                                                \
      Set##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE, value,   \
                         extension.descriptor);                             \
    }                                                                       \
  } break

      HANDLE_SVARINT_TYPE(SINT32, Int32, 32);
      HANDLE_SVARINT_TYPE(SINT64, Int64, 64);
#undef HANDLE_SVARINT_TYPE
#define HANDLE_FIXED_TYPE(UPPERCASE, CPP_CAMELCASE, CPPTYPE)                \
  case WireFormatLite::TYPE_##UPPERCASE: {                                  \
    auto value = UnalignedLoad<CPPTYPE>(ptr);                               \
    ptr += sizeof(CPPTYPE);                                                 \
    if (extension.is_repeated) {                                            \
      Add##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE,          \
                         extension.is_packed, value, extension.descriptor); \
    } else {                                                                \
      Set##CPP_CAMELCASE(number, WireFormatLite::TYPE_##UPPERCASE, value,   \
                         extension.descriptor);                             \
    }                                                                       \
  } break

      HANDLE_FIXED_TYPE(FIXED32, UInt32, uint32_t);
      HANDLE_FIXED_TYPE(FIXED64, UInt64, uint64_t);
      HANDLE_FIXED_TYPE(SFIXED32, Int32, int32_t);
      HANDLE_FIXED_TYPE(SFIXED64, Int64, int64_t);
      HANDLE_FIXED_TYPE(FLOAT, Float, float);
      HANDLE_FIXED_TYPE(DOUBLE, Double, double);
#undef HANDLE_FIXED_TYPE

      case WireFormatLite::TYPE_ENUM: {
        uint64_t tmp;
        ptr = VarintParse(ptr, &tmp);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
        int value = tmp;

        if (!extension.enum_validity_check.IsValid(value)) {
          WriteVarint(number, value, metadata->mutable_unknown_fields<T>());
        } else if (extension.is_repeated) {
          AddEnum(number, WireFormatLite::TYPE_ENUM, extension.is_packed, value,
                  extension.descriptor);
        } else {
          SetEnum(number, WireFormatLite::TYPE_ENUM, value,
                  extension.descriptor);
        }
        break;
      }

      case WireFormatLite::TYPE_BYTES:
      case WireFormatLite::TYPE_STRING: {
        std::string* value =
            extension.is_repeated
                ? AddString(number, WireFormatLite::TYPE_STRING,
                            extension.descriptor)
                : MutableString(number, WireFormatLite::TYPE_STRING,
                                extension.descriptor);
        int size = ReadSize(&ptr);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
        return ctx->ReadString(ptr, size, value);
      }

      case WireFormatLite::TYPE_GROUP: {
        MessageLite* value =
            extension.is_repeated
                ? AddMessage(number, WireFormatLite::TYPE_GROUP,
                             *extension.message_info.prototype,
                             extension.descriptor)
                : MutableMessage(number, WireFormatLite::TYPE_GROUP,
                                 *extension.message_info.prototype,
                                 extension.descriptor);
        uint32_t tag = (number << 3) + WireFormatLite::WIRETYPE_START_GROUP;
        return ctx->ParseGroup(value, ptr, tag);
      }

      case WireFormatLite::TYPE_MESSAGE: {
        MessageLite* value =
            extension.is_repeated
                ? AddMessage(number, WireFormatLite::TYPE_MESSAGE,
                             *extension.message_info.prototype,
                             extension.descriptor)
                : MutableMessage(number, WireFormatLite::TYPE_MESSAGE,
                                 *extension.message_info.prototype,
                                 extension.descriptor);
        return ctx->ParseMessage(value, ptr);
      }
    }
  }
  return ptr;
}

template <typename Msg, typename T>
const char* ExtensionSet::ParseMessageSetItemTmpl(
    const char* ptr, const Msg* extendee, internal::InternalMetadata* metadata,
    internal::ParseContext* ctx) {
  std::string payload;
  uint32_t type_id = 0;
  enum class State { kNoTag, kHasType, kHasPayload, kDone };
  State state = State::kNoTag;

  while (!ctx->Done(&ptr)) {
    uint32_t tag = static_cast<uint8_t>(*ptr++);
    if (tag == WireFormatLite::kMessageSetTypeIdTag) {
      uint64_t tmp;
      ptr = ParseBigVarint(ptr, &tmp);
      // We should fail parsing if type id is 0 after cast to uint32.
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr != nullptr &&
                                     static_cast<uint32_t>(tmp) != 0);
      if (state == State::kNoTag) {
        type_id = static_cast<uint32_t>(tmp);
        state = State::kHasType;
      } else if (state == State::kHasPayload) {
        type_id = static_cast<uint32_t>(tmp);
        ExtensionInfo extension;
        bool was_packed_on_wire;
        if (!FindExtension(2, type_id, extendee, ctx, &extension,
                           &was_packed_on_wire)) {
          WriteLengthDelimited(type_id, payload,
                               metadata->mutable_unknown_fields<T>());
        } else {
          MessageLite* value =
              extension.is_repeated
                  ? AddMessage(type_id, WireFormatLite::TYPE_MESSAGE,
                               *extension.message_info.prototype,
                               extension.descriptor)
                  : MutableMessage(type_id, WireFormatLite::TYPE_MESSAGE,
                                   *extension.message_info.prototype,
                                   extension.descriptor);

          const char* p;
          // We can't use regular parse from string as we have to track
          // proper recursion depth and descriptor pools. Spawn a new
          // ParseContext inheriting those attributes.
          ParseContext tmp_ctx(ParseContext::kSpawn, *ctx, &p, payload);
          GOOGLE_PROTOBUF_PARSER_ASSERT(value->_InternalParse(p, &tmp_ctx) &&
                                         tmp_ctx.EndedAtLimit());
        }
        state = State::kDone;
      }
    } else if (tag == WireFormatLite::kMessageSetMessageTag) {
      if (state == State::kHasType) {
        ptr = ParseFieldMaybeLazily(static_cast<uint64_t>(type_id) * 8 + 2, ptr,
                                    extendee, metadata, ctx);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr != nullptr);
        state = State::kDone;
      } else {
        std::string tmp;
        int32_t size = ReadSize(&ptr);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
        ptr = ctx->ReadString(ptr, size, &tmp);
        GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
        if (state == State::kNoTag) {
          payload = std::move(tmp);
          state = State::kHasPayload;
        }
      }
    } else {
      ptr = ReadTag(ptr - 1, &tag);
      if (tag == 0 || (tag & 7) == 4) {
        ctx->SetLastTag(tag);
        return ptr;
      }
      ptr = ParseField(tag, ptr, extendee, metadata, ctx);
      GOOGLE_PROTOBUF_PARSER_ASSERT(ptr);
    }
  }
  return ptr;
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_EXTENSION_SET_INL_H__
