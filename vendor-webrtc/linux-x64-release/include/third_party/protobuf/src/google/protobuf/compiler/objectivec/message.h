// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MESSAGE_H__
#define GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MESSAGE_H__

#include <memory>
#include <string>
#include <vector>

#include "absl/container/btree_set.h"
#include "absl/container/flat_hash_set.h"
#include "google/protobuf/compiler/objectivec/field.h"
#include "google/protobuf/compiler/objectivec/oneof.h"
#include "google/protobuf/compiler/objectivec/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace objectivec {

class ExtensionGenerator;

class MessageGenerator {
 public:
  MessageGenerator(const std::string& file_description_name,
                   const Descriptor* descriptor,
                   const GenerationOptions& generation_options);
  ~MessageGenerator() = default;

  MessageGenerator(const MessageGenerator&) = delete;
  MessageGenerator& operator=(const MessageGenerator&) = delete;

  void AddExtensionGenerators(
      std::vector<std::unique_ptr<ExtensionGenerator>>* extension_generators);

  void GenerateMessageHeader(io::Printer* printer) const;
  void GenerateSource(io::Printer* printer) const;
  void DetermineObjectiveCClassDefinitions(
      absl::btree_set<std::string>* fwd_decls) const;
  void DetermineForwardDeclarations(absl::btree_set<std::string>* fwd_decls,
                                    bool include_external_types) const;
  void DetermineNeededFiles(
      absl::flat_hash_set<const FileDescriptor*>* deps) const;

  // Checks if the message or a nested message includes a oneof definition.
  bool IncludesOneOfDefinition() const { return !oneof_generators_.empty(); }

 private:
  const std::string file_description_name_;
  const Descriptor* descriptor_;
  const GenerationOptions& generation_options_;
  FieldGeneratorMap field_generators_;
  const std::string class_name_;
  const std::string deprecated_attribute_;
  std::vector<const ExtensionGenerator*> extension_generators_;
  std::vector<std::unique_ptr<OneofGenerator>> oneof_generators_;
  int sizeof_has_storage_;
};

}  // namespace objectivec
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_OBJECTIVEC_MESSAGE_H__
