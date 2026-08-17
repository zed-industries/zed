// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_LITE_H__

#include <string>
#include <vector>

#include "google/protobuf/compiler/java/generator_factory.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {
class Context;            // context.h
class ClassNameResolver;  // name_resolver.h
}  // namespace java
}  // namespace compiler
namespace io {
class Printer;  // printer.h
}
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class EnumLiteGenerator : public EnumGenerator {
 public:
  EnumLiteGenerator(const EnumDescriptor* descriptor, bool immutable_api,
                    Context* context);
  EnumLiteGenerator(const EnumLiteGenerator&) = delete;
  EnumLiteGenerator& operator=(const EnumLiteGenerator&) = delete;
  ~EnumLiteGenerator();

  void Generate(io::Printer* printer) override;

 private:
  const EnumDescriptor* descriptor_;

  // The proto language allows multiple enum constants to have the same
  // numeric value.  Java, however, does not allow multiple enum constants to
  // be considered equivalent.  We treat the first defined constant for any
  // given numeric value as "canonical" and the rest as aliases of that
  // canonical value.
  std::vector<const EnumValueDescriptor*> canonical_values_;

  struct Alias {
    const EnumValueDescriptor* value;
    const EnumValueDescriptor* canonical_value;
  };
  std::vector<Alias> aliases_;

  bool immutable_api_;

  Context* context_;
  ClassNameResolver* name_resolver_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_LITE_H__
