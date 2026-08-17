// Protocol Buffers - Google's data interchange format
// Copyright 2015 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MAP_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MAP_FIELD_H__

#include <memory>
#include <string>

#include "absl/container/btree_set.h"
#include "absl/container/flat_hash_set.h"
#include "google/protobuf/compiler/objectivec/field.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class MapFieldGenerator : public RepeatedFieldGenerator {
  friend FieldGenerator* FieldGenerator::Make(
      const FieldDescriptor* field,
      const GenerationOptions& generation_options);

 public:
  void EmitArrayComment(io::Printer* printer) const override;

  MapFieldGenerator(const MapFieldGenerator&) = delete;
  MapFieldGenerator& operator=(const MapFieldGenerator&) = delete;

 protected:
  MapFieldGenerator(const FieldDescriptor* descriptor,
                    const GenerationOptions& generation_options);
  ~MapFieldGenerator() override = default;

  void DetermineObjectiveCClassDefinitions(
      absl::btree_set<std::string>* fwd_decls) const override;
  void DetermineForwardDeclarations(absl::btree_set<std::string>* fwd_decls,
                                    bool include_external_types) const override;
  void DetermineNeededFiles(
      absl::flat_hash_set<const FileDescriptor*>* deps) const override;

 private:
  std::unique_ptr<FieldGenerator> value_field_generator_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MAP_FIELD_H__
