//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_DISTRIBUTOR_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_DISTRIBUTOR_H

#include <grpc/support/port_platform.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/transport/tls/ssl_utils.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/sync.h"

struct grpc_tls_identity_pairs {
  grpc_core::PemKeyCertPairList pem_key_cert_pairs;
};

// TLS certificate distributor.
struct grpc_tls_certificate_distributor
    : public grpc_core::RefCounted<grpc_tls_certificate_distributor> {
 public:
  // Interface for watching TLS certificates update.
  class TlsCertificatesWatcherInterface {
   public:
    virtual ~TlsCertificatesWatcherInterface() = default;

    // Handles the delivery of the updated root and identity certificates.
    // An std::nullopt value indicates no corresponding contents for
    // root_certs or key_cert_pairs. Note that we will send updates of the
    // latest contents for both root and identity certificates, even when only
    // one side of it got updated.
    //
    // @param root_certs the contents of the reloaded root certs.
    // @param key_cert_pairs the contents of the reloaded identity key-cert
    // pairs.
    virtual void OnCertificatesChanged(
        std::optional<absl::string_view> root_certs,
        std::optional<grpc_core::PemKeyCertPairList> key_cert_pairs) = 0;

    // Handles an error that occurs while attempting to fetch certificate data.
    // Note that if a watcher sees an error, it simply means the Provider is
    // having problems renewing new data. If the watcher has previously received
    // several OnCertificatesChanged, all the data received from that function
    // is valid.
    // In that case, watcher might simply log the error. If the watcher hasn't
    // received any OnCertificatesChanged before the error occurs, no valid
    // data is available yet, and the watcher should either fail or "waiting"
    // for the valid data in a non-blocking way.
    //
    // @param root_cert_error the error occurred while reloading root
    // certificates.
    // @param identity_cert_error the error occurred while reloading identity
    // certificates.
    virtual void OnError(grpc_error_handle root_cert_error,
                         grpc_error_handle identity_cert_error) = 0;
  };

  // Sets the key materials based on their certificate name.
  //
  // @param cert_name The name of the certificates being updated.
  // @param pem_root_certs The content of root certificates.
  // @param pem_key_cert_pairs The content of identity key-cert pairs.
  void SetKeyMaterials(
      const std::string& cert_name, std::optional<std::string> pem_root_certs,
      std::optional<grpc_core::PemKeyCertPairList> pem_key_cert_pairs);

  bool HasRootCerts(const std::string& root_cert_name);

  bool HasKeyCertPairs(const std::string& identity_cert_name);

  // Propagates the error that the caller (e.g. Producer) encounters to all the
  // watchers watching a particular certificate name.
  //
  // @param cert_name The watching cert name of the watchers that the caller
  // wants to notify when encountering error.
  // @param root_cert_error The error that the caller encounters when reloading
  // root certs.
  // @param identity_cert_error The error that the caller encounters when
  // reloading identity certs.
  void SetErrorForCert(const std::string& cert_name,
                       std::optional<grpc_error_handle> root_cert_error,
                       std::optional<grpc_error_handle> identity_cert_error);

  // Propagates the error that the caller (e.g. Producer) encounters to all
  // watchers.
  //
  // @param error The error that the caller encounters.
  void SetError(grpc_error_handle error);

  // Sets the TLS certificate watch status callback function. The
  // grpc_tls_certificate_distributor will invoke this callback when a new
  // certificate name is watched by a newly registered watcher, or when a
  // certificate name is no longer watched by any watchers.
  // Note that when the callback shows a cert is no longer being watched, the
  // distributor will delete the corresponding certificate data from its cache,
  // and clear the corresponding error, if there is any. This means that if the
  // callback subsequently says the same cert is now being watched again, the
  // provider must re-provide the credentials or re-invoke the errors to the
  // distributor, to indicate a successful or failed reloading.
  // @param callback The callback function being set by the caller, e.g the
  // Producer. Note that this callback will be invoked for each certificate
  // name.
  //
  // For the parameters in the callback function:
  // string_value The name of the certificates being watched.
  // bool_value_1 If the root certificates with the specific name are being
  // watched. bool_value_2 If the identity certificates with the specific name
  // are being watched.
  void SetWatchStatusCallback(
      std::function<void(std::string, bool, bool)> callback) {
    grpc_core::MutexLock lock(&callback_mu_);
    watch_status_callback_ = std::move(callback);
  };

  // Registers a watcher. The caller may keep a raw pointer to the watcher,
  // which may be used only for cancellation. (Because the caller does not own
  // the watcher, the pointer must not be used for any other purpose.) At least
  // one of root_cert_name and identity_cert_name must be specified.
  //
  // @param watcher The watcher being registered.
  // @param root_cert_name The name of the root certificates that will be
  // watched. If set to std::nullopt, the root certificates won't be watched.
  // @param identity_cert_name The name of the identity certificates that will
  // be watched. If set to std::nullopt, the identity certificates won't be
  // watched.
  void WatchTlsCertificates(
      std::unique_ptr<TlsCertificatesWatcherInterface> watcher,
      std::optional<std::string> root_cert_name,
      std::optional<std::string> identity_cert_name);

  // Cancels a watcher.
  //
  // @param watcher The watcher being cancelled.
  void CancelTlsCertificatesWatch(TlsCertificatesWatcherInterface* watcher);

 private:
  // Contains the information about each watcher.
  struct WatcherInfo {
    std::unique_ptr<TlsCertificatesWatcherInterface> watcher;
    std::optional<std::string> root_cert_name;
    std::optional<std::string> identity_cert_name;
  };
  // CertificateInfo contains the credential contents and some additional
  // watcher information.
  // Note that having errors doesn't indicate the corresponding credentials are
  // invalid. For example, if root_cert_error != nullptr but pem_root_certs has
  // value, it simply means an error occurs while trying to fetch the latest
  // root certs, while pem_root_certs still contains the valid old data.
  struct CertificateInfo {
    // The contents of the root certificates.
    std::string pem_root_certs;
    // The contents of the identity key-certificate pairs.
    grpc_core::PemKeyCertPairList pem_key_cert_pairs;
    // The root cert reloading error propagated by the caller.
    grpc_error_handle root_cert_error;
    // The identity cert reloading error propagated by the caller.
    grpc_error_handle identity_cert_error;
    // The set of watchers watching root certificates.
    // This is mainly used for quickly looking up the affected watchers while
    // performing a credential reloading.
    std::set<TlsCertificatesWatcherInterface*> root_cert_watchers;
    // The set of watchers watching identity certificates. This is mainly used
    // for quickly looking up the affected watchers while performing a
    // credential reloading.
    std::set<TlsCertificatesWatcherInterface*> identity_cert_watchers;

    ~CertificateInfo() {}
    void SetRootError(grpc_error_handle error) { root_cert_error = error; }
    void SetIdentityError(grpc_error_handle error) {
      identity_cert_error = error;
    }
  };

  grpc_core::Mutex mu_;
  // We need a dedicated mutex for watch_status_callback_ for allowing
  // callers(e.g. Producer) to directly set key materials in the callback
  // functions.
  grpc_core::Mutex callback_mu_;
  // Stores information about each watcher.
  std::map<TlsCertificatesWatcherInterface*, WatcherInfo> watchers_
      ABSL_GUARDED_BY(mu_);
  // The callback to notify the caller, e.g. the Producer, that the watch status
  // is changed.
  std::function<void(std::string, bool, bool)> watch_status_callback_
      ABSL_GUARDED_BY(callback_mu_);
  // Stores the names of each certificate, and their corresponding credential
  // contents as well as some additional watcher information.
  std::map<std::string, CertificateInfo> certificate_info_map_
      ABSL_GUARDED_BY(mu_);
};

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_GRPC_TLS_CERTIFICATE_DISTRIBUTOR_H
