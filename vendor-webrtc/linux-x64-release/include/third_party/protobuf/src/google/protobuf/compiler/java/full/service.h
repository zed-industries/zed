// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_SERVICE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_SERVICE_H__

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

class ImmutableServiceGenerator : public ServiceGenerator {
 public:
  ImmutableServiceGenerator(const ServiceDescriptor* descriptor,
                            Context* context);
  ImmutableServiceGenerator(const ImmutableServiceGenerator&) = delete;
  ImmutableServiceGenerator& operator=(const ImmutableServiceGenerator&) =
      delete;
  ~ImmutableServiceGenerator() override;

  void Generate(io::Printer* printer) override;

 private:
  // Generate the getDescriptorForType() method.
  void GenerateGetDescriptorForType(io::Printer* printer);

  // Generate a Java interface for the service.
  void GenerateInterface(io::Printer* printer);

  // Generate newReflectiveService() method.
  void GenerateNewReflectiveServiceMethod(io::Printer* printer);

  // Generate newReflectiveBlockingService() method.
  void GenerateNewReflectiveBlockingServiceMethod(io::Printer* printer);

  // Generate abstract method declarations for all methods.
  void GenerateAbstractMethods(io::Printer* printer);

  // Generate the implementation of Service.callMethod().
  void GenerateCallMethod(io::Printer* printer);

  // Generate the implementation of BlockingService.callBlockingMethod().
  void GenerateCallBlockingMethod(io::Printer* printer);

  // Generate the implementations of Service.get{Request,Response}Prototype().
  void GenerateGetPrototype(RequestOrResponse which, io::Printer* printer);

  // Generate a stub implementation of the service.
  void GenerateStub(io::Printer* printer);

  // Generate a method signature, possibly abstract, without body or trailing
  // semicolon.
  void GenerateMethodSignature(io::Printer* printer,
                               const MethodDescriptor* method,
                               IsAbstract is_abstract);

  // Generate a blocking stub interface and implementation of the service.
  void GenerateBlockingStub(io::Printer* printer);

  // Generate the method signature for one method of a blocking stub.
  void GenerateBlockingMethodSignature(io::Printer* printer,
                                       const MethodDescriptor* method);

  // Return the output type of the method.
  std::string GetOutput(const MethodDescriptor* method);

  Context* context_;
  ClassNameResolver* name_resolver_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // NET_PROTO2_COMPILER_JAVA_SERVICE_H__
