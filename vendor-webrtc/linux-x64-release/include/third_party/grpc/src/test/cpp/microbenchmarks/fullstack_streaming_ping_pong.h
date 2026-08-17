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

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_STREAMING_PING_PONG_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_STREAMING_PING_PONG_H

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

// Repeatedly makes Streaming Bidi calls (exchanging a configurable number of
// messages in each call) in a loop on a single channel
//
//  First parameter (i.e state.range(0)):  Message size (in bytes) to use
//  Second parameter (i.e state.range(1)): Number of ping pong messages.
//      Note: One ping-pong means two messages (one from client to server and
//      the other from server to client):
template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_StreamingPingPong(benchmark::State& state) {
  const int msg_size = state.range(0);
  const int max_ping_pongs = state.range(1);

  EchoTestService::AsyncService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  {
    EchoResponse send_response;
    EchoResponse recv_response;
    EchoRequest send_request;
    EchoRequest recv_request;

    if (msg_size > 0) {
      send_request.set_message(std::string(msg_size, 'a'));
      send_response.set_message(std::string(msg_size, 'b'));
    }

    std::unique_ptr<EchoTestService::Stub> stub(
        EchoTestService::NewStub(fixture->channel()));

    for (auto _ : state) {
      ServerContext svr_ctx;
      ServerContextMutator svr_ctx_mut(&svr_ctx);
      ServerAsyncReaderWriter<EchoResponse, EchoRequest> response_rw(&svr_ctx);
      service.RequestBidiStream(&svr_ctx, &response_rw, fixture->cq(),
                                fixture->cq(), tag(0));

      ClientContext cli_ctx;
      ClientContextMutator cli_ctx_mut(&cli_ctx);
      auto request_rw = stub->AsyncBidiStream(&cli_ctx, fixture->cq(), tag(1));

      // Establish async stream between client side and server side
      void* t;
      bool ok;
      int need_tags = (1 << 0) | (1 << 1);
      while (need_tags) {
        CHECK(fixture->cq()->Next(&t, &ok));
        CHECK(ok);
        int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
        CHECK(need_tags & (1 << i));
        need_tags &= ~(1 << i);
      }

      // Send 'max_ping_pongs' number of ping pong messages
      int ping_pong_cnt = 0;
      while (ping_pong_cnt < max_ping_pongs) {
        request_rw->Write(send_request, tag(0));   // Start client send
        response_rw.Read(&recv_request, tag(1));   // Start server recv
        request_rw->Read(&recv_response, tag(2));  // Start client recv

        need_tags = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);
        while (need_tags) {
          CHECK(fixture->cq()->Next(&t, &ok));
          CHECK(ok);
          int i = static_cast<int>(reinterpret_cast<intptr_t>(t));

          // If server recv is complete, start the server send operation
          if (i == 1) {
            response_rw.Write(send_response, tag(3));
          }

          CHECK(need_tags & (1 << i));
          need_tags &= ~(1 << i);
        }

        ping_pong_cnt++;
      }

      request_rw->WritesDone(tag(0));
      response_rw.Finish(Status::OK, tag(1));

      Status recv_status;
      request_rw->Finish(&recv_status, tag(2));

      need_tags = (1 << 0) | (1 << 1) | (1 << 2);
      while (need_tags) {
        CHECK(fixture->cq()->Next(&t, &ok));
        int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
        CHECK(need_tags & (1 << i));
        need_tags &= ~(1 << i);
      }

      CHECK(recv_status.ok());
    }
  }

  fixture.reset();
  state.SetBytesProcessed(msg_size * state.iterations() * max_ping_pongs * 2);
}

// Repeatedly sends ping pong messages in a single streaming Bidi call in a loop
//     First parameter (i.e state.range(0)):  Message size (in bytes) to use
template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_StreamingPingPongMsgs(benchmark::State& state) {
  const int msg_size = state.range(0);

  EchoTestService::AsyncService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  {
    EchoResponse send_response;
    EchoResponse recv_response;
    EchoRequest send_request;
    EchoRequest recv_request;

    if (msg_size > 0) {
      send_request.set_message(std::string(msg_size, 'a'));
      send_response.set_message(std::string(msg_size, 'b'));
    }

    std::unique_ptr<EchoTestService::Stub> stub(
        EchoTestService::NewStub(fixture->channel()));

    ServerContext svr_ctx;
    ServerContextMutator svr_ctx_mut(&svr_ctx);
    ServerAsyncReaderWriter<EchoResponse, EchoRequest> response_rw(&svr_ctx);
    service.RequestBidiStream(&svr_ctx, &response_rw, fixture->cq(),
                              fixture->cq(), tag(0));

    ClientContext cli_ctx;
    ClientContextMutator cli_ctx_mut(&cli_ctx);
    auto request_rw = stub->AsyncBidiStream(&cli_ctx, fixture->cq(), tag(1));

    // Establish async stream between client side and server side
    void* t;
    bool ok;
    int need_tags = (1 << 0) | (1 << 1);
    while (need_tags) {
      CHECK(fixture->cq()->Next(&t, &ok));
      CHECK(ok);
      int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
      CHECK(need_tags & (1 << i));
      need_tags &= ~(1 << i);
    }

    for (auto _ : state) {
      request_rw->Write(send_request, tag(0));   // Start client send
      response_rw.Read(&recv_request, tag(1));   // Start server recv
      request_rw->Read(&recv_response, tag(2));  // Start client recv

      need_tags = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);
      while (need_tags) {
        CHECK(fixture->cq()->Next(&t, &ok));
        CHECK(ok);
        int i = static_cast<int>(reinterpret_cast<intptr_t>(t));

        // If server recv is complete, start the server send operation
        if (i == 1) {
          response_rw.Write(send_response, tag(3));
        }

        CHECK(need_tags & (1 << i));
        need_tags &= ~(1 << i);
      }
    }

    request_rw->WritesDone(tag(0));
    response_rw.Finish(Status::OK, tag(1));
    Status recv_status;
    request_rw->Finish(&recv_status, tag(2));

    need_tags = (1 << 0) | (1 << 1) | (1 << 2);
    while (need_tags) {
      CHECK(fixture->cq()->Next(&t, &ok));
      int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
      CHECK(need_tags & (1 << i));
      need_tags &= ~(1 << i);
    }

    CHECK(recv_status.ok());
  }

  fixture.reset();
  state.SetBytesProcessed(msg_size * state.iterations() * 2);
}

// Repeatedly makes Streaming Bidi calls (exchanging a configurable number of
// messages in each call) in a loop on a single channel. Different from
// BM_StreamingPingPong we are using stream coalescing api, e.g. WriteLast,
// WriteAndFinish, set_initial_metadata_corked. These apis aim at saving
// sendmsg syscalls for streaming by coalescing 1. initial metadata with first
// message; 2. final streaming message with trailing metadata.
//
//  First parameter (i.e state.range(0)):  Message size (in bytes) to use
//  Second parameter (i.e state.range(1)): Number of ping pong messages.
//      Note: One ping-pong means two messages (one from client to server and
//      the other from server to client):
//  Third parameter (i.e state.range(2)): Switch between using WriteAndFinish
//  API and WriteLast API for server.
template <class Fixture, class ClientContextMutator, class ServerContextMutator>
static void BM_StreamingPingPongWithCoalescingApi(benchmark::State& state) {
  const int msg_size = state.range(0);
  const int max_ping_pongs = state.range(1);
  // This options is used to test out server API: WriteLast and WriteAndFinish
  // respectively, since we can not use both of them on server side at the same
  // time. Value 1 means we are testing out the WriteAndFinish API, and
  // otherwise we are testing out the WriteLast API.
  const int write_and_finish = state.range(2);

  EchoTestService::AsyncService service;
  std::unique_ptr<Fixture> fixture(new Fixture(&service));
  {
    EchoResponse send_response;
    EchoResponse recv_response;
    EchoRequest send_request;
    EchoRequest recv_request;

    if (msg_size > 0) {
      send_request.set_message(std::string(msg_size, 'a'));
      send_response.set_message(std::string(msg_size, 'b'));
    }

    std::unique_ptr<EchoTestService::Stub> stub(
        EchoTestService::NewStub(fixture->channel()));

    for (auto _ : state) {
      ServerContext svr_ctx;
      ServerContextMutator svr_ctx_mut(&svr_ctx);
      ServerAsyncReaderWriter<EchoResponse, EchoRequest> response_rw(&svr_ctx);
      service.RequestBidiStream(&svr_ctx, &response_rw, fixture->cq(),
                                fixture->cq(), tag(0));

      ClientContext cli_ctx;
      ClientContextMutator cli_ctx_mut(&cli_ctx);
      cli_ctx.set_initial_metadata_corked(true);
      // tag:1 here will never comes up, since we are not performing any op due
      // to initial metadata coalescing.
      auto request_rw = stub->AsyncBidiStream(&cli_ctx, fixture->cq(), tag(1));

      void* t;
      bool ok;
      int expect_tags = 0;

      // Send 'max_ping_pongs' number of ping pong messages
      int ping_pong_cnt = 0;
      while (ping_pong_cnt < max_ping_pongs) {
        if (ping_pong_cnt == max_ping_pongs - 1) {
          request_rw->WriteLast(send_request, WriteOptions(), tag(2));
        } else {
          request_rw->Write(send_request, tag(2));  // Start client send
        }

        int await_tags = (1 << 2);

        if (ping_pong_cnt == 0) {
          // wait for the server call structure (call_hook, etc.) to be
          // initialized (async stream between client side and server side
          // established). It is necessary when client init metadata is
          // coalesced
          CHECK(fixture->cq()->Next(&t, &ok));
          while (static_cast<int>(reinterpret_cast<intptr_t>(t)) != 0) {
            // In some cases tag:2 comes before tag:0 (write tag comes out
            // first), this while loop is to make sure get tag:0.
            int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
            CHECK(await_tags & (1 << i));
            await_tags &= ~(1 << i);
            CHECK(fixture->cq()->Next(&t, &ok));
          }
        }

        response_rw.Read(&recv_request, tag(3));   // Start server recv
        request_rw->Read(&recv_response, tag(4));  // Start client recv

        await_tags |= (1 << 3) | (1 << 4);
        expect_tags = await_tags;
        await_tags |= (1 << 5);

        while (await_tags != 0) {
          CHECK(fixture->cq()->Next(&t, &ok));
          CHECK(ok);
          int i = static_cast<int>(reinterpret_cast<intptr_t>(t));

          // If server recv is complete, start the server send operation
          if (i == 3) {
            if (ping_pong_cnt == max_ping_pongs - 1) {
              if (write_and_finish == 1) {
                response_rw.WriteAndFinish(send_response, WriteOptions(),
                                           Status::OK, tag(5));
                expect_tags |= (1 << 5);
              } else {
                response_rw.WriteLast(send_response, WriteOptions(), tag(5));
                // WriteLast buffers the write, so it's possible neither server
                // write op nor client read op will finish inside the while
                // loop.
                await_tags &= ~(1 << 4);
                await_tags &= ~(1 << 5);
                expect_tags |= (1 << 5);
              }
            } else {
              response_rw.Write(send_response, tag(5));
              expect_tags |= (1 << 5);
            }
          }

          CHECK(expect_tags & (1 << i));
          expect_tags &= ~(1 << i);
          await_tags &= ~(1 << i);
        }

        ping_pong_cnt++;
      }

      if (max_ping_pongs == 0) {
        expect_tags |= (1 << 6) | (1 << 7) | (1 << 8);
      } else {
        if (write_and_finish == 1) {
          expect_tags |= (1 << 8);
        } else {
          // server's buffered write and the client's read of the buffered write
          // tags should come up.
          expect_tags |= (1 << 7) | (1 << 8);
        }
      }

      // No message write or initial metadata write happened yet.
      if (max_ping_pongs == 0) {
        request_rw->WritesDone(tag(6));
        // wait for server call data structure(call_hook, etc.) to be
        // initialized, since initial metadata is corked.
        CHECK(fixture->cq()->Next(&t, &ok));
        while (static_cast<int>(reinterpret_cast<intptr_t>(t)) != 0) {
          int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
          CHECK(expect_tags & (1 << i));
          expect_tags &= ~(1 << i);
          CHECK(fixture->cq()->Next(&t, &ok));
        }
        response_rw.Finish(Status::OK, tag(7));
      } else {
        if (write_and_finish != 1) {
          response_rw.Finish(Status::OK, tag(7));
        }
      }

      Status recv_status;
      request_rw->Finish(&recv_status, tag(8));

      while (expect_tags) {
        CHECK(fixture->cq()->Next(&t, &ok));
        int i = static_cast<int>(reinterpret_cast<intptr_t>(t));
        CHECK(expect_tags & (1 << i));
        expect_tags &= ~(1 << i);
      }

      CHECK(recv_status.ok());
    }
  }

  fixture.reset();
  state.SetBytesProcessed(msg_size * state.iterations() * max_ping_pongs * 2);
}
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_FULLSTACK_STREAMING_PING_PONG_H
