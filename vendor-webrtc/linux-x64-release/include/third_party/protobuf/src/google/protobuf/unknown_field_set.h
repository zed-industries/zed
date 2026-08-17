// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// Contains classes used to keep track of unrecognized fields seen while
// parsing a protocol message.

#ifndef GOOGLE_PROTOBUF_UNKNOWN_FIELD_SET_H__
#define GOOGLE_PROTOBUF_UNKNOWN_FIELD_SET_H__

#include <assert.h>

#include <cstddef>
#include <cstdint>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/internal_visibility.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/io/zero_copy_stream_impl_lite.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/parse_context.h"
#include "google/protobuf/port.h"
#include "google/protobuf/repeated_field.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {
namespace internal {
class InternalMetadata;           // metadata_lite.h
class WireFormat;                 // wire_format.h
class MessageSetFieldSkipperUsingCord;
// extension_set_heavy.cc
class UnknownFieldParserHelper;
struct UnknownFieldSetTestPeer;

#if defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
using UFSStringView = absl::string_view;
#else
using UFSStringView = const std::string&;
#endif
}  // namespace internal

class Message;       // message.h

// Represents one field in an UnknownFieldSet.
class PROTOBUF_EXPORT UnknownField {
 public:
  enum Type {
    TYPE_VARINT,
    TYPE_FIXED32,
    TYPE_FIXED64,
    TYPE_LENGTH_DELIMITED,
    TYPE_GROUP
  };

  // The field's field number, as seen on the wire.
  inline int number() const;

  // The field type.
  inline Type type() const;

  // Accessors -------------------------------------------------------
  // Each method works only for UnknownFields of the corresponding type.

  inline uint64_t varint() const;
  inline uint32_t fixed32() const;
  inline uint64_t fixed64() const;
  inline internal::UFSStringView length_delimited() const;
  inline const UnknownFieldSet& group() const;

  inline void set_varint(uint64_t value);
  inline void set_fixed32(uint32_t value);
  inline void set_fixed64(uint64_t value);
  inline void set_length_delimited(absl::string_view value);
  // template to avoid ambiguous overload resolution.
  template <int&...>
  inline void set_length_delimited(std::string&& value);
  inline void set_length_delimited(const absl::Cord& value);
#if !defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
  inline std::string* mutable_length_delimited();
#endif  // PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE
  inline UnknownFieldSet* mutable_group();

  inline size_t GetLengthDelimitedSize() const;
  uint8_t* InternalSerializeLengthDelimitedNoTag(
      uint8_t* target, io::EpsCopyOutputStream* stream) const;

 private:
  friend class UnknownFieldSet;

  // If this UnknownField contains a pointer, delete it.
  void Delete();

  // Make a deep copy of any pointers in this UnknownField.
  UnknownField DeepCopy(Arena* arena) const;

  // Set the wire type of this UnknownField. Should only be used when this
  // UnknownField is being created.
  inline void SetType(Type type);

  uint32_t number_;
  uint32_t type_;
  union {
    uint64_t varint;
    uint32_t fixed32;
    uint64_t fixed64;
    std::string* string_value;
    UnknownFieldSet* group;
  } data_;
};

// An UnknownFieldSet contains fields that were encountered while parsing a
// message but were not defined by its type.  Keeping track of these can be
// useful, especially in that they may be written if the message is serialized
// again without being cleared in between.  This means that software which
// simply receives messages and forwards them to other servers does not need
// to be updated every time a new field is added to the message definition.
//
// To get the UnknownFieldSet attached to any message, call
// Reflection::GetUnknownFields().
//
// This class is necessarily tied to the protocol buffer wire format, unlike
// the Reflection interface which is independent of any serialization scheme.
class PROTOBUF_EXPORT UnknownFieldSet {
 public:
  constexpr UnknownFieldSet();
  UnknownFieldSet(const UnknownFieldSet&) = delete;
  UnknownFieldSet& operator=(const UnknownFieldSet&) = delete;
  ~UnknownFieldSet();

  // Remove all fields.
  inline void Clear();

  // Remove all fields and deallocate internal data objects
  void ClearAndFreeMemory();

  // Is this set empty?
  inline bool empty() const;

  // Merge the contents of some other UnknownFieldSet with this one.
  void MergeFrom(const UnknownFieldSet& other);

  // Similar to above, but this function will destroy the contents of other.
  void MergeFromAndDestroy(UnknownFieldSet* other);

  // Merge the contents an UnknownFieldSet with the UnknownFieldSet in
  // *metadata, if there is one.  If *metadata doesn't have an UnknownFieldSet
  // then add one to it and make it be a copy of the first arg.
  static void MergeToInternalMetadata(const UnknownFieldSet& other,
                                      internal::InternalMetadata* metadata);

  // Swaps the contents of some other UnknownFieldSet with this one.
  inline void Swap(UnknownFieldSet* x);

  // Computes (an estimate of) the total number of bytes currently used for
  // storing the unknown fields in memory. Does NOT include
  // sizeof(*this) in the calculation.
  size_t SpaceUsedExcludingSelfLong() const;

  int SpaceUsedExcludingSelf() const {
    return internal::ToIntSize(SpaceUsedExcludingSelfLong());
  }

  // Version of SpaceUsed() including sizeof(*this).
  size_t SpaceUsedLong() const;

  int SpaceUsed() const { return internal::ToIntSize(SpaceUsedLong()); }

  // Returns the number of fields present in the UnknownFieldSet.
  inline int field_count() const;
  // Get a field in the set, where 0 <= index < field_count().  The fields
  // appear in the order in which they were added.
  inline const UnknownField& field(int index) const;
  // Get a mutable pointer to a field in the set, where
  // 0 <= index < field_count().  The fields appear in the order in which
  // they were added.
  inline UnknownField* mutable_field(int index);

  // Adding fields ---------------------------------------------------

  void AddVarint(int number, uint64_t value);
  void AddFixed32(int number, uint32_t value);
  void AddFixed64(int number, uint64_t value);
  void AddLengthDelimited(int number, absl::string_view value);
  // template to avoid ambiguous overload resolution.
  template <int&...>
  void AddLengthDelimited(int number, std::string&& value);
  void AddLengthDelimited(int number, const absl::Cord& value);

#if !defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
  std::string* AddLengthDelimited(int number);
#endif  // PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE
  UnknownFieldSet* AddGroup(int number);

  // Adds an unknown field from another set.
  void AddField(const UnknownField& field);

  // Delete fields with indices in the range [start .. start+num-1].
  // Caution: implementation moves all fields with indices [start+num .. ].
  void DeleteSubrange(int start, int num);

  // Delete all fields with a specific field number. The order of left fields
  // is preserved.
  // Caution: implementation moves all fields after the first deleted field.
  void DeleteByNumber(int number);

  // Parsing helpers -------------------------------------------------
  // These work exactly like the similarly-named methods of Message.

  bool MergeFromCodedStream(io::CodedInputStream* input);
  bool ParseFromCodedStream(io::CodedInputStream* input);
  bool ParseFromZeroCopyStream(io::ZeroCopyInputStream* input);
  bool ParseFromArray(const void* data, int size);
  inline bool ParseFromString(const absl::string_view data) {
    return ParseFromArray(data.data(), static_cast<int>(data.size()));
  }

  // Merges this message's unknown field data (if any).  This works whether
  // the message is a lite or full proto (for legacy reasons, lite and full
  // return different types for MessageType::unknown_fields()).
  template <typename MessageType>
  bool MergeFromMessage(const MessageType& message);

  // Serialization.
  bool SerializeToString(std::string* output) const;
  bool SerializeToCord(absl::Cord* output) const;
  bool SerializeToCodedStream(io::CodedOutputStream* output) const;
  static const UnknownFieldSet& default_instance();

  UnknownFieldSet(internal::InternalVisibility, Arena* arena)
      : UnknownFieldSet(arena) {}

 private:
  friend internal::WireFormat;
  friend internal::UnknownFieldParserHelper;
  friend internal::UnknownFieldSetTestPeer;

#if defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
  std::string* AddLengthDelimited(int number);
#endif  // PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE

  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;

  friend class google::protobuf::Arena;
  explicit UnknownFieldSet(Arena* arena) : fields_(arena) {}

  Arena* arena() { return fields_.GetArena(); }

  void ClearFallback();
  void SwapSlow(UnknownFieldSet* other);

  template <typename MessageType,
            typename std::enable_if_t<
                std::is_base_of<Message, MessageType>::value, int> = 0>
  bool InternalMergeFromMessage(const MessageType& message) {
    MergeFrom(message.GetReflection()->GetUnknownFields(message));
    return true;
  }

  template <typename MessageType,
            typename std::enable_if<
                std::is_base_of<MessageLite, MessageType>::value &&
                    !std::is_base_of<Message, MessageType>::value,
                int>::type = 0>
  bool InternalMergeFromMessage(const MessageType& message) {
    const auto& unknown_fields = message.unknown_fields();
    io::ArrayInputStream array_stream(unknown_fields.data(),
                                      unknown_fields.size());
    io::CodedInputStream coded_stream(&array_stream);
    return MergeFromCodedStream(&coded_stream);
  }

  RepeatedField<UnknownField> fields_;
};

namespace internal {

inline void WriteVarint(uint32_t num, uint64_t val, UnknownFieldSet* unknown) {
  unknown->AddVarint(num, val);
}
inline void WriteLengthDelimited(uint32_t num, absl::string_view val,
                                 UnknownFieldSet* unknown) {
  unknown->AddLengthDelimited(num, val);
}

PROTOBUF_EXPORT
const char* UnknownGroupParse(UnknownFieldSet* unknown, const char* ptr,
                              ParseContext* ctx);
PROTOBUF_EXPORT
const char* UnknownFieldParse(uint64_t tag, UnknownFieldSet* unknown,
                              const char* ptr, ParseContext* ctx);

}  // namespace internal

// ===================================================================
// inline implementations

constexpr UnknownFieldSet::UnknownFieldSet() = default;

inline UnknownFieldSet::~UnknownFieldSet() { Clear(); }

inline const UnknownFieldSet& UnknownFieldSet::default_instance() {
  PROTOBUF_ATTRIBUTE_NO_DESTROY PROTOBUF_CONSTINIT static const UnknownFieldSet
      instance;
  return instance;
}

inline void UnknownFieldSet::ClearAndFreeMemory() { Clear(); }

inline void UnknownFieldSet::Clear() {
  if (!fields_.empty()) {
    ClearFallback();
  }
}

inline bool UnknownFieldSet::empty() const { return fields_.empty(); }

inline void UnknownFieldSet::Swap(UnknownFieldSet* x) {
  if (arena() == x->arena()) {
    fields_.Swap(&x->fields_);
  } else {
    // We might need to do a deep copy, so use Merge instead
    SwapSlow(x);
  }
}

inline int UnknownFieldSet::field_count() const {
  return static_cast<int>(fields_.size());
}
inline const UnknownField& UnknownFieldSet::field(int index) const {
  return (fields_)[static_cast<size_t>(index)];
}
inline UnknownField* UnknownFieldSet::mutable_field(int index) {
  return &(fields_)[static_cast<size_t>(index)];
}

inline void UnknownFieldSet::AddLengthDelimited(int number,
                                                const absl::string_view value) {
  AddLengthDelimited(number)->assign(value.data(), value.size());
}

inline int UnknownField::number() const { return static_cast<int>(number_); }
inline UnknownField::Type UnknownField::type() const {
  return static_cast<Type>(type_);
}

inline uint64_t UnknownField::varint() const {
  assert(type() == TYPE_VARINT);
  return data_.varint;
}
inline uint32_t UnknownField::fixed32() const {
  assert(type() == TYPE_FIXED32);
  return data_.fixed32;
}
inline uint64_t UnknownField::fixed64() const {
  assert(type() == TYPE_FIXED64);
  return data_.fixed64;
}
inline internal::UFSStringView UnknownField::length_delimited() const {
  assert(type() == TYPE_LENGTH_DELIMITED);
  return *data_.string_value;
}
inline const UnknownFieldSet& UnknownField::group() const {
  assert(type() == TYPE_GROUP);
  return *data_.group;
}

inline void UnknownField::set_varint(uint64_t value) {
  assert(type() == TYPE_VARINT);
  data_.varint = value;
}
inline void UnknownField::set_fixed32(uint32_t value) {
  assert(type() == TYPE_FIXED32);
  data_.fixed32 = value;
}
inline void UnknownField::set_fixed64(uint64_t value) {
  assert(type() == TYPE_FIXED64);
  data_.fixed64 = value;
}
inline void UnknownField::set_length_delimited(const absl::string_view value) {
  assert(type() == TYPE_LENGTH_DELIMITED);
  data_.string_value->assign(value.data(), value.size());
}
template <int&...>
inline void UnknownField::set_length_delimited(std::string&& value) {
  assert(type() == TYPE_LENGTH_DELIMITED);
  *data_.string_value = std::move(value);
}
inline void UnknownField::set_length_delimited(const absl::Cord& value) {
  assert(type() == TYPE_LENGTH_DELIMITED);
  absl::CopyCordToString(value, data_.string_value);
}
#if !defined(PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE)
inline std::string* UnknownField::mutable_length_delimited() {
  assert(type() == TYPE_LENGTH_DELIMITED);
  return data_.string_value;
}
#endif  // PROTOBUF_FUTURE_STRING_VIEW_RETURN_TYPE
inline UnknownFieldSet* UnknownField::mutable_group() {
  assert(type() == TYPE_GROUP);
  return data_.group;
}
template <typename MessageType>
bool UnknownFieldSet::MergeFromMessage(const MessageType& message) {
  // SFINAE will route to the right version.
  return InternalMergeFromMessage(message);
}

inline size_t UnknownField::GetLengthDelimitedSize() const {
  ABSL_DCHECK_EQ(TYPE_LENGTH_DELIMITED, type());
  return data_.string_value->size();
}

inline void UnknownField::SetType(Type type) { type_ = type; }

extern template void UnknownFieldSet::AddLengthDelimited(int, std::string&&);

namespace internal {

// Add specialization of InternalMetadata::Container to provide arena support.
template <>
struct InternalMetadata::Container<UnknownFieldSet>
    : public InternalMetadata::ContainerBase {
  UnknownFieldSet unknown_fields;

  explicit Container(Arena* input_arena)
      : unknown_fields(InternalVisibility{}, input_arena) {}

  using InternalArenaConstructable_ = void;
  using DestructorSkippable_ = void;
};
}  // namespace internal

}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_UNKNOWN_FIELD_SET_H__
