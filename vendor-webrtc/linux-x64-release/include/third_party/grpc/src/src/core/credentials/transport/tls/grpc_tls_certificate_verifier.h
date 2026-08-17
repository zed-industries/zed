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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_VERIFIER_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_VERIFIER_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include <functional>
#include <map>

#include "absl/base/thread_annotations.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/sync.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

// An abstraction of the verifier that all verifier subclasses should extend.
struct grpc_tls_certificate_verifier
    : public grpc_core::RefCounted<grpc_tls_certificate_verifier> {
 public:
  ~grpc_tls_certificate_verifier() override = default;
  // Verifies the specific request. It can be processed in sync or async mode.
  // If the caller want it to be processed asynchronously, return false
  // immediately, and at the end of the async operation, invoke the callback
  // with the verification results stored in absl::Status. Otherwise, populate
  // the verification results in |sync_status| and return true. The caller is
  // expected to populate verification results by setting request.
  virtual bool Verify(grpc_tls_custom_verification_check_request* request,
                      std::function<void(absl::Status)> callback,
                      absl::Status* sync_status) = 0;
  // Operations that will be performed when a request is cancelled.
  // This is only needed when in async mode.
  // TODO(roth): This needs to take an absl::Status argument so that we
  // can pass the cancellation status through to the check_peer callback.
  virtual void Cancel(grpc_tls_custom_verification_check_request* request) = 0;

  // Compares this grpc_tls_certificate_verifier object with \a other.
  // If this method returns 0, it means that gRPC can treat the two certificate
  // verifiers as effectively the same.
  int Compare(const grpc_tls_certificate_verifier* other) const {
    CHECK_NE(other, nullptr);
    int r = type().Compare(other->type());
    if (r != 0) return r;
    return CompareImpl(other);
  }

  // The pointer value \a type is used to uniquely identify a verifier
  // implementation for down-casting purposes. Every verifier implementation
  // should use a unique string instance, which should be returned by all
  // instances of that verifier implementation.
  virtual grpc_core::UniqueTypeName type() const = 0;

 private:
  // Implementation for `Compare` method intended to be overridden by
  // subclasses. Only invoked if `type()` and `other->type()` point to the same
  // string.
  virtual int CompareImpl(const grpc_tls_certificate_verifier* other) const = 0;
};

namespace grpc_core {

// A verifier that will transform grpc_tls_certificate_verifier_external to a
// verifier that extends grpc_tls_certificate_verifier.
class ExternalCertificateVerifier : public grpc_tls_certificate_verifier {
 public:
  explicit ExternalCertificateVerifier(
      grpc_tls_certificate_verifier_external* external_verifier)
      : external_verifier_(external_verifier) {}

  ~ExternalCertificateVerifier() override {
    if (external_verifier_->destruct != nullptr) {
      external_verifier_->destruct(external_verifier_->user_data);
    }
  }

  bool Verify(grpc_tls_custom_verification_check_request* request,
              std::function<void(absl::Status)> callback,
              absl::Status* sync_status) override;

  void Cancel(grpc_tls_custom_verification_check_request* request) override {
    external_verifier_->cancel(external_verifier_->user_data, request);
  }

  UniqueTypeName type() const override;

 private:
  int CompareImpl(const grpc_tls_certificate_verifier* other) const override {
    const auto* o = static_cast<const ExternalCertificateVerifier*>(other);
    return QsortCompare(external_verifier_, o->external_verifier_);
  }

  static void OnVerifyDone(grpc_tls_custom_verification_check_request* request,
                           void* callback_arg, grpc_status_code status,
                           const char* error_details);

  grpc_tls_certificate_verifier_external* external_verifier_;

  // Guards members below.
  Mutex mu_;
  // stores each check request and its corresponding callback function.
  std::map<grpc_tls_custom_verification_check_request*,
           std::function<void(absl::Status)>>
      request_map_ ABSL_GUARDED_BY(mu_);
};

// An internal verifier that won't perform any post-handshake checks.
// Note: using this solely without any other authentication mechanisms on the
// peer identity will leave your applications to the MITM(Man-In-The-Middle)
// attacks. Users should avoid doing so in production environments.
class NoOpCertificateVerifier : public grpc_tls_certificate_verifier {
 public:
  bool Verify(grpc_tls_custom_verification_check_request*,
              std::function<void(absl::Status)>, absl::Status*) override {
    return true;  // synchronous check
  };
  void Cancel(grpc_tls_custom_verification_check_request*) override {}

  UniqueTypeName type() const override;

 private:
  int CompareImpl(
      const grpc_tls_certificate_verifier* /* other */) const override {
    // No differentiating factor between different NoOpCertificateVerifier
    // objects.
    return 0;
  }
};

// An internal verifier that will perform hostname verification check.
class HostNameCertificateVerifier : public grpc_tls_certificate_verifier {
 public:
  bool Verify(grpc_tls_custom_verification_check_request* request,
              std::function<void(absl::Status)> callback,
              absl::Status* sync_status) override;
  void Cancel(grpc_tls_custom_verification_check_request*) override {}

  UniqueTypeName type() const override;

 private:
  int CompareImpl(
      const grpc_tls_certificate_verifier* /* other */) const override {
    // No differentiating factor between different HostNameCertificateVerifier
    // objects.
    return 0;
  }
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_VERIFIER_H
