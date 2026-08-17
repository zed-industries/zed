// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_PARSED_METADATA_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_PARSED_METADATA_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <string.h>

#include <cstdint>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/functional/function_ref.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/escaping.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/util/time.h"

namespace grpc_core {

using MetadataParseErrorFn =
    absl::FunctionRef<void(absl::string_view error, const Slice& value)>;

namespace metadata_detail {

// Helper to determine whether a traits metadata is inlinable inside a memento,
// or (if not) we'll need to take the memory allocation path.
template <typename Which>
struct HasSimpleMemento {
  static constexpr bool value =
      (std::is_trivial<typename Which::MementoType>::value &&
       sizeof(typename Which::MementoType) <= sizeof(grpc_slice)) ||
      std::is_same<typename Which::MementoType, Duration>::value;
};

// Storage type for a single metadata entry.
union Buffer {
  uint8_t trivial[sizeof(grpc_slice)];
  void* pointer;
  grpc_slice slice;
};

// Given a key and a value, concatenate together to make a debug string.
// Split out to avoid template bloat.
std::string MakeDebugString(absl::string_view key, absl::string_view value);

// Wrapper around MakeDebugString.
// For the value part, use two functions - one to extract a typed field from
// Buffer, and a second (sourced from the trait) to generate a displayable debug
// string from the field value. We try to maximize indirection/code sharing here
// as this is not critical path code and we'd like to avoid some code bloat -
// better to scale by number of types than then number of metadata traits!
template <typename Field, typename CompatibleWithField, typename Display>
GPR_ATTRIBUTE_NOINLINE std::string MakeDebugStringPipeline(
    absl::string_view key, const Buffer& value,
    Field (*field_from_buffer)(const Buffer&),
    Display (*display_from_field)(CompatibleWithField)) {
  return MakeDebugString(
      key, absl::StrCat(display_from_field(field_from_buffer(value))));
}

// Extract a trivial field value from a Buffer - for MakeDebugStringPipeline.
template <typename Field>
Field FieldFromTrivial(const Buffer& value) {
  Field field;
  memcpy(&field, value.trivial, sizeof(Field));
  return field;
}

// Extract a pointer field value from a Buffer - for MakeDebugStringPipeline.
template <typename Field>
Field FieldFromPointer(const Buffer& value) {
  return *static_cast<const Field*>(value.pointer);
}

// Extract a Slice from a Buffer.
Slice SliceFromBuffer(const Buffer& buffer);

// Unref the grpc_slice part of a Buffer (assumes it is in fact a grpc_slice).
void DestroySliceValue(const Buffer& value);

// Destroy a trivial memento (empty function).
void DestroyTrivialMemento(const Buffer& value);

// Set a slice value in a container
template <Slice (*MementoToValue)(Slice)>
void SetSliceValue(Slice* set, const Buffer& value) {
  *set = MementoToValue(SliceFromBuffer(value));
}

}  // namespace metadata_detail

// A parsed metadata value.
// This type captures a type erased MementoType from one trait of
// MetadataContainer, and provides utilities to manipulate that and to set it on
// a MetadataContainer.
template <typename MetadataContainer>
class ParsedMetadata {
 public:
  // Construct metadata from a trait Which of MetadataContainer.
  // Two versions: the first is for simple inlinable mementos, and the second
  // forces an allocation.
  template <typename Which>
  ParsedMetadata(
      Which,
      absl::enable_if_t<metadata_detail::HasSimpleMemento<Which>::value,
                        typename Which::MementoType>
          value,
      uint32_t transport_size)
      : vtable_(ParsedMetadata::template TrivialTraitVTable<Which>()),
        transport_size_(transport_size) {
    memcpy(value_.trivial, &value, sizeof(value));
  }
  template <typename Which>
  ParsedMetadata(
      Which,
      absl::enable_if_t<
          !metadata_detail::HasSimpleMemento<Which>::value &&
              !std::is_convertible<typename Which::MementoType, Slice>::value,
          typename Which::MementoType>
          value,
      uint32_t transport_size)
      : vtable_(ParsedMetadata::template NonTrivialTraitVTable<Which>()),
        transport_size_(transport_size) {
    value_.pointer = new typename Which::MementoType(std::move(value));
  }
  // Construct metadata from a Slice typed value.
  template <typename Which>
  ParsedMetadata(Which, Slice value, uint32_t transport_size)
      : vtable_(ParsedMetadata::template SliceTraitVTable<Which>()),
        transport_size_(transport_size) {
    value_.slice = value.TakeCSlice();
  }
  // Construct metadata from a string key, slice value pair.
  // FromSlicePair() is used to adjust the overload set so that we don't
  // inadvertently match against any of the previous overloads.
  // TODO(ctiller): re-evaluate the overload functions here so and maybe
  // introduce some factory functions?
  struct FromSlicePair {};
  ParsedMetadata(FromSlicePair, Slice key, Slice value, uint32_t transport_size)
      : vtable_(ParsedMetadata::KeyValueVTable(key.as_string_view())),
        transport_size_(transport_size) {
    value_.pointer =
        new std::pair<Slice, Slice>(std::move(key), std::move(value));
  }
  ParsedMetadata() : vtable_(EmptyVTable()), transport_size_(0) {}
  ~ParsedMetadata() { vtable_->destroy(value_); }

  // Non copyable, but movable.
  ParsedMetadata(const ParsedMetadata&) = delete;
  ParsedMetadata& operator=(const ParsedMetadata&) = delete;
  ParsedMetadata(ParsedMetadata&& other) noexcept
      : vtable_(other.vtable_),
        value_(other.value_),
        transport_size_(other.transport_size_) {
    other.vtable_ = EmptyVTable();
  }
  ParsedMetadata& operator=(ParsedMetadata&& other) noexcept {
    vtable_ = other.vtable_;
    value_ = other.value_;
    transport_size_ = other.transport_size_;
    other.vtable_ = EmptyVTable();
    return *this;
  }

  // Set this parsed value on a container.
  void SetOnContainer(MetadataContainer* container) const {
    vtable_->set(value_, container);
  }

  // Is this a binary header or not?
  bool is_binary_header() const { return vtable_->is_binary_header; }
  // HTTP2 defined storage size of this metadatum.
  uint32_t transport_size() const { return transport_size_; }
  // Create a new parsed metadata with the same key but a different value.
  ParsedMetadata WithNewValue(Slice value, bool will_keep_past_request_lifetime,
                              uint32_t value_wire_size,
                              MetadataParseErrorFn on_error) const {
    ParsedMetadata result;
    result.vtable_ = vtable_;
    result.value_ = value_;
    result.transport_size_ =
        TransportSize(static_cast<uint32_t>(key().length()), value_wire_size);
    vtable_->with_new_value(&value, will_keep_past_request_lifetime, on_error,
                            &result);
    return result;
  }
  std::string DebugString() const { return vtable_->debug_string(value_); }
  absl::string_view key() const {
    if (vtable_->key == nullptr) return vtable_->key_value;
    return vtable_->key(value_);
  }

  // TODO(ctiller): move to transport
  static uint32_t TransportSize(uint32_t key_size, uint32_t value_size) {
    // TODO(ctiller): use hpack constant?
    return key_size + value_size + 32;
  }

 private:
  using Buffer = metadata_detail::Buffer;

  struct VTable {
    const bool is_binary_header;
    void (*const destroy)(const Buffer& value);
    void (*const set)(const Buffer& value, MetadataContainer* container);
    // result is a bitwise copy of the originating ParsedMetadata.
    void (*const with_new_value)(Slice* new_value,
                                 bool will_keep_past_request_lifetime,
                                 MetadataParseErrorFn on_error,
                                 ParsedMetadata* result);
    std::string (*const debug_string)(const Buffer& value);
    // The key - if key is null, use key_value, otherwise call key.
    absl::string_view key_value;
    absl::string_view (*const key)(const Buffer& value);
  };

  static const VTable* EmptyVTable();
  static const VTable* KeyValueVTable(absl::string_view key);
  template <typename Which>
  static const VTable* TrivialTraitVTable();
  template <typename Which>
  static const VTable* NonTrivialTraitVTable();
  template <typename Which>
  static const VTable* SliceTraitVTable();

  template <Slice (*ParseMemento)(Slice, bool, MetadataParseErrorFn)>
  GPR_ATTRIBUTE_NOINLINE static void WithNewValueSetSlice(
      Slice* slice, bool will_keep_past_request_lifetime,
      MetadataParseErrorFn on_error, ParsedMetadata* result) {
    result->value_.slice =
        ParseMemento(std::move(*slice), will_keep_past_request_lifetime,
                     on_error)
            .TakeCSlice();
  }

  template <typename T, T (*ParseMemento)(Slice, bool, MetadataParseErrorFn)>
  GPR_ATTRIBUTE_NOINLINE static void WithNewValueSetTrivial(
      Slice* slice, bool will_keep_past_request_lifetime,
      MetadataParseErrorFn on_error, ParsedMetadata* result) {
    T memento = ParseMemento(std::move(*slice), will_keep_past_request_lifetime,
                             on_error);
    memcpy(result->value_.trivial, &memento, sizeof(memento));
  }

  const VTable* vtable_;
  Buffer value_;
  uint32_t transport_size_;
};

namespace metadata_detail {}  // namespace metadata_detail

template <typename MetadataContainer>
const typename ParsedMetadata<MetadataContainer>::VTable*
ParsedMetadata<MetadataContainer>::EmptyVTable() {
  static const VTable vtable = {
      false,
      // destroy
      metadata_detail::DestroyTrivialMemento,
      // set
      [](const Buffer&, MetadataContainer*) {},
      // with_new_value
      [](Slice*, bool, MetadataParseErrorFn, ParsedMetadata*) {},
      // debug_string
      [](const Buffer&) -> std::string { return "empty"; },
      // key
      "",
      nullptr,
  };
  return &vtable;
}

template <typename MetadataContainer>
template <typename Which>
const typename ParsedMetadata<MetadataContainer>::VTable*
ParsedMetadata<MetadataContainer>::TrivialTraitVTable() {
  static const VTable vtable = {
      absl::EndsWith(Which::key(), "-bin"),
      // destroy
      metadata_detail::DestroyTrivialMemento,
      // set
      [](const Buffer& value, MetadataContainer* map) {
        map->Set(
            Which(),
            Which::MementoToValue(
                metadata_detail::FieldFromTrivial<typename Which::MementoType>(
                    value)));
      },
      // with_new_value
      WithNewValueSetTrivial<typename Which::MementoType, Which::ParseMemento>,
      // debug_string
      [](const Buffer& value) {
        return metadata_detail::MakeDebugStringPipeline(
            Which::key(), value,
            metadata_detail::FieldFromTrivial<typename Which::MementoType>,
            Which::DisplayMemento);
      },
      // key
      Which::key(),
      nullptr,
  };
  return &vtable;
}

template <typename MetadataContainer>
template <typename Which>
const typename ParsedMetadata<MetadataContainer>::VTable*
ParsedMetadata<MetadataContainer>::NonTrivialTraitVTable() {
  static const VTable vtable = {
      absl::EndsWith(Which::key(), "-bin"),
      // destroy
      [](const Buffer& value) {
        delete static_cast<typename Which::MementoType*>(value.pointer);
      },
      // set
      [](const Buffer& value, MetadataContainer* map) {
        auto* p = static_cast<typename Which::MementoType*>(value.pointer);
        map->Set(Which(), Which::MementoToValue(*p));
      },
      // with_new_value
      [](Slice* value, bool will_keep_past_request_lifetime,
         MetadataParseErrorFn on_error, ParsedMetadata* result) {
        result->value_.pointer =
            new typename Which::MementoType(Which::ParseMemento(
                std::move(*value), will_keep_past_request_lifetime, on_error));
      },
      // debug_string
      [](const Buffer& value) {
        return metadata_detail::MakeDebugStringPipeline(
            Which::key(), value,
            metadata_detail::FieldFromPointer<typename Which::MementoType>,
            Which::DisplayMemento);
      },
      // key
      Which::key(),
      nullptr,
  };
  return &vtable;
}

template <typename MetadataContainer>
template <typename Which>
const typename ParsedMetadata<MetadataContainer>::VTable*
ParsedMetadata<MetadataContainer>::SliceTraitVTable() {
  static const VTable vtable = {
      absl::EndsWith(Which::key(), "-bin"),
      // destroy
      metadata_detail::DestroySliceValue,
      // set
      [](const Buffer& value, MetadataContainer* map) {
        metadata_detail::SetSliceValue<Which::MementoToValue>(
            map->GetOrCreatePointer(Which()), value);
      },
      // with_new_value
      WithNewValueSetSlice<Which::ParseMemento>,
      // debug_string
      [](const Buffer& value) {
        return metadata_detail::MakeDebugStringPipeline(
            Which::key(), value, metadata_detail::SliceFromBuffer,
            Which::DisplayMemento);
      },
      // key
      Which::key(),
      nullptr,
  };
  return &vtable;
}

template <typename MetadataContainer>
const typename ParsedMetadata<MetadataContainer>::VTable*
ParsedMetadata<MetadataContainer>::KeyValueVTable(absl::string_view key) {
  using KV = std::pair<Slice, Slice>;
  static const auto destroy = [](const Buffer& value) {
    delete static_cast<KV*>(value.pointer);
  };
  static const auto set = [](const Buffer& value, MetadataContainer* map) {
    auto* p = static_cast<KV*>(value.pointer);
    map->unknown_.Append(p->first.as_string_view(), p->second.Ref());
  };
  static const auto with_new_value =
      [](Slice* value, bool will_keep_past_request_lifetime,
         MetadataParseErrorFn, ParsedMetadata* result) {
        auto* p = new KV{
            static_cast<KV*>(result->value_.pointer)->first.Ref(),
            will_keep_past_request_lifetime ? value->TakeUniquelyOwned()
                                            : std::move(*value),
        };
        result->value_.pointer = p;
      };
  static const auto debug_string = [](const Buffer& value) {
    auto* p = static_cast<KV*>(value.pointer);
    return absl::StrCat(p->first.as_string_view(), ": ",
                        p->second.as_string_view());
  };
  static const auto binary_debug_string = [](const Buffer& value) {
    auto* p = static_cast<KV*>(value.pointer);
    return absl::StrCat(p->first.as_string_view(), ": \"",
                        absl::CEscape(p->second.as_string_view()), "\"");
  };
  static const auto key_fn = [](const Buffer& value) {
    return static_cast<KV*>(value.pointer)->first.as_string_view();
  };
  static const VTable vtable[2] = {
      {false, destroy, set, with_new_value, debug_string, "", key_fn},
      {true, destroy, set, with_new_value, binary_debug_string, "", key_fn},
  };
  return &vtable[absl::EndsWith(key, "-bin")];
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_PARSED_METADATA_H
