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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_MAP_PIPE_H
#define GRPC_SRC_CORE_LIB_PROMISE_MAP_PIPE_H

#include <grpc/support/port_platform.h>

#include "absl/log/log.h"
#include "absl/status/status.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/for_each.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/pipe.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/try_seq.h"

namespace grpc_core {

// Apply a (possibly async) mapping function to src, and output into dst.
//
// In pseudo-code:
// for each element in wait_for src.Next:
//   x = wait_for filter_factory(element)
//   wait_for dst.Push(x)
template <typename T, typename Filter>
auto MapPipe(PipeReceiver<T> src, PipeSender<T> dst, Filter filter_factory) {
  return ForEach(
      std::move(src),
      [filter_factory = promise_detail::RepeatedPromiseFactory<T, Filter>(
           std::move(filter_factory)),
       dst = std::move(dst)](T t) mutable {
        return TrySeq(
            [] {
              GRPC_TRACE_VLOG(promise_primitives, 2) << "MapPipe: start map";
              return Empty{};
            },
            filter_factory.Make(std::move(t)),
            [&dst](T t) {
              GRPC_TRACE_VLOG(promise_primitives, 2) << "MapPipe: start push";
              return Map(dst.Push(std::move(t)), [](bool successful_push) {
                if (successful_push) {
                  return absl::OkStatus();
                }
                return absl::CancelledError();
              });
            });
      });
}

// Helper to intecept a pipe and apply a mapping function.
// Each of the `Intercept` constructors will take a PipeSender or PipeReceiver,
// construct a new pipe, and then replace the passed in pipe with its new end.
// In this way it can interject logic per-element.
// Next, the TakeAndRun function will return a promise that can be run to apply
// a mapping promise to each element of the pipe.
template <typename T>
class PipeMapper {
 public:
  static PipeMapper Intercept(PipeSender<T>& intercept_sender) {
    PipeMapper<T> r;
    r.interceptor_.sender.Swap(&intercept_sender);
    return r;
  }

  static PipeMapper Intercept(PipeReceiver<T>& intercept_receiver) {
    PipeMapper<T> r;
    r.interceptor_.receiver.Swap(&intercept_receiver);
    return r;
  }

  template <typename Filter>
  auto TakeAndRun(Filter filter) {
    return MapPipe(std::move(interceptor_.receiver),
                   std::move(interceptor_.sender), std::move(filter));
  }

 private:
  PipeMapper() = default;
  Pipe<T> interceptor_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_MAP_PIPE_H
