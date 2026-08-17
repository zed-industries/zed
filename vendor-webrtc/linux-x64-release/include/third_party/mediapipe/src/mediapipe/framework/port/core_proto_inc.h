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
//
// core_proto_inc.h includes the most commonly used proto header files, and
// it can satisfy the majority of the use cases.
#ifndef MEDIAPIPE_PORT_CORE_PROTO_INC_H_
#define MEDIAPIPE_PORT_CORE_PROTO_INC_H_

#include "google/protobuf/io/tokenizer.h"
#include "google/protobuf/message_lite.h"
#include "google/protobuf/repeated_field.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/proto_ns.h"

#if !defined(MEDIAPIPE_PROTO_LITE)
#include "google/protobuf/text_format.h"
#endif  // !defined(MEDIAPIPE_PROTO_LITE)

#endif  // MEDIAPIPE_PORT_CORE_PROTO_INC_H_
