//
//
// Copyright 2016 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_TEST_CPP_QPS_QPS_SERVER_BUILDER_H
#define GRPC_TEST_CPP_QPS_QPS_SERVER_BUILDER_H

#include <grpcpp/server_builder.h>

#include <functional>
#include <memory>

namespace grpc {
namespace testing {

// CreateQpsServerBuilder creates a new ServerBuilder.
// This uses the "create ServerBuilder" func that was set
// in SetCreateQpsServerBuilderFunc if one has been set,
// otherwise, this defaults to creating a new ServerBuilder
// with only its default constructor.
std::unique_ptr<ServerBuilder> CreateQpsServerBuilder();

// SetCreateQpsServerBuilderFunc sets a function to use to create new
// ServerBuilders in "CreateQpsServerBuilder". It can be used to modify options
// that the server is built with.
void SetCreateQpsServerBuilderFunc(
    std::function<std::unique_ptr<ServerBuilder>()>);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_QPS_SERVER_BUILDER_H
