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

#ifndef GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_H
#define GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_H

#include "src/compiler/config.h"

namespace grpc_objective_c_generator {

struct Parameters {
  // Do not generate V1 interface and implementation
  bool no_v1_compatibility;
};

using ::grpc::protobuf::FileDescriptor;
using ::grpc::protobuf::ServiceDescriptor;

// Returns forward declaration of classes in the generated header file.
std::string GetAllMessageClasses(const FileDescriptor* file);

// Returns the content to be included defining the @protocol segment at the
// insertion point of the generated implementation file. This interface is
// legacy and for backwards compatibility.
std::string GetProtocol(const ServiceDescriptor* service,
                        const Parameters& generator_params);

// Returns the content to be included defining the @protocol segment at the
// insertion point of the generated implementation file.
std::string GetV2Protocol(const ServiceDescriptor* service);

// Returns the content to be included defining the @interface segment at the
// insertion point of the generated implementation file.
std::string GetInterface(const ServiceDescriptor* service,
                         const Parameters& generator_params);

// Returns the content to be included in the "global_scope" insertion point of
// the generated implementation file.
std::string GetSource(const ServiceDescriptor* service,
                      const Parameters& generator_params);

}  // namespace grpc_objective_c_generator

#endif  // GRPC_INTERNAL_COMPILER_OBJECTIVE_C_GENERATOR_H
