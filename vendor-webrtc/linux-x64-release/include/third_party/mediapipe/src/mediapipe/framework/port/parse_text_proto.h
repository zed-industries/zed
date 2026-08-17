// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_PORT_PARSE_TEXT_PROTO_H_
#define MEDIAPIPE_PORT_PARSE_TEXT_PROTO_H_

#include "absl/log/absl_check.h"
#include "mediapipe/framework/port/core_proto_inc.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/proto_ns.h"

namespace mediapipe {

template <typename T>
bool ParseTextProto(const std::string& input, T* proto) {
  return proto_ns::TextFormat::ParseFromString(input, proto);
}

template <typename T>
T ParseTextProtoOrDie(const std::string& input) {
  T result;
  ABSL_CHECK(ParseTextProto(input, &result));
  return result;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_PORT_PARSE_TEXT_PROTO_H_
