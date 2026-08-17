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

#ifndef GRPC_INTERNAL_COMPILER_CPP_GENERATOR_H
#define GRPC_INTERNAL_COMPILER_CPP_GENERATOR_H

// cpp_generator.h/.cc do not directly depend on GRPC/ProtoBuf, such that they
// can be used to generate code for other serialization systems, such as
// FlatBuffers.

#include <memory>
#include <string>
#include <vector>

#include "src/compiler/config.h"
#include "src/compiler/schema_interface.h"

#ifdef GRPC_CUSTOM_STRING
#warning GRPC_CUSTOM_STRING is no longer supported. Please use std::string.
#endif

namespace grpc {

// Using grpc::string and grpc::to_string is discouraged in favor of
// std::string and std::to_string. This is only for legacy code using
// them explictly.
using std::string;     // deprecated
using std::to_string;  // deprecated

}  // namespace grpc

namespace grpc_cpp_generator {

// Contains all the parameters that are parsed from the command line.
struct Parameters {
  // Puts the service into a namespace
  std::string services_namespace;
  // Use system includes (<>) or local includes ("")
  bool use_system_headers;
  // Prefix to any grpc include
  std::string grpc_search_path;
  // Generate Google Mock code to facilitate unit testing.
  bool generate_mock_code;
  // Google Mock search path, when non-empty, local includes will be used.
  std::string gmock_search_path;
  // *EXPERIMENTAL* Additional include files in grpc.pb.h
  std::vector<std::string> additional_header_includes;
  // By default, use "pb.h"
  std::string message_header_extension;
  // Whether to include headers corresponding to imports in source file.
  bool include_import_headers;
  // Whether to expose synchronous server API.
  bool allow_sync_server_api;
  // Whether to generate completion queue API.
  bool allow_cq_api;
};

// Return the prologue of the generated header file.
std::string GetHeaderPrologue(grpc_generator::File* file,
                              const Parameters& params);

// Return the includes needed for generated header file.
std::string GetHeaderIncludes(grpc_generator::File* file,
                              const Parameters& params);

// Return the includes needed for generated source file.
std::string GetSourceIncludes(grpc_generator::File* file,
                              const Parameters& params);

// Return the epilogue of the generated header file.
std::string GetHeaderEpilogue(grpc_generator::File* file,
                              const Parameters& params);

// Return the prologue of the generated source file.
std::string GetSourcePrologue(grpc_generator::File* file,
                              const Parameters& params);

// Return the services for generated header file.
std::string GetHeaderServices(grpc_generator::File* file,
                              const Parameters& params);

// Return the services for generated source file.
std::string GetSourceServices(grpc_generator::File* file,
                              const Parameters& params);

// Return the epilogue of the generated source file.
std::string GetSourceEpilogue(grpc_generator::File* file,
                              const Parameters& params);

// Return the prologue of the generated mock file.
std::string GetMockPrologue(grpc_generator::File* file,
                            const Parameters& params);

// Return the includes needed for generated mock file.
std::string GetMockIncludes(grpc_generator::File* file,
                            const Parameters& params);

// Return the services for generated mock file.
std::string GetMockServices(grpc_generator::File* file,
                            const Parameters& params);

// Return the epilogue of generated mock file.
std::string GetMockEpilogue(grpc_generator::File* file,
                            const Parameters& params);

// Return the prologue of the generated mock file.
std::string GetMockPrologue(grpc_generator::File* file,
                            const Parameters& params);

// Return the includes needed for generated mock file.
std::string GetMockIncludes(grpc_generator::File* file,
                            const Parameters& params);

// Return the services for generated mock file.
std::string GetMockServices(grpc_generator::File* file,
                            const Parameters& params);

// Return the epilogue of generated mock file.
std::string GetMockEpilogue(grpc_generator::File* file,
                            const Parameters& params);

}  // namespace grpc_cpp_generator

#endif  // GRPC_INTERNAL_COMPILER_CPP_GENERATOR_H
