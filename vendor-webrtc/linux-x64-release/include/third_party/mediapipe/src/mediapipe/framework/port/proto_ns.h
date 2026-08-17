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

#ifndef MEDIAPIPE_PORT_PROTO_NS_H_
#define MEDIAPIPE_PORT_PROTO_NS_H_

#include <string>

// Temporary forward declarations for proto2 support on portable targets.
// Use proto_ns inside namespace mediapipe instead of proto2 namespace.
#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/repeated_field.h"
#include "mediapipe/framework/port.h"

namespace mediapipe {
namespace proto_ns = ::google::protobuf;
typedef ::std::string ProtoString;
}  // namespace mediapipe.

// Legacy namespace support.
namespace mediapipe {
namespace proto_ns = mediapipe::proto_ns;
typedef ::std::string ProtoString;
}  // namespace mediapipe

#endif  // MEDIAPIPE_PORT_PROTO_NS_H_
