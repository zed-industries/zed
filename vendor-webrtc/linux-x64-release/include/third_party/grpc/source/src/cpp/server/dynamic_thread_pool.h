//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CPP_SERVER_DYNAMIC_THREAD_POOL_H
#define GRPC_SRC_CPP_SERVER_DYNAMIC_THREAD_POOL_H

#include <grpc/event_engine/event_engine.h>

#include <functional>
#include <memory>

#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/cpp/server/thread_pool_interface.h"

namespace grpc {

class DynamicThreadPool final : public ThreadPoolInterface {
 public:
  void Add(const std::function<void()>& callback) override {
    event_engine_->Run(callback);
  }

 private:
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_ =
      grpc_event_engine::experimental::GetDefaultEventEngine();
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_DYNAMIC_THREAD_POOL_H
