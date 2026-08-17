// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_TEST_UTIL_LITE_H__
#define GOOGLE_PROTOBUF_TEST_UTIL_LITE_H__

#include "google/protobuf/unittest_lite.pb.h"

namespace google {
namespace protobuf {

namespace unittest = proto2_unittest;
namespace unittest_import = proto2_unittest_import;

class TestUtilLite {
 public:
  TestUtilLite() = delete;

  // Set every field in the message to a unique value.
  static void SetAllFields(unittest::TestAllTypesLite* message);
  static void SetAllExtensions(unittest::TestAllExtensionsLite* message);
  static void SetPackedFields(unittest::TestPackedTypesLite* message);
  static void SetPackedExtensions(unittest::TestPackedExtensionsLite* message);

  // Use the repeated versions of the set_*() accessors to modify all the
  // repeated fields of the message (which should already have been
  // initialized with Set*Fields()).  Set*Fields() itself only tests
  // the add_*() accessors.
  static void ModifyRepeatedFields(unittest::TestAllTypesLite* message);
  static void ModifyRepeatedExtensions(
      unittest::TestAllExtensionsLite* message);
  static void ModifyPackedFields(unittest::TestPackedTypesLite* message);
  static void ModifyPackedExtensions(
      unittest::TestPackedExtensionsLite* message);

  // Check that all fields have the values that they should have after
  // Set*Fields() is called.
  static void ExpectAllFieldsSet(const unittest::TestAllTypesLite& message);
  static void ExpectAllExtensionsSet(
      const unittest::TestAllExtensionsLite& message);
  static void ExpectPackedFieldsSet(
      const unittest::TestPackedTypesLite& message);
  static void ExpectPackedExtensionsSet(
      const unittest::TestPackedExtensionsLite& message);

  // Expect that the message is modified as would be expected from
  // Modify*Fields().
  static void ExpectRepeatedFieldsModified(
      const unittest::TestAllTypesLite& message);
  static void ExpectRepeatedExtensionsModified(
      const unittest::TestAllExtensionsLite& message);
  static void ExpectPackedFieldsModified(
      const unittest::TestPackedTypesLite& message);
  static void ExpectPackedExtensionsModified(
      const unittest::TestPackedExtensionsLite& message);

  // Check that all fields have their default values.
  static void ExpectClear(const unittest::TestAllTypesLite& message);
  static void ExpectExtensionsClear(
      const unittest::TestAllExtensionsLite& message);
  static void ExpectPackedClear(const unittest::TestPackedTypesLite& message);
  static void ExpectPackedExtensionsClear(
      const unittest::TestPackedExtensionsLite& message);
};

}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_TEST_UTIL_LITE_H__
