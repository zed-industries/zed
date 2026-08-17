// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//         atenasio@google.com (Chris Atenasio) (ZigZag transform)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This header is logically internal, but is made public because it is used
// from protocol-compiler-generated code, which may reside in other components.

#ifndef GOOGLE_PROTOBUF_WIRE_FORMAT_H__
#define GOOGLE_PROTOBUF_WIRE_FORMAT_H__

#include <cstddef>
#include <cstdint>

#include "absl/base/casts.h"
#include "absl/log/absl_check.h"
#include "absl/strings/cord.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/generated_message_util.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/message.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/parse_context.h"
#include "google/protobuf/wire_format_lite.h"

#ifdef SWIG
#error "You cannot SWIG proto headers"
#endif

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
class MapKey;           // map_field.h
class UnknownFieldSet;  // unknown_field_set.h
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace internal {

class TcParser;

// This class is for internal use by the protocol buffer library and by
// protocol-compiler-generated message classes.  It must not be called
// directly by clients.
//
// This class contains code for implementing the binary protocol buffer
// wire format via reflection.  The WireFormatLite class implements the
// non-reflection based routines.
//
// This class is really a namespace that contains only static methods
class PROTOBUF_EXPORT WireFormat {
 public:
  WireFormat() = delete;

  // Given a field return its WireType
  static inline WireFormatLite::WireType WireTypeForField(
      const FieldDescriptor* field);

  // Given a FieldDescriptor::Type return its WireType
  static inline WireFormatLite::WireType WireTypeForFieldType(
      FieldDescriptor::Type type);

  // Compute the byte size of a tag.  For groups, this includes both the start
  // and end tags.
  static inline size_t TagSize(int field_number, FieldDescriptor::Type type);

  // These procedures can be used to implement the methods of Message which
  // handle parsing and serialization of the protocol buffer wire format
  // using only the Reflection interface.  When you ask the protocol
  // compiler to optimize for code size rather than speed, it will implement
  // those methods in terms of these procedures.  Of course, these are much
  // slower than the specialized implementations which the protocol compiler
  // generates when told to optimize for speed.

  // Read a message in protocol buffer wire format.
  //
  // This procedure reads either to the end of the input stream or through
  // a WIRETYPE_END_GROUP tag ending the message, whichever comes first.
  // It returns false if the input is invalid.
  //
  // Required fields are NOT checked by this method.  You must call
  // IsInitialized() on the resulting message yourself.
  static bool ParseAndMergePartial(io::CodedInputStream* input,
                                   Message* message);

  // This is meant for internal protobuf use (WireFormat is an internal class).
  // This is the reflective implementation of the _InternalParse functionality.
  static const char* _InternalParse(Message* msg, const char* ptr,
                                    internal::ParseContext* ctx);

  // Serialize a message in protocol buffer wire format.
  //
  // Any embedded messages within the message must have their correct sizes
  // cached.  However, the top-level message need not; its size is passed as
  // a parameter to this procedure.
  //
  // These return false iff the underlying stream returns a write error.
  static void SerializeWithCachedSizes(const Message& message, int size,
                                       io::CodedOutputStream* output) {
    int expected_endpoint = output->ByteCount() + size;
    output->SetCur(
        _InternalSerialize(message, output->Cur(), output->EpsCopy()));
    ABSL_CHECK_EQ(output->ByteCount(), expected_endpoint)
        << ": Protocol message serialized to a size different from what was "
           "originally expected.  Perhaps it was modified by another thread "
           "during serialization?";
  }
  static uint8_t* _InternalSerialize(const Message& message, uint8_t* target,
                                     io::EpsCopyOutputStream* stream);

  // Implements Message::ByteSize() via reflection.  WARNING:  The result
  // of this method is *not* cached anywhere.  However, all embedded messages
  // will have their ByteSize() methods called, so their sizes will be cached.
  // Therefore, calling this method is sufficient to allow you to call
  // WireFormat::SerializeWithCachedSizes() on the same object.
  static size_t ByteSize(const Message& message);

  // -----------------------------------------------------------------
  // Helpers for dealing with unknown fields

  // Skips a field value of the given WireType.  The input should start
  // positioned immediately after the tag.  If unknown_fields is non-nullptr,
  // the contents of the field will be added to it.
  static bool SkipField(io::CodedInputStream* input, uint32_t tag,
                        UnknownFieldSet* unknown_fields);

  // Reads and ignores a message from the input.  If unknown_fields is
  // non-nullptr, the contents will be added to it.
  static bool SkipMessage(io::CodedInputStream* input,
                          UnknownFieldSet* unknown_fields);

  // Read a packed enum field. If the is_valid function is not nullptr, values
  // for which is_valid(value) returns false are appended to
  // unknown_fields_stream.
  static bool ReadPackedEnumPreserveUnknowns(io::CodedInputStream* input,
                                             uint32_t field_number,
                                             bool (*is_valid)(int),
                                             UnknownFieldSet* unknown_fields,
                                             RepeatedField<int>* values);

  // Write the contents of an UnknownFieldSet to the output.
  static void SerializeUnknownFields(const UnknownFieldSet& unknown_fields,
                                     io::CodedOutputStream* output) {
    output->SetCur(InternalSerializeUnknownFieldsToArray(
        unknown_fields, output->Cur(), output->EpsCopy()));
  }
  // Same as above, except writing directly to the provided buffer.
  // Requires that the buffer have sufficient capacity for
  // ComputeUnknownFieldsSize(unknown_fields).
  //
  // Returns a pointer past the last written byte.
  static uint8_t* SerializeUnknownFieldsToArray(
      const UnknownFieldSet& unknown_fields, uint8_t* target) {
    io::EpsCopyOutputStream stream(
        target, static_cast<int>(ComputeUnknownFieldsSize(unknown_fields)),
        io::CodedOutputStream::IsDefaultSerializationDeterministic());
    return InternalSerializeUnknownFieldsToArray(unknown_fields, target,
                                                 &stream);
  }
  static uint8_t* InternalSerializeUnknownFieldsToArray(
      const UnknownFieldSet& unknown_fields, uint8_t* target,
      io::EpsCopyOutputStream* stream);

  // Same thing except for messages that have the message_set_wire_format
  // option.
  static void SerializeUnknownMessageSetItems(
      const UnknownFieldSet& unknown_fields, io::CodedOutputStream* output) {
    output->SetCur(InternalSerializeUnknownMessageSetItemsToArray(
        unknown_fields, output->Cur(), output->EpsCopy()));
  }
  // Same as above, except writing directly to the provided buffer.
  // Requires that the buffer have sufficient capacity for
  // ComputeUnknownMessageSetItemsSize(unknown_fields).
  //
  // Returns a pointer past the last written byte.
  static uint8_t* SerializeUnknownMessageSetItemsToArray(
      const UnknownFieldSet& unknown_fields, uint8_t* target);
  static uint8_t* InternalSerializeUnknownMessageSetItemsToArray(
      const UnknownFieldSet& unknown_fields, uint8_t* target,
      io::EpsCopyOutputStream* stream);

  // Compute the size of the UnknownFieldSet on the wire.
  static size_t ComputeUnknownFieldsSize(const UnknownFieldSet& unknown_fields);

  // Same thing except for messages that have the message_set_wire_format
  // option.
  static size_t ComputeUnknownMessageSetItemsSize(
      const UnknownFieldSet& unknown_fields);

  // Helper functions for encoding and decoding tags.  (Inlined below and in
  // _inl.h)
  //
  // This is different from MakeTag(field->number(), field->type()) in the
  // case of packed repeated fields.
  static uint32_t MakeTag(const FieldDescriptor* field);

  // Parse a single field.  The input should start out positioned immediately
  // after the tag.
  static bool ParseAndMergeField(
      uint32_t tag,
      const FieldDescriptor* field,  // May be nullptr for unknown
      Message* message, io::CodedInputStream* input);

  // Serialize a single field.
  static void SerializeFieldWithCachedSizes(
      const FieldDescriptor* field,  // Cannot be nullptr
      const Message& message, io::CodedOutputStream* output) {
    output->SetCur(InternalSerializeField(field, message, output->Cur(),
                                          output->EpsCopy()));
  }
  static uint8_t* InternalSerializeField(
      const FieldDescriptor* field,  // Cannot be nullptr
      const Message& message, uint8_t* target, io::EpsCopyOutputStream* stream);

  // Compute size of a single field.  If the field is a message type, this
  // will call ByteSize() for the embedded message, insuring that it caches
  // its size.
  static size_t FieldByteSize(const FieldDescriptor* field,  // Can't be nullptr
                              const Message& message);

  // Parse/serialize a MessageSet::Item group.  Used with messages that use
  // option message_set_wire_format = true.
  static bool ParseAndMergeMessageSetItem(io::CodedInputStream* input,
                                          Message* message);
  static void SerializeMessageSetItemWithCachedSizes(
      const FieldDescriptor* field, const Message& message,
      io::CodedOutputStream* output) {
    output->SetCur(InternalSerializeMessageSetItem(
        field, message, output->Cur(), output->EpsCopy()));
  }
  static uint8_t* InternalSerializeMessageSetItem(
      const FieldDescriptor* field, const Message& message, uint8_t* target,
      io::EpsCopyOutputStream* stream);
  static size_t MessageSetItemByteSize(const FieldDescriptor* field,
                                       const Message& message);

  // Computes the byte size of a field, excluding tags. For packed fields, it
  // only includes the size of the raw data, and not the size of the total
  // length, but for other length-prefixed types, the size of the length is
  // included.
  static size_t FieldDataOnlyByteSize(
      const FieldDescriptor* field,  // Cannot be nullptr
      const Message& message);

  enum Operation {
    PARSE = 0,
    SERIALIZE = 1,
  };

  // Verifies that a string field is valid UTF8, logging an error if not.
  // This function will not be called by newly generated protobuf code
  // but remains present to support existing code.
  static void VerifyUTF8String(const char* data, int size, Operation op);
  // The NamedField variant takes a field name in order to produce an
  // informative error message if verification fails.
  static void VerifyUTF8StringNamedField(const char* data, int size,
                                         Operation op,
                                         absl::string_view field_name);

 private:
  struct MessageSetParser;
  friend class TcParser;
  // Skip a MessageSet field.
  static bool SkipMessageSetField(io::CodedInputStream* input,
                                  uint32_t field_number,
                                  UnknownFieldSet* unknown_fields);

  // Parse a MessageSet field.
  static bool ParseAndMergeMessageSetField(uint32_t field_number,
                                           const FieldDescriptor* field,
                                           Message* message,
                                           io::CodedInputStream* input);
  // Parses the value from the wire that belongs to tag.
  static const char* _InternalParseAndMergeField(Message* msg, const char* ptr,
                                                 internal::ParseContext* ctx,
                                                 uint64_t tag,
                                                 const Reflection* reflection,
                                                 const FieldDescriptor* field);
};

// Subclass of FieldSkipper which saves skipped fields to an UnknownFieldSet.
class PROTOBUF_EXPORT UnknownFieldSetFieldSkipper : public FieldSkipper {
 public:
  explicit UnknownFieldSetFieldSkipper(UnknownFieldSet* unknown_fields)
      : unknown_fields_(unknown_fields) {}
  ~UnknownFieldSetFieldSkipper() override = default;

  // implements FieldSkipper -----------------------------------------
  bool SkipField(io::CodedInputStream* input, uint32_t tag) override;
  bool SkipMessage(io::CodedInputStream* input) override;
  void SkipUnknownEnum(int field_number, int value) override;

 protected:
  UnknownFieldSet* unknown_fields_;
};

// inline methods ====================================================

inline WireFormatLite::WireType WireFormat::WireTypeForField(
    const FieldDescriptor* field) {
  if (field->is_packed()) {
    return WireFormatLite::WIRETYPE_LENGTH_DELIMITED;
  } else {
    return WireTypeForFieldType(field->type());
  }
}

inline WireFormatLite::WireType WireFormat::WireTypeForFieldType(
    FieldDescriptor::Type type) {
  // Some compilers don't like enum -> enum casts, so we implicit_cast to
  // int first.
  return WireFormatLite::WireTypeForFieldType(
      static_cast<WireFormatLite::FieldType>(absl::implicit_cast<int>(type)));
}

inline uint32_t WireFormat::MakeTag(const FieldDescriptor* field) {
  return WireFormatLite::MakeTag(field->number(), WireTypeForField(field));
}

inline size_t WireFormat::TagSize(int field_number,
                                  FieldDescriptor::Type type) {
  // Some compilers don't like enum -> enum casts, so we implicit_cast to
  // int first.
  return WireFormatLite::TagSize(
      field_number,
      static_cast<WireFormatLite::FieldType>(absl::implicit_cast<int>(type)));
}

inline void WireFormat::VerifyUTF8String(const char* data, int size,
                                         WireFormat::Operation op) {
#ifdef GOOGLE_PROTOBUF_UTF8_VALIDATION_ENABLED
  WireFormatLite::VerifyUtf8String(data, size,
                                   static_cast<WireFormatLite::Operation>(op),
                                   /* field_name = */ "");
#else
  // Avoid the compiler warning about unused variables.
  (void)data;
  (void)size;
  (void)op;
#endif
}

inline void WireFormat::VerifyUTF8StringNamedField(
    const char* data, int size, WireFormat::Operation op,
    const absl::string_view field_name) {
#ifdef GOOGLE_PROTOBUF_UTF8_VALIDATION_ENABLED
  WireFormatLite::VerifyUtf8String(
      data, size, static_cast<WireFormatLite::Operation>(op), field_name);
#else
  // Avoid the compiler warning about unused variables.
  (void)data;
  (void)size;
  (void)op;
  (void)field_name;
#endif
}


inline uint8_t* InternalSerializeUnknownMessageSetItemsToArray(
    const UnknownFieldSet& unknown_fields, uint8_t* target,
    io::EpsCopyOutputStream* stream) {
  return WireFormat::InternalSerializeUnknownMessageSetItemsToArray(
      unknown_fields, target, stream);
}

inline size_t ComputeUnknownMessageSetItemsSize(
    const UnknownFieldSet& unknown_fields) {
  return WireFormat::ComputeUnknownMessageSetItemsSize(unknown_fields);
}

// Compute the size of the UnknownFieldSet on the wire.
PROTOBUF_EXPORT
size_t ComputeUnknownFieldsSize(const InternalMetadata& metadata, size_t size,
                                CachedSize* cached_size);

size_t MapKeyDataOnlyByteSize(const FieldDescriptor* field,
                              const MapKey& value);

uint8_t* SerializeMapKeyWithCachedSizes(const FieldDescriptor* field,
                                        const MapKey& value, uint8_t* target,
                                        io::EpsCopyOutputStream* stream);
}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_WIRE_FORMAT_H__
