// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_ONEOF_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_ONEOF_H__

#include <string>

#include "google/protobuf/compiler/objectivec/helpers.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class OneofGenerator {
 public:
  OneofGenerator(const OneofDescriptor* descriptor,
                 const GenerationOptions& generation_options);
  ~OneofGenerator() = default;

  OneofGenerator(const OneofGenerator&) = delete;
  OneofGenerator& operator=(const OneofGenerator&) = delete;

  void SetOneofIndexBase(int index_base);

  void GenerateCaseEnum(io::Printer* printer) const;

  void GeneratePublicCasePropertyDeclaration(io::Printer* printer) const;
  void GenerateClearFunctionDeclaration(io::Printer* printer) const;

  void GeneratePropertyImplementation(io::Printer* printer) const;
  void GenerateClearFunctionImplementation(io::Printer* printer) const;

  std::string DescriptorName() const;
  std::string HasIndexAsString() const;

 private:
  const OneofDescriptor* descriptor_;
  const GenerationOptions& generation_options_;
  SubstitutionMap variables_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_ONEOF_H__
