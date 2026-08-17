// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_PRIMITIVE_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_PRIMITIVE_FIELD_H__

#include "google/protobuf/compiler/objectivec/field.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class PrimitiveFieldGenerator : public SingleFieldGenerator {
  friend FieldGenerator* FieldGenerator::Make(
      const FieldDescriptor* field,
      const GenerationOptions& generation_options);

 protected:
  PrimitiveFieldGenerator(const FieldDescriptor* descriptor,
                          const GenerationOptions& generation_options);
  ~PrimitiveFieldGenerator() override = default;

  PrimitiveFieldGenerator(const PrimitiveFieldGenerator&) = delete;
  PrimitiveFieldGenerator& operator=(const PrimitiveFieldGenerator&) = delete;

  void GenerateFieldStorageDeclaration(io::Printer* printer) const override;

  int ExtraRuntimeHasBitsNeeded() const override;
  void SetExtraRuntimeHasBitsBase(int index_base) override;
};

class PrimitiveObjFieldGenerator : public ObjCObjFieldGenerator {
  friend FieldGenerator* FieldGenerator::Make(
      const FieldDescriptor* field,
      const GenerationOptions& generation_options);

 protected:
  PrimitiveObjFieldGenerator(const FieldDescriptor* descriptor,
                             const GenerationOptions& generation_options);
  ~PrimitiveObjFieldGenerator() override = default;

  PrimitiveObjFieldGenerator(const PrimitiveObjFieldGenerator&) = delete;
  PrimitiveObjFieldGenerator& operator=(const PrimitiveObjFieldGenerator&) =
      delete;
};

class RepeatedPrimitiveFieldGenerator : public RepeatedFieldGenerator {
  friend FieldGenerator* FieldGenerator::Make(
      const FieldDescriptor* field,
      const GenerationOptions& generation_options);

 protected:
  RepeatedPrimitiveFieldGenerator(const FieldDescriptor* descriptor,
                                  const GenerationOptions& generation_options);
  ~RepeatedPrimitiveFieldGenerator() override = default;

  RepeatedPrimitiveFieldGenerator(const RepeatedPrimitiveFieldGenerator&) =
      delete;
  RepeatedPrimitiveFieldGenerator& operator=(
      const RepeatedPrimitiveFieldGenerator&) = delete;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_PRIMITIVE_FIELD_H__
