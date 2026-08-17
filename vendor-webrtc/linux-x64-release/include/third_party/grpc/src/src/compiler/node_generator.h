/*
 *
 * Copyright 2016 gRPC authors.
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

#ifndef GRPC_INTERNAL_COMPILER_NODE_GENERATOR_H
#define GRPC_INTERNAL_COMPILER_NODE_GENERATOR_H

#include "src/compiler/config.h"

namespace grpc_node_generator {

// Contains all the parameters that are parsed from the command line.
struct Parameters {
  // Sets the earliest version of nodejs that needs to be supported.
  int minimum_node_version;
};

std::string GenerateFile(const grpc::protobuf::FileDescriptor* file,
                         const Parameters& params);

}  // namespace grpc_node_generator

#endif  // GRPC_INTERNAL_COMPILER_NODE_GENERATOR_H
