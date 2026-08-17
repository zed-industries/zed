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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_COMMON_CLOSURES_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_COMMON_CLOSURES_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/functional/any_invocable.h"

namespace grpc_event_engine::experimental {

class AnyInvocableClosure : public EventEngine::Closure {
 public:
  explicit AnyInvocableClosure(absl::AnyInvocable<void()> cb)
      : cb_(std::move(cb)) {}
  void Run() override { cb_(); }

 private:
  absl::AnyInvocable<void()> cb_;
};

class SelfDeletingClosure : public EventEngine::Closure {
 public:
  // Creates a SelfDeletingClosure.
  // The closure will be deleted after Run is called.
  static Closure* Create(absl::AnyInvocable<void()> cb) {
    return new SelfDeletingClosure(std::move(cb), nullptr);
  }
  // Creates a SelfDeletingClosure with a custom destructor.
  static Closure* Create(absl::AnyInvocable<void()> cb,
                         absl::AnyInvocable<void()> dest_cb) {
    return new SelfDeletingClosure(std::move(cb), std::move(dest_cb));
  }
  ~SelfDeletingClosure() override {
    if (dest_cb_) dest_cb_();
  };

  void Run() override {
    cb_();
    delete this;
  }

 private:
  explicit SelfDeletingClosure(absl::AnyInvocable<void()> cb,
                               absl::AnyInvocable<void()> dest_cb)
      : cb_(std::move(cb)), dest_cb_(std::move(dest_cb)) {}
  absl::AnyInvocable<void()> cb_;
  absl::AnyInvocable<void()> dest_cb_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_COMMON_CLOSURES_H
