//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_TEST_CORE_END2END_FIXTURES_H2_TLS_COMMON_H
#define GRPC_TEST_CORE_END2END_FIXTURES_H2_TLS_COMMON_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/impl/channel_arg_names.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <stdint.h>
#include <string.h>

#include <string>

#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/transport/tls/grpc_tls_credentials_options.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/slice/slice_internal.h"
#include "test/core/end2end/end2end_tests.h"
#include "test/core/end2end/fixtures/secure_fixture.h"
#include "test/core/test_util/tls_utils.h"

// For normal TLS connections.
#define CA_CERT_PATH "src/core/tsi/test_creds/ca.pem"
#define SERVER_CERT_PATH "src/core/tsi/test_creds/server1.pem"
#define SERVER_KEY_PATH "src/core/tsi/test_creds/server1.key"

struct SecurityPrimitives {
  enum ProviderType { STATIC_PROVIDER = 0, FILE_PROVIDER = 1 } provider_type;
  enum VerifierType {
    EXTERNAL_SYNC_VERIFIER = 0,
    EXTERNAL_ASYNC_VERIFIER = 1,
    HOSTNAME_VERIFIER = 2
  } verifier_type;
  enum TlsVersion { V_12 = 0, V_13 = 1 } tls_version;
};

inline void process_auth_failure(void* state, grpc_auth_context* /*ctx*/,
                                 const grpc_metadata* /*md*/,
                                 size_t /*md_count*/,
                                 grpc_process_auth_metadata_done_cb cb,
                                 void* user_data) {
  CHECK_EQ(state, nullptr);
  cb(user_data, nullptr, 0, nullptr, 0, GRPC_STATUS_UNAUTHENTICATED, nullptr);
}

class TlsFixture : public SecureFixture {
 public:
  TlsFixture(SecurityPrimitives::TlsVersion tls_version,
             SecurityPrimitives::ProviderType provider_type,
             SecurityPrimitives::VerifierType verifier_type) {
    switch (tls_version) {
      case SecurityPrimitives::TlsVersion::V_12: {
        tls_version_ = grpc_tls_version::TLS1_2;
        break;
      }
      case SecurityPrimitives::TlsVersion::V_13: {
        tls_version_ = grpc_tls_version::TLS1_3;
        break;
      }
    }
    switch (provider_type) {
      case SecurityPrimitives::ProviderType::STATIC_PROVIDER: {
        std::string root_cert =
            grpc_core::testing::GetFileContents(CA_CERT_PATH);
        std::string identity_cert =
            grpc_core::testing::GetFileContents(SERVER_CERT_PATH);
        std::string private_key =
            grpc_core::testing::GetFileContents(SERVER_KEY_PATH);
        grpc_tls_identity_pairs* client_pairs =
            grpc_tls_identity_pairs_create();
        grpc_tls_identity_pairs_add_pair(client_pairs, private_key.c_str(),
                                         identity_cert.c_str());
        client_provider_ = grpc_tls_certificate_provider_static_data_create(
            root_cert.c_str(), client_pairs);
        grpc_tls_identity_pairs* server_pairs =
            grpc_tls_identity_pairs_create();
        grpc_tls_identity_pairs_add_pair(server_pairs, private_key.c_str(),
                                         identity_cert.c_str());
        server_provider_ = grpc_tls_certificate_provider_static_data_create(
            root_cert.c_str(), server_pairs);
        break;
      }
      case SecurityPrimitives::ProviderType::FILE_PROVIDER: {
        client_provider_ = grpc_tls_certificate_provider_file_watcher_create(
            SERVER_KEY_PATH, SERVER_CERT_PATH, CA_CERT_PATH, 1);
        server_provider_ = grpc_tls_certificate_provider_file_watcher_create(
            SERVER_KEY_PATH, SERVER_CERT_PATH, CA_CERT_PATH, 1);
        break;
      }
    }
    switch (verifier_type) {
      case SecurityPrimitives::VerifierType::EXTERNAL_SYNC_VERIFIER: {
        auto* client_sync_verifier =
            new grpc_core::testing::SyncExternalVerifier(true);
        client_verifier_ = grpc_tls_certificate_verifier_external_create(
            client_sync_verifier->base());
        auto* server_sync_verifier =
            new grpc_core::testing::SyncExternalVerifier(true);
        server_verifier_ = grpc_tls_certificate_verifier_external_create(
            server_sync_verifier->base());
        check_call_host_ = false;
        break;
      }
      case SecurityPrimitives::VerifierType::EXTERNAL_ASYNC_VERIFIER: {
        auto* client_async_verifier =
            new grpc_core::testing::AsyncExternalVerifier(true);
        client_verifier_ = grpc_tls_certificate_verifier_external_create(
            client_async_verifier->base());
        auto* server_async_verifier =
            new grpc_core::testing::AsyncExternalVerifier(true);
        server_verifier_ = grpc_tls_certificate_verifier_external_create(
            server_async_verifier->base());
        check_call_host_ = false;
        break;
      }
      case SecurityPrimitives::VerifierType::HOSTNAME_VERIFIER: {
        client_verifier_ = grpc_tls_certificate_verifier_host_name_create();
        // Hostname verifier couldn't be applied to the server side, so we will
        // use sync external verifier here.
        auto* server_async_verifier =
            new grpc_core::testing::AsyncExternalVerifier(true);
        server_verifier_ = grpc_tls_certificate_verifier_external_create(
            server_async_verifier->base());
        break;
      }
    }
  }
  ~TlsFixture() override {
    grpc_tls_certificate_provider_release(client_provider_);
    grpc_tls_certificate_provider_release(server_provider_);
    grpc_tls_certificate_verifier_release(client_verifier_);
    grpc_tls_certificate_verifier_release(server_verifier_);
  }

 private:
  grpc_core::ChannelArgs MutateClientArgs(
      grpc_core::ChannelArgs args) override {
    return args.Set(GRPC_SSL_TARGET_NAME_OVERRIDE_ARG, "foo.test.google.fr");
  }

  grpc_channel_credentials* MakeClientCreds(
      const grpc_core::ChannelArgs&) override {
    grpc_tls_credentials_options* options =
        grpc_tls_credentials_options_create();
    grpc_tls_credentials_options_set_verify_server_cert(
        options, 1 /* = verify server certs */);
    options->set_min_tls_version(tls_version_);
    options->set_max_tls_version(tls_version_);
    // Set credential provider.
    grpc_tls_credentials_options_set_certificate_provider(options,
                                                          client_provider_);
    grpc_tls_credentials_options_watch_root_certs(options);
    grpc_tls_credentials_options_watch_identity_key_cert_pairs(options);
    // Set credential verifier.
    grpc_tls_credentials_options_set_certificate_verifier(options,
                                                          client_verifier_);
    grpc_tls_credentials_options_set_check_call_host(options, check_call_host_);
    // Create TLS channel credentials.
    grpc_channel_credentials* creds = grpc_tls_credentials_create(options);
    return creds;
  }

  grpc_server_credentials* MakeServerCreds(
      const grpc_core::ChannelArgs& args) override {
    grpc_tls_credentials_options* options =
        grpc_tls_credentials_options_create();
    options->set_min_tls_version(tls_version_);
    options->set_max_tls_version(tls_version_);
    // Set credential provider.
    grpc_tls_credentials_options_set_certificate_provider(options,
                                                          server_provider_);
    grpc_tls_credentials_options_watch_root_certs(options);
    grpc_tls_credentials_options_watch_identity_key_cert_pairs(options);
    // Set client certificate request type.
    grpc_tls_credentials_options_set_cert_request_type(
        options, GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY);
    // Set credential verifier.
    grpc_tls_credentials_options_set_certificate_verifier(options,
                                                          server_verifier_);
    grpc_server_credentials* creds =
        grpc_tls_server_credentials_create(options);
    if (args.Contains(FAIL_AUTH_CHECK_SERVER_ARG_NAME)) {
      grpc_auth_metadata_processor processor = {process_auth_failure, nullptr,
                                                nullptr};
      grpc_server_credentials_set_auth_metadata_processor(creds, processor);
    }
    return creds;
  }

  grpc_tls_version tls_version_;
  grpc_tls_certificate_provider* client_provider_ = nullptr;
  grpc_tls_certificate_provider* server_provider_ = nullptr;
  grpc_tls_certificate_verifier* client_verifier_ = nullptr;
  grpc_tls_certificate_verifier* server_verifier_ = nullptr;
  bool check_call_host_ = true;
};

static const uint32_t kH2TLSFeatureMask =
    FEATURE_MASK_SUPPORTS_PER_CALL_CREDENTIALS |
    FEATURE_MASK_SUPPORTS_CLIENT_CHANNEL | FEATURE_MASK_IS_HTTP2;

#endif  // GRPC_TEST_CORE_END2END_FIXTURES_H2_TLS_COMMON_H
