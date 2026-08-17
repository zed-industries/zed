/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_INTERNAL_COMPILER_SCHEMA_INTERFACE_H
#define GRPC_INTERNAL_COMPILER_SCHEMA_INTERFACE_H

#include <map>
#include <memory>
#include <vector>

#ifndef GRPC_CUSTOM_STRING
#  include <string>
#  define GRPC_CUSTOM_STRING std::string
#endif

namespace grpc {

typedef GRPC_CUSTOM_STRING string;

}  // namespace grpc

namespace grpc_generator {

// A common interface for objects having comments in the source.
// Return formatted comments to be inserted in generated code.
struct CommentHolder {
  virtual ~CommentHolder() {}
  virtual grpc::string GetLeadingComments(const grpc::string prefix) const = 0;
  virtual grpc::string GetTrailingComments(const grpc::string prefix) const = 0;
  virtual std::vector<grpc::string> GetAllComments() const = 0;
};

// An abstract interface representing a method.
struct Method : public CommentHolder {
  virtual ~Method() {}

  virtual grpc::string name() const = 0;

  virtual grpc::string input_type_name() const = 0;
  virtual grpc::string output_type_name() const = 0;

  virtual bool get_module_and_message_path_input(
      grpc::string *str, grpc::string generator_file_name,
      bool generate_in_pb2_grpc, grpc::string import_prefix) const = 0;
  virtual bool get_module_and_message_path_output(
      grpc::string *str, grpc::string generator_file_name,
      bool generate_in_pb2_grpc, grpc::string import_prefix) const = 0;

  virtual std::vector<grpc::string> get_input_namespace_parts() const = 0;
  virtual grpc::string get_input_type_name() const = 0;
  virtual std::vector<grpc::string> get_output_namespace_parts() const = 0;
  virtual grpc::string get_output_type_name() const = 0;

  virtual grpc::string get_fb_builder() const = 0;

  virtual bool NoStreaming() const = 0;
  virtual bool ClientStreaming() const = 0;
  virtual bool ServerStreaming() const = 0;
  virtual bool BidiStreaming() const = 0;
};

// An abstract interface representing a service.
struct Service : public CommentHolder {
  virtual ~Service() {}

  virtual std::vector<grpc::string> namespace_parts() const = 0;
  virtual grpc::string name() const = 0;
  virtual bool is_internal() const = 0;

  virtual int method_count() const = 0;
  virtual std::unique_ptr<const Method> method(int i) const = 0;
};

struct Printer {
  virtual ~Printer() {}

  virtual void Print(const std::map<grpc::string, grpc::string> &vars,
                     const char *template_string) = 0;
  virtual void Print(const char *string) = 0;
  virtual void SetIndentationSize(const size_t size) = 0;
  virtual void Indent() = 0;
  virtual void Outdent() = 0;
};

// An interface that allows the source generated to be output using various
// libraries/idls/serializers.
struct File : public CommentHolder {
  virtual ~File() {}

  virtual grpc::string filename() const = 0;
  virtual grpc::string filename_without_ext() const = 0;
  virtual grpc::string package() const = 0;
  virtual std::vector<grpc::string> package_parts() const = 0;
  virtual grpc::string additional_headers() const = 0;

  virtual int service_count() const = 0;
  virtual std::unique_ptr<const Service> service(int i) const = 0;

  virtual std::unique_ptr<Printer> CreatePrinter(
      grpc::string *str, const char indentation_type = ' ') const = 0;
};
}  // namespace grpc_generator

#endif  // GRPC_INTERNAL_COMPILER_SCHEMA_INTERFACE_H
