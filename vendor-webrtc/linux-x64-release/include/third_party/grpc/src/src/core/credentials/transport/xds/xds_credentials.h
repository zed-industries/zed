//
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
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_XDS_XDS_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_XDS_XDS_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <functional>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/tls/grpc_tls_certificate_verifier.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/matchers.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/xds/grpc/xds_certificate_provider.h"

namespace grpc_core {

class XdsCertificateVerifier : public grpc_tls_certificate_verifier {
 public:
  explicit XdsCertificateVerifier(
      RefCountedPtr<XdsCertificateProvider> xds_certificate_provider);

  bool Verify(grpc_tls_custom_verification_check_request* request,
              std::function<void(absl::Status)>,
              absl::Status* sync_status) override;
  void Cancel(grpc_tls_custom_verification_check_request*) override;

  UniqueTypeName type() const override;

 private:
  int CompareImpl(const grpc_tls_certificate_verifier* other) const override;

  RefCountedPtr<XdsCertificateProvider> xds_certificate_provider_;
};

class XdsCredentials final : public grpc_channel_credentials {
 public:
  explicit XdsCredentials(
      RefCountedPtr<grpc_channel_credentials> fallback_credentials)
      : fallback_credentials_(std::move(fallback_credentials)) {}

  RefCountedPtr<grpc_channel_security_connector> create_security_connector(
      RefCountedPtr<grpc_call_credentials> call_creds, const char* target_name,
      ChannelArgs* args) override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override {
    auto* o = static_cast<const XdsCredentials*>(other);
    return fallback_credentials_->cmp(o->fallback_credentials_.get());
  }

  RefCountedPtr<grpc_channel_credentials> fallback_credentials_;
};

class XdsServerCredentials final : public grpc_server_credentials {
 public:
  explicit XdsServerCredentials(
      RefCountedPtr<grpc_server_credentials> fallback_credentials)
      : fallback_credentials_(std::move(fallback_credentials)) {}

  RefCountedPtr<grpc_server_security_connector> create_security_connector(
      const ChannelArgs& /* args */) override;

  static UniqueTypeName Type();

  UniqueTypeName type() const override { return Type(); }

 private:
  RefCountedPtr<grpc_server_credentials> fallback_credentials_;
};

bool TestOnlyXdsVerifySubjectAlternativeNames(
    const char* const* subject_alternative_names,
    size_t subject_alternative_names_size,
    const std::vector<StringMatcher>& matchers);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_XDS_XDS_CREDENTIALS_H
