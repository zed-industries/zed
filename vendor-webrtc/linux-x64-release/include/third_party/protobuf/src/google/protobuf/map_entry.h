// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_MAP_ENTRY_H__
#define GOOGLE_PROTOBUF_MAP_ENTRY_H__

#include <cstddef>
#include <cstdint>
#include <string>
#include <type_traits>

#include "google/protobuf/generated_message_reflection.h"
#include "google/protobuf/has_bits.h"
#include "google/protobuf/map_type_handler.h"
#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/parse_context.h"
#include "google/protobuf/unknown_field_set.h"
#include "google/protobuf/wire_format_lite.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {

class Arena;

namespace internal {

// MapEntry is the returned google::protobuf::Message when calling AddMessage of
// google::protobuf::Reflection. In order to let it work with generated message
// reflection, its in-memory type is the same as generated message with the same
// fields. However, in order to decide the in-memory type of key/value, we need
// to know both their cpp type in generated api and proto type. In
// implementation, all in-memory types have related wire format functions to
// support except ArenaStringPtr. Therefore, we need to define another type with
// supporting wire format functions. Since this type is only used as return type
// of MapEntry accessors, it's named MapEntry accessor type.
//
// cpp type:               the type visible to users in public API.
// proto type:             WireFormatLite::FieldType of the field.
// in-memory type:         type of the data member used to stored this field.
// MapEntry accessor type: type used in MapEntry getters/mutators to access the
//                         field.
//
// cpp type | proto type  | in-memory type | MapEntry accessor type
// int32_t    TYPE_INT32    int32_t          int32_t
// int32_t    TYPE_FIXED32  int32_t          int32_t
// string     TYPE_STRING   ArenaStringPtr   string
// FooEnum    TYPE_ENUM     int              int
// FooMessage TYPE_MESSAGE  FooMessage*      FooMessage
//
// The in-memory types of primitive types can be inferred from its proto type,
// while we need to explicitly specify the cpp type if proto type is
// TYPE_MESSAGE to infer the in-memory type.
template <typename Key, typename Value, WireFormatLite::FieldType kKeyFieldType,
          WireFormatLite::FieldType kValueFieldType>
class MapEntry : public Message {
  // Provide utilities to parse/serialize key/value.  Provide utilities to
  // manipulate internal stored type.
  using KeyTypeHandler = MapTypeHandler<kKeyFieldType, Key>;
  using ValueTypeHandler = MapTypeHandler<kValueFieldType, Value>;

  // Define internal memory layout. Strings and messages are stored as
  // pointers, while other types are stored as values.
  using KeyOnMemory = typename KeyTypeHandler::TypeOnMemory;
  using ValueOnMemory = typename ValueTypeHandler::TypeOnMemory;

 public:
#if !defined(PROTOBUF_CUSTOM_VTABLE)
  constexpr MapEntry() {}
#endif  // PROTOBUF_CUSTOM_VTABLE
  using Message::Message;

  MapEntry(const MapEntry&) = delete;
  MapEntry& operator=(const MapEntry&) = delete;

  ~MapEntry() PROTOBUF_OVERRIDE {
    // Make sure that `Value` is never a derived message type.
    // We don't want to instantiate the template with every unique derived type.
    // The assertion is in the destructor because we need `Value` to be
    // complete to test it.
    static_assert(!std::is_base_of<Message, Value>::value ||
                      std::is_same<Message, Value>::value,
                  "");

    if (GetArena() != nullptr) return;
    SharedDtor(*this);
  }

  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  struct _Internal;

 protected:
  friend class google::protobuf::Arena;

  static void SharedDtor(MessageLite& msg) {
    auto& this_ = static_cast<MapEntry&>(msg);
    this_._internal_metadata_.template Delete<UnknownFieldSet>();
    KeyTypeHandler::DeleteNoArena(this_._impl_.key_);
    ValueTypeHandler::DeleteNoArena(this_._impl_.value_);
  }

  // Field naming follows the convention of generated messages to make code
  // sharing easier.
  struct {
    HasBits<1> _has_bits_{};
    CachedSize _cached_size_{};

    KeyOnMemory key_{KeyTypeHandler::Constinit()};
    ValueOnMemory value_{ValueTypeHandler::Constinit()};
  } _impl_;
};

template <typename Key, typename Value, WireFormatLite::FieldType kKeyFieldType,
          WireFormatLite::FieldType kValueFieldType>
struct MapEntry<Key, Value, kKeyFieldType, kValueFieldType>::_Internal {
  static constexpr ::int32_t kHasBitsOffset =
      8 * PROTOBUF_FIELD_OFFSET(MapEntry, _impl_._has_bits_);
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_MAP_ENTRY_H__
