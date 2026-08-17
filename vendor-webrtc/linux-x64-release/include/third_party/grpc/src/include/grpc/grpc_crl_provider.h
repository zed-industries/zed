//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_GRPC_CRL_PROVIDER_H
#define GRPC_GRPC_CRL_PROVIDER_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace grpc_core {
namespace experimental {

// Opaque representation of a CRL. Must be thread safe.
class Crl {
 public:
  static absl::StatusOr<std::unique_ptr<Crl>> Parse(
      absl::string_view crl_string);
  virtual ~Crl() = default;
  virtual absl::string_view Issuer() = 0;
};

// Information about a certificate to be used to fetch its associated CRL. Must
// be thread safe.
class CertificateInfo {
 public:
  virtual ~CertificateInfo() = default;
  virtual absl::string_view Issuer() const = 0;
  virtual absl::string_view AuthorityKeyIdentifier() const = 0;
};

// The base class for CRL Provider implementations.
// CrlProviders can be passed in as a way to supply CRLs during handshakes.
// CrlProviders must be thread safe. They are on the critical path of gRPC
// creating a connection and doing a handshake, so the implementation of
// `GetCrl` should be very fast. It is suggested to have an in-memory map of
// CRLs for quick lookup and return, and doing expensive updates to this map
// asynchronously.
class CrlProvider {
 public:
  virtual ~CrlProvider() = default;
  // Get the CRL associated with a certificate. Read-only.
  virtual std::shared_ptr<Crl> GetCrl(
      const CertificateInfo& certificate_info) = 0;
};

absl::StatusOr<std::shared_ptr<CrlProvider>> CreateStaticCrlProvider(
    absl::Span<const std::string> crls);

// Creates a CRL Provider that periodically and asynchronously reloads a
// directory. The refresh_duration minimum is 60 seconds. The
// reload_error_callback provides a way for the user to specifically log or
// otherwise notify of errors during reloading. Since reloading is asynchronous
// and not on the main codepath, the grpc process will continue to run through
// reloading errors, so this mechanism is an important way to provide signals to
// your monitoring and alerting setup.
absl::StatusOr<std::shared_ptr<CrlProvider>> CreateDirectoryReloaderCrlProvider(
    absl::string_view directory, std::chrono::seconds refresh_duration,
    std::function<void(absl::Status)> reload_error_callback);

}  // namespace experimental
}  // namespace grpc_core

// TODO(gtcooke94) - Mark with api macro when all wrapped languages support C++
// in core APIs
/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the crl provider in the options.
 */
void grpc_tls_credentials_options_set_crl_provider(
    grpc_tls_credentials_options* options,
    std::shared_ptr<grpc_core::experimental::CrlProvider> provider);
#endif /* GRPC_GRPC_CRL_PROVIDER_H */
