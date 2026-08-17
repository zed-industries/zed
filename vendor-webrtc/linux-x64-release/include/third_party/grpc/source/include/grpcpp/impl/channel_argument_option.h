//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPCPP_IMPL_CHANNEL_ARGUMENT_OPTION_H
#define GRPCPP_IMPL_CHANNEL_ARGUMENT_OPTION_H

#include <grpcpp/impl/server_builder_option.h>
#include <grpcpp/support/channel_arguments.h>

#include <map>
#include <memory>

namespace grpc {

std::unique_ptr<ServerBuilderOption> MakeChannelArgumentOption(
    const std::string& name, const std::string& value);
std::unique_ptr<ServerBuilderOption> MakeChannelArgumentOption(
    const std::string& name, int value);
std::unique_ptr<ServerBuilderOption> MakeChannelArgumentOption(
    const std::string& name, void* value);

}  // namespace grpc

#endif  // GRPCPP_IMPL_CHANNEL_ARGUMENT_OPTION_H
