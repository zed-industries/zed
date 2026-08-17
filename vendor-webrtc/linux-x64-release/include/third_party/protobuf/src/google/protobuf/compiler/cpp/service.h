// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_SERVICE_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_SERVICE_H__

#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/cpp/options.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {
class ServiceGenerator {
 public:
  // See generator.cc for the meaning of dllexport_decl.
  ServiceGenerator(
      const ServiceDescriptor* descriptor,
      const absl::flat_hash_map<absl::string_view, std::string>& vars,
      const Options& options)
      : descriptor_(descriptor), options_(&options), vars_(vars) {
    vars_["classname"] = std::string(descriptor_->name());
    vars_["full_name"] = std::string(descriptor_->full_name());
  }

  ServiceGenerator(const ServiceGenerator&) = delete;
  ServiceGenerator& operator=(const ServiceGenerator&) = delete;
  ServiceGenerator(ServiceGenerator&&) = delete;
  ServiceGenerator& operator=(ServiceGenerator&&) = delete;

  ~ServiceGenerator() = default;

  // Generate the class definitions for the service's interface and the
  // stub implementation.
  void GenerateDeclarations(io::Printer* printer);

  // Generate implementations of everything declared by
  // GenerateDeclarations().
  void GenerateImplementation(io::Printer* printer);

 private:
  enum RequestOrResponse { kRequest, kResponse };
  enum VirtualOrNot { kVirtual, kNonVirtual };

  // Prints signatures for all methods in the
  void GenerateMethodSignatures(VirtualOrNot virtual_or_not,
                                io::Printer* printer);

  // Generate the default implementations of the service methods, which
  // produce a "not implemented" error.
  void GenerateNotImplementedMethods(io::Printer* printer);

  // Generate the CallMethod() method of the service.
  void GenerateCallMethod(io::Printer* printer);

  // Generate the Get{Request,Response}Prototype() methods.
  void GenerateGetPrototype(RequestOrResponse which, io::Printer* printer);

  // Generate the cases in CallMethod().
  void GenerateCallMethodCases(io::Printer* printer);

  // Generate the stub's implementations of the service methods.
  void GenerateStubMethods(io::Printer* printer);

  const ServiceDescriptor* descriptor_;
  const Options* options_;
  absl::flat_hash_map<absl::string_view, std::string> vars_;

  int index_in_metadata_;

  friend class FileGenerator;
};
}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_SERVICE_H__
