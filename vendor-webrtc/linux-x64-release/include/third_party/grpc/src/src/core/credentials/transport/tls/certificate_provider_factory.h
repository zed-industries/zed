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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_CERTIFICATE_PROVIDER_FACTORY_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_CERTIFICATE_PROVIDER_FACTORY_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <string>

#include "absl/strings/string_view.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

// Factories for plugins. Each plugin implementation should create its own
// factory implementation and register an instance with the registry.
class CertificateProviderFactory {
 public:
  // Interface for configs for CertificateProviders.
  class Config : public RefCounted<Config> {
   public:
    ~Config() override = default;

    // Name of the type of the CertificateProvider. Unique to each type of
    // config.
    virtual absl::string_view name() const = 0;

    virtual std::string ToString() const = 0;
  };

  virtual ~CertificateProviderFactory() = default;

  // Name of the plugin.
  virtual absl::string_view name() const = 0;

  virtual RefCountedPtr<Config> CreateCertificateProviderConfig(
      const Json& config_json, const JsonArgs& args,
      ValidationErrors* errors) = 0;

  // Create a CertificateProvider instance from config.
  virtual RefCountedPtr<grpc_tls_certificate_provider>
  CreateCertificateProvider(RefCountedPtr<Config> config) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_TLS_CERTIFICATE_PROVIDER_FACTORY_H
