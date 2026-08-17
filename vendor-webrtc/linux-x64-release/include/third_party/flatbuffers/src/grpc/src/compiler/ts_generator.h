#include <memory>
#include <set>
#include <vector>

#include "src/compiler/schema_interface.h"

#ifndef GRPC_CUSTOM_STRING
#  include <string>
#  define GRPC_CUSTOM_STRING std::string
#endif

namespace grpc {

typedef GRPC_CUSTOM_STRING string;

}  // namespace grpc

namespace grpc_ts_generator {
grpc::string Generate(grpc_generator::File *file,
                      const grpc_generator::Service *service,
                      const grpc::string &filename);

grpc::string GenerateInterface(grpc_generator::File *file,
                               const grpc_generator::Service *service,
                               const grpc::string &filename);
}  // namespace grpc_ts_generator
