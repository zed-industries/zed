// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: jieluo@google.com (Jie Luo)
//
// Generates Python stub (.pyi) for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_PYTHON_PYI_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_PYTHON_PYI_GENERATOR_H__

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/synchronization/mutex.h"
#include "google/protobuf/compiler/code_generator.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
class Descriptor;
class EnumDescriptor;
class FieldDescriptor;
class MethodDescriptor;
class ServiceDescriptor;

namespace io {
class Printer;
}

namespace compiler {
namespace python {

class PROTOC_EXPORT PyiGenerator : public google::protobuf::compiler::CodeGenerator {
 public:
  PyiGenerator();
  PyiGenerator(const PyiGenerator&) = delete;
  PyiGenerator& operator=(const PyiGenerator&) = delete;
  ~PyiGenerator() override;

  // CodeGenerator methods.
  uint64_t GetSupportedFeatures() const override {
    // Code generators must explicitly support proto3 optional.
    return Feature::FEATURE_PROTO3_OPTIONAL |
           Feature::FEATURE_SUPPORTS_EDITIONS;
  }
  bool Generate(const FileDescriptor* file, const std::string& parameter,
                GeneratorContext* generator_context,
                std::string* error) const override;

  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }
  std::vector<const FieldDescriptor*> GetFeatureExtensions() const override {
    return {};
  }

 private:
  void PrintImportForDescriptor(const FileDescriptor& desc,
                                absl::flat_hash_set<std::string>* seen_aliases,
                                bool* has_importlib) const;
  template <typename DescriptorT>
  void Annotate(const std::string& label, const DescriptorT* descriptor) const;
  void PrintImports() const;
  void PrintTopLevelEnums() const;
  void PrintEnum(const EnumDescriptor& enum_descriptor) const;
  void PrintEnumValues(const EnumDescriptor& enum_descriptor,
                       bool is_classvar = false) const;
  template <typename DescriptorT>
  void PrintExtensions(const DescriptorT& descriptor) const;
  void PrintMessages() const;
  void PrintMessage(const Descriptor& message_descriptor, bool is_nested) const;
  void PrintServices() const;
  std::string GetFieldType(
      const FieldDescriptor& field_des, const Descriptor& containing_des) const;
  template <typename DescriptorT>
  std::string ModuleLevelName(const DescriptorT& descriptor) const;
  std::string PublicPackage() const;
  std::string InternalPackage() const;

  bool opensource_runtime_ = true;

  // Very coarse-grained lock to ensure that Generate() is reentrant.
  // Guards file_, printer_, and import_map_.
  mutable absl::Mutex mutex_;
  mutable const FileDescriptor* file_;  // Set in Generate().  Under mutex_.
  mutable io::Printer* printer_;        // Set in Generate().  Under mutex_.
  mutable bool strip_nonfunctional_codegen_ = false;  // Set in Generate().
  // import_map will be a mapping from filename to module alias, e.g.
  // "google3/foo/bar.py" -> "_bar"
  mutable absl::flat_hash_map<std::string, std::string> import_map_;
};

}  // namespace python
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_PYTHON_PYI_GENERATOR_H__
