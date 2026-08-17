#ifndef GRPC_INTERNAL_COMPILER_GO_GENERATOR_H
#define GRPC_INTERNAL_COMPILER_GO_GENERATOR_H

// go generator is used to generate GRPC code for serialization system, such as
// flatbuffers
#include <memory>
#include <vector>

#include "src/compiler/schema_interface.h"

namespace grpc_go_generator {

struct Parameters {
  // Defines the custom parameter types for methods
  // eg: flatbuffers uses flatbuffers.Builder as input for the client and output
  // for the server
  grpc::string custom_method_io_type;

  // Package name for the service
  grpc::string package_name;

  // Prefix for RPC Calls
  grpc::string service_prefix;
};

// Return the source of the generated service file.
grpc::string GenerateServiceSource(grpc_generator::File *file,
                                   const grpc_generator::Service *service,
                                   grpc_go_generator::Parameters *parameters);

}  // namespace grpc_go_generator

#endif  // GRPC_INTERNAL_COMPILER_GO_GENERATOR_H
