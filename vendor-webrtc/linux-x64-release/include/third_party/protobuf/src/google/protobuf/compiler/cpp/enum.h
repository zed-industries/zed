// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_ENUM_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_ENUM_H__

#include <string>

#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {
class EnumGenerator {
 public:
  EnumGenerator(const EnumDescriptor* descriptor, const Options& options);

  EnumGenerator(const EnumGenerator&) = delete;
  EnumGenerator& operator=(const EnumGenerator&) = delete;

  ~EnumGenerator() = default;

  // Generate header code defining the enum.  This code should be placed
  // within the enum's package namespace, but NOT within any class, even for
  // nested enums.
  void GenerateDefinition(io::Printer* p);

  // Generate specialization of GetEnumDescriptor<MyEnum>().
  // Precondition: in ::google::protobuf namespace.
  void GenerateGetEnumDescriptorSpecializations(io::Printer* p);


  // For enums nested within a message, generate code to import all the enum's
  // symbols (e.g. the enum type name, all its values, etc.) into the class's
  // namespace.  This should be placed inside the class definition in the
  // header.
  void GenerateSymbolImports(io::Printer* p) const;

  // Generate the `inline` implementation of the _IsValid function.
  void GenerateIsValid(io::Printer* p) const;

  // Source file stuff.

  // Generate non-inline methods related to the enum, such as IsValidValue().
  // Goes in the .cc file. EnumDescriptors are stored in an array, idx is
  // the index in this array that corresponds with this enum.
  void GenerateMethods(int idx, io::Printer* p);

 private:
  friend class FileGenerator;

  struct ValueLimits {
    const EnumValueDescriptor* min;
    const EnumValueDescriptor* max;

    static ValueLimits FromEnum(const EnumDescriptor* descriptor);
  };

  const EnumDescriptor* enum_;
  Options options_;
  std::vector<int> sorted_unique_values_;

  bool generate_array_size_;
  bool should_cache_;
  bool has_reflection_;
  ValueLimits limits_;
};

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_ENUM_H__
