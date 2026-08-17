//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_TEST_CPP_UTIL_CHANNEL_TRACE_PROTO_HELPER_H
#define GRPC_TEST_CPP_UTIL_CHANNEL_TRACE_PROTO_HELPER_H

#include "absl/strings/string_view.h"

namespace grpc {
namespace testing {

void ValidateChannelTraceProtoJsonTranslation(absl::string_view json_string);
void ValidateChannelProtoJsonTranslation(absl::string_view json_string);
void ValidateGetTopChannelsResponseProtoJsonTranslation(
    absl::string_view json_string);
void ValidateGetChannelResponseProtoJsonTranslation(
    absl::string_view json_string);
void ValidateGetServerResponseProtoJsonTranslation(
    absl::string_view json_string);
void ValidateSubchannelProtoJsonTranslation(absl::string_view json_string);
void ValidateServerProtoJsonTranslation(absl::string_view json_string);
void ValidateGetServersResponseProtoJsonTranslation(
    absl::string_view json_string);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_CHANNEL_TRACE_PROTO_HELPER_H
