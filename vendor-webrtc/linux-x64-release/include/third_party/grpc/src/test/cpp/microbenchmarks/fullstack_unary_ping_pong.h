//
//
// Copyright 2016 gRPC authors.
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

// Benchmark gRPC end2end in various configurations

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_UNARY_PING_PONG_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_UNARY_PING_PONG_H

#include <benchmark/benchmark.h>

#include <sstream>

#include "absl/log/check.h"
#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "test/cpp/microbenchmarks/fullstack_context_mutators.h"
#include "test/cpp/microbenchmarks/fullstack_fixtures.h"

namespace grpc {
namespace testing {

//******************************************************************************
// BENCHMARKING KERNELS
//

static void* tag(intptr_t x) { return reinterpret_cast<void*>(x); }

template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_UnaryPingPong(benchmark::State& state) {
  GRPC_LATENT_SEE_PARENT_SCOPE("BM_UnaryPingPong");
  EchoTestService::AsyncService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  EchoRequest send_request;
  EchoResponse send_response;
  EchoResponse recv_response;
  if (state.range(0) > 0) {
    send_request.set_message(std::string(state.range(0), 'a'));
  }
  if (state.range(1) > 0) {
    send_response.set_message(std::string(state.range(1), 'a'));
  }
  Status recv_status;
  struct ServerEnv {
    ServerContext ctx;
    EchoRequest recv_request;
    grpc::ServerAsyncResponseWriter<EchoResponse> response_writer;
    ServerEnv() : response_writer(&ctx) {}
  };
  uint8_t server_env_buffer[2 * sizeof(ServerEnv)];
  ServerEnv* server_env[2] = {
      reinterpret_cast<ServerEnv*>(server_env_buffer),
      reinterpret_cast<ServerEnv*>(server_env_buffer + sizeof(ServerEnv))};
  new (server_env[0]) ServerEnv;
  new (server_env[1]) ServerEnv;
  service.RequestEcho(&server_env[0]->ctx, &server_env[0]->recv_request,
                      &server_env[0]->response_writer, fixture->cq(),
                      fixture->cq(), tag(0));
  service.RequestEcho(&server_env[1]->ctx, &server_env[1]->recv_request,
                      &server_env[1]->response_writer, fixture->cq(),
                      fixture->cq(), tag(1));
  std::unique_ptr<EchoTestService::Stub> stub(
      EchoTestService::NewStub(fixture->channel()));
  for (auto _ : state) {
    GRPC_LATENT_SEE_PARENT_SCOPE("OneRequest");
    recv_response.Clear();
    ClientContext cli_ctx;
    ClientContextMutator cli_ctx_mut(&cli_ctx);
    std::unique_ptr<ClientAsyncResponseReader<EchoResponse>> response_reader(
        stub->AsyncEcho(&cli_ctx, send_request, fixture->cq()));
    response_reader->Finish(&recv_response, &recv_status, tag(4));
    void* t;
    bool ok;
    {
      GRPC_LATENT_SEE_INNER_SCOPE("WaitForRequest");
      CHECK(fixture->cq()->Next(&t, &ok));
    }
    CHECK(ok);
    CHECK(t == tag(0) || t == tag(1));
    intptr_t slot = reinterpret_cast<intptr_t>(t);
    ServerEnv* senv = server_env[slot];
    ServerContextMutator svr_ctx_mut(&senv->ctx);
    senv->response_writer.Finish(send_response, Status::OK, tag(3));
    {
      GRPC_LATENT_SEE_INNER_SCOPE("WaitForCqs");
      for (int i = (1 << 3) | (1 << 4); i != 0;) {
        CHECK(fixture->cq()->Next(&t, &ok));
        CHECK(ok);
        int tagnum = static_cast<int>(reinterpret_cast<intptr_t>(t));
        CHECK(i & (1 << tagnum));
        i -= 1 << tagnum;
      }
      CHECK(recv_status.ok());
    }
    {
      GRPC_LATENT_SEE_INNER_SCOPE("RequestEcho");
      senv->~ServerEnv();
      senv = new (senv) ServerEnv();
      service.RequestEcho(&senv->ctx, &senv->recv_request,
                          &senv->response_writer, fixture->cq(), fixture->cq(),
                          tag(slot));
    }
  }
  stub.reset();
  fixture.reset();
  server_env[0]->~ServerEnv();
  server_env[1]->~ServerEnv();
  state.SetBytesProcessed((state.range(0) * state.iterations()) +
                          (state.range(1) * state.iterations()));
}
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_UNARY_PING_PONG_H
