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

#ifndef GRPC_INTERNAL_COMPILER_CPP_GENERATOR_HELPERS_H
#define GRPC_INTERNAL_COMPILER_CPP_GENERATOR_HELPERS_H

#include <map>

#include "src/compiler/config.h"
#include "src/compiler/generator_helpers.h"

namespace grpc_cpp_generator {

inline std::string DotsToColons(const std::string& name) {
  return grpc_generator::StringReplace(name, ".", "::");
}

inline std::string DotsToUnderscores(const std::string& name) {
  return grpc_generator::StringReplace(name, ".", "_");
}

inline std::string ClassName(const grpc::protobuf::Descriptor* descriptor,
                             bool qualified) {
  // Find "outer", the descriptor of the top-level message in which
  // "descriptor" is embedded.
  const grpc::protobuf::Descriptor* outer = descriptor;
  while (outer->containing_type() != NULL) outer = outer->containing_type();

  std::string outer_name(outer->full_name());
  std::string inner_name(descriptor->full_name().substr(outer_name.size()));

  if (qualified) {
    return "::" + DotsToColons(outer_name) + DotsToUnderscores(inner_name);
  } else {
    return std::string(outer->name()) + DotsToUnderscores(inner_name);
  }
}

// Get leading or trailing comments in a string. Comment lines start with "// ".
// Leading detached comments are put in front of leading comments.
template <typename DescriptorType>
inline std::string GetCppComments(const DescriptorType* desc, bool leading) {
  return grpc_generator::GetPrefixedComments(desc, leading, "//");
}

}  // namespace grpc_cpp_generator

#endif  // GRPC_INTERNAL_COMPILER_CPP_GENERATOR_HELPERS_H
