// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TRANSPORT_UTIL_TRANSPORT_TEST_H
#define GRPC_TEST_CORE_TRANSPORT_UTIL_TRANSPORT_TEST_H

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "src/core/call/call_arena_allocator.h"
#include "src/core/call/call_spine.h"
#include "src/core/config/core_configuration.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/event_engine/event_engine_context.h"
#include "src/core/lib/iomgr/timer_manager.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"

namespace grpc_core {
namespace util {
namespace testing {

class TransportTest : public ::testing::Test {
 protected:
  const std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>&
  event_engine() {
    return event_engine_;
  }

  ChannelArgs GetChannelArgs() {
    return CoreConfiguration::Get()
        .channel_args_preconditioning()
        .PreconditionChannelArgs(nullptr);
  }

  RefCountedPtr<Arena> MakeArena() {
    auto arena = call_arena_allocator_->MakeArena();
    arena->SetContext<grpc_event_engine::experimental::EventEngine>(
        event_engine_.get());
    return arena;
  }

  RefCountedPtr<CallArenaAllocator> call_arena_allocator() {
    return call_arena_allocator_;
  }

  auto MakeCall(ClientMetadataHandle client_initial_metadata) {
    return MakeCallPair(std::move(client_initial_metadata), MakeArena());
  }

 private:
  std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>
      event_engine_{
          std::make_shared<grpc_event_engine::experimental::FuzzingEventEngine>(
              []() {
                grpc_timer_manager_set_threading(false);
                grpc_event_engine::experimental::FuzzingEventEngine::Options
                    options;
                return options;
              }(),
              fuzzing_event_engine::Actions())};
  RefCountedPtr<CallArenaAllocator> call_arena_allocator_{
      MakeRefCounted<CallArenaAllocator>(
          MakeResourceQuota("test-quota")
              ->memory_quota()
              ->CreateMemoryAllocator("test-allocator"),
          1024)};
};

}  // namespace testing
}  // namespace util
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TRANSPORT_UTIL_TRANSPORT_TEST_H
