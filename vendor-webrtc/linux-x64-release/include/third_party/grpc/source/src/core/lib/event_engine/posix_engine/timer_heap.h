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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_HEAP_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_HEAP_H

#include <grpc/support/port_platform.h>

#include <cstddef>
#include <vector>

namespace grpc_event_engine::experimental {

struct Timer;

class TimerHeap {
 public:
  // return true if the new timer is the first timer in the heap
  bool Add(Timer* timer);

  void Remove(Timer* timer);
  Timer* Top();
  void Pop();

  bool is_empty();

  const std::vector<Timer*>& TestOnlyGetTimers() const { return timers_; }

 private:
  void AdjustUpwards(size_t i, Timer* t);
  void AdjustDownwards(size_t i, Timer* t);
  void NoteChangedPriority(Timer* timer);

  std::vector<Timer*> timers_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_HEAP_H
