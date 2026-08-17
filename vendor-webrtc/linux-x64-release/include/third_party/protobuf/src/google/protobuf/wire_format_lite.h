// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//         atenasio@google.com (Chris Atenasio) (ZigZag transform)
//         wink@google.com (Wink Saville) (refactored from wire_format.h)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// This header is logically internal, but is made public because it is used
// from protocol-compiler-generated code, which may reside in other components.

#ifndef GOOGLE_PROTOBUF_WIRE_FORMAT_LITE_H__
#define GOOGLE_PROTOBUF_WIRE_FORMAT_LITE_H__

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <string>
#include <utility>

#include "absl/base/casts.h"
#include "absl/base/optimization.h"
#include "absl/log/absl_check.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/port.h"
#include "google/protobuf/repeated_field.h"

#ifndef NDEBUG
#define GOOGLE_PROTOBUF_UTF8_VALIDATION_ENABLED
#endif

// Avoid conflict with iOS where <ConditionalMacros.h> #defines TYPE_BOOL.
//
// If some one needs the macro TYPE_BOOL in a file that includes this header,
// it's possible to bring it back using push/pop_macro as follows.
//
// #pragma push_macro("TYPE_BOOL")
// #include this header and/or all headers that need the macro to be undefined.
// #pragma pop_macro("TYPE_BOOL")
#undef TYPE_BOOL


// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// This class is for internal use by the protocol buffer library and by
// protocol-compiler-generated message classes.  It must not be called
// directly by clients.
//
// This class contains helpers for implementing the binary protocol buffer
// wire format without the need for reflection. Use WireFormat when using
// reflection.
//
// This class is really a namespace that contains only static methods.
class PROTOBUF_EXPORT WireFormatLite {
 public:
  WireFormatLite() = delete;
  // -----------------------------------------------------------------
  // Helper constants and functions related to the format.  These are
  // mostly meant for internal and generated code to use.

  // The wire format is composed of a sequence of tag/value pairs, each
  // of which contains the value of one field (or one element of a repeated
  // field).  Each tag is encoded as a varint.  The lower bits of the tag
  // identify its wire type, which specifies the format of the data to follow.
  // The rest of the bits contain the field number.  Each type of field (as
  // declared by FieldDescriptor::Type, in descriptor.h) maps to one of
  // these wire types.  Immediately following each tag is the field's value,
  // encoded in the format specified by the wire type.  Because the tag
  // identifies the encoding of this data, it is possible to skip
  // unrecognized fields for forwards compatibility.

  enum WireType
#ifndef SWIG
      : int
#endif  // !SWIG
  {
    WIRETYPE_VARINT = 0,
    WIRETYPE_FIXED64 = 1,
    WIRETYPE_LENGTH_DELIMITED = 2,
    WIRETYPE_START_GROUP = 3,
    WIRETYPE_END_GROUP = 4,
    WIRETYPE_FIXED32 = 5,
  };

  // Lite alternative to FieldDescriptor::Type.  Must be kept in sync.
  enum FieldType {
    TYPE_DOUBLE = 1,
    TYPE_FLOAT = 2,
    TYPE_INT64 = 3,
    TYPE_UINT64 = 4,
    TYPE_INT32 = 5,
    TYPE_FIXED64 = 6,
    TYPE_FIXED32 = 7,
    TYPE_BOOL = 8,
    TYPE_STRING = 9,
    TYPE_GROUP = 10,
    TYPE_MESSAGE = 11,
    TYPE_BYTES = 12,
    TYPE_UINT32 = 13,
    TYPE_ENUM = 14,
    TYPE_SFIXED32 = 15,
    TYPE_SFIXED64 = 16,
    TYPE_SINT32 = 17,
    TYPE_SINT64 = 18,
    MAX_FIELD_TYPE = 18,
  };

  // Lite alternative to FieldDescriptor::CppType.  Must be kept in sync.
  enum CppType {
    CPPTYPE_INT32 = 1,
    CPPTYPE_INT64 = 2,
    CPPTYPE_UINT32 = 3,
    CPPTYPE_UINT64 = 4,
    CPPTYPE_DOUBLE = 5,
    CPPTYPE_FLOAT = 6,
    CPPTYPE_BOOL = 7,
    CPPTYPE_ENUM = 8,
    CPPTYPE_STRING = 9,
    CPPTYPE_MESSAGE = 10,
    MAX_CPPTYPE = 10,
  };

  // Helper method to get the CppType for a particular Type.
  static CppType FieldTypeToCppType(FieldType type);

  // Given a FieldDescriptor::Type return its WireType
  static inline WireFormatLite::WireType WireTypeForFieldType(
      WireFormatLite::FieldType type) {
    return kWireTypeForFieldType[type];
  }

  // Number of bits in a tag which identify the wire type.
  static constexpr int kTagTypeBits = 3;
  // Mask for those bits.
  static constexpr uint32_t kTagTypeMask = (1 << kTagTypeBits) - 1;

  // Helper functions for encoding and decoding tags.  (Inlined below and in
  // _inl.h)
  //
  // This is different from MakeTag(field->number(), field->type()) in the
  // case of packed repeated fields.
  constexpr static uint32_t MakeTag(int field_number, WireType type);
  static WireType GetTagWireType(uint32_t tag);
  static int GetTagFieldNumber(uint32_t tag);

  // Compute the byte size of a tag.  For groups, this includes both the start
  // and end tags.
  static inline size_t TagSize(int field_number,
                               WireFormatLite::FieldType type);

  // Skips a field value with the given tag.  The input should start
  // positioned immediately after the tag.  Skipped values are simply
  // discarded, not recorded anywhere.  See WireFormat::SkipField() for a
  // version that records to an UnknownFieldSet.
  static bool SkipField(io::CodedInputStream* input, uint32_t tag);

  // Skips a field value with the given tag.  The input should start
  // positioned immediately after the tag. Skipped values are recorded to a
  // CodedOutputStream.
  static bool SkipField(io::CodedInputStream* input, uint32_t tag,
                        io::CodedOutputStream* output);

  // Reads and ignores a message from the input.  Skipped values are simply
  // discarded, not recorded anywhere.  See WireFormat::SkipMessage() for a
  // version that records to an UnknownFieldSet.
  static bool SkipMessage(io::CodedInputStream* input);

  // Reads and ignores a message from the input.  Skipped values are recorded
  // to a CodedOutputStream.
  static bool SkipMessage(io::CodedInputStream* input,
                          io::CodedOutputStream* output);

  // This macro does the same thing as WireFormatLite::MakeTag(), but the
  // result is usable as a compile-time constant, which makes it usable
  // as a switch case or a template input.  WireFormatLite::MakeTag() is more
  // type-safe, though, so prefer it if possible.
#define GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(FIELD_NUMBER, TYPE) \
  static_cast<uint32_t>((static_cast<uint32_t>(FIELD_NUMBER) << 3) | (TYPE))

  // These are the tags for the old MessageSet format, which was defined as:
  //   message MessageSet {
  //     repeated group Item = 1 {
  //       required int32 type_id = 2;
  //       required string message = 3;
  //     }
  //   }
  static constexpr int kMessageSetItemNumber = 1;
  static constexpr int kMessageSetTypeIdNumber = 2;
  static constexpr int kMessageSetMessageNumber = 3;
  static const int kMessageSetItemStartTag = GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(
      kMessageSetItemNumber, WireFormatLite::WIRETYPE_START_GROUP);
  static const int kMessageSetItemEndTag = GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(
      kMessageSetItemNumber, WireFormatLite::WIRETYPE_END_GROUP);
  static const int kMessageSetTypeIdTag = GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(
      kMessageSetTypeIdNumber, WireFormatLite::WIRETYPE_VARINT);
  static const int kMessageSetMessageTag = GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(
      kMessageSetMessageNumber, WireFormatLite::WIRETYPE_LENGTH_DELIMITED);

  // Byte size of all tags of a MessageSet::Item combined.
  static const size_t kMessageSetItemTagsSize;

  // Helper functions for converting between floats/doubles and IEEE-754
  // uint32s/uint64s so that they can be written.  (Assumes your platform
  // uses IEEE-754 floats.)
  static uint32_t EncodeFloat(float value);
  static float DecodeFloat(uint32_t value);
  static uint64_t EncodeDouble(double value);
  static double DecodeDouble(uint64_t value);

  // Helper functions for mapping signed integers to unsigned integers in
  // such a way that numbers with small magnitudes will encode to smaller
  // varints.  If you simply static_cast a negative number to an unsigned
  // number and varint-encode it, it will always take 10 bytes, defeating
  // the purpose of varint.  So, for the "sint32" and "sint64" field types,
  // we ZigZag-encode the values.
  static uint32_t ZigZagEncode32(int32_t n);
  static int32_t ZigZagDecode32(uint32_t n);
  static uint64_t ZigZagEncode64(int64_t n);
  static int64_t ZigZagDecode64(uint64_t n);

  // =================================================================
  // Methods for reading/writing individual field.

  // Read fields, not including tags.  The assumption is that you already
  // read the tag to determine what field to read.

  // For primitive fields, we just use a templatized routine parameterized by
  // the represented type and the FieldType. These are specialized with the
  // appropriate definition for each declared type.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static bool ReadPrimitive(io::CodedInputStream* input,
                                                   CType* value);

  // Reads repeated primitive values, with optimizations for repeats.
  // tag_size and tag should both be compile-time constants provided by the
  // protocol compiler.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static bool ReadRepeatedPrimitive(
      int tag_size, uint32_t tag, io::CodedInputStream* input,
      RepeatedField<CType>* value);

  // Identical to ReadRepeatedPrimitive, except will not inline the
  // implementation.
  template <typename CType, enum FieldType DeclaredType>
  static bool ReadRepeatedPrimitiveNoInline(int tag_size, uint32_t tag,
                                            io::CodedInputStream* input,
                                            RepeatedField<CType>* value);

  // Reads a primitive value directly from the provided buffer. It returns a
  // pointer past the segment of data that was read.
  //
  // This is only implemented for the types with fixed wire size, e.g.
  // float, double, and the (s)fixed* types.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static const uint8_t* ReadPrimitiveFromArray(
      const uint8_t* buffer, CType* value);

  // Reads a primitive packed field.
  //
  // This is only implemented for packable types.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static bool ReadPackedPrimitive(
      io::CodedInputStream* input, RepeatedField<CType>* value);

  // Identical to ReadPackedPrimitive, except will not inline the
  // implementation.
  template <typename CType, enum FieldType DeclaredType>
  static bool ReadPackedPrimitiveNoInline(io::CodedInputStream* input,
                                          RepeatedField<CType>* value);

  // Read a packed enum field. If the is_valid function is not nullptr, values
  // for which is_valid(value) returns false are silently dropped.
  static bool ReadPackedEnumNoInline(io::CodedInputStream* input,
                                     bool (*is_valid)(int),
                                     RepeatedField<int>* values);

  // Read a packed enum field. If the is_valid function is not nullptr, values
  // for which is_valid(value) returns false are appended to
  // unknown_fields_stream.
  static bool ReadPackedEnumPreserveUnknowns(
      io::CodedInputStream* input, int field_number, bool (*is_valid)(int),
      io::CodedOutputStream* unknown_fields_stream, RepeatedField<int>* values);

  // Read a string.  ReadString(..., std::string* value) requires an
  // existing std::string.
  static inline bool ReadString(io::CodedInputStream* input,
                                std::string* value);
  // ReadString(..., std::string** p) is internal-only, and should only be
  // called from generated code. It starts by setting *p to "new std::string" if
  // *p == &GetEmptyStringAlreadyInited().  It then invokes
  // ReadString(io::CodedInputStream* input, *p).  This is useful for reducing
  // code size.
  static inline bool ReadString(io::CodedInputStream* input, std::string** p);
  // Analogous to ReadString().
  static bool ReadBytes(io::CodedInputStream* input, std::string* value);
  static bool ReadBytes(io::CodedInputStream* input, std::string** p);

  static inline bool ReadBytes(io::CodedInputStream* input, absl::Cord* value);
  static inline bool ReadBytes(io::CodedInputStream* input, absl::Cord** p);

  enum Operation {
    PARSE = 0,
    SERIALIZE = 1,
  };

  // Returns true if the data is valid UTF-8.
  static bool VerifyUtf8String(const char* data, int size, Operation op,
                               absl::string_view field_name);

  template <typename MessageType>
  static inline bool ReadGroup(int field_number, io::CodedInputStream* input,
                               MessageType* value);

  template <typename MessageType>
  static inline bool ReadMessage(io::CodedInputStream* input,
                                 MessageType* value);

  template <typename MessageType>
  static inline bool ReadMessageNoVirtual(io::CodedInputStream* input,
                                          MessageType* value) {
    return ReadMessage(input, value);
  }

  // Write a tag.  The Write*() functions typically include the tag, so
  // normally there's no need to call this unless using the Write*NoTag()
  // variants.
  PROTOBUF_NDEBUG_INLINE static void WriteTag(int field_number, WireType type,
                                              io::CodedOutputStream* output);

  // Write fields, without tags.
  PROTOBUF_NDEBUG_INLINE static void WriteInt32NoTag(
      int32_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteInt64NoTag(
      int64_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteUInt32NoTag(
      uint32_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteUInt64NoTag(
      uint64_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteSInt32NoTag(
      int32_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteSInt64NoTag(
      int64_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteFixed32NoTag(
      uint32_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteFixed64NoTag(
      uint64_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteSFixed32NoTag(
      int32_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteSFixed64NoTag(
      int64_t value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteFloatNoTag(
      float value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteDoubleNoTag(
      double value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteBoolNoTag(
      bool value, io::CodedOutputStream* output);
  PROTOBUF_NDEBUG_INLINE static void WriteEnumNoTag(
      int value, io::CodedOutputStream* output);

  // Write array of primitive fields, without tags
  static void WriteFloatArray(const float* a, int n,
                              io::CodedOutputStream* output);
  static void WriteDoubleArray(const double* a, int n,
                               io::CodedOutputStream* output);
  static void WriteFixed32Array(const uint32_t* a, int n,
                                io::CodedOutputStream* output);
  static void WriteFixed64Array(const uint64_t* a, int n,
                                io::CodedOutputStream* output);
  static void WriteSFixed32Array(const int32_t* a, int n,
                                 io::CodedOutputStream* output);
  static void WriteSFixed64Array(const int64_t* a, int n,
                                 io::CodedOutputStream* output);
  static void WriteBoolArray(const bool* a, int n,
                             io::CodedOutputStream* output);

  // Write fields, including tags.
  static void WriteInt32(int field_number, int32_t value,
                         io::CodedOutputStream* output);
  static void WriteInt64(int field_number, int64_t value,
                         io::CodedOutputStream* output);
  static void WriteUInt32(int field_number, uint32_t value,
                          io::CodedOutputStream* output);
  static void WriteUInt64(int field_number, uint64_t value,
                          io::CodedOutputStream* output);
  static void WriteSInt32(int field_number, int32_t value,
                          io::CodedOutputStream* output);
  static void WriteSInt64(int field_number, int64_t value,
                          io::CodedOutputStream* output);
  static void WriteFixed32(int field_number, uint32_t value,
                           io::CodedOutputStream* output);
  static void WriteFixed64(int field_number, uint64_t value,
                           io::CodedOutputStream* output);
  static void WriteSFixed32(int field_number, int32_t value,
                            io::CodedOutputStream* output);
  static void WriteSFixed64(int field_number, int64_t value,
                            io::CodedOutputStream* output);
  static void WriteFloat(int field_number, float value,
                         io::CodedOutputStream* output);
  static void WriteDouble(int field_number, double value,
                          io::CodedOutputStream* output);
  static void WriteBool(int field_number, bool value,
                        io::CodedOutputStream* output);
  static void WriteEnum(int field_number, int value,
                        io::CodedOutputStream* output);

  static void WriteString(int field_number, const std::string& value,
                          io::CodedOutputStream* output);
  static void WriteBytes(int field_number, const std::string& value,
                         io::CodedOutputStream* output);
  static void WriteStringMaybeAliased(int field_number,
                                      const std::string& value,
                                      io::CodedOutputStream* output);
  static void WriteBytesMaybeAliased(int field_number, const std::string& value,
                                     io::CodedOutputStream* output);

  static void WriteGroup(int field_number, const MessageLite& value,
                         io::CodedOutputStream* output);
  static void WriteMessage(int field_number, const MessageLite& value,
                           io::CodedOutputStream* output);
  // Like above, but these will check if the output stream has enough
  // space to write directly to a flat array.
  static void WriteGroupMaybeToArray(int field_number, const MessageLite& value,
                                     io::CodedOutputStream* output);
  static void WriteMessageMaybeToArray(int field_number,
                                       const MessageLite& value,
                                       io::CodedOutputStream* output);

  // Like above, but de-virtualize the call to SerializeWithCachedSizes().  The
  // pointer must point at an instance of MessageType, *not* a subclass (or
  // the subclass must not override SerializeWithCachedSizes()).
  template <typename MessageType>
  static inline void WriteGroupNoVirtual(int field_number,
                                         const MessageType& value,
                                         io::CodedOutputStream* output);
  template <typename MessageType>
  static inline void WriteMessageNoVirtual(int field_number,
                                           const MessageType& value,
                                           io::CodedOutputStream* output);

  // Like above, but use only *ToArray methods of CodedOutputStream.
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteTagToArray(int field_number,
                                                         WireType type,
                                                         uint8_t* target);

  // Write fields, without tags.
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt32NoTagToArray(
      int32_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt64NoTagToArray(
      int64_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt32NoTagToArray(
      uint32_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt64NoTagToArray(
      uint64_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt32NoTagToArray(
      int32_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt64NoTagToArray(
      int64_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed32NoTagToArray(
      uint32_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed64NoTagToArray(
      uint64_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed32NoTagToArray(
      int32_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed64NoTagToArray(
      int64_t value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFloatNoTagToArray(
      float value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteDoubleNoTagToArray(
      double value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteBoolNoTagToArray(bool value,
                                                               uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteEnumNoTagToArray(int value,
                                                               uint8_t* target);

  // Write fields, without tags.  These require that value.size() > 0.
  template <typename T>
  PROTOBUF_NDEBUG_INLINE static uint8_t* WritePrimitiveNoTagToArray(
      const RepeatedField<T>& value, uint8_t* (*Writer)(T, uint8_t*),
      uint8_t* target);
  template <typename T>
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixedNoTagToArray(
      const RepeatedField<T>& value, uint8_t* (*Writer)(T, uint8_t*),
      uint8_t* target);

  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt32NoTagToArray(
      const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt64NoTagToArray(
      const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt32NoTagToArray(
      const RepeatedField<uint32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt64NoTagToArray(
      const RepeatedField<uint64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt32NoTagToArray(
      const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt64NoTagToArray(
      const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed32NoTagToArray(
      const RepeatedField<uint32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed64NoTagToArray(
      const RepeatedField<uint64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed32NoTagToArray(
      const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed64NoTagToArray(
      const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFloatNoTagToArray(
      const RepeatedField<float>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteDoubleNoTagToArray(
      const RepeatedField<double>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteBoolNoTagToArray(
      const RepeatedField<bool>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteEnumNoTagToArray(
      const RepeatedField<int>& value, uint8_t* target);

  // Write fields, including tags.
  template <int field_number>
  PROTOBUF_NOINLINE static uint8_t* WriteInt32ToArrayWithField(
      ::google::protobuf::io::EpsCopyOutputStream* stream, int32_t value,
      uint8_t* target) {
    target = stream->EnsureSpace(target);
    return WriteInt32ToArray(field_number, value, target);
  }

  template <int field_number>
  PROTOBUF_NOINLINE static uint8_t* WriteInt64ToArrayWithField(
      ::google::protobuf::io::EpsCopyOutputStream* stream, int64_t value,
      uint8_t* target) {
    target = stream->EnsureSpace(target);
    return WriteInt64ToArray(field_number, value, target);
  }

  template <int field_number>
  PROTOBUF_NOINLINE static uint8_t* WriteEnumToArrayWithField(
      ::google::protobuf::io::EpsCopyOutputStream* stream, int value, uint8_t* target) {
    target = stream->EnsureSpace(target);
    return WriteEnumToArray(field_number, value, target);
  }

  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt32ToArray(int field_number,
                                                           int32_t value,
                                                           uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt64ToArray(int field_number,
                                                           int64_t value,
                                                           uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt32ToArray(int field_number,
                                                            uint32_t value,
                                                            uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt64ToArray(int field_number,
                                                            uint64_t value,
                                                            uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt32ToArray(int field_number,
                                                            int32_t value,
                                                            uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt64ToArray(int field_number,
                                                            int64_t value,
                                                            uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed32ToArray(int field_number,
                                                             uint32_t value,
                                                             uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed64ToArray(int field_number,
                                                             uint64_t value,
                                                             uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed32ToArray(int field_number,
                                                              int32_t value,
                                                              uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed64ToArray(int field_number,
                                                              int64_t value,
                                                              uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFloatToArray(int field_number,
                                                           float value,
                                                           uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteDoubleToArray(int field_number,
                                                            double value,
                                                            uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteBoolToArray(int field_number,
                                                          bool value,
                                                          uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteEnumToArray(int field_number,
                                                          int value,
                                                          uint8_t* target);

  template <typename T>
  PROTOBUF_NDEBUG_INLINE static uint8_t* WritePrimitiveToArray(
      int field_number, const RepeatedField<T>& value,
      uint8_t* (*Writer)(int, T, uint8_t*), uint8_t* target);

  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt32ToArray(
      int field_number, const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteInt64ToArray(
      int field_number, const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt32ToArray(
      int field_number, const RepeatedField<uint32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteUInt64ToArray(
      int field_number, const RepeatedField<uint64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt32ToArray(
      int field_number, const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSInt64ToArray(
      int field_number, const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed32ToArray(
      int field_number, const RepeatedField<uint32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFixed64ToArray(
      int field_number, const RepeatedField<uint64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed32ToArray(
      int field_number, const RepeatedField<int32_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteSFixed64ToArray(
      int field_number, const RepeatedField<int64_t>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteFloatToArray(
      int field_number, const RepeatedField<float>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteDoubleToArray(
      int field_number, const RepeatedField<double>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteBoolToArray(
      int field_number, const RepeatedField<bool>& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteEnumToArray(
      int field_number, const RepeatedField<int>& value, uint8_t* target);

  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteStringToArray(
      int field_number, const std::string& value, uint8_t* target);
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteBytesToArray(
      int field_number, const std::string& value, uint8_t* target);

  // Whether to serialize deterministically (e.g., map keys are
  // sorted) is a property of a CodedOutputStream, and in the process
  // of serialization, the "ToArray" variants may be invoked.  But they don't
  // have a CodedOutputStream available, so they get an additional parameter
  // telling them whether to serialize deterministically.
  static uint8_t* InternalWriteGroup(int field_number, const MessageLite& value,
                                     uint8_t* target,
                                     io::EpsCopyOutputStream* stream);
  static uint8_t* InternalWriteMessage(int field_number,
                                       const MessageLite& value,
                                       int cached_size, uint8_t* target,
                                       io::EpsCopyOutputStream* stream);

  // Like above, but de-virtualize the call to SerializeWithCachedSizes().
  template <typename MessageType>
  PROTOBUF_NDEBUG_INLINE static uint8_t* InternalWriteGroupNoVirtualToArray(
      int field_number, const MessageType& value, uint8_t* target);
  template <typename MessageType>
  PROTOBUF_NDEBUG_INLINE static uint8_t* InternalWriteMessageNoVirtualToArray(
      int field_number, const MessageType& value, uint8_t* target);

  // For backward-compatibility, the last four methods also have versions
  // that are non-deterministic always.
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteGroupToArray(
      int field_number, const MessageLite& value, uint8_t* target) {
    io::EpsCopyOutputStream stream(
        target,
        value.GetCachedSize() +
            static_cast<int>(2 * io::CodedOutputStream::VarintSize32(
                                     static_cast<uint32_t>(field_number) << 3)),
        io::CodedOutputStream::IsDefaultSerializationDeterministic());
    return InternalWriteGroup(field_number, value, target, &stream);
  }
  PROTOBUF_NDEBUG_INLINE static uint8_t* WriteMessageToArray(
      int field_number, const MessageLite& value, uint8_t* target) {
    int size = value.GetCachedSize();
    io::EpsCopyOutputStream stream(
        target,
        size + static_cast<int>(io::CodedOutputStream::VarintSize32(
                                    static_cast<uint32_t>(field_number) << 3) +
                                io::CodedOutputStream::VarintSize32(size)),
        io::CodedOutputStream::IsDefaultSerializationDeterministic());
    return InternalWriteMessage(field_number, value, value.GetCachedSize(),
                                target, &stream);
  }

  // Compute the byte size of a field.  The XxSize() functions do NOT include
  // the tag, so you must also call TagSize().  (This is because, for repeated
  // fields, you should only call TagSize() once and multiply it by the element
  // count, but you may have to call XxSize() for each individual element.)
  static inline size_t Int32Size(int32_t value);
  static inline size_t Int64Size(int64_t value);
  static inline size_t UInt32Size(uint32_t value);
  static inline size_t UInt64Size(uint64_t value);
  static inline size_t SInt32Size(int32_t value);
  static inline size_t SInt64Size(int64_t value);
  static inline size_t EnumSize(int value);
  static inline size_t Int32SizePlusOne(int32_t value);
  static inline size_t Int64SizePlusOne(int64_t value);
  static inline size_t UInt32SizePlusOne(uint32_t value);
  static inline size_t UInt64SizePlusOne(uint64_t value);
  static inline size_t SInt32SizePlusOne(int32_t value);
  static inline size_t SInt64SizePlusOne(int64_t value);
  static inline size_t EnumSizePlusOne(int value);

  static size_t Int32Size(const RepeatedField<int32_t>& value);
  static size_t Int64Size(const RepeatedField<int64_t>& value);
  static size_t UInt32Size(const RepeatedField<uint32_t>& value);
  static size_t UInt64Size(const RepeatedField<uint64_t>& value);
  static size_t SInt32Size(const RepeatedField<int32_t>& value);
  static size_t SInt64Size(const RepeatedField<int64_t>& value);
  static size_t EnumSize(const RepeatedField<int>& value);

  static size_t Int32SizeWithPackedTagSize(
      const RepeatedField<int32_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t Int64SizeWithPackedTagSize(
      const RepeatedField<int64_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t UInt32SizeWithPackedTagSize(
      const RepeatedField<uint32_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t UInt64SizeWithPackedTagSize(
      const RepeatedField<uint64_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t SInt32SizeWithPackedTagSize(
      const RepeatedField<int32_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t SInt64SizeWithPackedTagSize(
      const RepeatedField<int64_t>& value, size_t tag_size,
      const internal::CachedSize& cached_size);
  static size_t EnumSizeWithPackedTagSize(
      const RepeatedField<int>& value, size_t tag_size,
      const internal::CachedSize& cached_size);

  // These types always have the same size.
  static constexpr size_t kFixed32Size = 4;
  static constexpr size_t kFixed64Size = 8;
  static constexpr size_t kSFixed32Size = 4;
  static constexpr size_t kSFixed64Size = 8;
  static constexpr size_t kFloatSize = 4;
  static constexpr size_t kDoubleSize = 8;
  static constexpr size_t kBoolSize = 1;

  static inline size_t StringSize(const std::string& value);
  static inline size_t StringSize(const absl::Cord& value);
  static inline size_t BytesSize(const std::string& value);
  static inline size_t BytesSize(const absl::Cord& value);
  static inline size_t StringSize(absl::string_view value);
  static inline size_t BytesSize(absl::string_view value);

  template <typename MessageType>
  static inline size_t GroupSize(const MessageType& value);
  template <typename MessageType>
  static inline size_t MessageSize(const MessageType& value);

  // Like above, but de-virtualize the call to ByteSize().  The
  // pointer must point at an instance of MessageType, *not* a subclass (or
  // the subclass must not override ByteSize()).
  template <typename MessageType>
  static inline size_t GroupSizeNoVirtual(const MessageType& value);
  template <typename MessageType>
  static inline size_t MessageSizeNoVirtual(const MessageType& value);

  // Given the length of data, calculate the byte size of the data on the
  // wire if we encode the data as a length delimited field.
  static inline size_t LengthDelimitedSize(size_t length);

 private:
  // A helper method for the repeated primitive reader. This method has
  // optimizations for primitive types that have fixed size on the wire, and
  // can be read using potentially faster paths.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static bool ReadRepeatedFixedSizePrimitive(
      int tag_size, uint32_t tag, io::CodedInputStream* input,
      RepeatedField<CType>* value);

  // Like ReadRepeatedFixedSizePrimitive but for packed primitive fields.
  template <typename CType, enum FieldType DeclaredType>
  PROTOBUF_NDEBUG_INLINE static bool ReadPackedFixedSizePrimitive(
      io::CodedInputStream* input, RepeatedField<CType>* value);

  static const CppType kFieldTypeToCppTypeMap[];
  static const WireFormatLite::WireType kWireTypeForFieldType[];
  static void WriteSubMessageMaybeToArray(int size, const MessageLite& value,
                                          io::CodedOutputStream* output);
};

// A class which deals with unknown values.  The default implementation just
// discards them.  WireFormat defines a subclass which writes to an
// UnknownFieldSet.  This class is used by ExtensionSet::ParseField(), since
// ExtensionSet is part of the lite library but UnknownFieldSet is not.
class PROTOBUF_EXPORT FieldSkipper {
 public:
  FieldSkipper() = default;
  virtual ~FieldSkipper() = default;

  // Skip a field whose tag has already been consumed.
  virtual bool SkipField(io::CodedInputStream* input, uint32_t tag);

  // Skip an entire message or group, up to an end-group tag (which is consumed)
  // or end-of-stream.
  virtual bool SkipMessage(io::CodedInputStream* input);

  // Deal with an already-parsed unrecognized enum value.  The default
  // implementation does nothing, but the UnknownFieldSet-based implementation
  // saves it as an unknown varint.
  virtual void SkipUnknownEnum(int field_number, int value);
};

// Subclass of FieldSkipper which saves skipped fields to a CodedOutputStream.

class PROTOBUF_EXPORT CodedOutputStreamFieldSkipper : public FieldSkipper {
 public:
  explicit CodedOutputStreamFieldSkipper(io::CodedOutputStream* unknown_fields)
      : unknown_fields_(unknown_fields) {}
  ~CodedOutputStreamFieldSkipper() override = default;

  // implements FieldSkipper -----------------------------------------
  bool SkipField(io::CodedInputStream* input, uint32_t tag) override;
  bool SkipMessage(io::CodedInputStream* input) override;
  void SkipUnknownEnum(int field_number, int value) override;

 protected:
  io::CodedOutputStream* unknown_fields_;
};

// inline methods ====================================================

inline WireFormatLite::CppType WireFormatLite::FieldTypeToCppType(
    FieldType type) {
  return kFieldTypeToCppTypeMap[type];
}

constexpr inline uint32_t WireFormatLite::MakeTag(int field_number,
                                                  WireType type) {
  return GOOGLE_PROTOBUF_WIRE_FORMAT_MAKE_TAG(field_number, type);
}

inline WireFormatLite::WireType WireFormatLite::GetTagWireType(uint32_t tag) {
  return static_cast<WireType>(tag & kTagTypeMask);
}

inline int WireFormatLite::GetTagFieldNumber(uint32_t tag) {
  return static_cast<int>(tag >> kTagTypeBits);
}

inline size_t WireFormatLite::TagSize(int field_number,
                                      WireFormatLite::FieldType type) {
  size_t result = io::CodedOutputStream::VarintSize32(
      static_cast<uint32_t>(field_number << kTagTypeBits));
  if (type == TYPE_GROUP) {
    // Groups have both a start and an end tag.
    return result * 2;
  } else {
    return result;
  }
}

inline uint32_t WireFormatLite::EncodeFloat(float value) {
  return absl::bit_cast<uint32_t>(value);
}

inline float WireFormatLite::DecodeFloat(uint32_t value) {
  return absl::bit_cast<float>(value);
}

inline uint64_t WireFormatLite::EncodeDouble(double value) {
  return absl::bit_cast<uint64_t>(value);
}

inline double WireFormatLite::DecodeDouble(uint64_t value) {
  return absl::bit_cast<double>(value);
}

// ZigZag Transform:  Encodes signed integers so that they can be
// effectively used with varint encoding.
//
// varint operates on unsigned integers, encoding smaller numbers into
// fewer bytes.  If you try to use it on a signed integer, it will treat
// this number as a very large unsigned integer, which means that even
// small signed numbers like -1 will take the maximum number of bytes
// (10) to encode.  ZigZagEncode() maps signed integers to unsigned
// in such a way that those with a small absolute value will have smaller
// encoded values, making them appropriate for encoding using varint.
//
//       int32_t ->     uint32_t
// -------------------------
//           0 ->          0
//          -1 ->          1
//           1 ->          2
//          -2 ->          3
//         ... ->        ...
//  2147483647 -> 4294967294
// -2147483648 -> 4294967295
//
//        >> encode >>
//        << decode <<

inline uint32_t WireFormatLite::ZigZagEncode32(int32_t n) {
  // Note:  the right-shift must be arithmetic
  // Note:  left shift must be unsigned because of overflow
  return (static_cast<uint32_t>(n) << 1) ^ static_cast<uint32_t>(n >> 31);
}

inline int32_t WireFormatLite::ZigZagDecode32(uint32_t n) {
  // Note:  Using unsigned types prevent undefined behavior
  return static_cast<int32_t>((n >> 1) ^ (~(n & 1) + 1));
}

inline uint64_t WireFormatLite::ZigZagEncode64(int64_t n) {
  // Note:  the right-shift must be arithmetic
  // Note:  left shift must be unsigned because of overflow
  return (static_cast<uint64_t>(n) << 1) ^ static_cast<uint64_t>(n >> 63);
}

inline int64_t WireFormatLite::ZigZagDecode64(uint64_t n) {
  // Note:  Using unsigned types prevent undefined behavior
  return static_cast<int64_t>((n >> 1) ^ (~(n & 1) + 1));
}

// String is for UTF-8 text only, but, even so, ReadString() can simply
// call ReadBytes().

inline bool WireFormatLite::ReadString(io::CodedInputStream* input,
                                       std::string* value) {
  return ReadBytes(input, value);
}

inline bool WireFormatLite::ReadString(io::CodedInputStream* input,
                                       std::string** p) {
  return ReadBytes(input, p);
}

inline uint8_t* InternalSerializeUnknownMessageSetItemsToArray(
    const std::string& unknown_fields, uint8_t* target,
    io::EpsCopyOutputStream* stream) {
  return stream->WriteRaw(unknown_fields.data(),
                          static_cast<int>(unknown_fields.size()), target);
}

inline size_t ComputeUnknownMessageSetItemsSize(
    const std::string& unknown_fields) {
  return unknown_fields.size();
}

// Implementation details of ReadPrimitive.

template <>
inline bool WireFormatLite::ReadPrimitive<int32_t, WireFormatLite::TYPE_INT32>(
    io::CodedInputStream* input, int32_t* value) {
  uint32_t temp;
  if (!input->ReadVarint32(&temp)) return false;
  *value = static_cast<int32_t>(temp);
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<int64_t, WireFormatLite::TYPE_INT64>(
    io::CodedInputStream* input, int64_t* value) {
  uint64_t temp;
  if (!input->ReadVarint64(&temp)) return false;
  *value = static_cast<int64_t>(temp);
  return true;
}
template <>
inline bool
WireFormatLite::ReadPrimitive<uint32_t, WireFormatLite::TYPE_UINT32>(
    io::CodedInputStream* input, uint32_t* value) {
  return input->ReadVarint32(value);
}
template <>
inline bool
WireFormatLite::ReadPrimitive<uint64_t, WireFormatLite::TYPE_UINT64>(
    io::CodedInputStream* input, uint64_t* value) {
  return input->ReadVarint64(value);
}
template <>
inline bool WireFormatLite::ReadPrimitive<int32_t, WireFormatLite::TYPE_SINT32>(
    io::CodedInputStream* input, int32_t* value) {
  uint32_t temp;
  if (!input->ReadVarint32(&temp)) return false;
  *value = ZigZagDecode32(temp);
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<int64_t, WireFormatLite::TYPE_SINT64>(
    io::CodedInputStream* input, int64_t* value) {
  uint64_t temp;
  if (!input->ReadVarint64(&temp)) return false;
  *value = ZigZagDecode64(temp);
  return true;
}
template <>
inline bool
WireFormatLite::ReadPrimitive<uint32_t, WireFormatLite::TYPE_FIXED32>(
    io::CodedInputStream* input, uint32_t* value) {
  return input->ReadLittleEndian32(value);
}
template <>
inline bool
WireFormatLite::ReadPrimitive<uint64_t, WireFormatLite::TYPE_FIXED64>(
    io::CodedInputStream* input, uint64_t* value) {
  return input->ReadLittleEndian64(value);
}
template <>
inline bool
WireFormatLite::ReadPrimitive<int32_t, WireFormatLite::TYPE_SFIXED32>(
    io::CodedInputStream* input, int32_t* value) {
  uint32_t temp;
  if (!input->ReadLittleEndian32(&temp)) return false;
  *value = static_cast<int32_t>(temp);
  return true;
}
template <>
inline bool
WireFormatLite::ReadPrimitive<int64_t, WireFormatLite::TYPE_SFIXED64>(
    io::CodedInputStream* input, int64_t* value) {
  uint64_t temp;
  if (!input->ReadLittleEndian64(&temp)) return false;
  *value = static_cast<int64_t>(temp);
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<float, WireFormatLite::TYPE_FLOAT>(
    io::CodedInputStream* input, float* value) {
  uint32_t temp;
  if (!input->ReadLittleEndian32(&temp)) return false;
  *value = DecodeFloat(temp);
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<double, WireFormatLite::TYPE_DOUBLE>(
    io::CodedInputStream* input, double* value) {
  uint64_t temp;
  if (!input->ReadLittleEndian64(&temp)) return false;
  *value = DecodeDouble(temp);
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<bool, WireFormatLite::TYPE_BOOL>(
    io::CodedInputStream* input, bool* value) {
  uint64_t temp;
  if (!input->ReadVarint64(&temp)) return false;
  *value = temp != 0;
  return true;
}
template <>
inline bool WireFormatLite::ReadPrimitive<int, WireFormatLite::TYPE_ENUM>(
    io::CodedInputStream* input, int* value) {
  uint32_t temp;
  if (!input->ReadVarint32(&temp)) return false;
  *value = static_cast<int>(temp);
  return true;
}

template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<uint32_t, WireFormatLite::TYPE_FIXED32>(
    const uint8_t* buffer, uint32_t* value) {
  return io::CodedInputStream::ReadLittleEndian32FromArray(buffer, value);
}
template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<uint64_t, WireFormatLite::TYPE_FIXED64>(
    const uint8_t* buffer, uint64_t* value) {
  return io::CodedInputStream::ReadLittleEndian64FromArray(buffer, value);
}
template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<int32_t, WireFormatLite::TYPE_SFIXED32>(
    const uint8_t* buffer, int32_t* value) {
  uint32_t temp;
  buffer = io::CodedInputStream::ReadLittleEndian32FromArray(buffer, &temp);
  *value = static_cast<int32_t>(temp);
  return buffer;
}
template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<int64_t, WireFormatLite::TYPE_SFIXED64>(
    const uint8_t* buffer, int64_t* value) {
  uint64_t temp;
  buffer = io::CodedInputStream::ReadLittleEndian64FromArray(buffer, &temp);
  *value = static_cast<int64_t>(temp);
  return buffer;
}
template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<float, WireFormatLite::TYPE_FLOAT>(
    const uint8_t* buffer, float* value) {
  uint32_t temp;
  buffer = io::CodedInputStream::ReadLittleEndian32FromArray(buffer, &temp);
  *value = DecodeFloat(temp);
  return buffer;
}
template <>
inline const uint8_t*
WireFormatLite::ReadPrimitiveFromArray<double, WireFormatLite::TYPE_DOUBLE>(
    const uint8_t* buffer, double* value) {
  uint64_t temp;
  buffer = io::CodedInputStream::ReadLittleEndian64FromArray(buffer, &temp);
  *value = DecodeDouble(temp);
  return buffer;
}

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
inline bool WireFormatLite::ReadRepeatedPrimitive(
    int,  // tag_size, unused.
    uint32_t tag, io::CodedInputStream* input, RepeatedField<CType>* values) {
  CType value;
  if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
  values->Add(value);
  int elements_already_reserved = values->Capacity() - values->size();
  while (elements_already_reserved > 0 && input->ExpectTag(tag)) {
    if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
    values->AddAlreadyReserved(value);
    elements_already_reserved--;
  }
  return true;
}

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
inline bool WireFormatLite::ReadRepeatedFixedSizePrimitive(
    int tag_size, uint32_t tag, io::CodedInputStream* input,
    RepeatedField<CType>* values) {
  ABSL_DCHECK_EQ(UInt32Size(tag), static_cast<size_t>(tag_size));
  CType value;
  if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
  values->Add(value);

  // For fixed size values, repeated values can be read more quickly by
  // reading directly from a raw array.
  //
  // We can get a tight loop by only reading as many elements as can be
  // added to the RepeatedField without having to do any resizing. Additionally,
  // we only try to read as many elements as are available from the current
  // buffer space. Doing so avoids having to perform boundary checks when
  // reading the value: the maximum number of elements that can be read is
  // known outside of the loop.
  const void* void_pointer;
  int size;
  input->GetDirectBufferPointerInline(&void_pointer, &size);
  if (size > 0) {
    const uint8_t* buffer = reinterpret_cast<const uint8_t*>(void_pointer);
    // The number of bytes each type occupies on the wire.
    const int per_value_size = tag_size + static_cast<int>(sizeof(value));

    // parentheses around (std::min) prevents macro expansion of min(...)
    int elements_available =
        (std::min)(values->Capacity() - values->size(), size / per_value_size);
    int num_read = 0;
    while (num_read < elements_available &&
           (buffer = io::CodedInputStream::ExpectTagFromArray(buffer, tag)) !=
               nullptr) {
      buffer = ReadPrimitiveFromArray<CType, DeclaredType>(buffer, &value);
      values->AddAlreadyReserved(value);
      ++num_read;
    }
    const int read_bytes = num_read * per_value_size;
    if (read_bytes > 0) {
      input->Skip(read_bytes);
    }
  }
  return true;
}

// Specializations of ReadRepeatedPrimitive for the fixed size types, which use
// the optimized code path.
#define READ_REPEATED_FIXED_SIZE_PRIMITIVE(CPPTYPE, DECLARED_TYPE)        \
  template <>                                                             \
  inline bool WireFormatLite::ReadRepeatedPrimitive<                      \
      CPPTYPE, WireFormatLite::DECLARED_TYPE>(                            \
      int tag_size, uint32_t tag, io::CodedInputStream* input,            \
      RepeatedField<CPPTYPE>* values) {                                   \
    return ReadRepeatedFixedSizePrimitive<CPPTYPE,                        \
                                          WireFormatLite::DECLARED_TYPE>( \
        tag_size, tag, input, values);                                    \
  }

READ_REPEATED_FIXED_SIZE_PRIMITIVE(uint32_t, TYPE_FIXED32)
READ_REPEATED_FIXED_SIZE_PRIMITIVE(uint64_t, TYPE_FIXED64)
READ_REPEATED_FIXED_SIZE_PRIMITIVE(int32_t, TYPE_SFIXED32)
READ_REPEATED_FIXED_SIZE_PRIMITIVE(int64_t, TYPE_SFIXED64)
READ_REPEATED_FIXED_SIZE_PRIMITIVE(float, TYPE_FLOAT)
READ_REPEATED_FIXED_SIZE_PRIMITIVE(double, TYPE_DOUBLE)

#undef READ_REPEATED_FIXED_SIZE_PRIMITIVE

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
bool WireFormatLite::ReadRepeatedPrimitiveNoInline(
    int tag_size, uint32_t tag, io::CodedInputStream* input,
    RepeatedField<CType>* value) {
  return ReadRepeatedPrimitive<CType, DeclaredType>(tag_size, tag, input,
                                                    value);
}

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
inline bool WireFormatLite::ReadPackedPrimitive(io::CodedInputStream* input,
                                                RepeatedField<CType>* values) {
  int length;
  if (!input->ReadVarintSizeAsInt(&length)) return false;
  io::CodedInputStream::Limit limit = input->PushLimit(length);
  while (input->BytesUntilLimit() > 0) {
    CType value;
    if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
    values->Add(value);
  }
  input->PopLimit(limit);
  return true;
}

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
inline bool WireFormatLite::ReadPackedFixedSizePrimitive(
    io::CodedInputStream* input, RepeatedField<CType>* values) {
  int length;
  if (!input->ReadVarintSizeAsInt(&length)) return false;
  const int old_entries = values->size();
  const int new_entries = length / static_cast<int>(sizeof(CType));
  const int new_bytes = new_entries * static_cast<int>(sizeof(CType));
  if (new_bytes != length) return false;
  // We would *like* to pre-allocate the buffer to write into (for
  // speed), but *must* avoid performing a very large allocation due
  // to a malicious user-supplied "length" above.  So we have a fast
  // path that pre-allocates when the "length" is less than a bound.
  // We determine the bound by calling BytesUntilTotalBytesLimit() and
  // BytesUntilLimit().  These return -1 to mean "no limit set".
  // There are four cases:
  // TotalBytesLimit  Limit
  // -1               -1     Use slow path.
  // -1               >= 0   Use fast path if length <= Limit.
  // >= 0             -1     Use slow path.
  // >= 0             >= 0   Use fast path if length <= min(both limits).
  int64_t bytes_limit = input->BytesUntilTotalBytesLimit();
  if (bytes_limit == -1) {
    bytes_limit = input->BytesUntilLimit();
  } else {
    // parentheses around (std::min) prevents macro expansion of min(...)
    bytes_limit =
        (std::min)(bytes_limit, static_cast<int64_t>(input->BytesUntilLimit()));
  }
  if (bytes_limit >= new_bytes) {
    // Fast-path that pre-allocates *values to the final size.
#if defined(ABSL_IS_LITTLE_ENDIAN)
    values->Resize(old_entries + new_entries, 0);
    // values->mutable_data() may change after Resize(), so do this after:
    void* dest = reinterpret_cast<void*>(values->mutable_data() + old_entries);
    if (!input->ReadRaw(dest, new_bytes)) {
      values->Truncate(old_entries);
      return false;
    }
#else
    values->Reserve(old_entries + new_entries);
    CType value;
    for (int i = 0; i < new_entries; ++i) {
      if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
      values->AddAlreadyReserved(value);
    }
#endif
  } else {
    // This is the slow-path case where "length" may be too large to
    // safely allocate.  We read as much as we can into *values
    // without pre-allocating "length" bytes.
    CType value;
    for (int i = 0; i < new_entries; ++i) {
      if (!ReadPrimitive<CType, DeclaredType>(input, &value)) return false;
      values->Add(value);
    }
  }
  return true;
}

// Specializations of ReadPackedPrimitive for the fixed size types, which use
// an optimized code path.
#define READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(CPPTYPE, DECLARED_TYPE)      \
  template <>                                                                  \
  inline bool                                                                  \
  WireFormatLite::ReadPackedPrimitive<CPPTYPE, WireFormatLite::DECLARED_TYPE>( \
      io::CodedInputStream * input, RepeatedField<CPPTYPE> * values) {         \
    return ReadPackedFixedSizePrimitive<CPPTYPE,                               \
                                        WireFormatLite::DECLARED_TYPE>(        \
        input, values);                                                        \
  }

READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(uint32_t, TYPE_FIXED32)
READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(uint64_t, TYPE_FIXED64)
READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(int32_t, TYPE_SFIXED32)
READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(int64_t, TYPE_SFIXED64)
READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(float, TYPE_FLOAT)
READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE(double, TYPE_DOUBLE)

#undef READ_REPEATED_PACKED_FIXED_SIZE_PRIMITIVE

template <typename CType, enum WireFormatLite::FieldType DeclaredType>
bool WireFormatLite::ReadPackedPrimitiveNoInline(io::CodedInputStream* input,
                                                 RepeatedField<CType>* values) {
  return ReadPackedPrimitive<CType, DeclaredType>(input, values);
}

inline bool WireFormatLite::ReadBytes(io::CodedInputStream* input,
                                      absl::Cord* value) {
  int length;
  return input->ReadVarintSizeAsInt(&length) && input->ReadCord(value, length);
}

inline bool WireFormatLite::ReadBytes(io::CodedInputStream* input,
                                      absl::Cord** p) {
  return ReadBytes(input, *p);
}


template <typename MessageType>
inline bool WireFormatLite::ReadGroup(int field_number,
                                      io::CodedInputStream* input,
                                      MessageType* value) {
  if (!input->IncrementRecursionDepth()) return false;
  if (!value->MergePartialFromCodedStream(input)) return false;
  input->UnsafeDecrementRecursionDepth();
  // Make sure the last thing read was an end tag for this group.
  if (!input->LastTagWas(MakeTag(field_number, WIRETYPE_END_GROUP))) {
    return false;
  }
  return true;
}
template <typename MessageType>
inline bool WireFormatLite::ReadMessage(io::CodedInputStream* input,
                                        MessageType* value) {
  int length;
  if (!input->ReadVarintSizeAsInt(&length)) return false;
  std::pair<io::CodedInputStream::Limit, int> p =
      input->IncrementRecursionDepthAndPushLimit(length);
  if (p.second < 0 || !value->MergePartialFromCodedStream(input)) return false;
  // Make sure that parsing stopped when the limit was hit, not at an endgroup
  // tag.
  return input->DecrementRecursionDepthAndPopLimit(p.first);
}

// ===================================================================

inline void WireFormatLite::WriteTag(int field_number, WireType type,
                                     io::CodedOutputStream* output) {
  output->WriteTag(MakeTag(field_number, type));
}

inline void WireFormatLite::WriteInt32NoTag(int32_t value,
                                            io::CodedOutputStream* output) {
  output->WriteVarint32SignExtended(value);
}
inline void WireFormatLite::WriteInt64NoTag(int64_t value,
                                            io::CodedOutputStream* output) {
  output->WriteVarint64(static_cast<uint64_t>(value));
}
inline void WireFormatLite::WriteUInt32NoTag(uint32_t value,
                                             io::CodedOutputStream* output) {
  output->WriteVarint32(value);
}
inline void WireFormatLite::WriteUInt64NoTag(uint64_t value,
                                             io::CodedOutputStream* output) {
  output->WriteVarint64(value);
}
inline void WireFormatLite::WriteSInt32NoTag(int32_t value,
                                             io::CodedOutputStream* output) {
  output->WriteVarint32(ZigZagEncode32(value));
}
inline void WireFormatLite::WriteSInt64NoTag(int64_t value,
                                             io::CodedOutputStream* output) {
  output->WriteVarint64(ZigZagEncode64(value));
}
inline void WireFormatLite::WriteFixed32NoTag(uint32_t value,
                                              io::CodedOutputStream* output) {
  output->WriteLittleEndian32(value);
}
inline void WireFormatLite::WriteFixed64NoTag(uint64_t value,
                                              io::CodedOutputStream* output) {
  output->WriteLittleEndian64(value);
}
inline void WireFormatLite::WriteSFixed32NoTag(int32_t value,
                                               io::CodedOutputStream* output) {
  output->WriteLittleEndian32(static_cast<uint32_t>(value));
}
inline void WireFormatLite::WriteSFixed64NoTag(int64_t value,
                                               io::CodedOutputStream* output) {
  output->WriteLittleEndian64(static_cast<uint64_t>(value));
}
inline void WireFormatLite::WriteFloatNoTag(float value,
                                            io::CodedOutputStream* output) {
  output->WriteLittleEndian32(EncodeFloat(value));
}
inline void WireFormatLite::WriteDoubleNoTag(double value,
                                             io::CodedOutputStream* output) {
  output->WriteLittleEndian64(EncodeDouble(value));
}
inline void WireFormatLite::WriteBoolNoTag(bool value,
                                           io::CodedOutputStream* output) {
  output->WriteVarint32(value ? 1 : 0);
}
inline void WireFormatLite::WriteEnumNoTag(int value,
                                           io::CodedOutputStream* output) {
  output->WriteVarint32SignExtended(value);
}

// See comment on ReadGroupNoVirtual to understand the need for this template
// parameter name.
template <typename MessageType_WorkAroundCppLookupDefect>
inline void WireFormatLite::WriteGroupNoVirtual(
    int field_number, const MessageType_WorkAroundCppLookupDefect& value,
    io::CodedOutputStream* output) {
  WriteTag(field_number, WIRETYPE_START_GROUP, output);
  value.MessageType_WorkAroundCppLookupDefect::SerializeWithCachedSizes(output);
  WriteTag(field_number, WIRETYPE_END_GROUP, output);
}
template <typename MessageType_WorkAroundCppLookupDefect>
inline void WireFormatLite::WriteMessageNoVirtual(
    int field_number, const MessageType_WorkAroundCppLookupDefect& value,
    io::CodedOutputStream* output) {
  WriteTag(field_number, WIRETYPE_LENGTH_DELIMITED, output);
  output->WriteVarint32(
      value.MessageType_WorkAroundCppLookupDefect::GetCachedSize());
  value.MessageType_WorkAroundCppLookupDefect::SerializeWithCachedSizes(output);
}

// ===================================================================

inline uint8_t* WireFormatLite::WriteTagToArray(int field_number, WireType type,
                                                uint8_t* target) {
  return io::CodedOutputStream::WriteTagToArray(MakeTag(field_number, type),
                                                target);
}

inline uint8_t* WireFormatLite::WriteInt32NoTagToArray(int32_t value,
                                                       uint8_t* target) {
  return io::CodedOutputStream::WriteVarint32SignExtendedToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteInt64NoTagToArray(int64_t value,
                                                       uint8_t* target) {
  return io::CodedOutputStream::WriteVarint64ToArray(
      static_cast<uint64_t>(value), target);
}
inline uint8_t* WireFormatLite::WriteUInt32NoTagToArray(uint32_t value,
                                                        uint8_t* target) {
  return io::CodedOutputStream::WriteVarint32ToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteUInt64NoTagToArray(uint64_t value,
                                                        uint8_t* target) {
  return io::CodedOutputStream::WriteVarint64ToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSInt32NoTagToArray(int32_t value,
                                                        uint8_t* target) {
  return io::CodedOutputStream::WriteVarint32ToArray(ZigZagEncode32(value),
                                                     target);
}
inline uint8_t* WireFormatLite::WriteSInt64NoTagToArray(int64_t value,
                                                        uint8_t* target) {
  return io::CodedOutputStream::WriteVarint64ToArray(ZigZagEncode64(value),
                                                     target);
}
inline uint8_t* WireFormatLite::WriteFixed32NoTagToArray(uint32_t value,
                                                         uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian32ToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteFixed64NoTagToArray(uint64_t value,
                                                         uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian64ToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSFixed32NoTagToArray(int32_t value,
                                                          uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian32ToArray(
      static_cast<uint32_t>(value), target);
}
inline uint8_t* WireFormatLite::WriteSFixed64NoTagToArray(int64_t value,
                                                          uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian64ToArray(
      static_cast<uint64_t>(value), target);
}
inline uint8_t* WireFormatLite::WriteFloatNoTagToArray(float value,
                                                       uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian32ToArray(EncodeFloat(value),
                                                           target);
}
inline uint8_t* WireFormatLite::WriteDoubleNoTagToArray(double value,
                                                        uint8_t* target) {
  return io::CodedOutputStream::WriteLittleEndian64ToArray(EncodeDouble(value),
                                                           target);
}
inline uint8_t* WireFormatLite::WriteBoolNoTagToArray(bool value,
                                                      uint8_t* target) {
  return io::CodedOutputStream::WriteVarint32ToArray(value ? 1 : 0, target);
}
inline uint8_t* WireFormatLite::WriteEnumNoTagToArray(int value,
                                                      uint8_t* target) {
  return io::CodedOutputStream::WriteVarint32SignExtendedToArray(value, target);
}

template <typename T>
inline uint8_t* WireFormatLite::WritePrimitiveNoTagToArray(
    const RepeatedField<T>& value, uint8_t* (*Writer)(T, uint8_t*),
    uint8_t* target) {
  const int n = value.size();
  ABSL_DCHECK_GT(n, 0);

  const T* ii = value.data();
  int i = 0;
  do {
    target = Writer(ii[i], target);
  } while (++i < n);

  return target;
}

template <typename T>
inline uint8_t* WireFormatLite::WriteFixedNoTagToArray(
    const RepeatedField<T>& value, uint8_t* (*Writer)(T, uint8_t*),
    uint8_t* target) {
#if defined(ABSL_IS_LITTLE_ENDIAN)
  (void)Writer;

  const int n = value.size();
  ABSL_DCHECK_GT(n, 0);

  const T* ii = value.data();
  const int bytes = n * static_cast<int>(sizeof(ii[0]));
  memcpy(target, ii, static_cast<size_t>(bytes));
  return target + bytes;
#else
  return WritePrimitiveNoTagToArray(value, Writer, target);
#endif
}

inline uint8_t* WireFormatLite::WriteInt32NoTagToArray(
    const RepeatedField<int32_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteInt32NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteInt64NoTagToArray(
    const RepeatedField<int64_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteInt64NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteUInt32NoTagToArray(
    const RepeatedField<uint32_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteUInt32NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteUInt64NoTagToArray(
    const RepeatedField<uint64_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteUInt64NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteSInt32NoTagToArray(
    const RepeatedField<int32_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteSInt32NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteSInt64NoTagToArray(
    const RepeatedField<int64_t>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteSInt64NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteFixed32NoTagToArray(
    const RepeatedField<uint32_t>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteFixed32NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteFixed64NoTagToArray(
    const RepeatedField<uint64_t>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteFixed64NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteSFixed32NoTagToArray(
    const RepeatedField<int32_t>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteSFixed32NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteSFixed64NoTagToArray(
    const RepeatedField<int64_t>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteSFixed64NoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteFloatNoTagToArray(
    const RepeatedField<float>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteFloatNoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteDoubleNoTagToArray(
    const RepeatedField<double>& value, uint8_t* target) {
  return WriteFixedNoTagToArray(value, WriteDoubleNoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteBoolNoTagToArray(
    const RepeatedField<bool>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteBoolNoTagToArray, target);
}
inline uint8_t* WireFormatLite::WriteEnumNoTagToArray(
    const RepeatedField<int>& value, uint8_t* target) {
  return WritePrimitiveNoTagToArray(value, WriteEnumNoTagToArray, target);
}

inline uint8_t* WireFormatLite::WriteInt32ToArray(int field_number,
                                                  int32_t value,
                                                  uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteInt32NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteInt64ToArray(int field_number,
                                                  int64_t value,
                                                  uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteInt64NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteUInt32ToArray(int field_number,
                                                   uint32_t value,
                                                   uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteUInt32NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteUInt64ToArray(int field_number,
                                                   uint64_t value,
                                                   uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteUInt64NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSInt32ToArray(int field_number,
                                                   int32_t value,
                                                   uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteSInt32NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSInt64ToArray(int field_number,
                                                   int64_t value,
                                                   uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteSInt64NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteFixed32ToArray(int field_number,
                                                    uint32_t value,
                                                    uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED32, target);
  return WriteFixed32NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteFixed64ToArray(int field_number,
                                                    uint64_t value,
                                                    uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED64, target);
  return WriteFixed64NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSFixed32ToArray(int field_number,
                                                     int32_t value,
                                                     uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED32, target);
  return WriteSFixed32NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteSFixed64ToArray(int field_number,
                                                     int64_t value,
                                                     uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED64, target);
  return WriteSFixed64NoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteFloatToArray(int field_number, float value,
                                                  uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED32, target);
  return WriteFloatNoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteDoubleToArray(int field_number,
                                                   double value,
                                                   uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_FIXED64, target);
  return WriteDoubleNoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteBoolToArray(int field_number, bool value,
                                                 uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteBoolNoTagToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteEnumToArray(int field_number, int value,
                                                 uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_VARINT, target);
  return WriteEnumNoTagToArray(value, target);
}

template <typename T>
inline uint8_t* WireFormatLite::WritePrimitiveToArray(
    int field_number, const RepeatedField<T>& value,
    uint8_t* (*Writer)(int, T, uint8_t*), uint8_t* target) {
  const int n = value.size();
  if (n == 0) {
    return target;
  }

  const T* ii = value.data();
  int i = 0;
  do {
    target = Writer(field_number, ii[i], target);
  } while (++i < n);

  return target;
}

inline uint8_t* WireFormatLite::WriteInt32ToArray(
    int field_number, const RepeatedField<int32_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteInt32ToArray, target);
}
inline uint8_t* WireFormatLite::WriteInt64ToArray(
    int field_number, const RepeatedField<int64_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteInt64ToArray, target);
}
inline uint8_t* WireFormatLite::WriteUInt32ToArray(
    int field_number, const RepeatedField<uint32_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteUInt32ToArray, target);
}
inline uint8_t* WireFormatLite::WriteUInt64ToArray(
    int field_number, const RepeatedField<uint64_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteUInt64ToArray, target);
}
inline uint8_t* WireFormatLite::WriteSInt32ToArray(
    int field_number, const RepeatedField<int32_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteSInt32ToArray, target);
}
inline uint8_t* WireFormatLite::WriteSInt64ToArray(
    int field_number, const RepeatedField<int64_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteSInt64ToArray, target);
}
inline uint8_t* WireFormatLite::WriteFixed32ToArray(
    int field_number, const RepeatedField<uint32_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteFixed32ToArray,
                               target);
}
inline uint8_t* WireFormatLite::WriteFixed64ToArray(
    int field_number, const RepeatedField<uint64_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteFixed64ToArray,
                               target);
}
inline uint8_t* WireFormatLite::WriteSFixed32ToArray(
    int field_number, const RepeatedField<int32_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteSFixed32ToArray,
                               target);
}
inline uint8_t* WireFormatLite::WriteSFixed64ToArray(
    int field_number, const RepeatedField<int64_t>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteSFixed64ToArray,
                               target);
}
inline uint8_t* WireFormatLite::WriteFloatToArray(
    int field_number, const RepeatedField<float>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteFloatToArray, target);
}
inline uint8_t* WireFormatLite::WriteDoubleToArray(
    int field_number, const RepeatedField<double>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteDoubleToArray, target);
}
inline uint8_t* WireFormatLite::WriteBoolToArray(
    int field_number, const RepeatedField<bool>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteBoolToArray, target);
}
inline uint8_t* WireFormatLite::WriteEnumToArray(
    int field_number, const RepeatedField<int>& value, uint8_t* target) {
  return WritePrimitiveToArray(field_number, value, WriteEnumToArray, target);
}
inline uint8_t* WireFormatLite::WriteStringToArray(int field_number,
                                                   const std::string& value,
                                                   uint8_t* target) {
  // String is for UTF-8 text only
  // WARNING:  In wire_format.cc, both strings and bytes are handled by
  //   WriteString() to avoid code duplication.  If the implementations become
  //   different, you will need to update that usage.
  target = WriteTagToArray(field_number, WIRETYPE_LENGTH_DELIMITED, target);
  return io::CodedOutputStream::WriteStringWithSizeToArray(value, target);
}
inline uint8_t* WireFormatLite::WriteBytesToArray(int field_number,
                                                  const std::string& value,
                                                  uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_LENGTH_DELIMITED, target);
  return io::CodedOutputStream::WriteStringWithSizeToArray(value, target);
}


// See comment on ReadGroupNoVirtual to understand the need for this template
// parameter name.
template <typename MessageType_WorkAroundCppLookupDefect>
inline uint8_t* WireFormatLite::InternalWriteGroupNoVirtualToArray(
    int field_number, const MessageType_WorkAroundCppLookupDefect& value,
    uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_START_GROUP, target);
  target = value.MessageType_WorkAroundCppLookupDefect::
               SerializeWithCachedSizesToArray(target);
  return WriteTagToArray(field_number, WIRETYPE_END_GROUP, target);
}
template <typename MessageType_WorkAroundCppLookupDefect>
inline uint8_t* WireFormatLite::InternalWriteMessageNoVirtualToArray(
    int field_number, const MessageType_WorkAroundCppLookupDefect& value,
    uint8_t* target) {
  target = WriteTagToArray(field_number, WIRETYPE_LENGTH_DELIMITED, target);
  target = io::CodedOutputStream::WriteVarint32ToArray(
      static_cast<uint32_t>(
          value.MessageType_WorkAroundCppLookupDefect::GetCachedSize()),
      target);
  return value
      .MessageType_WorkAroundCppLookupDefect::SerializeWithCachedSizesToArray(
          target);
}

// ===================================================================

inline size_t WireFormatLite::Int32Size(int32_t value) {
  return io::CodedOutputStream::VarintSize32SignExtended(value);
}
inline size_t WireFormatLite::Int64Size(int64_t value) {
  return io::CodedOutputStream::VarintSize64(static_cast<uint64_t>(value));
}
inline size_t WireFormatLite::UInt32Size(uint32_t value) {
  return io::CodedOutputStream::VarintSize32(value);
}
inline size_t WireFormatLite::UInt64Size(uint64_t value) {
  return io::CodedOutputStream::VarintSize64(value);
}
inline size_t WireFormatLite::SInt32Size(int32_t value) {
  return io::CodedOutputStream::VarintSize32(ZigZagEncode32(value));
}
inline size_t WireFormatLite::SInt64Size(int64_t value) {
  return io::CodedOutputStream::VarintSize64(ZigZagEncode64(value));
}
inline size_t WireFormatLite::EnumSize(int value) {
  return io::CodedOutputStream::VarintSize32SignExtended(value);
}
inline size_t WireFormatLite::Int32SizePlusOne(int32_t value) {
  return io::CodedOutputStream::VarintSize32SignExtendedPlusOne(value);
}
inline size_t WireFormatLite::Int64SizePlusOne(int64_t value) {
  return io::CodedOutputStream::VarintSize64PlusOne(
      static_cast<uint64_t>(value));
}
inline size_t WireFormatLite::UInt32SizePlusOne(uint32_t value) {
  return io::CodedOutputStream::VarintSize32PlusOne(value);
}
inline size_t WireFormatLite::UInt64SizePlusOne(uint64_t value) {
  return io::CodedOutputStream::VarintSize64PlusOne(value);
}
inline size_t WireFormatLite::SInt32SizePlusOne(int32_t value) {
  return io::CodedOutputStream::VarintSize32PlusOne(ZigZagEncode32(value));
}
inline size_t WireFormatLite::SInt64SizePlusOne(int64_t value) {
  return io::CodedOutputStream::VarintSize64PlusOne(ZigZagEncode64(value));
}
inline size_t WireFormatLite::EnumSizePlusOne(int value) {
  return io::CodedOutputStream::VarintSize32SignExtendedPlusOne(value);
}

inline size_t WireFormatLite::StringSize(const std::string& value) {
  return LengthDelimitedSize(value.size());
}
inline size_t WireFormatLite::BytesSize(const std::string& value) {
  return LengthDelimitedSize(value.size());
}

inline size_t WireFormatLite::BytesSize(const absl::Cord& value) {
  return LengthDelimitedSize(value.size());
}

inline size_t WireFormatLite::StringSize(const absl::Cord& value) {
  return LengthDelimitedSize(value.size());
}

inline size_t WireFormatLite::StringSize(const absl::string_view value) {
  // WARNING:  In wire_format.cc, both strings and bytes are handled by
  //   StringSize() to avoid code duplication.  If the implementations become
  //   different, you will need to update that usage.
  return LengthDelimitedSize(value.size());
}
inline size_t WireFormatLite::BytesSize(const absl::string_view value) {
  return LengthDelimitedSize(value.size());
}

template <typename MessageType>
inline size_t WireFormatLite::GroupSize(const MessageType& value) {
  return value.ByteSizeLong();
}
template <typename MessageType>
inline size_t WireFormatLite::MessageSize(const MessageType& value) {
  return LengthDelimitedSize(value.ByteSizeLong());
}

// See comment on ReadGroupNoVirtual to understand the need for this template
// parameter name.
template <typename MessageType_WorkAroundCppLookupDefect>
inline size_t WireFormatLite::GroupSizeNoVirtual(
    const MessageType_WorkAroundCppLookupDefect& value) {
  return value.MessageType_WorkAroundCppLookupDefect::ByteSizeLong();
}
template <typename MessageType_WorkAroundCppLookupDefect>
inline size_t WireFormatLite::MessageSizeNoVirtual(
    const MessageType_WorkAroundCppLookupDefect& value) {
  return LengthDelimitedSize(
      value.MessageType_WorkAroundCppLookupDefect::ByteSizeLong());
}

inline size_t WireFormatLite::LengthDelimitedSize(size_t length) {
  // The static_cast here prevents an error in certain compiler configurations
  // but is not technically correct--if length is too large to fit in a uint32_t
  // then it will be silently truncated. We will need to fix this if we ever
  // decide to start supporting serialized messages greater than 2 GiB in size.
  return length +
         io::CodedOutputStream::VarintSize32(static_cast<uint32_t>(length));
}

template <typename MS>
bool ParseMessageSetItemImpl(io::CodedInputStream* input, MS ms) {
  // This method parses a group which should contain two fields:
  //   required int32 type_id = 2;
  //   required data message = 3;

  uint32_t last_type_id = 0;

  // If we see message data before the type_id, we'll append it to this so
  // we can parse it later.
  std::string message_data;

  enum class State { kNoTag, kHasType, kHasPayload, kDone };
  State state = State::kNoTag;

  while (true) {
    const uint32_t tag = input->ReadTagNoLastTag();
    if (tag == 0) return false;

    switch (tag) {
      case WireFormatLite::kMessageSetTypeIdTag: {
        uint32_t type_id;
        // We should fail parsing if type id is 0.
        if (!input->ReadVarint32(&type_id) || type_id == 0) return false;
        if (state == State::kNoTag) {
          last_type_id = type_id;
          state = State::kHasType;
        } else if (state == State::kHasPayload) {
          // We saw some message data before the type_id.  Have to parse it
          // now.
          io::CodedInputStream sub_input(
              reinterpret_cast<const uint8_t*>(message_data.data()),
              static_cast<int>(message_data.size()));
          sub_input.SetRecursionLimit(input->RecursionBudget());
          if (!ms.ParseField(type_id, &sub_input)) {
            return false;
          }
          message_data.clear();
          state = State::kDone;
        }

        break;
      }

      case WireFormatLite::kMessageSetMessageTag: {
        if (state == State::kHasType) {
          // Already saw type_id, so we can parse this directly.
          if (!ms.ParseField(last_type_id, input)) {
            return false;
          }
          state = State::kDone;
        } else if (state == State::kNoTag) {
          // We haven't seen a type_id yet.  Append this data to message_data.
          uint32_t length;
          if (!input->ReadVarint32(&length)) return false;
          if (static_cast<int32_t>(length) < 0) return false;
          uint32_t size = static_cast<uint32_t>(
              length + io::CodedOutputStream::VarintSize32(length));
          message_data.resize(size);
          auto ptr = reinterpret_cast<uint8_t*>(&message_data[0]);
          ptr = io::CodedOutputStream::WriteVarint32ToArray(length, ptr);
          if (!input->ReadRaw(ptr, length)) return false;
          state = State::kHasPayload;
        } else {
          if (!ms.SkipField(tag, input)) return false;
        }

        break;
      }

      case WireFormatLite::kMessageSetItemEndTag: {
        return true;
      }

      default: {
        if (!ms.SkipField(tag, input)) return false;
      }
    }
  }
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_WIRE_FORMAT_LITE_H__
