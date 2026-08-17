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

#ifndef GRPC_SRC_CORE_XDS_GRPC_CERTIFICATE_PROVIDER_STORE_H
#define GRPC_SRC_CORE_XDS_GRPC_CERTIFICATE_PROVIDER_STORE_H

#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <map>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "src/core/credentials/transport/tls/certificate_provider_factory.h"
#include "src/core/credentials/transport/tls/grpc_tls_certificate_distributor.h"
#include "src/core/credentials/transport/tls/grpc_tls_certificate_provider.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

// Map for xDS based grpc_tls_certificate_provider instances.
class CertificateProviderStore final
    : public InternallyRefCounted<CertificateProviderStore> {
 public:
  struct PluginDefinition {
    std::string plugin_name;
    RefCountedPtr<CertificateProviderFactory::Config> config;

    static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
    void JsonPostLoad(const Json& json, const JsonArgs& args,
                      ValidationErrors* errors);
  };

  // Maps plugin instance (opaque) name to plugin definition.
  typedef std::map<std::string, PluginDefinition> PluginDefinitionMap;

  explicit CertificateProviderStore(PluginDefinitionMap plugin_config_map)
      : plugin_config_map_(std::move(plugin_config_map)) {}

  // If a certificate provider corresponding to the instance name \a key is
  // found, a ref to the grpc_tls_certificate_provider is returned. If no
  // provider is found for the key, a new provider is created from the plugin
  // definition map.
  // Returns nullptr on failure to get or create a new certificate provider.
  RefCountedPtr<grpc_tls_certificate_provider> CreateOrGetCertificateProvider(
      absl::string_view key);

  void Orphan() override { Unref(); }

 private:
  // A thin wrapper around `grpc_tls_certificate_provider` which allows removing
  // the entry from the CertificateProviderStore when the refcount reaches zero.
  class CertificateProviderWrapper final
      : public grpc_tls_certificate_provider {
   public:
    CertificateProviderWrapper(
        RefCountedPtr<grpc_tls_certificate_provider> certificate_provider,
        RefCountedPtr<CertificateProviderStore> store, absl::string_view key)
        : certificate_provider_(std::move(certificate_provider)),
          store_(std::move(store)),
          key_(key) {}

    ~CertificateProviderWrapper() override {
      store_->ReleaseCertificateProvider(key_, this);
    }

    RefCountedPtr<grpc_tls_certificate_distributor> distributor()
        const override {
      return certificate_provider_->distributor();
    }

    int CompareImpl(const grpc_tls_certificate_provider* other) const override {
      // TODO(yashykt): This should probably delegate to the `Compare` method of
      // the wrapped certificate_provider_ object.
      return QsortCompare(
          static_cast<const grpc_tls_certificate_provider*>(this), other);
    }

    UniqueTypeName type() const override;

    absl::string_view key() const { return key_; }

   private:
    RefCountedPtr<grpc_tls_certificate_provider> certificate_provider_;
    RefCountedPtr<CertificateProviderStore> store_;
    absl::string_view key_;
  };

  RefCountedPtr<CertificateProviderWrapper> CreateCertificateProviderLocked(
      absl::string_view key) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  // Releases a previously created certificate provider from the certificate
  // provider map if the value matches \a wrapper.
  void ReleaseCertificateProvider(absl::string_view key,
                                  CertificateProviderWrapper* wrapper);

  Mutex mu_;
  // Map of plugin configurations
  const PluginDefinitionMap plugin_config_map_;
  // Underlying map for the providers.
  std::map<absl::string_view, CertificateProviderWrapper*>
      certificate_providers_map_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_GRPC_CERTIFICATE_PROVIDER_STORE_H
