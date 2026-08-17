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
// This header is logically internal, but is made public because it is used
// from protocol-compiler-generated code, which may reside in other components.

#ifndef GOOGLE_PROTOBUF_GENERATED_MESSAGE_REFLECTION_H__
#define GOOGLE_PROTOBUF_GENERATED_MESSAGE_REFLECTION_H__

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <string>

#include "absl/base/call_once.h"
#include "absl/base/optimization.h"
#include "absl/log/absl_check.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/generated_enum_reflection.h"
#include "google/protobuf/unknown_field_set.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

namespace google {
namespace protobuf {
class MapKey;
class MapValueRef;
class MessageLayoutInspector;
class Message;
struct Metadata;

namespace io {
class CodedOutputStream;
}
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace internal {
class DefaultEmptyOneof;
// Defined in other files.
class ExtensionSet;  // extension_set.h
class WeakFieldMap;  // weak_field_map.h

// Tag used on offsets for fields that don't have a real offset.
// For example, weak message fields go into the WeakFieldMap and not in an
// actual field.
constexpr uint32_t kInvalidFieldOffsetTag = 0x40000000u;

// Mask used on offsets for split fields.
constexpr uint32_t kSplitFieldOffsetMask = 0x80000000u;
constexpr uint32_t kLazyMask = 0x1u;
constexpr uint32_t kInlinedMask = 0x1u;

// This struct describes the internal layout of the message, hence this is
// used to act on the message reflectively.
//   default_instance:  The default instance of the message.  This is only
//                  used to obtain pointers to default instances of embedded
//                  messages, which GetMessage() will return if the particular
//                  sub-message has not been initialized yet.  (Thus, all
//                  embedded message fields *must* have non-null pointers
//                  in the default instance.)
//   offsets:       An array of ints giving the byte offsets.
//                  For each oneof or weak field, the offset is relative to the
//                  default_instance. These can be computed at compile time
//                  using the
//                  PROTO2_GENERATED_DEFAULT_ONEOF_FIELD_OFFSET()
//                  macro. For each none oneof field, the offset is related to
//                  the start of the message object.  These can be computed at
//                  compile time using the
//                  PROTO2_GENERATED_MESSAGE_FIELD_OFFSET() macro.
//                  Besides offsets for all fields, this array also contains
//                  offsets for oneof unions. The offset of the i-th oneof union
//                  is offsets[descriptor->field_count() + i].
//   has_bit_indices:  Mapping from field indexes to their index in the has
//                  bit array.
//   has_bits_offset:  Offset in the message of an array of uint32s of size
//                  descriptor->field_count()/32, rounded up.  This is a
//                  bitfield where each bit indicates whether or not the
//                  corresponding field of the message has been initialized.
//                  The bit for field index i is obtained by the expression:
//                    has_bits[i / 32] & (1 << (i % 32))
//   unknown_fields_offset:  Offset in the message of the UnknownFieldSet for
//                  the message.
//   extensions_offset:  Offset in the message of the ExtensionSet for the
//                  message, or -1 if the message type has no extension
//                  ranges.
//   oneof_case_offset:  Offset in the message of an array of uint32s of
//                  size descriptor->oneof_decl_count().  Each uint32_t
//                  indicates what field is set for each oneof.
//   object_size:   The size of a message object of this type, as measured
//                  by sizeof().
//   arena_offset:  If a message doesn't have a unknown_field_set that stores
//                  the arena, it must have a direct pointer to the arena.
//   weak_field_map_offset: If the message proto has weak fields, this is the
//                  offset of _weak_field_map_ in the generated proto. Otherwise
//                  -1.
struct ReflectionSchema {
 public:
  // Size of a google::protobuf::Message object of this type.
  uint32_t GetObjectSize() const { return static_cast<uint32_t>(object_size_); }

  bool InRealOneof(const FieldDescriptor* field) const {
    return field->real_containing_oneof();
  }

  // Offset of a non-oneof field.  Getting a field offset is slightly more
  // efficient when we know statically that it is not a oneof field.
  uint32_t GetFieldOffsetNonOneof(const FieldDescriptor* field) const {
    ABSL_DCHECK(!InRealOneof(field));
    return OffsetValue(offsets_[field->index()], field->type());
  }

  // Offset of any field.
  uint32_t GetFieldOffset(const FieldDescriptor* field) const {
    if (InRealOneof(field)) {
      size_t offset =
          static_cast<size_t>(field->containing_type()->field_count()) +
          field->containing_oneof()->index();
      return OffsetValue(offsets_[offset], field->type());
    } else {
      return GetFieldOffsetNonOneof(field);
    }
  }

  bool IsFieldInlined(const FieldDescriptor* field) const {
    return Inlined(offsets_[field->index()], field->type());
  }

  uint32_t GetOneofCaseOffset(const OneofDescriptor* oneof_descriptor) const {
    return static_cast<uint32_t>(oneof_case_offset_) +
           static_cast<uint32_t>(
               static_cast<size_t>(oneof_descriptor->index()) *
               sizeof(uint32_t));
  }

  // Returns true iff the field object has usable hasbit offset.
  // Note that this is not necessarily correlated with *field presence* :
  // Fields with implicit presence (i.e. ones that don't expose has_foo API)
  // can still have hasbits in their underlying implementation.
  bool HasHasbits() const { return has_bits_offset_ != -1; }

  // Bit index within the bit array of hasbits.  Bit order is low-to-high.
  uint32_t HasBitIndex(const FieldDescriptor* field) const {
    ABSL_DCHECK(!field->is_extension());
    if (has_bits_offset_ == -1) return static_cast<uint32_t>(-1);
    ABSL_DCHECK(HasHasbits());
    return has_bit_indices_[field->index()];
  }

  // Byte offset of the hasbits array.
  uint32_t HasBitsOffset() const {
    ABSL_DCHECK(HasHasbits());
    return static_cast<uint32_t>(has_bits_offset_);
  }

  bool HasInlinedString() const { return inlined_string_donated_offset_ != -1; }

  // Bit index within the bit array of _inlined_string_donated_.  Bit order is
  // low-to-high.
  uint32_t InlinedStringIndex(const FieldDescriptor* field) const {
    ABSL_DCHECK(HasInlinedString());
    return inlined_string_indices_[field->index()];
  }

  // Byte offset of the _inlined_string_donated_ array.
  uint32_t InlinedStringDonatedOffset() const {
    ABSL_DCHECK(HasInlinedString());
    return static_cast<uint32_t>(inlined_string_donated_offset_);
  }

  // The offset of the InternalMetadataWithArena member.
  // For Lite this will actually be an InternalMetadataWithArenaLite.
  // The schema doesn't contain enough information to distinguish between
  // these two cases.
  uint32_t GetMetadataOffset() const {
    return static_cast<uint32_t>(metadata_offset_);
  }

  // Whether this message has an ExtensionSet.
  bool HasExtensionSet() const { return extensions_offset_ != -1; }

  // The offset of the ExtensionSet in this message.
  uint32_t GetExtensionSetOffset() const {
    ABSL_DCHECK(HasExtensionSet());
    return static_cast<uint32_t>(extensions_offset_);
  }

  // The off set of WeakFieldMap when the message contains weak fields.
  // The default is 0 for now.
  int GetWeakFieldMapOffset() const { return weak_field_map_offset_; }

  bool IsDefaultInstance(const Message& message) const {
    return &message == default_instance_;
  }

  // Returns a pointer to the default value for this field.  The size and type
  // of the underlying data depends on the field's type.
  const void* GetFieldDefault(const FieldDescriptor* field) const {
    return reinterpret_cast<const uint8_t*>(default_instance_) +
           OffsetValue(offsets_[field->index()], field->type());
  }

  // Returns true if the field is implicitly backed by LazyField.
  bool IsEagerlyVerifiedLazyField(const FieldDescriptor* field) const {
    ABSL_DCHECK_EQ(field->type(), FieldDescriptor::TYPE_MESSAGE);
    (void)field;
    return false;
  }

  bool IsSplit() const { return split_offset_ != -1; }

  bool IsSplit(const FieldDescriptor* field) const {
    return split_offset_ != -1 &&
           (offsets_[field->index()] & kSplitFieldOffsetMask) != 0;
  }

  // Byte offset of _split_.
  uint32_t SplitOffset() const {
    ABSL_DCHECK(IsSplit());
    return static_cast<uint32_t>(split_offset_);
  }

  uint32_t SizeofSplit() const {
    ABSL_DCHECK(IsSplit());
    return static_cast<uint32_t>(sizeof_split_);
  }


  bool HasWeakFields() const { return weak_field_map_offset_ > 0; }

  // These members are intended to be private, but we cannot actually make them
  // private because this prevents us from using aggregate initialization of
  // them, ie.
  //
  //   ReflectionSchema schema = {a, b, c, d, e, ...};
  // private:
  const Message* default_instance_;
  const uint32_t* offsets_;
  const uint32_t* has_bit_indices_;
  int has_bits_offset_;
  int metadata_offset_;
  int extensions_offset_;
  int oneof_case_offset_;
  int object_size_;
  int weak_field_map_offset_;
  const uint32_t* inlined_string_indices_;
  int inlined_string_donated_offset_;
  int split_offset_;
  int sizeof_split_;

  // We tag offset values to provide additional data about fields (such as
  // "unused" or "lazy" or "inlined").
  static uint32_t OffsetValue(uint32_t v, FieldDescriptor::Type type) {
    if (type == FieldDescriptor::TYPE_MESSAGE ||
        type == FieldDescriptor::TYPE_STRING ||
        type == FieldDescriptor::TYPE_BYTES) {
      return v & (~kSplitFieldOffsetMask) & (~kInlinedMask) & (~kLazyMask);
    }
    return v & (~kSplitFieldOffsetMask);
  }

  static bool Inlined(uint32_t v, FieldDescriptor::Type type) {
    if (type == FieldDescriptor::TYPE_STRING ||
        type == FieldDescriptor::TYPE_BYTES) {
      return (v & kInlinedMask) != 0u;
    } else {
      // Non string/byte fields are not inlined.
      return false;
    }
  }
};

// Structs that the code generator emits directly to describe a message.
// These should never used directly except to build a ReflectionSchema
// object.
//
// EXPERIMENTAL: these are changing rapidly, and may completely disappear
// or merge with ReflectionSchema.
struct MigrationSchema {
  int32_t offsets_index;
  int32_t has_bit_indices_index;
  int32_t inlined_string_indices_index;
  int object_size;
};

// This struct tries to reduce unnecessary padding.
// The num_xxx might not be close to their respective pointer, but this saves
// padding.
struct PROTOBUF_EXPORT DescriptorTable {
  mutable bool is_initialized;
  bool is_eager;
  int size;  // of serialized descriptor
  const char* descriptor;
  const char* filename;
  absl::once_flag* once;
  const DescriptorTable* const* deps;
  int num_deps;
  int num_messages;
  const MigrationSchema* schemas;
  const Message* const* default_instances;
  const uint32_t* offsets;
  // update the following descriptor arrays.
  const EnumDescriptor** file_level_enum_descriptors;
  const ServiceDescriptor** file_level_service_descriptors;
};

// AssignDescriptors() pulls the compiled FileDescriptor from the DescriptorPool
// and uses it to populate all of the global variables which store pointers to
// the descriptor objects.  It also constructs the reflection objects.  It is
// called the first time anyone calls descriptor() or GetReflection() on one of
// the types defined in the file.  AssignDescriptors() is thread-safe.
void PROTOBUF_EXPORT AssignDescriptors(const DescriptorTable* table);
// As above, but the caller did the call_once call already.
void PROTOBUF_EXPORT
AssignDescriptorsOnceInnerCall(const DescriptorTable* table);

// These cannot be in lite so we put them in the reflection.
PROTOBUF_EXPORT void UnknownFieldSetSerializer(const uint8_t* base,
                                               uint32_t offset, uint32_t tag,
                                               uint32_t has_offset,
                                               io::CodedOutputStream* output);

PROTOBUF_EXPORT void InitializeFileDescriptorDefaultInstances();

PROTOBUF_EXPORT void AddDescriptors(const DescriptorTable* table);

struct PROTOBUF_EXPORT AddDescriptorsRunner {
  explicit AddDescriptorsRunner(const DescriptorTable* table);
};

// Retrieves the existing prototype out of a descriptor table.
// If it doesn't exist:
//  - If force_build is true, asks the generated message factory for one.
//  - Otherwise, return null
const Message* GetPrototypeForWeakDescriptor(const DescriptorTable* table,
                                             int index, bool force_build);

struct DenseEnumCacheInfo {
  std::atomic<const std::string**> cache;
  int min_val;
  int max_val;
  const EnumDescriptor* (*descriptor_fn)();
};
PROTOBUF_EXPORT const std::string& NameOfDenseEnumSlow(int v,
                                                       DenseEnumCacheInfo*);

// Similar to the routine NameOfEnum, this routine returns the name of an enum.
// Unlike that routine, it allocates, on-demand, a block of pointers to the
// std::string objects allocated by reflection to store the enum names. This
// way, as long as the enum values are fairly dense, looking them up can be
// very fast. This assumes all the enums fall in the range [min_val .. max_val].
template <const EnumDescriptor* (*descriptor_fn)(), int min_val, int max_val>
const std::string& NameOfDenseEnum(int v) {
  static_assert(max_val - min_val >= 0, "Too many enums between min and max.");
  static DenseEnumCacheInfo deci = {/* atomic ptr */ {}, min_val, max_val,
                                    descriptor_fn};
  const std::string** cache = deci.cache.load(std::memory_order_acquire );
  if (ABSL_PREDICT_TRUE(cache != nullptr)) {
    if (ABSL_PREDICT_TRUE(v >= min_val && v <= max_val)) {
      return *cache[v - min_val];
    }
  }
  return NameOfDenseEnumSlow(v, &deci);
}

// Returns whether this type of field is stored in the split struct as a raw
// pointer.
PROTOBUF_EXPORT bool SplitFieldHasExtraIndirection(
    const FieldDescriptor* field);

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_GENERATED_MESSAGE_REFLECTION_H__
