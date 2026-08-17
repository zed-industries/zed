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

#ifndef GRPC_TEST_CPP_IOS_CRONETTESTS_TESTHELPER_H
#define GRPC_TEST_CPP_IOS_CRONETTESTS_TESTHELPER_H

#import <XCTest/XCTest.h>
#import <grpc/support/time.h>
#import <grpcpp/support/client_interceptor.h>
#import <grpcpp/support/config.h>
#import <grpcpp/support/string_ref.h>

#import <map>
#import <sstream>

#import "src/proto/grpc/testing/echo.grpc.pb.h"

const char* const kServerFinishAfterNReads = "server_finish_after_n_reads";
const char* const kServerResponseStreamsToSend = "server_responses_to_send";
const int kServerDefaultResponseStreamsToSend = 3;
const char* const kDebugInfoTrailerKey = "debug-info-bin";

std::string ToString(const grpc::string_ref& r);
void configureCronet(void);
bool CheckIsLocalhost(const std::string& addr);

class PhonyInterceptor : public grpc::experimental::Interceptor {
 public:
  PhonyInterceptor() {}
  virtual void Intercept(grpc::experimental::InterceptorBatchMethods* methods);
  static void Reset();
  static int GetNumTimesRun();

 private:
  static std::atomic<int> num_times_run_;
  static std::atomic<int> num_times_run_reverse_;
};

class PhonyInterceptorFactory
    : public grpc::experimental::ClientInterceptorFactoryInterface {
 public:
  virtual grpc::experimental::Interceptor* CreateClientInterceptor(
      grpc::experimental::ClientRpcInfo* info) override {
    return new PhonyInterceptor();
  }
};

class TestServiceImpl : public grpc::testing::EchoTestService::Service {
 public:
  grpc::Status Echo(grpc::ServerContext* context,
                    const grpc::testing::EchoRequest* request,
                    grpc::testing::EchoResponse* response);
  grpc::Status RequestStream(
      grpc::ServerContext* context,
      grpc::ServerReader<grpc::testing::EchoRequest>* reader,
      grpc::testing::EchoResponse* response);
  grpc::Status ResponseStream(
      grpc::ServerContext* context, const grpc::testing::EchoRequest* request,
      grpc::ServerWriter<grpc::testing::EchoResponse>* writer);

  grpc::Status BidiStream(
      grpc::ServerContext* context,
      grpc::ServerReaderWriter<grpc::testing::EchoResponse,
                               grpc::testing::EchoRequest>* stream);
};

#endif  // GRPC_TEST_CPP_IOS_CRONETTESTS_TESTHELPER_H
