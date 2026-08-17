// Copyright 2024 gRPC authors.
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

#ifndef GRPCPP_SUPPORT_GLOBAL_CALLBACK_HOOK_H
#define GRPCPP_SUPPORT_GLOBAL_CALLBACK_HOOK_H

#include "absl/functional/function_ref.h"

struct grpc_call;

namespace grpc {

class GlobalCallbackHook {
 public:
  virtual ~GlobalCallbackHook() = default;
  virtual void RunCallback(grpc_call* call,
                           absl::FunctionRef<void()> callback) = 0;

 protected:
  // An exception-safe way of invoking a user-specified callback function.
  template <class Func, class... Args>
  void CatchingCallback(Func&& func, Args&&... args) {
#if GRPC_ALLOW_EXCEPTIONS
    try {
      func(std::forward<Args>(args)...);
    } catch (...) {
      // nothing to return or change here, just don't crash the library
    }
#else   // GRPC_ALLOW_EXCEPTIONS
    func(std::forward<Args>(args)...);
#endif  // GRPC_ALLOW_EXCEPTIONS
  }
};

class DefaultGlobalCallbackHook final : public GlobalCallbackHook {
 public:
  void RunCallback(grpc_call* call,
                   absl::FunctionRef<void()> callback) override {
    CatchingCallback(callback);
  }
};

std::shared_ptr<GlobalCallbackHook> GetGlobalCallbackHook();
void SetGlobalCallbackHook(GlobalCallbackHook* hook);
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_GLOBAL_CALLBACK_HOOK_H
