//
// Copyright 2021 gRPC authors.
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

#ifndef GRPC_TEST_CPP_UTIL_TLS_TEST_UTILS_H
#define GRPC_TEST_CPP_UTIL_TLS_TEST_UTILS_H

#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpcpp/security/server_credentials.h>

#include <deque>

#include "src/core/util/thd.h"

namespace grpc {
namespace testing {

class SyncCertificateVerifier
    : public grpc::experimental::ExternalCertificateVerifier {
 public:
  explicit SyncCertificateVerifier(bool success) : success_(success) {}

  ~SyncCertificateVerifier() override {}

  bool Verify(grpc::experimental::TlsCustomVerificationCheckRequest* request,
              std::function<void(grpc::Status)> callback,
              grpc::Status* sync_status) override;

  void Cancel(grpc::experimental::TlsCustomVerificationCheckRequest*) override {
  }

 private:
  bool success_ = false;
};

class AsyncCertificateVerifier
    : public grpc::experimental::ExternalCertificateVerifier {
 public:
  explicit AsyncCertificateVerifier(bool success);

  ~AsyncCertificateVerifier() override;

  bool Verify(grpc::experimental::TlsCustomVerificationCheckRequest* request,
              std::function<void(grpc::Status)> callback,
              grpc::Status* sync_status) override;

  void Cancel(grpc::experimental::TlsCustomVerificationCheckRequest*) override {
  }

 private:
  // A request to pass to the worker thread.
  struct Request {
    grpc::experimental::TlsCustomVerificationCheckRequest* request;
    std::function<void(grpc::Status)> callback;
    bool shutdown;  // If true, thread will exit.
  };

  static void WorkerThread(void* arg);

  bool success_ = false;
  grpc_core::Thread thread_;
  grpc::internal::Mutex mu_;
  std::deque<Request> queue_ ABSL_GUARDED_BY(mu_);
};

class VerifiedRootCertSubjectVerifier
    : public grpc::experimental::ExternalCertificateVerifier {
 public:
  explicit VerifiedRootCertSubjectVerifier(absl::string_view expected_subject)
      : expected_subject_(expected_subject) {}

  ~VerifiedRootCertSubjectVerifier() override {}

  bool Verify(grpc::experimental::TlsCustomVerificationCheckRequest* request,
              std::function<void(grpc::Status)> callback,
              grpc::Status* sync_status) override;

  void Cancel(grpc::experimental::TlsCustomVerificationCheckRequest*) override {
  }

 private:
  std::string expected_subject_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_TLS_TEST_UTILS_H
