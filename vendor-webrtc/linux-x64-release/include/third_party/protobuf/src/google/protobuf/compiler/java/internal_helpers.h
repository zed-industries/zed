// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_INTERNAL_HELPERS_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_INTERNAL_HELPERS_H__

#include "google/protobuf/compiler/java/java_features.pb.h"
#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/generator.h"
#include "google/protobuf/compiler/java/name_resolver.h"
#include "google/protobuf/compiler/java/names.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"

// Must be last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// Whether unknown enum values are kept (i.e., not stored in UnknownFieldSet
// but in the message and can be queried using additional getters that return
// ints.
inline bool SupportUnknownEnumValue(const FieldDescriptor* field) {
  if (JavaGenerator::GetResolvedSourceFeatures(*field)
          .GetExtension(pb::java)
          .legacy_closed_enum()) {
    return false;
  }
  return field->enum_type() != nullptr && !field->enum_type()->is_closed();
}

inline bool CheckUtf8(const FieldDescriptor* descriptor) {
  if (JavaGenerator::GetResolvedSourceFeatures(*descriptor)
          .GetExtension(pb::java)
          .utf8_validation() == pb::JavaFeatures::VERIFY) {
    return true;
  }
  return JavaGenerator::GetResolvedSourceFeatures(*descriptor)
                 .utf8_validation() == FeatureSet::VERIFY ||
         // For legacy syntax. This is not allowed under Editions.
         descriptor->file()->options().java_string_check_utf8();
}


// Only the lowest two bytes of the return value are used. The lowest byte
// is the integer value of a j/c/g/protobuf/FieldType enum. For the other
// byte:
//    bit 0: whether the field is required.
//    bit 1: whether the field requires UTF-8 validation.
//    bit 2: whether the field needs isInitialized check.
//    bit 3: whether the field is a closed enum.
//    bits 4-7: unused
int GetExperimentalJavaFieldType(const FieldDescriptor* field);

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_INTERNAL_HELPERS_H__
