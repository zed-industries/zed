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

#ifndef GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_HELPERS_H
#define GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_HELPERS_H

#include <google/protobuf/compiler/objectivec/names.h>

#include <map>

#include "src/compiler/config.h"
#include "src/compiler/generator_helpers.h"

namespace grpc_objective_c_generator {

using ::grpc::protobuf::FileDescriptor;
using ::grpc::protobuf::MethodDescriptor;
using ::grpc::protobuf::ServiceDescriptor;

inline std::string MessageHeaderName(const FileDescriptor* file) {
  return google::protobuf::compiler::objectivec::FilePath(file) + ".pbobjc.h";
}

inline bool AsciiIsUpper(char c) { return c >= 'A' && c <= 'Z'; }

inline ::std::string ServiceClassName(const ServiceDescriptor* service) {
  const FileDescriptor* file = service->file();
  ::std::string prefix =
      google::protobuf::compiler::objectivec::FileClassPrefix(file);
  ::std::string class_name(service->name());
  // We add the prefix in the cases where the string is missing a prefix.
  // We define "missing a prefix" as where 'input':
  // a) Doesn't start with the prefix or
  // b) Isn't equivalent to the prefix or
  // c) Has the prefix, but the letter after the prefix is lowercase
  // This is the same semantics as the Objective-C protoc.
  // https://github.com/protocolbuffers/protobuf/blob/c160ae52a91ca4c76936531d68cc846f8230dbb1/src/google/protobuf/compiler/objectivec/objectivec_helpers.cc#L389
  if (class_name.rfind(prefix, 0) == 0) {
    if (class_name.length() == prefix.length() ||
        !AsciiIsUpper(class_name[prefix.length()])) {
      return prefix + class_name;
    } else {
      return class_name;
    }
  } else {
    return prefix + class_name;
  }
}

inline ::std::string LocalImport(const ::std::string& import) {
  return ::std::string("#import \"" + import + "\"\n");
}

inline ::std::string FrameworkImport(const ::std::string& import,
                                     const ::std::string& framework) {
  // Flattens the directory structure: grab the file name only
  std::size_t pos = import.rfind("/");
  // If pos is npos, pos + 1 is 0, which gives us the entire string,
  // so there's no need to check that
  ::std::string filename = import.substr(pos + 1, import.size() - (pos + 1));
  return ::std::string("#import <" + framework + "/" + filename + ">\n");
}

inline ::std::string SystemImport(const ::std::string& import) {
  return ::std::string("#import <" + import + ">\n");
}

inline ::std::string PreprocConditional(::std::string symbol, bool invert) {
  return invert ? "!defined(" + symbol + ") || !" + symbol
                : "defined(" + symbol + ") && " + symbol;
}

inline ::std::string PreprocIf(const ::std::string& symbol,
                               const ::std::string& if_true) {
  return ::std::string("#if " + PreprocConditional(symbol, false) + "\n" +
                       if_true + "#endif\n");
}

inline ::std::string PreprocIfNot(const ::std::string& symbol,
                                  const ::std::string& if_true) {
  return ::std::string("#if " + PreprocConditional(symbol, true) + "\n" +
                       if_true + "#endif\n");
}

inline ::std::string PreprocIfElse(const ::std::string& symbol,
                                   const ::std::string& if_true,
                                   const ::std::string& if_false) {
  return ::std::string("#if " + PreprocConditional(symbol, false) + "\n" +
                       if_true + "#else\n" + if_false + "#endif\n");
}

inline ::std::string PreprocIfNotElse(const ::std::string& symbol,
                                      const ::std::string& if_true,
                                      const ::std::string& if_false) {
  return ::std::string("#if " + PreprocConditional(symbol, true) + "\n" +
                       if_true + "#else\n" + if_false + "#endif\n");
}

inline bool ShouldIncludeMethod(const MethodDescriptor* method) {
#ifdef OBJC_SKIP_METHODS_WITHOUT_MESSAGE_PREFIX
  return (method->input_type()->file()->options().has_objc_class_prefix() &&
          method->output_type()->file()->options().has_objc_class_prefix());
#else
  (void)method;  // to silence the unused warning for method.
  return true;
#endif
}

}  // namespace grpc_objective_c_generator
#endif  // GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_HELPERS_H
