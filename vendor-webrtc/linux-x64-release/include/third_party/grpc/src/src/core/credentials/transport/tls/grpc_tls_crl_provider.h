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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CRL_PROVIDER_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CRL_PROVIDER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc_crl_provider.h>
#include <grpc/support/port_platform.h>
#include <openssl/crypto.h>

#include <chrono>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/util/directory_reader.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

namespace grpc_core {
namespace experimental {

class StaticCrlProvider : public CrlProvider {
 public:
  // Each element of the input vector is expected to be the raw contents of a
  // CRL file.
  explicit StaticCrlProvider(
      absl::flat_hash_map<std::string, std::shared_ptr<Crl>> crls)
      : crls_(std::move(crls)) {}
  std::shared_ptr<Crl> GetCrl(const CertificateInfo& certificate_info) override;

 private:
  const absl::flat_hash_map<std::string, std::shared_ptr<Crl>> crls_;
};

class CrlImpl : public Crl {
 public:
  static absl::StatusOr<std::unique_ptr<CrlImpl>> Create(X509_CRL* crl);
  // Takes ownership of the X509_CRL pointer.
  CrlImpl(X509_CRL* crl, absl::string_view issuer)
      : crl_(crl), issuer_(issuer) {}
  ~CrlImpl() override;
  // Returns a string view representation of the issuer pulled from the CRL.
  absl::string_view Issuer() override { return issuer_; }
  // The caller should not take ownership of the returned pointer.
  X509_CRL* crl() const { return crl_; }

 private:
  X509_CRL* crl_;
  const std::string issuer_;
};

class CertificateInfoImpl : public CertificateInfo {
 public:
  explicit CertificateInfoImpl(absl::string_view issuer,
                               absl::string_view authority_key_identifier = "")
      : issuer_(issuer), authority_key_identifier_(authority_key_identifier) {}
  absl::string_view Issuer() const override { return issuer_; }
  absl::string_view AuthorityKeyIdentifier() const override {
    return authority_key_identifier_;
  }

 private:
  const std::string issuer_;
  const std::string authority_key_identifier_;
};

// Defining this here lets us hide implementation details (and includes) from
// the header in include
class DirectoryReloaderCrlProvider
    : public CrlProvider,
      public std::enable_shared_from_this<DirectoryReloaderCrlProvider> {
 public:
  DirectoryReloaderCrlProvider(
      std::chrono::seconds duration, std::function<void(absl::Status)> callback,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine,
      std::shared_ptr<DirectoryReader> directory_impl);

  ~DirectoryReloaderCrlProvider() override;
  std::shared_ptr<Crl> GetCrl(const CertificateInfo& certificate_info) override;
  // Reads the configured directory and updates the internal crls_ map, called
  // asynchronously by event engine then schedules the timer for the next
  // update.
  void UpdateAndStartTimer();

 private:
  // Reads the configured directory and updates the internal crls_ map, called
  // asynchronously by event engine.
  absl::Status Update();
  Duration refresh_duration_;
  std::function<void(::absl::Status)> reload_error_callback_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
  std::shared_ptr<DirectoryReader> crl_directory_;
  // guards the crls_ map
  Mutex mu_;
  absl::flat_hash_map<::std::string, ::std::shared_ptr<Crl>> crls_
      ABSL_GUARDED_BY(mu_);
  std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
      refresh_handle_;
};

}  // namespace experimental
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CRL_PROVIDER_H