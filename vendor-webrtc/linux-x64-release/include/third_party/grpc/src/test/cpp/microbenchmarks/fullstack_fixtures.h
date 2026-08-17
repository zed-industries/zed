//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_FIXTURES_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_FIXTURES_H

#include <grpc/grpc.h>
#include <grpc/support/atm.h>
#include <grpcpp/channel.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>
#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server.h>
#include <grpcpp/server_builder.h>

#include "absl/log/check.h"
#include "src/core/config/core_configuration.h"
#include "src/core/ext/transport/chttp2/transport/chttp2_transport.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/endpoint_pair.h"
#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/tcp_posix.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/surface/channel_create.h"
#include "src/core/lib/surface/completion_queue.h"
#include "src/core/server/server.h"
#include "src/core/util/crash.h"
#include "src/cpp/client/create_channel_internal.h"
#include "test/core/test_util/port.h"
#include "test/core/test_util/test_config.h"
#include "test/cpp/microbenchmarks/helpers.h"

namespace grpc {
namespace testing {

class FixtureConfiguration {
 public:
  virtual ~FixtureConfiguration() {}
  virtual void ApplyCommonChannelArguments(ChannelArguments* c) const {
    c->SetInt(GRPC_ARG_MAX_RECEIVE_MESSAGE_LENGTH, INT_MAX);
    c->SetInt(GRPC_ARG_MAX_SEND_MESSAGE_LENGTH, INT_MAX);
    c->SetInt(GRPC_ARG_ENABLE_RETRIES, 0);
    c->SetResourceQuota(ResourceQuota());
  }

  virtual void ApplyCommonServerBuilderConfig(ServerBuilder* b) const {
    b->SetMaxReceiveMessageSize(INT_MAX);
    b->SetMaxSendMessageSize(INT_MAX);
  }
};

class BaseFixture {
 public:
  virtual ~BaseFixture() = default;
};

class FullstackFixture : public BaseFixture {
 public:
  FullstackFixture(Service* service, const FixtureConfiguration& config,
                   const std::string& address) {
    ServerBuilder b;
    if (!address.empty()) {
      b.AddListeningPort(address, InsecureServerCredentials());
    }
    cq_ = b.AddCompletionQueue(true);
    b.RegisterService(service);
    config.ApplyCommonServerBuilderConfig(&b);
    server_ = b.BuildAndStart();
    ChannelArguments args;
    config.ApplyCommonChannelArguments(&args);
    if (!address.empty()) {
      channel_ = grpc::CreateCustomChannel(address,
                                           InsecureChannelCredentials(), args);
    } else {
      channel_ = server_->InProcessChannel(args);
    }
  }

  ~FullstackFixture() override {
    channel_.reset();
    server_->Shutdown(grpc_timeout_milliseconds_to_deadline(0));
    cq_->Shutdown();
    void* tag;
    bool ok;
    while (cq_->Next(&tag, &ok)) {
    }
  }

  ServerCompletionQueue* cq() { return cq_.get(); }
  std::shared_ptr<Channel> channel() { return channel_; }

 private:
  std::unique_ptr<Server> server_;
  std::unique_ptr<ServerCompletionQueue> cq_;
  std::shared_ptr<Channel> channel_;
};

class TCP : public FullstackFixture {
 public:
  explicit TCP(Service* service,
               const FixtureConfiguration& fixture_configuration =
                   FixtureConfiguration())
      : FullstackFixture(service, fixture_configuration, MakeAddress(&port_)) {}

  ~TCP() override { grpc_recycle_unused_port(port_); }

 private:
  int port_;

  static std::string MakeAddress(int* port) {
    *port = grpc_pick_unused_port_or_die();
    std::stringstream addr;
    addr << "localhost:" << *port;
    return addr.str();
  }
};

class UDS : public FullstackFixture {
 public:
  explicit UDS(Service* service,
               const FixtureConfiguration& fixture_configuration =
                   FixtureConfiguration())
      : FullstackFixture(service, fixture_configuration, MakeAddress(&port_)) {}

  ~UDS() override { grpc_recycle_unused_port(port_); }

 private:
  int port_;

  static std::string MakeAddress(int* port) {
    *port = grpc_pick_unused_port_or_die();  // just for a unique id - not a
                                             // real port
    std::stringstream addr;
    addr << "unix:/tmp/bm_fullstack." << *port;
    return addr.str();
  }
};

class InProcess : public FullstackFixture {
 public:
  explicit InProcess(Service* service,
                     const FixtureConfiguration& fixture_configuration =
                         FixtureConfiguration())
      : FullstackFixture(service, fixture_configuration, "") {}
  ~InProcess() override {}
};

class EndpointPairFixture : public BaseFixture {
 public:
  EndpointPairFixture(Service* service, grpc_endpoint_pair endpoints,
                      const FixtureConfiguration& fixture_configuration)
      : endpoint_pair_(endpoints) {
    ServerBuilder b;
    cq_ = b.AddCompletionQueue(true);
    b.RegisterService(service);
    fixture_configuration.ApplyCommonServerBuilderConfig(&b);
    server_ = b.BuildAndStart();
    grpc_core::ExecCtx exec_ctx;
    // add server endpoint to server_
    //
    {
      grpc_core::Server* core_server =
          grpc_core::Server::FromC(server_->c_server());
      grpc_core::ChannelArgs server_args = core_server->channel_args();
      server_transport_ = grpc_create_chttp2_transport(
          server_args,
          grpc_core::OrphanablePtr<grpc_endpoint>(endpoints.server),
          /*is_client=*/false);
      for (grpc_pollset* pollset : core_server->pollsets()) {
        grpc_endpoint_add_to_pollset(endpoints.server, pollset);
      }

      CHECK(GRPC_LOG_IF_ERROR(
          "SetupTransport", core_server->SetupTransport(server_transport_,
                                                        nullptr, server_args)));
      grpc_chttp2_transport_start_reading(server_transport_, nullptr, nullptr,
                                          nullptr, nullptr);
    }

    // create channel
    {
      grpc_core::ChannelArgs c_args;
      {
        ChannelArguments args;
        args.SetString(GRPC_ARG_DEFAULT_AUTHORITY, "test.authority");
        fixture_configuration.ApplyCommonChannelArguments(&args);
        // precondition
        grpc_channel_args tmp_args;
        args.SetChannelArgs(&tmp_args);
        c_args = grpc_core::CoreConfiguration::Get()
                     .channel_args_preconditioning()
                     .PreconditionChannelArgs(&tmp_args);
      }
      client_transport_ = grpc_create_chttp2_transport(
          c_args, grpc_core::OrphanablePtr<grpc_endpoint>(endpoints.client),
          /*is_client=*/true);
      CHECK(client_transport_);
      grpc_channel* channel =
          grpc_core::ChannelCreate("target", c_args, GRPC_CLIENT_DIRECT_CHANNEL,
                                   client_transport_)
              ->release()
              ->c_ptr();
      grpc_chttp2_transport_start_reading(client_transport_, nullptr, nullptr,
                                          nullptr, nullptr);

      channel_ = grpc::CreateChannelInternal(
          "", channel,
          std::vector<std::unique_ptr<
              experimental::ClientInterceptorFactoryInterface>>());
    }
  }

  ~EndpointPairFixture() override {
    server_->Shutdown(grpc_timeout_milliseconds_to_deadline(0));
    cq_->Shutdown();
    void* tag;
    bool ok;
    while (cq_->Next(&tag, &ok)) {
    }
  }

  ServerCompletionQueue* cq() { return cq_.get(); }
  std::shared_ptr<Channel> channel() { return channel_; }

 protected:
  grpc_endpoint_pair endpoint_pair_;
  grpc_core::Transport* client_transport_;
  grpc_core::Transport* server_transport_;

 private:
  std::unique_ptr<Server> server_;
  std::unique_ptr<ServerCompletionQueue> cq_;
  std::shared_ptr<Channel> channel_;
};

class SockPair : public EndpointPairFixture {
 public:
  explicit SockPair(Service* service,
                    const FixtureConfiguration& fixture_configuration =
                        FixtureConfiguration())
      : EndpointPairFixture(service,
                            grpc_iomgr_create_endpoint_pair("test", nullptr),
                            fixture_configuration) {}
};

////////////////////////////////////////////////////////////////////////////////
// Minimal stack fixtures

class MinStackConfiguration : public FixtureConfiguration {
  void ApplyCommonChannelArguments(ChannelArguments* a) const override {
    a->SetInt(GRPC_ARG_MINIMAL_STACK, 1);
    FixtureConfiguration::ApplyCommonChannelArguments(a);
  }

  void ApplyCommonServerBuilderConfig(ServerBuilder* b) const override {
    b->AddChannelArgument(GRPC_ARG_MINIMAL_STACK, 1);
    FixtureConfiguration::ApplyCommonServerBuilderConfig(b);
  }
};

template <class Base>
class MinStackize : public Base {
 public:
  explicit MinStackize(Service* service)
      : Base(service, MinStackConfiguration()) {}
};

typedef MinStackize<TCP> MinTCP;
typedef MinStackize<UDS> MinUDS;
typedef MinStackize<InProcess> MinInProcess;
typedef MinStackize<SockPair> MinSockPair;

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_FIXTURES_H
