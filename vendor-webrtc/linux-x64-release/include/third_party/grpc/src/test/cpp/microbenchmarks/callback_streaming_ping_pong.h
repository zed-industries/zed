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

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_STREAMING_PING_PONG_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_STREAMING_PING_PONG_H

#include <benchmark/benchmark.h>

#include <sstream>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "test/cpp/microbenchmarks/callback_test_service.h"
#include "test/cpp/microbenchmarks/fullstack_context_mutators.h"
#include "test/cpp/microbenchmarks/fullstack_fixtures.h"

namespace grpc {
namespace testing {

//******************************************************************************
// BENCHMARKING KERNELS
//

class BidiClient : public grpc::ClientBidiReactor<EchoRequest, EchoResponse> {
 public:
  BidiClient(benchmark::State* state, EchoTestService::Stub* stub,
             ClientContext* cli_ctx, EchoRequest* request,
             EchoResponse* response)
      : state_{state},
        stub_{stub},
        cli_ctx_{cli_ctx},
        request_{request},
        response_{response} {
    msgs_size_ = state->range(0);
    msgs_to_send_ = state->range(1);
    StartNewRpc();
  }

  void OnReadDone(bool ok) override {
    if (!ok) {
      LOG(ERROR) << "Client read failed";
      return;
    }
    MaybeWrite();
  }

  void OnWriteDone(bool ok) override {
    if (!ok) {
      LOG(ERROR) << "Client write failed";
      return;
    }
    writes_complete_++;
    StartRead(response_);
  }

  void OnDone(const Status& s) override {
    CHECK(s.ok());
    CHECK_EQ(writes_complete_, msgs_to_send_);
    if (state_->KeepRunning()) {
      writes_complete_ = 0;
      StartNewRpc();
    } else {
      std::unique_lock<std::mutex> l(mu);
      done = true;
      cv.notify_one();
    }
  }

  void StartNewRpc() {
    cli_ctx_->~ClientContext();
    new (cli_ctx_) ClientContext();
    cli_ctx_->AddMetadata(kServerMessageSize, std::to_string(msgs_size_));
    stub_->async()->BidiStream(cli_ctx_, this);
    MaybeWrite();
    StartCall();
  }

  void Await() {
    std::unique_lock<std::mutex> l(mu);
    while (!done) {
      cv.wait(l);
    }
  }

 private:
  void MaybeWrite() {
    if (writes_complete_ < msgs_to_send_) {
      StartWrite(request_);
    } else {
      StartWritesDone();
    }
  }

  benchmark::State* state_;
  EchoTestService::Stub* stub_;
  ClientContext* cli_ctx_;
  EchoRequest* request_;
  EchoResponse* response_;
  int writes_complete_{0};
  int msgs_to_send_;
  int msgs_size_;
  std::mutex mu;
  std::condition_variable cv;
  bool done = false;
};

template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_CallbackBidiStreaming(benchmark::State& state) {
  int message_size = state.range(0);
  int max_ping_pongs = state.range(1);
  CallbackStreamingTestService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  std::unique_ptr<EchoTestService::Stub> stub_(
      EchoTestService::NewStub(fixture->channel()));
  EchoRequest request;
  EchoResponse response;
  ClientContext cli_ctx;
  if (message_size > 0) {
    request.set_message(std::string(message_size, 'a'));
  } else {
    request.set_message("");
  }
  if (state.KeepRunning()) {
    BidiClient test{&state, stub_.get(), &cli_ctx, &request, &response};
    test.Await();
  }
  fixture.reset();
  state.SetBytesProcessed(2 * message_size * max_ping_pongs *
                          state.iterations());
}

}  // namespace testing
}  // namespace grpc
#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_STREAMING_PING_PONG_H
