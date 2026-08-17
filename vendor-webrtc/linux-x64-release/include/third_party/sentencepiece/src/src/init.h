// Copyright 2016 Google Inc.
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
// limitations under the License.!

#ifndef INIT_H_
#define INIT_H_

#include "common.h"

#ifdef _USE_EXTERNAL_PROTOBUF
#include "google/protobuf/message_lite.h"
#else
#include "third_party/protobuf/src/google/protobuf/message_lite.h"
#endif

namespace sentencepiece {

inline void ShutdownLibrary() {
  google::protobuf::ShutdownProtobufLibrary();
#ifdef HAS_ABSL_CLEANUP_FLAGS
  absl::CleanupFlags();
#endif
}

class ScopedResourceDestructor {
 public:
  ScopedResourceDestructor() {}
  ~ScopedResourceDestructor() { ShutdownLibrary(); }
};

}  // namespace sentencepiece

#endif  // INIT_H_
