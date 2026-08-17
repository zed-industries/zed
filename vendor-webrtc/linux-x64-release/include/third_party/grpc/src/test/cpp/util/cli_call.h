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

#ifndef GRPC_TEST_CPP_UTIL_CLI_CALL_H
#define GRPC_TEST_CPP_UTIL_CLI_CALL_H

#include <grpcpp/channel.h>
#include <grpcpp/completion_queue.h>
#include <grpcpp/generic/generic_stub.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/string_ref.h>

#include <map>

namespace grpc {

class ClientContext;

struct CliArgs {
  double timeout = -1;
};

namespace testing {

// CliCall handles the sending and receiving of generic messages given the name
// of the remote method. This class is only used by GrpcTool. Its thread-safe
// and thread-unsafe methods should not be used together.
class CliCall final {
 public:
  typedef std::multimap<std::string, std::string> OutgoingMetadataContainer;
  typedef std::multimap<grpc::string_ref, grpc::string_ref>
      IncomingMetadataContainer;

  CliCall(const std::shared_ptr<grpc::Channel>& channel,
          const std::string& method, const OutgoingMetadataContainer& metadata,
          CliArgs args);
  CliCall(const std::shared_ptr<grpc::Channel>& channel,
          const std::string& method, const OutgoingMetadataContainer& metadata)
      : CliCall(channel, method, metadata, CliArgs{}) {}

  ~CliCall();

  // Perform an unary generic RPC.
  Status Call(const std::string& request, std::string* response,
              IncomingMetadataContainer* server_initial_metadata,
              IncomingMetadataContainer* server_trailing_metadata);

  // Send a generic request message in a synchronous manner. NOT thread-safe.
  void Write(const std::string& request);

  // Send a generic request message in a synchronous manner. NOT thread-safe.
  void WritesDone();

  // Receive a generic response message in a synchronous manner.NOT thread-safe.
  bool Read(std::string* response,
            IncomingMetadataContainer* server_initial_metadata);

  // Thread-safe write. Must be used with ReadAndMaybeNotifyWrite. Send out a
  // generic request message and wait for ReadAndMaybeNotifyWrite to finish it.
  void WriteAndWait(const std::string& request);

  // Thread-safe WritesDone. Must be used with ReadAndMaybeNotifyWrite. Send out
  // WritesDone for gereneric request messages and wait for
  // ReadAndMaybeNotifyWrite to finish it.
  void WritesDoneAndWait();

  // Thread-safe Read. Blockingly receive a generic response message. Notify
  // writes if they are finished when this read is waiting for a response.
  bool ReadAndMaybeNotifyWrite(
      std::string* response,
      IncomingMetadataContainer* server_initial_metadata);

  // Finish the RPC.
  Status Finish(IncomingMetadataContainer* server_trailing_metadata);

  std::string peer() const { return ctx_.peer(); }

 private:
  std::unique_ptr<grpc::GenericStub> stub_;
  grpc::ClientContext ctx_;
  std::unique_ptr<grpc::GenericClientAsyncReaderWriter> call_;
  grpc::CompletionQueue cq_;
  gpr_mu write_mu_;
  gpr_cv write_cv_;  // Protected by write_mu_;
  bool write_done_;  // Protected by write_mu_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_CLI_CALL_H
