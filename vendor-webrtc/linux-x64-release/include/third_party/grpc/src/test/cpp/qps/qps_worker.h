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

#ifndef GRPC_TEST_CPP_QPS_QPS_WORKER_H
#define GRPC_TEST_CPP_QPS_QPS_WORKER_H

#include <grpc/support/atm.h>
#include <grpcpp/server.h>
#include <grpcpp/support/channel_arguments.h>
#include <grpcpp/support/config.h>

#include <memory>

#include "test/cpp/qps/server.h"

namespace grpc {

namespace testing {

class WorkerServiceImpl;

extern std::vector<grpc::testing::Server*>* g_inproc_servers;

class QpsWorker {
 public:
  explicit QpsWorker(int driver_port, int server_port,
                     const std::string& credential_type);
  ~QpsWorker();

  bool Done() const;
  void MarkDone();

  std::shared_ptr<Channel> InProcessChannel(const ChannelArguments& args) {
    return server_->InProcessChannel(args);
  }

 private:
  std::unique_ptr<WorkerServiceImpl> impl_;
  std::unique_ptr<grpc::Server> server_;

  gpr_atm done_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_QPS_QPS_WORKER_H
