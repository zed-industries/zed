// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
//
// This file contains definitions for the descriptors, so they can be used
// without importing descriptor.h

#ifndef GOOGLE_PROTOBUF_DESCRIPTOR_LITE_H__
#define GOOGLE_PROTOBUF_DESCRIPTOR_LITE_H__

namespace google {
namespace protobuf {
namespace internal {

class FieldDescriptorLite {
 public:
  // Identifies a field type.  0 is reserved for errors.
  // The order is weird for historical reasons.
  // Types 12 and up are new in proto2.
  enum Type {
    TYPE_DOUBLE = 1,    // double, exactly eight bytes on the wire.
    TYPE_FLOAT = 2,     // float, exactly four bytes on the wire.
    TYPE_INT64 = 3,     // int64, varint on the wire.  Negative numbers
                        // take 10 bytes.  Use TYPE_SINT64 if negative
                        // values are likely.
    TYPE_UINT64 = 4,    // uint64, varint on the wire.
    TYPE_INT32 = 5,     // int32, varint on the wire.  Negative numbers
                        // take 10 bytes.  Use TYPE_SINT32 if negative
                        // values are likely.
    TYPE_FIXED64 = 6,   // uint64, exactly eight bytes on the wire.
    TYPE_FIXED32 = 7,   // uint32, exactly four bytes on the wire.
    TYPE_BOOL = 8,      // bool, varint on the wire.
    TYPE_STRING = 9,    // UTF-8 text.
    TYPE_GROUP = 10,    // Tag-delimited message.  Deprecated.
    TYPE_MESSAGE = 11,  // Length-delimited message.

    TYPE_BYTES = 12,     // Arbitrary byte array.
    TYPE_UINT32 = 13,    // uint32, varint on the wire
    TYPE_ENUM = 14,      // Enum, varint on the wire
    TYPE_SFIXED32 = 15,  // int32, exactly four bytes on the wire
    TYPE_SFIXED64 = 16,  // int64, exactly eight bytes on the wire
    TYPE_SINT32 = 17,    // int32, ZigZag-encoded varint on the wire
    TYPE_SINT64 = 18,    // int64, ZigZag-encoded varint on the wire

    MAX_TYPE = 18,  // Constant useful for defining lookup tables
                    // indexed by Type.
  };

  // Specifies the C++ data type used to represent the field.  There is a
  // fixed mapping from Type to CppType where each Type maps to exactly one
  // CppType.  0 is reserved for errors.
  enum CppType {
    CPPTYPE_INT32 = 1,     // TYPE_INT32, TYPE_SINT32, TYPE_SFIXED32
    CPPTYPE_INT64 = 2,     // TYPE_INT64, TYPE_SINT64, TYPE_SFIXED64
    CPPTYPE_UINT32 = 3,    // TYPE_UINT32, TYPE_FIXED32
    CPPTYPE_UINT64 = 4,    // TYPE_UINT64, TYPE_FIXED64
    CPPTYPE_DOUBLE = 5,    // TYPE_DOUBLE
    CPPTYPE_FLOAT = 6,     // TYPE_FLOAT
    CPPTYPE_BOOL = 7,      // TYPE_BOOL
    CPPTYPE_ENUM = 8,      // TYPE_ENUM
    CPPTYPE_STRING = 9,    // TYPE_STRING, TYPE_BYTES
    CPPTYPE_MESSAGE = 10,  // TYPE_MESSAGE, TYPE_GROUP

    MAX_CPPTYPE = 10,  // Constant useful for defining lookup tables
                       // indexed by CppType.
  };

  // Identifies whether the field is optional, required, or repeated.  0 is
  // reserved for errors.
  enum Label {
    LABEL_OPTIONAL = 1,  // optional
    LABEL_REQUIRED = 2,  // required
    LABEL_REPEATED = 3,  // repeated

    MAX_LABEL = 3,  // Constant useful for defining lookup tables
                    // indexed by Label.
  };

  // Identifies the storage type of a C++ string field.  This corresponds to
  // pb.CppFeatures.StringType, but is compatible with ctype prior to Edition
  // 2024.  0 is reserved for errors.
#ifndef SWIG
  enum class CppStringType {
    kView = 1,
    kCord = 2,
    kString = 3,
  };
#endif
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_DESCRIPTOR_LITE_H__
