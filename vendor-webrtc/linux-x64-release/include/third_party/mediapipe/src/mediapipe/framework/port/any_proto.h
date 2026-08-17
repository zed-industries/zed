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

#ifndef MEDIAPIPE_PORT_ANY_PROTO_H_
#define MEDIAPIPE_PORT_ANY_PROTO_H_

#include "mediapipe/framework/port/core_proto_inc.h"

namespace mediapipe {
namespace protobuf {

#if !defined(MEDIAPIPE_PROTO_LITE) || !defined(MEDIAPIPE_PROTO_THIRD_PARTY)
// The full definition of protobuf::Any for most platforms.
using Any = google::protobuf::AnyLite;
#else
// A dummy definition of protobuf::Any for third_party/protobuf:protobuf-lite.
class Any {
 public:
  bool UnpackTo(proto_ns::Message* message) const { return false; }
  template <typename T>
  bool Is() const {
    return false;
  }
  absl::string_view type_url() const { return ""; }
  static const Any& default_instance() {
    static Any _Any_default_instance_;
    return _Any_default_instance_;
  }
};
#endif

}  // namespace protobuf
}  // namespace mediapipe

#endif  // MEDIAPIPE_PORT_ANY_PROTO_H_
