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

#ifndef GRPC_TEST_CORE_TRANSPORT_TEST_SUITE_TRANSPORT_TEST_H
#define GRPC_TEST_CORE_TRANSPORT_TEST_SUITE_TRANSPORT_TEST_H

#include <memory>
#include <queue>

#include "absl/random/bit_gen_ref.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "src/core/config/core_configuration.h"
#include "src/core/ext/transport/chaotic_good/config.h"
#include "src/core/lib/transport/transport.h"
#include "test/core/call/yodel/yodel_test.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"

namespace grpc_core {

struct ClientAndServerTransportPair {
  OrphanablePtr<Transport> client;
  OrphanablePtr<Transport> server;
  bool is_slow = false;
};

extern ClientAndServerTransportPair (*g_create_transport_test_fixture)(
    std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>);

class TransportTest : public YodelTest {
 protected:
  using YodelTest::YodelTest;

  void SetServerCallDestination();
  CallInitiator CreateCall(ClientMetadataHandle client_initial_metadata);

  CallHandler TickUntilServerCall();

  ChannelArgs MakeChannelArgs() {
    return CoreConfiguration::Get()
        .channel_args_preconditioning()
        .PreconditionChannelArgs(nullptr)
        .SetObject<grpc_event_engine::experimental::EventEngine>(
            event_engine());
  }

  template <typename... PromiseEndpoints>
  chaotic_good::Config MakeConfig(PromiseEndpoints... promise_endpoints) {
    chaotic_good::Config config(MakeChannelArgs());
    auto name_endpoint = [i = 0]() mutable { return absl::StrCat(++i); };
    std::vector<int> this_is_only_here_to_unpack_the_following_statement{
        (config.ServerAddPendingDataEndpoint(ImmediateConnection(
             name_endpoint(), std::move(promise_endpoints))),
         0)...};
    return config;
  }

 private:
  class ServerCallDestination final : public UnstartedCallDestination {
   public:
    void StartCall(UnstartedCallHandler unstarted_call_handler) override;
    void Orphaned() override {}
    std::optional<CallHandler> PopHandler();

   private:
    std::queue<CallHandler> handlers_;
  };

  void InitTest() override {
    CHECK(g_create_transport_test_fixture != nullptr);
    transport_pair_ = (*g_create_transport_test_fixture)(event_engine());
    if (transport_pair_.is_slow) {
      SetMaxRandomMessageSize(1024);
    }
  }

  void Shutdown() override {
    transport_pair_.client.reset();
    transport_pair_.server.reset();
  }

  RefCountedPtr<ServerCallDestination> server_call_destination_ =
      MakeRefCounted<ServerCallDestination>();
  ClientAndServerTransportPair transport_pair_;
};

}  // namespace grpc_core

#define TRANSPORT_TEST(name) YODEL_TEST(TransportTest, name)

// Only one fixture can be created per binary.
#define TRANSPORT_FIXTURE(name)                                                \
  static grpc_core::ClientAndServerTransportPair name(                         \
      std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>     \
          event_engine);                                                       \
  int g_##name = [] {                                                          \
    CHECK(g_create_transport_test_fixture == nullptr);                         \
    g_create_transport_test_fixture = name;                                    \
    return 0;                                                                  \
  }();                                                                         \
  static grpc_core::ClientAndServerTransportPair name(                         \
      GRPC_UNUSED                                                              \
          std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine> \
              event_engine)

#endif  // GRPC_TEST_CORE_TRANSPORT_TEST_SUITE_TRANSPORT_TEST_H
