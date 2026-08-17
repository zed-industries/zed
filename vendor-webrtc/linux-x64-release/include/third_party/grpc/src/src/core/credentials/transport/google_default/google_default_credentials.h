//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_GOOGLE_DEFAULT_GOOGLE_DEFAULT_CREDENTIALS_H
#define GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_GOOGLE_DEFAULT_GOOGLE_DEFAULT_CREDENTIALS_H
#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <utility>

#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"
#include "src/core/util/useful.h"

#define GRPC_GOOGLE_CLOUD_SDK_CONFIG_DIRECTORY "gcloud"
#define GRPC_GOOGLE_WELL_KNOWN_CREDENTIALS_FILE \
  "application_default_credentials.json"

#ifdef GPR_WINDOWS
#define GRPC_GOOGLE_CREDENTIALS_PATH_ENV_VAR "APPDATA"
#define GRPC_GOOGLE_CREDENTIALS_PATH_SUFFIX \
  GRPC_GOOGLE_CLOUD_SDK_CONFIG_DIRECTORY    \
  "/" GRPC_GOOGLE_WELL_KNOWN_CREDENTIALS_FILE
#else
#define GRPC_GOOGLE_CREDENTIALS_PATH_ENV_VAR "HOME"
#define GRPC_GOOGLE_CREDENTIALS_PATH_SUFFIX         \
  ".config/" GRPC_GOOGLE_CLOUD_SDK_CONFIG_DIRECTORY \
  "/" GRPC_GOOGLE_WELL_KNOWN_CREDENTIALS_FILE
#endif

class grpc_google_default_channel_credentials
    : public grpc_channel_credentials {
 public:
  grpc_google_default_channel_credentials(
      grpc_core::RefCountedPtr<grpc_channel_credentials> alts_creds,
      grpc_core::RefCountedPtr<grpc_channel_credentials> ssl_creds)
      : alts_creds_(std::move(alts_creds)), ssl_creds_(std::move(ssl_creds)) {}

  ~grpc_google_default_channel_credentials() override = default;

  grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target, grpc_core::ChannelArgs* args) override;

  grpc_core::ChannelArgs update_arguments(grpc_core::ChannelArgs args) override;

  static grpc_core::UniqueTypeName Type();

  grpc_core::UniqueTypeName type() const override { return Type(); }

  const grpc_channel_credentials* alts_creds() const {
    return alts_creds_.get();
  }
  const grpc_channel_credentials* ssl_creds() const { return ssl_creds_.get(); }

 private:
  int cmp_impl(const grpc_channel_credentials* other) const override {
    // TODO(yashykt): Check if we can do something better here
    return grpc_core::QsortCompare(
        static_cast<const grpc_channel_credentials*>(this), other);
  }

  grpc_core::RefCountedPtr<grpc_channel_credentials> alts_creds_;
  grpc_core::RefCountedPtr<grpc_channel_credentials> ssl_creds_;
};

namespace grpc_core {
namespace internal {

typedef bool (*grpc_gce_tenancy_checker)(void);

void set_gce_tenancy_checker_for_testing(grpc_gce_tenancy_checker checker);

// TEST-ONLY. Reset the internal global state.
void grpc_flush_cached_google_default_credentials(void);

}  // namespace internal
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CREDENTIALS_TRANSPORT_GOOGLE_DEFAULT_GOOGLE_DEFAULT_CREDENTIALS_H
