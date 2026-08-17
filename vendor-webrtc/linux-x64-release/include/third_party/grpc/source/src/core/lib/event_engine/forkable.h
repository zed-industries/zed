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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_FORKABLE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_FORKABLE_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <vector>

#include "src/core/lib/debug/trace.h"

namespace grpc_event_engine::experimental {

// An interface to be implemented by EventEngines that wish to have managed fork
// support. The child class must guarantee that those methods are thread-safe.
class Forkable {
 public:
  virtual ~Forkable() = default;
  virtual void PrepareFork() = 0;
  virtual void PostforkParent() = 0;
  virtual void PostforkChild() = 0;
};

// ObjectGroupForkHandler is meant to be used as a static object in each
// translation unit where Forkables are created and registered with the
// ObjectGroupForkHandler. It essentially provides storage for Forkables'
// instances (as a vector of weak pointers) and helper methods that are meant to
// be invoked inside the fork handlers (see pthread_atfork(3)). The idea is to
// have different Forkables (e.g. PosixEventPoller) to store their instances
// (e.g. a PosixEventPoller object) in a single place separated from other
// Forkables (a sharded approach). Forkables need to register their pthread fork
// handlers and manage the relative ordering themselves. This object is
// thread-unsafe.
class ObjectGroupForkHandler {
 public:
  // Registers a Forkable with this ObjectGroupForkHandler, the Forkable must be
  // created as a shared pointer.
  void RegisterForkable(std::shared_ptr<Forkable> forkable,
                        GRPC_UNUSED void (*prepare)(void),
                        GRPC_UNUSED void (*parent)(void),
                        GRPC_UNUSED void (*child)(void));

  void Prefork();
  void PostforkParent();
  void PostforkChild();

 private:
  GRPC_UNUSED bool registered_ = false;
  bool is_forking_ = false;
  std::vector<std::weak_ptr<Forkable> > forkables_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_FORKABLE_H
