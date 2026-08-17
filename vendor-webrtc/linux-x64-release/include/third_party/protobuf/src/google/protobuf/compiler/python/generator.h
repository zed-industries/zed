// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: robinson@google.com (Will Robinson)
//
// Generates Python code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_PYTHON_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_PYTHON_GENERATOR_H__

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "absl/synchronization/mutex.h"
#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/descriptor.pb.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Descriptor;
class EnumDescriptor;
class EnumValueDescriptor;
class FieldDescriptor;
class OneofDescriptor;
class ServiceDescriptor;

namespace io {
class Printer;
}

namespace compiler {
namespace python {

// CodeGenerator implementation for generated Python protocol buffer classes.
// If you create your own protocol compiler binary and you want it to support
// Python output, you can do so by registering an instance of this
// CodeGenerator with the CommandLineInterface in your main() function.

struct GeneratorOptions {
  bool generate_pyi = false;
  bool annotate_pyi = false;
  bool bootstrap = false;
  bool strip_nonfunctional_codegen = false;
};

class PROTOC_EXPORT Generator : public CodeGenerator {
 public:
  Generator();
  Generator(const Generator&) = delete;
  Generator& operator=(const Generator&) = delete;
  ~Generator() override;

  // CodeGenerator methods.
  bool Generate(const FileDescriptor* file, const std::string& parameter,
                GeneratorContext* generator_context,
                std::string* error) const override;

  uint64_t GetSupportedFeatures() const override {
    return Feature::FEATURE_PROTO3_OPTIONAL |
           Feature::FEATURE_SUPPORTS_EDITIONS;
  }
  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }
  std::vector<const FieldDescriptor*> GetFeatureExtensions() const override {
    return {};
  }

  void set_opensource_runtime(bool opensource) {
    opensource_runtime_ = opensource;
  }

 private:
  GeneratorOptions ParseParameter(absl::string_view parameter,
                                  std::string* error) const;
  void PrintImports() const;
  template <typename DescriptorT>
  std::string GetResolvedFeatures(const DescriptorT& descriptor) const;
  void PrintResolvedFeatures() const;
  void PrintFileDescriptor() const;
  void PrintAllEnumsInFile() const;
  void PrintNestedEnums(const Descriptor& descriptor,
                        const DescriptorProto& proto) const;
  void PrintEnum(const EnumDescriptor& enum_descriptor,
                 const EnumDescriptorProto& proto) const;

  void PrintFieldDescriptor(const FieldDescriptor& field,
                            const FieldDescriptorProto& proto) const;
  void PrintFieldDescriptorsInDescriptor(
      const Descriptor& message_descriptor, const DescriptorProto& proto,
      bool is_extension, absl::string_view list_variable_name) const;
  void PrintFieldsInDescriptor(const Descriptor& message_descriptor,
                               const DescriptorProto& proto) const;
  void PrintExtensionsInDescriptor(const Descriptor& message_descriptor,
                                   const DescriptorProto& proto) const;
  void PrintMessageDescriptors() const;
  void PrintDescriptor(const Descriptor& message_descriptor,
                       const DescriptorProto& proto) const;
  void PrintNestedDescriptors(const Descriptor& containing_descriptor,
                              const DescriptorProto& proto) const;

  void PrintMessages() const;
  void PrintMessage(const Descriptor& message_descriptor,
                    absl::string_view prefix,
                    std::vector<std::string>* to_register,
                    bool is_nested) const;
  void PrintNestedMessages(const Descriptor& containing_descriptor,
                           absl::string_view prefix,
                           std::vector<std::string>* to_register) const;

  void FixForeignFieldsInDescriptors() const;
  void FixForeignFieldsInDescriptor(
      const Descriptor& descriptor,
      const Descriptor* containing_descriptor) const;
  void FixForeignFieldsInField(const Descriptor* containing_type,
                               const FieldDescriptor& field,
                               absl::string_view python_dict_name) const;
  void AddMessageToFileDescriptor(const Descriptor& descriptor) const;
  void AddEnumToFileDescriptor(const EnumDescriptor& descriptor) const;
  void AddExtensionToFileDescriptor(const FieldDescriptor& descriptor) const;
  void AddServiceToFileDescriptor(const ServiceDescriptor& descriptor) const;
  std::string FieldReferencingExpression(
      const Descriptor* containing_type, const FieldDescriptor& field,
      absl::string_view python_dict_name) const;
  template <typename DescriptorT>
  void FixContainingTypeInDescriptor(
      const DescriptorT& descriptor,
      const Descriptor* containing_descriptor) const;

  void PrintTopBoilerplate() const;
  void PrintServices() const;
  void PrintServiceDescriptors() const;
  void PrintServiceDescriptor(const ServiceDescriptor& descriptor) const;
  void PrintServiceClass(const ServiceDescriptor& descriptor) const;
  void PrintServiceStub(const ServiceDescriptor& descriptor) const;
  void PrintDescriptorKeyAndModuleName(
      const ServiceDescriptor& descriptor) const;

  void PrintEnumValueDescriptor(const EnumValueDescriptor& descriptor,
                                const EnumValueDescriptorProto& proto) const;
  bool GeneratingDescriptorProto() const;

  template <typename DescriptorT>
  std::string ModuleLevelDescriptorName(const DescriptorT& descriptor) const;
  std::string ModuleLevelMessageName(const Descriptor& descriptor) const;
  std::string ModuleLevelServiceDescriptorName(
      const ServiceDescriptor& descriptor) const;

  template <typename DescriptorProtoT>
  void PrintSerializedPbInterval(const DescriptorProtoT& descriptor_proto,
                                 absl::string_view name) const;

  template <typename DescriptorT>
  bool PrintDescriptorOptionsFixingCode(
      const DescriptorT& descriptor, const typename DescriptorT::Proto& proto,
      absl::string_view descriptor_str) const;

  void FixAllDescriptorOptions() const;
  void FixOptionsForField(const FieldDescriptor& field,
                          const FieldDescriptorProto& proto) const;
  void FixOptionsForOneof(const OneofDescriptor& oneof,
                          const OneofDescriptorProto& proto) const;
  void FixOptionsForEnum(const EnumDescriptor& descriptor,
                         const EnumDescriptorProto& proto) const;
  void FixOptionsForService(const ServiceDescriptor& descriptor,
                            const ServiceDescriptorProto& proto) const;
  void FixOptionsForMessage(const Descriptor& descriptor,
                            const DescriptorProto& proto) const;

  void SetSerializedPbInterval(const FileDescriptorProto& file) const;
  void SetMessagePbInterval(const DescriptorProto& message_proto,
                            const Descriptor& descriptor) const;

  void CopyPublicDependenciesAliases(absl::string_view copy_from,
                                     const FileDescriptor* file) const;

  // Very coarse-grained lock to ensure that Generate() is reentrant.
  // Guards file_, printer_ and file_descriptor_serialized_.
  mutable absl::Mutex mutex_;
  mutable const FileDescriptor* file_;  // Set in Generate().  Under mutex_.
  mutable FileDescriptorProto proto_;   // Set in Generate().  Under mutex_.
  mutable std::string file_descriptor_serialized_;
  mutable io::Printer* printer_;  // Set in Generate().  Under mutex_.

  bool opensource_runtime_ = true;
};

}  // namespace python
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_PYTHON_GENERATOR_H__
