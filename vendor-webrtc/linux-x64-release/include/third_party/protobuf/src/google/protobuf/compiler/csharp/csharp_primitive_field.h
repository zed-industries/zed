// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_PRIMITIVE_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_PRIMITIVE_FIELD_H__

#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/compiler/csharp/csharp_field_base.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

struct Options;

class PrimitiveFieldGenerator : public FieldGeneratorBase {
 public:
  PrimitiveFieldGenerator(const FieldDescriptor* descriptor,
                          int presenceIndex,
                          const Options *options);
  ~PrimitiveFieldGenerator();

  PrimitiveFieldGenerator(const PrimitiveFieldGenerator&) = delete;
  PrimitiveFieldGenerator& operator=(const PrimitiveFieldGenerator&) = delete;

  virtual void GenerateCodecCode(io::Printer* printer) override;
  virtual void GenerateCloningCode(io::Printer* printer) override;
  virtual void GenerateMembers(io::Printer* printer) override;
  virtual void GenerateMergingCode(io::Printer* printer) override;
  virtual void GenerateParsingCode(io::Printer* printer) override;
  virtual void GenerateSerializationCode(io::Printer* printer) override;
  virtual void GenerateSerializedSizeCode(io::Printer* printer) override;
  virtual void GenerateExtensionCode(io::Printer* printer) override;

  virtual void WriteHash(io::Printer* printer) override;
  virtual void WriteEquals(io::Printer* printer) override;
  virtual void WriteToString(io::Printer* printer) override;

 protected:
  bool is_value_type;
};

class PrimitiveOneofFieldGenerator : public PrimitiveFieldGenerator {
 public:
  PrimitiveOneofFieldGenerator(const FieldDescriptor* descriptor,
                               int presenceIndex,
                               const Options *options);
  ~PrimitiveOneofFieldGenerator();

  PrimitiveOneofFieldGenerator(const PrimitiveOneofFieldGenerator&) = delete;
  PrimitiveOneofFieldGenerator& operator=(const PrimitiveOneofFieldGenerator&) =
      delete;

  virtual void GenerateCloningCode(io::Printer* printer) override;
  virtual void GenerateMembers(io::Printer* printer) override;
  virtual void GenerateMergingCode(io::Printer* printer) override;
  virtual void WriteToString(io::Printer* printer) override;
  virtual void GenerateParsingCode(io::Printer* printer) override;
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_PRIMITIVE_FIELD_H__

