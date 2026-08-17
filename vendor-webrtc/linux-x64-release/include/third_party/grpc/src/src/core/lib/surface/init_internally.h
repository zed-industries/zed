// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_INIT_INTERNALLY_H
#define GRPC_SRC_CORE_LIB_SURFACE_INIT_INTERNALLY_H

namespace grpc_core {

// Function pointers that should be used in preference to grpc_init,
// grpc_shutdown from within core, but otherwise do the same thing.
// Avoids a build dependency cycle between grpc and grpc_base (and friends).
extern void (*InitInternally)();
extern void (*ShutdownInternally)();
extern bool (*IsInitializedInternally)();

class KeepsGrpcInitialized {
 public:
  explicit KeepsGrpcInitialized(bool enabled = true) : enabled_(enabled) {
    if (enabled_) {
      InitInternally();
    }
  }
  ~KeepsGrpcInitialized() {
    if (enabled_) {
      ShutdownInternally();
    }
  }
  KeepsGrpcInitialized(const KeepsGrpcInitialized&) = delete;
  KeepsGrpcInitialized& operator=(const KeepsGrpcInitialized&) = delete;

 private:
  bool enabled_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_INIT_INTERNALLY_H
