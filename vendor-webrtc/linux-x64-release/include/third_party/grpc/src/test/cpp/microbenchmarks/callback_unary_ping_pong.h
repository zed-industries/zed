//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_UNARY_PING_PONG_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_UNARY_PING_PONG_H

#include <benchmark/benchmark.h>

#include <sstream>

#include "absl/log/check.h"
#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "test/cpp/microbenchmarks/callback_test_service.h"
#include "test/cpp/microbenchmarks/fullstack_context_mutators.h"
#include "test/cpp/microbenchmarks/fullstack_fixtures.h"

namespace grpc {
namespace testing {

//******************************************************************************
// BENCHMARKING KERNELS
//

inline void SendCallbackUnaryPingPong(
    benchmark::State* state, ClientContext* cli_ctx, EchoRequest* request,
    EchoResponse* response, EchoTestService::Stub* stub_, bool* done,
    std::mutex* mu, std::condition_variable* cv) {
  int response_msgs_size = state->range(1);
  cli_ctx->AddMetadata(kServerMessageSize, std::to_string(response_msgs_size));
  stub_->async()->Echo(
      cli_ctx, request, response,
      [state, cli_ctx, request, response, stub_, done, mu, cv](Status s) {
        CHECK(s.ok());
        if (state->KeepRunning()) {
          cli_ctx->~ClientContext();
          new (cli_ctx) ClientContext();
          SendCallbackUnaryPingPong(state, cli_ctx, request, response, stub_,
                                    done, mu, cv);
        } else {
          std::lock_guard<std::mutex> l(*mu);
          *done = true;
          cv->notify_one();
        }
      });
};

template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_CallbackUnaryPingPong(benchmark::State& state) {
  int request_msgs_size = state.range(0);
  int response_msgs_size = state.range(1);
  CallbackStreamingTestService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  std::unique_ptr<EchoTestService::Stub> stub_(
      EchoTestService::NewStub(fixture->channel()));
  EchoRequest request;
  EchoResponse response;
  ClientContext cli_ctx;

  if (request_msgs_size > 0) {
    request.set_message(std::string(request_msgs_size, 'a'));
  } else {
    request.set_message("");
  }

  std::mutex mu;
  std::condition_variable cv;
  bool done = false;
  if (state.KeepRunning()) {
    SendCallbackUnaryPingPong(&state, &cli_ctx, &request, &response,
                              stub_.get(), &done, &mu, &cv);
  }
  std::unique_lock<std::mutex> l(mu);
  while (!done) {
    cv.wait(l);
  }
  fixture.reset();
  state.SetBytesProcessed((request_msgs_size * state.iterations()) +
                          (response_msgs_size * state.iterations()));
}

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_UNARY_PING_PONG_H
