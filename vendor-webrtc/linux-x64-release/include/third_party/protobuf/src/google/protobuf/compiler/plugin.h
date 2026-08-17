// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//
// Front-end for protoc code generator plugins written in C++.
//
// To implement a protoc plugin in C++, simply write an implementation of
// CodeGenerator, then create a main() function like:
//   int main(int argc, char* argv[]) {
//     MyCodeGenerator generator;
//     return google::protobuf::compiler::PluginMain(argc, argv, &generator);
//   }
// You must link your plugin against libprotobuf and libprotoc.
//
// The core part of PluginMain is to invoke the given CodeGenerator on a
// CodeGeneratorRequest to generate a CodeGeneratorResponse. This part is
// abstracted out and made into function GenerateCode so that it can be reused,
// for example, to implement a variant of PluginMain that does some
// preprocessing on the input CodeGeneratorRequest before feeding the request
// to the given code generator.
//
// To get protoc to use the plugin, do one of the following:
// * Place the plugin binary somewhere in the PATH and give it the name
//   "protoc-gen-NAME" (replacing "NAME" with the name of your plugin).  If you
//   then invoke protoc with the parameter --NAME_out=OUT_DIR (again, replace
//   "NAME" with your plugin's name), protoc will invoke your plugin to generate
//   the output, which will be placed in OUT_DIR.
// * Place the plugin binary anywhere, with any name, and pass the --plugin
//   parameter to protoc to direct it to your plugin like so:
//     protoc --plugin=protoc-gen-NAME=path/to/mybinary --NAME_out=OUT_DIR
//   On Windows, make sure to include the .exe suffix:
//     protoc --plugin=protoc-gen-NAME=path/to/mybinary.exe --NAME_out=OUT_DIR

#ifndef GOOGLE_PROTOBUF_COMPILER_PLUGIN_H__
#define GOOGLE_PROTOBUF_COMPILER_PLUGIN_H__

#include <string>

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {

class CodeGenerator;  // code_generator.h
class CodeGeneratorRequest;
class CodeGeneratorResponse;

// Implements main() for a protoc plugin exposing the given code generator.
PROTOC_EXPORT int PluginMain(int argc, char* argv[],
                             const CodeGenerator* generator);


// Generates code using the given code generator. Returns true if the code
// generation is successful. If the code generation fails, error_msg may be
// populated to describe the failure cause.
bool GenerateCode(const CodeGeneratorRequest& request,
                  const CodeGenerator& generator,
                  CodeGeneratorResponse* response, std::string* error_msg);

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_PLUGIN_H__
