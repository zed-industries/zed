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

#ifndef GRPCPP_TEST_MOCK_STREAM_H
#define GRPCPP_TEST_MOCK_STREAM_H

#include <grpcpp/impl/call.h>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/async_unary_call.h>
#include <grpcpp/support/sync_stream.h>
#include <stdint.h>

#include "gmock/gmock.h"

namespace grpc {
namespace testing {

template <class R>
class MockClientReader : public grpc::ClientReaderInterface<R> {
 public:
  MockClientReader() = default;

  /// ClientStreamingInterface
  MOCK_METHOD0_T(Finish, Status());

  /// ReaderInterface
  MOCK_METHOD1_T(NextMessageSize, bool(uint32_t*));
  MOCK_METHOD1_T(Read, bool(R*));

  /// ClientReaderInterface
  MOCK_METHOD0_T(WaitForInitialMetadata, void());
};

template <class W>
class MockClientWriter : public grpc::ClientWriterInterface<W> {
 public:
  MockClientWriter() = default;

  /// ClientStreamingInterface
  MOCK_METHOD0_T(Finish, Status());

  /// WriterInterface
  MOCK_METHOD2_T(Write, bool(const W&, const WriteOptions));

  /// ClientWriterInterface
  MOCK_METHOD0_T(WritesDone, bool());
};

template <class W, class R>
class MockClientReaderWriter : public grpc::ClientReaderWriterInterface<W, R> {
 public:
  MockClientReaderWriter() = default;

  /// ClientStreamingInterface
  MOCK_METHOD0_T(Finish, Status());

  /// ReaderInterface
  MOCK_METHOD1_T(NextMessageSize, bool(uint32_t*));
  MOCK_METHOD1_T(Read, bool(R*));

  /// WriterInterface
  MOCK_METHOD2_T(Write, bool(const W&, const WriteOptions));

  /// ClientReaderWriterInterface
  MOCK_METHOD0_T(WaitForInitialMetadata, void());
  MOCK_METHOD0_T(WritesDone, bool());
};

/// TODO: We do not support mocking an async RPC for now.

template <class R>
class MockClientAsyncResponseReader
    : public grpc::ClientAsyncResponseReaderInterface<R> {
 public:
  MockClientAsyncResponseReader() = default;

  /// ClientAsyncResponseReaderInterface
  MOCK_METHOD0_T(StartCall, void());
  MOCK_METHOD1_T(ReadInitialMetadata, void(void*));
  MOCK_METHOD3_T(Finish, void(R*, Status*, void*));
};

template <class R>
class MockClientAsyncReader : public ClientAsyncReaderInterface<R> {
 public:
  MockClientAsyncReader() = default;

  /// ClientAsyncStreamingInterface
  MOCK_METHOD1_T(StartCall, void(void*));
  MOCK_METHOD1_T(ReadInitialMetadata, void(void*));
  MOCK_METHOD2_T(Finish, void(Status*, void*));

  /// AsyncReaderInterface
  MOCK_METHOD2_T(Read, void(R*, void*));
};

template <class W>
class MockClientAsyncWriter : public grpc::ClientAsyncWriterInterface<W> {
 public:
  MockClientAsyncWriter() = default;

  /// ClientAsyncStreamingInterface
  MOCK_METHOD1_T(StartCall, void(void*));
  MOCK_METHOD1_T(ReadInitialMetadata, void(void*));
  MOCK_METHOD2_T(Finish, void(Status*, void*));

  /// AsyncWriterInterface
  MOCK_METHOD2_T(Write, void(const W&, void*));
  MOCK_METHOD3_T(Write, void(const W&, grpc::WriteOptions, void*));

  /// ClientAsyncWriterInterface
  MOCK_METHOD1_T(WritesDone, void(void*));
};

template <class W, class R>
class MockClientAsyncReaderWriter
    : public ClientAsyncReaderWriterInterface<W, R> {
 public:
  MockClientAsyncReaderWriter() = default;

  /// ClientAsyncStreamingInterface
  MOCK_METHOD1_T(StartCall, void(void*));
  MOCK_METHOD1_T(ReadInitialMetadata, void(void*));
  MOCK_METHOD2_T(Finish, void(Status*, void*));

  /// AsyncWriterInterface
  MOCK_METHOD2_T(Write, void(const W&, void*));
  MOCK_METHOD3_T(Write, void(const W&, grpc::WriteOptions, void*));

  /// AsyncReaderInterface
  MOCK_METHOD2_T(Read, void(R*, void*));

  /// ClientAsyncReaderWriterInterface
  MOCK_METHOD1_T(WritesDone, void(void*));
};

template <class R>
class MockServerReader : public grpc::ServerReaderInterface<R> {
 public:
  MockServerReader() = default;

  /// ServerStreamingInterface
  MOCK_METHOD0_T(SendInitialMetadata, void());

  /// ReaderInterface
  MOCK_METHOD1_T(NextMessageSize, bool(uint32_t*));
  MOCK_METHOD1_T(Read, bool(R*));
};

template <class W>
class MockServerWriter : public grpc::ServerWriterInterface<W> {
 public:
  MockServerWriter() = default;

  /// ServerStreamingInterface
  MOCK_METHOD0_T(SendInitialMetadata, void());

  /// WriterInterface
  MOCK_METHOD2_T(Write, bool(const W&, const WriteOptions));
};

template <class W, class R>
class MockServerReaderWriter : public grpc::ServerReaderWriterInterface<W, R> {
 public:
  MockServerReaderWriter() = default;

  /// ServerStreamingInterface
  MOCK_METHOD0_T(SendInitialMetadata, void());

  /// ReaderInterface
  MOCK_METHOD1_T(NextMessageSize, bool(uint32_t*));
  MOCK_METHOD1_T(Read, bool(R*));

  /// WriterInterface
  MOCK_METHOD2_T(Write, bool(const W&, const WriteOptions));
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPCPP_TEST_MOCK_STREAM_H
